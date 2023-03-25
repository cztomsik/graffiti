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
    lib.main_pkg_path = ".";

    // glfw3
    lib.linkSystemLibrary("glfw3");

    // nanovg
    const nanovg = b.createModule(.{ .source_file = .{ .path = "libs/nanovg-zig/src/nanovg.zig" } });
    lib.addModule("nanovg", nanovg);
    lib.addIncludePath("libs/nanovg-zig/lib/gl2/include");
    lib.addCSourceFile("libs/nanovg-zig/lib/gl2/src/glad.c", &.{});
    nanovg_build.addCSourceFiles(lib);

    // napigen
    const napigenModule = b.createModule(.{ .source_file = .{ .path = "libs/napigen/napigen.zig" } });
    lib.addModule("napigen", napigenModule);
    lib.linker_allow_shlib_undefined = true;

    // build .dylib & copy as .node
    lib.install();
    const copy_node_step = b.addInstallLibFile(lib.getOutputSource(), "graffiti.node");
    b.getInstallStep().dependOn(&copy_node_step.step);

    const main_tests = b.addTest(.{
        .root_source_file = .{ .path = "src/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    const test_step = b.step("test", "Run library tests");
    test_step.dependOn(&main_tests.step);
}
