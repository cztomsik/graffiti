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

    // cross-platform windowing
    lib.linkSystemLibrary("glfw3");

    // GL canvas library
    const nanovg = b.createModule(.{ .source_file = .{ .path = "libs/nanovg-zig/src/nanovg.zig" } });
    lib.addModule("nanovg", nanovg);
    lib.addIncludePath("libs/nanovg-zig/lib/gl2/include");
    lib.addCSourceFile("libs/nanovg-zig/lib/gl2/src/glad.c", &.{});
    nanovg_build.addCSourceFiles(lib);

    // layout
    const emlay = b.createModule(.{ .source_file = .{ .path = "libs/emlay/src/main.zig" } });
    lib.addModule("emlay", emlay);

    // JS bindings generator
    const napigen = b.createModule(.{ .source_file = .{ .path = "libs/napigen/napigen.zig" } });
    lib.addModule("napigen", napigen);
    lib.linker_allow_shlib_undefined = true;

    // build .dylib & copy as .node
    b.installArtifact(lib);
    const copy_node_step = b.addInstallLibFile(lib.getOutputSource(), "graffiti.node");
    b.getInstallStep().dependOn(&copy_node_step.step);

    const tests = b.addTest(.{
        .root_source_file = .{ .path = "src/main.zig" },
        .target = target,
        .optimize = optimize,
    });
    tests.main_pkg_path = ".";
    tests.addModule("emlay", emlay);
    tests.addModule("nanovg", nanovg);
    var run_tests = b.addRunArtifact(tests);

    const test_step = b.step("test", "Run library tests");
    test_step.dependOn(&run_tests.step);
}
