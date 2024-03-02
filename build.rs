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

fn main() {
    #[cfg(feature = "vergen")]
    vergen();
}

