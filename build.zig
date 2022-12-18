const std = @import("std");
const nanovg = @import("libs/nanovg-zig/build.zig");

pub fn build(b: *std.build.Builder) void {
    const mode = b.standardReleaseOptions();
    const target = b.standardTargetOptions(.{});

    //const lib = b.addSharedLibrary("graffiti", "src/main.zig", .unversioned);
    const lib = b.addExecutable("hello-graffiti", "src/hello.zig");
    lib.setBuildMode(mode);
    lib.setTarget(target);
    lib.main_pkg_path = ".";
    lib.linkSystemLibrary("glfw3");
    nanovg.addNanoVGPackage(lib);
    lib.addIncludePath("libs/nanovg-zig/lib/gl2/include");
    lib.addCSourceFile("libs/nanovg-zig/lib/gl2/src/glad.c", &.{});
    //lib.linker_allow_shlib_undefined = true;
    //lib.addPackagePath("napigen", "libs/napigen/napigen.zig");
    lib.install();

    const run_cmd = lib.run();
    run_cmd.step.dependOn(b.getInstallStep());
    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);

    // copy result to a fixed filename with .node suffix
    // TODO: is this the way how to do such thing?
    //b.installLibFile(b.pathJoin(&.{ "zig-out/lib", lib.out_lib_filename }), "graffiti.node");

    const main_tests = b.addTest("src/main.zig");
    main_tests.setBuildMode(mode);

    const test_step = b.step("test", "Run library tests");
    test_step.dependOn(&main_tests.step);
}
