use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let target_arch = env("CARGO_CFG_TARGET_ARCH");
    let target_features = env("CARGO_CFG_TARGET_FEATURE");
    let target_features = target_features.split(',').collect::<Vec<_>>();
    let feature = |s: &str| target_features.iter().any(|&f| f == s);
    let target = match target_arch.as_str() {
        "x86_64" => {
            if feature("avx512f") {
                "AVX512"
            } else if feature("avx2") {
                "AVX2"
            } else if feature("avx") {
                "AVX"
            } else if feature("ssse3") {
                "SSSE3"
            } else {
                "generic64"
            }
        }
        "armv6" => "ARMv6",
        "armv7a" => "ARMv7A",
        "armv7" => "ARMv7M",
        "aarch64" => "ARMv8A",
        _ => match env("CARGO_CFG_TARGET_POINTER_WIDTH").as_str() {
            "64" => "generic64",
            "32" => "generic32",
            width => {
                eprintln!("no specialized target for pointer_width={width:?}");
                "compact"
            }
        },
    };

    let root = PathBuf::from(env("CARGO_MANIFEST_DIR"));
    let out_dir = PathBuf::from(env("OUT_DIR"));
    let xkcp_from = root.join("XKCP");
    rerun_if_changed(xkcp_from.to_str().unwrap());

    // copy XKCP to OUT_DIR
    let xkcp = out_dir.join("XKCP");
    cp_r(&xkcp_from, &xkcp);

    // build
    let status = Command::new("make")
        .current_dir(&xkcp)
        .arg(format!("{target}/libXKCP.a"))
        .status()
        .unwrap();
    if !status.success() {
        panic!("make failed");
    }

    // link
    let xkcp_out = xkcp.join("bin").join(target);
    println!("cargo:rustc-link-search={}", xkcp_out.display());
    println!("cargo:rustc-link-lib=static=XKCP");

    // bindgen
    let mut builder = bindgen::builder();
    let headers = xkcp_out.join("libXKCP.a.headers");
    for header in headers.read_dir().unwrap() {
        let path = header.unwrap().path();
        let stem = path.file_stem().unwrap().to_str().unwrap();
        if stem.ends_with("-reference") {
            continue;
        }
        // TODO: duplicate enumerators??
        if stem.starts_with("Xoofff") {
            continue;
        }
        builder = builder.header(path.to_str().unwrap());
    }
    builder
        .formatter(bindgen::Formatter::Prettyplease)
        .use_core()
        .derive_copy(false)
        .size_t_is_usize(true)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .generate()
        .unwrap()
        .write_to_file(out_dir.join("bindings.rs"))
        .unwrap();
}

fn rerun_if_changed(path: &str) {
    println!("cargo:rerun-if-changed={path}");
}

fn env(s: &str) -> String {
    println!("cargo:rerun-if-env-changed={s}");
    env::var(s).unwrap()
}

fn cp_r(from: &Path, to: &Path) {
    if to.file_name() == Some(".git".as_ref()) {
        return;
    }

    fs::create_dir_all(&to).unwrap();
    for entry in from.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let name = path.file_name().unwrap();
        let to = to.join(name);
        if path.is_dir() {
            fs::create_dir_all(&to).unwrap();
            cp_r(&path, &to);
        } else {
            fs::copy(&path, &to).unwrap();
        }
    }
}
