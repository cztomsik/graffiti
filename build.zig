const std = @import("std");
const nanovg = @import("libs/nanovg-zig/build.zig");

pub fn build(b: *std.build.Builder) void {
    const target = b.standardTargetOptions(.{});
    const mode = b.standardReleaseOptions();

    const exe = b.addExecutable("hello-zig", "src/main.zig");
    exe.setTarget(target);
    exe.setBuildMode(mode);
    exe.main_pkg_path = ".";
    // exe.use_stage1 = true;

    add_glfw(exe);
    nanovg.addNanoVGPackage(exe);
    exe.addIncludeDir("libs/nanovg-zig/lib/gl2/include");
    exe.addCSourceFile("libs/nanovg-zig/lib/gl2/src/glad.c", &.{});

    exe.install();

    const run_cmd = exe.run();
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);

    const exe_tests = b.addTest("src/main.zig");
    nanovg.addNanoVGPackage(exe_tests);
    exe_tests.setTarget(target);
    exe_tests.setBuildMode(mode);

    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&exe_tests.step);
}

// TODO: unfortunately it doesn't work for linux cross-compilation (-Dtarget=arm-linux-gnueabi)
//       (LLD Link... ld.lld: error: unable to find library -lglfw)
// TODO: we could zig translate-c glfw headers and keep them in repo (for the purpose of dynamic linking on linux machines)
fn add_glfw(exe: anytype) void {
    switch (exe.target.getOsTag()) {
        .macos => {
            exe.addIncludeDir("libs/glfw/include");

            exe.addCSourceFiles(&.{
                "libs/glfw/src/context.c",
                "libs/glfw/src/init.c",
                "libs/glfw/src/input.c",
                "libs/glfw/src/monitor.c",
                "libs/glfw/src/vulkan.c",
                "libs/glfw/src/window.c",
            }, &[_][]const u8{});

            exe.defineCMacro("_GLFW_COCOA", "1");
            exe.linkFramework("Cocoa");
            exe.linkFramework("IOKit");
            //exe.linkFramework("OpenGL");
            exe.addCSourceFiles(&.{
                "libs/glfw/src/cocoa_init.m",
                "libs/glfw/src/cocoa_joystick.m",
                "libs/glfw/src/cocoa_monitor.m",
                "libs/glfw/src/cocoa_window.m",
                "libs/glfw/src/cocoa_time.c",
                "libs/glfw/src/posix_thread.c",
                "libs/glfw/src/nsgl_context.m",
                "libs/glfw/src/egl_context.c",
                "libs/glfw/src/osmesa_context.c",
            }, &[_][]const u8{});
        },
        else => {
            exe.defineCMacro("GLFW_INCLUDE_NONE", "1");
            exe.linkSystemLibrary("glfw3");
            // exe.linkSystemLibrary("GL");
        },
    }
}
