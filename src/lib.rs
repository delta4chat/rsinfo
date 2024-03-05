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

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct rsinfo {
    cfg:    cfg::cfg_info,
    vergen: vergen::vergen_info,
    cargo:  cargo::cargo_info,
    env:    env::env_info,

    source:    [&'static str; source::SOURCE_CODE_LIST_LEN],
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
            .finish()
    }
}

pub const ALL_INFO: rsinfo = all_info();
pub const fn all_info() -> rsinfo {
    rsinfo {
        cfg:    cfg::ALL_INFO,
        vergen: vergen::ALL_INFO,
        cargo:  cargo::ALL_INFO,
        env:    env::ALL_INFO,
        source:    source::SOURCE_CODE_LIST,
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

/// source code info
pub mod source;
