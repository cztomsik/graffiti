const std = @import("std");
const nanovg = @import("libs/nanovg-zig/build.zig");

pub fn build(b: *std.build.Builder) void {
    const mode = b.standardReleaseOptions();
    const target = b.standardTargetOptions(.{});

    const lib = b.addSharedLibrary("graffiti", "src/main.zig", .unversioned);
    lib.setBuildMode(mode);
    lib.setTarget(target);
    // lib.use_stage1 = true;
    lib.main_pkg_path = ".";
    lib.linkSystemLibrary("glfw3");
    nanovg.addNanoVGPackage(lib);
    lib.addIncludePath("libs/nanovg-zig/lib/gl2/include");
    lib.addCSourceFile("libs/nanovg-zig/lib/gl2/src/glad.c", &.{});
    lib.linker_allow_shlib_undefined = true;
    lib.addPackagePath("napigen", "libs/napigen/napigen.zig");
    lib.install();

    const main_tests = b.addTest("src/main.zig");
    main_tests.setBuildMode(mode);

    const test_step = b.step("test", "Run library tests");
    test_step.dependOn(&main_tests.step);
}
