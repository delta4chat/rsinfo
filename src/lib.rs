#![allow(non_camel_case_types)]

//use serde::{Serialize};

macro_rules! get_env {
    ($k:tt) => {
        match option_env!($k) {
            Some(v) => v,
            None    => "",
        }
    }
}


/// Compile info obtained by `cfg!()`, the compile-time attribute values provided by Rust compiler.
pub mod cfg;
pub use cfg::cfg_info;

/// build info collects by [Vergen](https://docs.rs/vergen).
pub mod vergen;

/// Environment variables set by Cargo package manager.
pub mod cargo;

/// Other info get from environment variables.
/// this is very platform-specified.
pub mod env;

/// source code info (only rsinfo itself)
pub mod source;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct rsinfo {
    pub cfg:    cfg::cfg_info,
    pub vergen: vergen::vergen_info,
    pub cargo:  cargo::cargo_info,
    pub env:    env::env_info,

    pub source:    &'static [(&'static str, &'static str)],
}
#[cfg(feature = "json")]
impl rsinfo {
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "rsinfo": option_env!("CARGO_PKG_VERSION").unwrap_or("(N/A)"),
            "data": {
                "cfg": self.cfg.to_json(),
                "vergen": self.vergen.to_json(),
                "cargo": self.cargo.to_json(),
                "env": self.env.to_json(),
                "source": serde_json::json!(self.source),
            },
        })
    }
}
impl core::fmt::Debug for rsinfo {
    fn fmt(&self, f: &mut core::fmt::Formatter)
        -> Result<(), core::fmt::Error>
    {
        f.debug_struct("rsinfo")
            .field("cfg", &self.cfg)
            .field("vergen", &self.vergen)
            .field("cargo", &self.cargo)
            .field("env", &self.env)
            .field("source", &self.source)
            .finish()
    }
}

pub fn all_info() -> rsinfo {
    rsinfo {
        cfg:    cfg::ALL_INFO,
        vergen: vergen::all_info(),
        cargo:  cargo::ALL_INFO,
        env:    env::ALL_INFO,
        source: source::SOURCE_CODE_LIST,
    }
}

#[macro_export]
macro_rules! vergen {
    () => {
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
            eprintln!("unable to obtain 'vergen build info': {e:?}");
        }
    }
}
