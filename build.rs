use std::collections::HashMap;
use std::path::PathBuf;

#[cfg(feature = "vergen")]
fn vergen() {
    let mut b = vergen::EmitBuilder::builder();

    #[cfg(feature = "vergen-build")]
    {
        b = *b.all_build();
    }

    #[cfg(feature = "vergen-cargo")]
    {
        b = *b.all_cargo();
    }

    #[cfg(feature = "vergen-git")]
    {
        b = *b.all_git();
    }

    #[cfg(feature = "vergen-rustc")]
    {
        b = *b.all_rustc();
    }

    #[cfg(feature = "vergen-si")]
    {
        b = *b.all_sysinfo();
    }

    if let Err(e) = b.emit() {
        println!("cargo:warning=unable to obtain 'vergen build info': {e:?}");
    }
}

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
    list.push_str("pub const SOURCE_CODE_LIST: [&'static str; SOURCE_CODE_LIST_LEN] = [\n");

    let mut src = PathBuf::from(env.get("CARGO_MANIFEST_DIR").expect("no manifest dir set"));
    src.push("./src/");

    let src_iter =
    std::fs::read_dir(src)
    .expect("cannot get list of source code");

    let mut len: usize = 0;
    for entry in src_iter {
        let p = entry.expect("failed to listing source code").path();
        let p =
            std::fs::canonicalize(p).unwrap();

        list.push_str("include_str!(");
        list.push('"');
        list.push_str( &format!("{}", p.to_string_lossy()) );
        list.push('"');
        list.push_str("),\n");

        len += 1;
    }
    list.push_str("];\n");
    list.push_str(&format!("pub const SOURCE_CODE_LIST_LEN: usize = {len};\n"));

    let mut out = PathBuf::from(env.get("OUT_DIR").expect("no out dir set"));
    out.push("source-code-list.include.rs");

    std::fs::write(&out, list)
    .expect("cannot write source code list");

    println!("cargo:rustc-env=RSINFO_SOURCE_CODE_LIST={}", out.to_string_lossy());
}

fn main() {
    let env: HashMap<String, String> =
        std::env::vars()
        .map(|(k, v)|{ (k.to_uppercase(), v) })
        .collect();
    cargo_build_env(&env);

    list_of_source_code(&env);

    #[cfg(feature = "vergen")]
    vergen();
}

