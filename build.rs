fn main() {
    // NOTE: This will output everything, and requires all features enabled.
    // NOTE: See the EmitBuilder documentation for configuration options.
    if let Err(e) =
        vergen::EmitBuilder::builder()
        .all_build()
        .all_cargo()
        .all_git()
        .all_rustc()
        .all_sysinfo()
        .emit()
    {
        println!("cargo:warning=unable to obtain 'vergen build info': {e:?}");
    }
}
