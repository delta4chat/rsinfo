use std::collections::HashMap;
use std::path::PathBuf;

fn cargo_build_env(env: &HashMap<String, String>) {
    macro_rules! t {
        ($k:expr) => {
            match env.get($k) {
                Some(v) => {
                    println!("cargo:rustc-env=BUILD_RS_{}={}", $k, v);
                }
                None => {
                    println!("cargo:warning=no build env: {:?}", $k);
                }
            }
        }
    }

    t!("TARGET");
    t!("OPT_LEVEL");
    t!("PROFILE");
}

fn list_of_source_code(env: &HashMap<String, String>) {
    let mut list = String::new();
    list.push_str("pub const SOURCE_CODE_LIST: &[(&'static str, &'static str)] = &[\n");

    let mut src = PathBuf::from(env.get("CARGO_MANIFEST_DIR").expect("no manifest dir set"));
    src.push("./src/");

    let src_iter =
    std::fs::read_dir(src)
    .expect("cannot get list of source code");

    for maybe_entry in src_iter {
        let entry = maybe_entry.expect("failed to listing source code");
        if entry.file_type().expect("cannot get file type").is_dir() {
            continue;
        }

        let p = entry.path();
        let p =
            std::fs::canonicalize(p).unwrap();

        let p = p.to_string_lossy();

        list.push_str(format!("( {p:?}").as_str());

        list.push_str(", include_str!(");
        list.push_str(format!("{p:?}").as_str());
        list.push_str(") ),\n");
    }
    list.push_str("];\n");

    let mut out = PathBuf::from(env.get("OUT_DIR").expect("no out dir set"));
    out.push("source-code-list.include.rs");

    std::fs::write(&out, list).expect("cannot write source code list");

    println!("cargo:rustc-env=RSINFO_SOURCE_CODE_LIST={}", out.to_string_lossy());
}

fn main() {
    let env: HashMap<String, String> =
        std::env::vars()
        .map(|(k, v)|{ (k.to_uppercase(), v) })
        .collect();
    cargo_build_env(&env);

    list_of_source_code(&env);
}

