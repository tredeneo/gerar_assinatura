use slint_build::CompilerConfiguration;

fn main() {
    if std::env::var("PROFILE").unwrap() == "release" {
        println!("cargo:rustc-cfg=build_release");
    }
    slint_build::compile_with_config(
        "ui/main.slint",
        CompilerConfiguration::new().with_style("cosmic".to_string()),
    )
    .unwrap();
}
