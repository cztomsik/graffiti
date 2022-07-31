const std = @import("std");
const nanovg = @import("nanovg-zig/build.zig");

pub fn build(b: *std.build.Builder) void {
    const target = b.standardTargetOptions(.{});
    const mode = b.standardReleaseOptions();

    const exe = b.addExecutable("hello-zig", "src/main.zig");
    exe.setTarget(target);
    exe.setBuildMode(mode);

    add_glfw(exe);
    nanovg.addNanoVGPackage(exe);
    exe.addIncludeDir("nanovg-zig/lib/gl2/include");
    exe.addCSourceFile("nanovg-zig/lib/gl2/src/glad.c", &.{});

    exe.install();

    const run_cmd = exe.run();
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);

    const exe_tests = b.addTest("src/main.zig");
    exe_tests.setTarget(target);
    exe_tests.setBuildMode(mode);

    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&exe_tests.step);
}

// TODO: unfortunately it doesn't work for linux cross-compilation because
//       glfw3.h is then trying to find GL/gl.h which doesn't exists on macos
//       (-Dtarget=arm-linux-gnueabi)
fn add_glfw(exe: anytype) void {
    switch (exe.target.getOsTag()) {
        .macos => {
            exe.addIncludeDir("graffiti-glfw/glfw/include");

            exe.addCSourceFiles(&.{
                "graffiti-glfw/glfw/src/context.c",
                "graffiti-glfw/glfw/src/init.c",
                "graffiti-glfw/glfw/src/input.c",
                "graffiti-glfw/glfw/src/monitor.c",
                "graffiti-glfw/glfw/src/vulkan.c",
                "graffiti-glfw/glfw/src/window.c",
            }, &[_][]const u8{});

            exe.defineCMacro("_GLFW_COCOA", "1");
            exe.linkFramework("Cocoa");
            exe.linkFramework("IOKit");
            //exe.linkFramework("OpenGL");
            exe.addCSourceFiles(&.{
                "graffiti-glfw/glfw/src/cocoa_init.m",
                "graffiti-glfw/glfw/src/cocoa_joystick.m",
                "graffiti-glfw/glfw/src/cocoa_monitor.m",
                "graffiti-glfw/glfw/src/cocoa_window.m",
                "graffiti-glfw/glfw/src/cocoa_time.c",
                "graffiti-glfw/glfw/src/posix_thread.c",
                "graffiti-glfw/glfw/src/nsgl_context.m",
                "graffiti-glfw/glfw/src/egl_context.c",
                "graffiti-glfw/glfw/src/osmesa_context.c",
            }, &[_][]const u8{});
        },
        else => {
            exe.linkSystemLibrary("glfw3");
            exe.linkSystemLibrary("GL");
        },
    }
}
