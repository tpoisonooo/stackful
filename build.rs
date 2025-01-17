use std::env;

fn main() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let file = match &*target_arch {
        "x86_64" => "src/x86_64.s",
        "aarch64" => "src/aarch64.s",
        _ => {
            panic!("Current architecture is not supported");
        }
    };
    cc::Build::new().file(file).compile("stackful");
}
