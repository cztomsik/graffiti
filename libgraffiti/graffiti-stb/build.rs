use cc::Build;

fn main() {
    Build::new()
        .cpp(false)

        // optim
        .flag("-O3")

        .file("stb.c")
        .compile("libstb_image.a");
}
