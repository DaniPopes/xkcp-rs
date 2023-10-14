use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    need_command("make", Some("need GNU make"));
    need_command("xsltproc", Some("install xsltproc or libxslt"));

    let target_arch = env("CARGO_CFG_TARGET_ARCH");
    let target_features = env("CARGO_CFG_TARGET_FEATURE");
    let target_features = target_features.split(',').collect::<Vec<_>>();
    eprintln!("target features: {target_features:?}");
    let feature = |s: &str| target_features.iter().any(|&f| f == s);
    let mut xkcp_target = match target_arch.as_str() {
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
        "avr" => "AVR8",
        _ => "compact",
    };
    eprintln!("initial XKCP target: {xkcp_target:?}");

    if cfg!(feature = "avr8") {
        eprintln!("AVR8 XKCP target forced");
        xkcp_target = "AVR8";
    } else if cfg!(feature = "force-compact") {
        eprintln!("compact XKCP target forced");
        xkcp_target = "compact";
    } else if cfg!(feature = "force-generic") || xkcp_target == "compact" {
        let w = if cfg!(feature = "force-generic") {
            "forced"
        } else {
            "selected"
        };
        eprintln!("generic XKCP target {w}");
        xkcp_target = match env("CARGO_CFG_TARGET_POINTER_WIDTH").as_str() {
            "64" => "generic64",
            "32" => "generic32",
            width => {
                eprintln!("no specialized target for pointer_width={width:?}");
                "compact"
            }
        };
    }

    if cfg!(feature = "generic-lc") && xkcp_target.starts_with("generic") {
        eprintln!("generic-lc XKCP target");
        xkcp_target = match xkcp_target {
            "generic32" => "generic32lc",
            "generic64" => "generic64lc",
            _ => unreachable!("expected `generic{{32,64}}` target, got {xkcp_target:?}"),
        };
    }

    let xkcp_target = if let Ok(target) = env_opt("XKCP_RS_TARGET") {
        eprintln!("XKCP target overridden by environment variable");
        target
    } else {
        xkcp_target.to_string()
    };

    let root = PathBuf::from(env("CARGO_MANIFEST_DIR"));
    let out_dir = PathBuf::from(env("OUT_DIR"));
    let xkcp_from = root.join("XKCP");
    rerun_if_changed(xkcp_from.to_str().unwrap());

    // copy XKCP to OUT_DIR
    let xkcp = out_dir.join("XKCP");
    fs::remove_dir_all(&xkcp).unwrap();
    cp_r(&xkcp_from, &xkcp);

    // build
    eprintln!("final XKCP target: {xkcp_target:?}");
    patch_makefiles(&xkcp);
    let build = cc::Build::new();
    let cc = build.get_compiler();
    let ar = build.get_archiver();
    let status = Command::new("make")
        .arg(format!("{xkcp_target}/libXKCP.a"))
        .current_dir(&xkcp)
        .env("CC", cc.to_command().get_program())
        .env("CFLAGS", cc.cflags_env())
        .env("AR", ar.get_program())
        .status()
        .unwrap();
    if !status.success() {
        panic!("make failed");
    }

    // link
    let xkcp_out = xkcp.join("bin").join(xkcp_target);
    println!("cargo:rustc-link-search={}", xkcp_out.display());
    println!("cargo:rustc-link-lib=static=XKCP");

    // bindgen
    eprintln!("generating bindings");
    let mut builder = bindgen::builder();
    let headers = xkcp_out.join("libXKCP.a.headers");
    for header in headers.read_dir().unwrap() {
        let path = header.unwrap().path();
        let stem = path.file_stem().unwrap().to_str().unwrap();
        if stem.ends_with("-reference") {
            continue;
        }
        // TODO: duplicate enumerators?
        if stem.starts_with("Xoofff") {
            continue;
        }
        let path = path.to_str().unwrap();
        builder = builder.header(path).allowlist_file(path);
    }
    builder = builder
        .formatter(bindgen::Formatter::Prettyplease)
        .use_core()
        .derive_copy(false)
        .size_t_is_usize(true)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .layout_tests(false)
        .merge_extern_blocks(true)
        .generate_cstr(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    builder
        .generate()
        .unwrap()
        .write_to_file(out_dir.join("bindings.rs"))
        .unwrap();
}

fn rerun_if_changed(path: &str) {
    println!("cargo:rerun-if-changed={path}");
}

#[track_caller]
fn env(s: &str) -> String {
    env_opt(s).unwrap()
}

#[track_caller]
fn env_opt(s: &str) -> Result<String, env::VarError> {
    println!("cargo:rerun-if-env-changed={s}");
    env::var(s)
}

fn cp_r(from: &Path, to: &Path) {
    if from.to_str().unwrap().ends_with("XKCP/bin") {
        return;
    }
    if to.file_name() == Some(".git".as_ref()) {
        return;
    }

    fs::create_dir_all(to).unwrap();
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

fn need_command(s: &str, sugg: Option<&str>) {
    match Command::new(s).output() {
        Ok(_) => {}
        Err(_) => {
            panic!(
                "command {s} not found{}",
                sugg.map_or_else(String::new, |s| format!(": {s}"))
            );
        }
    }
}

/// Patches Makefiles to not unconditionaly enable optimizations.
fn patch_makefiles(xkcp: &Path) {
    eprintln!("out dir {}", xkcp.display());
    let low_level_build = xkcp.join("lib/LowLevel.build");
    let mut contents = fs::read_to_string(&low_level_build).unwrap();

    const TO_REMOVE: &[&str] = &[
        "<gcc>-fomit-frame-pointer</gcc>",
        "<gcc>-O2</gcc>",
        "<gcc>-g0</gcc>",
        "<gcc>-march=native</gcc>",
        "<gcc>-mtune=native</gcc>",
    ];
    for &to_remove in TO_REMOVE {
        contents.find(to_remove).unwrap();
        contents = contents.replace(to_remove, "");
    }

    fs::write(low_level_build, contents).unwrap();
}
