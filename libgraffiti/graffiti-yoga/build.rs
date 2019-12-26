use cc::Build;
use std::process::Command;

// based on:
// - https://github.com/facebook/yoga/blob/master/BUCK
// - https://github.com/facebook/yoga/blob/master/tools/build_defs/oss/yoga_defs.bzl
fn main() {
    Command::new("git")
        .args(&["submodule", "init"])
        .status()
        .expect("git submodule init");

    Command::new("git")
        .args(&["submodule", "update"])
        .status()
        .expect("git submodule update");

    Build::new()
        .cpp(true)
        .flag("-fno-omit-frame-pointer")
        .flag("-fexceptions")
        .flag("-std=c++1y")
        .flag("-fPIC")

        // https://clang.llvm.org/docs/UsersManual.html#id14
        //.flag("-Wall")
        //.flag("-Werror")
        // no warns
        .flag("-w")

        // optim
        .flag("-O3")
        // so that #include <yoga/*> works
        .include("yoga")
        // all c++ files in yoga dir
        .file("yoga/yoga/Utils.cpp")
        .file("yoga/yoga/YGConfig.cpp")
        .file("yoga/yoga/YGEnums.cpp")
        .file("yoga/yoga/YGLayout.cpp")
        .file("yoga/yoga/YGNode.cpp")
        //.file("yoga/yoga/YGNodePrint.cpp")
        .file("yoga/yoga/YGStyle.cpp")
        .file("yoga/yoga/YGValue.cpp")
        .file("yoga/yoga/Yoga.cpp")
        .file("yoga/yoga/log.cpp")
        .file("yoga/yoga/event/event.cpp")
        .compile("libyoga.a");
}
