const std = @import("std");
const nanovg_build = @import("libs/nanovg-zig/build.zig");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const lib = b.addSharedLibrary(.{
        .name = "graffiti",
        .root_source_file = .{ .path = "src/main.zig" },
        .target = target,
        .optimize = optimize,
    });
    // lib.use_stage1 = true;
    lib.main_pkg_path = ".";
    lib.linkSystemLibrary("glfw3");
    nanovg_build.addCSourceFiles(lib);
    lib.addIncludePath("libs/nanovg-zig/lib/gl2/include");
    lib.addCSourceFile("libs/nanovg-zig/lib/gl2/src/glad.c", &.{});
    lib.linker_allow_shlib_undefined = true;
    const napigenModule = b.createModule(.{ .source_file = .{ .path = "libs/napigen/napigen.zig" } });
    lib.addModule("napigen", napigenModule);
    const nanovg = b.createModule(.{ .source_file = .{ .path = "libs/nanovg-zig/src/nanovg.zig" } });
    lib.addModule("nanovg", nanovg);
    lib.install();

    // copy result to a fixed filename with .node suffix
    // TODO: is this the way how to do such thing?
    b.installLibFile(b.pathJoin(&.{ "zig-out/lib", lib.out_lib_filename }), "graffiti.node");

    const main_tests = b.addTest(.{
        .root_source_file = .{ .path = "src/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    const test_step = b.step("test", "Run library tests");
    test_step.dependOn(&main_tests.step);
}
