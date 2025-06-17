use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    #[cfg(unix)]
    create_cbindgen_header();
    create_engine_build_def();
}

fn create_cbindgen_header() {
    let config = cbindgen::Config::from_file("cbindgen.toml").unwrap();
    cbindgen::Builder::new()
        .with_crate(env::var("CARGO_MANIFEST_DIR").unwrap())
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("../examples/c_demo/ffi_client.h");
}

fn create_engine_build_def() {
    // 每次构建都更新
    println!("cargo:rerun-if-changed=src/build_info.rs");

    let dest_path = Path::new("src/build_info.rs");
    let mut f = File::create(&dest_path).expect("Problem creating the file");
    let version = env::var("CARGO_PKG_VERSION").unwrap();

    let mut write_line = |line: &str| {
        writeln!(f, "{}", line).expect("Could not write to file");
    };
    write_line(&format!("pub const RUST_SDK_VER: &str = \"{}\";", version));

    // 获取 Git commit ID
    let git_commit = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .map(|out| String::from_utf8_lossy(&out.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    write_line(&format!(
        "pub const RUST_SDK_COMMIT: &str = \"{}\";",
        git_commit
    ));

    // 获取构建时间（UTC 北京时间）
    let build_time = chrono::Utc::now()
        .with_timezone(&chrono::FixedOffset::east_opt(8 * 3600).unwrap())
        .to_rfc3339();
    write_line(&format!(
        "pub const RUST_SDK_BUILD_TIME: &str = \"{}\";",
        build_time
    ));

    // 获取构建目标 target，例如 aarch64-apple-ios
    let target = env::var("TARGET").unwrap_or_else(|_| "unknown".to_string());
    write_line(&format!(
        "pub const RUST_SDK_TARGET: &str = \"{}\";",
        target
    ));

    // 编译信息
    // 把编译信息写入 sdk，可以通过命令行直接查看 sdk 信息
    // ```
    // # 编译
    // cargo build -p my_lib
    // # 查看信息:
    // strings target/debug/libmy_lib.a | grep my_lib_version
    // # 输出示例：
    // {"my_lib_version":"0.1.0","commit":"9c13add","build_time":"2025-05-27T14:59:22.667099+08:00"}
    // ```
    let json_info = format!(
        r#"
{{"my_lib_version":"{version}","commit":"{commit}","build_time":"{time}","target":"{target}"}}
        "#,
        version = version,
        commit = git_commit,
        time = build_time,
        target = target,
    );

    write_line("#[used]");
    write_line(&format!(
        "pub static RUST_SDK_BUILD_INFO: &str = r#\"{}\"#;",
        json_info
    ));
}
