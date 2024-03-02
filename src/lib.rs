#![allow(non_camel_case_types)]

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

    src:    [&'static str; src::SOURCE_CODE_LIST_LEN],
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
        src:    src::SOURCE_CODE_LIST,
    }
}

/// Compile info obtained by `cfg!()`, the compile-time attribute values provided by Rust compiler.
pub mod cfg {
    pub use self::{
        target_arch::cfg_target_arch_info,
        target_feature::cfg_target_feature_info,
        target_os::cfg_target_os_info,
        target_family::cfg_target_family_info,
        target_env::cfg_target_env_info,
        target_endian::cfg_target_endian_info,
        target_pointer_width::cfg_target_pointer_width_info,
        target_has_atomic::cfg_target_has_atomic_info,
        target_vendor::cfg_target_vendor_info,
        panic::cfg_panic_info,
    };

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    pub struct cfg_info {
        pub target_arch:          cfg_target_arch_info,
        pub target_feature:       cfg_target_feature_info,
        pub target_os:            cfg_target_os_info,
        pub target_family:        cfg_target_family_info,
        pub target_env:           cfg_target_env_info,
        pub target_endian:        cfg_target_endian_info,
        pub target_pointer_width: cfg_target_pointer_width_info,
        pub target_has_atomic:    cfg_target_has_atomic_info,
        pub target_vendor:        cfg_target_vendor_info,

        pub panic:            cfg_panic_info,
        pub test:             bool,
        pub debug_assertions: bool,
        pub proc_macro:       bool,
    }
    pub const ALL_INFO: cfg_info = all_info();
    pub const fn all_info() -> cfg_info {
        cfg_info {
            target_arch:          target_arch::ALL_INFO,
            target_feature:       target_feature::ALL_INFO,
            target_os:            target_os::ALL_INFO,
            target_family:        target_family::ALL_INFO,
            target_env:           target_env::ALL_INFO,
            target_endian:        target_endian::ALL_INFO,
            target_pointer_width: target_pointer_width::ALL_INFO,
            target_has_atomic:    target_has_atomic::ALL_INFO,
            target_vendor:        target_vendor::ALL_INFO,

            panic:                panic::ALL_INFO,
            test:                 TEST,
            debug_assertions:     DEBUG_ASSERTIONS,
            proc_macro:           PROC_MACRO,
        }
    }

    pub mod target_arch {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        pub struct cfg_target_arch_info {
            pub x86:       bool,
            pub x86_64:    bool,
            pub mips:      bool,
            pub powerpc:   bool,
            pub powerpc64: bool,
            pub arm:       bool,
            pub aarch64:   bool,
        }
        pub const ALL_INFO: cfg_target_arch_info = all_info();
        pub const fn all_info() -> cfg_target_arch_info {
            cfg_target_arch_info {
                x86:       X86,
                x86_64:    X86_64,
                mips:      MIPS,
                powerpc:   POWERPC,
                powerpc64: POWERPC64,
                arm:       ARM,
                aarch64:   AARCH64,
            }
        }

        pub const X86:       bool = cfg!(target_arch="x86");
        pub const X86_64:    bool = cfg!(target_arch="x86_64");
        pub const MIPS:      bool = cfg!(target_arch="mips");
        pub const POWERPC:   bool = cfg!(target_arch="powerpc");
        pub const POWERPC64: bool = cfg!(target_arch="powerpc64");
        pub const ARM:       bool = cfg!(target_arch="arm");
        pub const AARCH64:   bool = cfg!(target_arch="aarch64");
    }

    pub mod target_feature {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        pub struct cfg_target_feature_info {
            pub avx:        bool,
            pub avx2:       bool,
            pub crt_static: bool,
            pub rdrand:     bool,
            pub sse:        bool,
            pub sse_2:      bool,
            pub sse_4_1:    bool,
        }
        pub const ALL_INFO: cfg_target_feature_info = all_info();
        pub const fn all_info() -> cfg_target_feature_info {
            cfg_target_feature_info {
                avx:        AVX,
                avx2:       AVX2,
                crt_static: CRT_STATIC,
                rdrand:     RDRAND,
                sse:        SSE,
                sse_2:      SSE_2,
                sse_4_1:    SSE_4_1,
            }
        }

        pub const AVX:        bool = cfg!(target_feature="avx");
        pub const AVX2:       bool = cfg!(target_feature="avx2");
        pub const CRT_STATIC: bool = cfg!(target_feature="crt-static");
        pub const RDRAND:     bool = cfg!(target_feature="rdrand");
        pub const SSE:        bool = cfg!(target_feature="sse");
        pub const SSE_2:      bool = cfg!(target_feature="sse2");
        pub const SSE_4_1:    bool = cfg!(target_feature="sse4.1");
    }

    pub mod target_os {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        pub struct cfg_target_os_info {
            pub linux:     bool,
            pub windows:   bool,
            pub macos:     bool,
            pub android:   bool,
            pub ios:       bool,
            pub freebsd:   bool,
            pub dragonfly: bool,
            pub openbsd:   bool,
            pub none:      bool,
        }
        pub const ALL_INFO: cfg_target_os_info = all_info();
        pub const fn all_info() -> cfg_target_os_info {
            cfg_target_os_info {
                linux:     LINUX,
                windows:   WINDOWS,
                macos:     MACOS,
                android:   ANDROID,
                ios:       IOS,
                freebsd:   FREEBSD,
                dragonfly: DRAGONFLY,
                openbsd:   OPENBSD,
                none:      NONE,
            }
        }

        pub const LINUX:     bool = cfg!(target_os="linux");
        pub const WINDOWS:   bool = cfg!(target_os="windows");
        pub const MACOS:     bool = cfg!(target_os="macos");
        pub const ANDROID:   bool = cfg!(target_os="android");
        pub const IOS:       bool = cfg!(target_os="ios");
        pub const FREEBSD:   bool = cfg!(target_os="freebsd");
        pub const DRAGONFLY: bool = cfg!(target_os="dragonfly");
        pub const OPENBSD:   bool = cfg!(target_os="openbsd");
        pub const NONE:      bool = cfg!(target_os="none");
    }

    pub mod target_family {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        pub struct cfg_target_family_info {
            pub unix:    bool,
            pub windows: bool,
            pub wasm:    bool,
        }
        pub const ALL_INFO: cfg_target_family_info = all_info();
        pub const fn all_info() -> cfg_target_family_info {
            cfg_target_family_info {
                unix:    UNIX,
                windows: WINDOWS,
                wasm:    WASM,
            }
        }

        pub const UNIX:    bool = cfg!(target_family="unix");
        pub const WINDOWS: bool = cfg!(target_family="windows");
        pub const WASM:    bool = cfg!(target_family="wasm");
    }

    pub mod target_env {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        pub struct cfg_target_env_info {
            pub gnu:  bool,
            pub musl: bool,
            pub sgx:  bool,
            pub msvc: bool,
        }
        pub const ALL_INFO: cfg_target_env_info = all_info();
        pub const fn all_info() -> cfg_target_env_info {
            cfg_target_env_info {
                gnu:  GNU,
                musl: MUSL,
                sgx:  SGX,
                msvc: MSVC,
            }
        }

        pub const GNU:  bool = cfg!(target_env="gnu");
        pub const MUSL: bool = cfg!(target_env="musl");
        pub const SGX:  bool = cfg!(target_env="sgx");
        pub const MSVC: bool = cfg!(target_env="msvc");
    }

    pub mod target_endian {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        pub struct cfg_target_endian_info {
            pub little: bool,
            pub big:    bool,
        }
        pub const ALL_INFO: cfg_target_endian_info = all_info();
        pub const fn all_info() -> cfg_target_endian_info {
            cfg_target_endian_info {
                little: LITTLE,
                big:    BIG,
            }
        }

        pub const LITTLE: bool = cfg!(target_endian="little");
        pub const BIG:    bool = cfg!(target_endian="big");
    }

    pub mod target_pointer_width {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        pub struct cfg_target_pointer_width_info {
            pub _64: bool,
            pub _32: bool,
            pub _16: bool,
        }
        pub const ALL_INFO: cfg_target_pointer_width_info = all_info();
        pub const fn all_info() -> cfg_target_pointer_width_info {
            cfg_target_pointer_width_info {
                _64,
                _32,
                _16,
            }
        }

        pub const _64: bool = cfg!(target_pointer_width="64");
        pub const _32: bool = cfg!(target_pointer_width="32");
        pub const _16: bool = cfg!(target_pointer_width="16");
    }

    pub mod target_has_atomic {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        pub struct cfg_target_has_atomic_info {
            pub _8:   bool,
            pub _16:  bool,
            pub _32:  bool,
            pub _64:  bool,
            pub _128: bool,
            pub ptr:  bool,
        }
        pub const ALL_INFO: cfg_target_has_atomic_info = all_info();
        pub const fn all_info() -> cfg_target_has_atomic_info {
            cfg_target_has_atomic_info {
                _8,
                _16,
                _32,
                _64,
                _128,
                ptr: PTR,
            }
        }

        pub const _8:   bool = cfg!(target_has_atomic="8");
        pub const _16:  bool = cfg!(target_has_atomic="16");
        pub const _32:  bool = cfg!(target_has_atomic="32");
        pub const _64:  bool = cfg!(target_has_atomic="64");
        pub const _128: bool = cfg!(target_has_atomic="128");
        pub const PTR:  bool = cfg!(target_has_atomic="ptr");
    }

    pub mod target_vendor {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        pub struct cfg_target_vendor_info {
            pub pc:       bool,
            pub apple:    bool,
            pub fortanix: bool,
            pub unknown:  bool,
        }
        pub const ALL_INFO: cfg_target_vendor_info = all_info();
        pub const fn all_info() -> cfg_target_vendor_info {
            cfg_target_vendor_info {
                pc:       PC,
                apple:    APPLE,
                fortanix: FORTANIX,
                unknown:  UNKNOWN,
            }
        }

        pub const PC:       bool = cfg!(target_vendor="pc");
        pub const APPLE:    bool = cfg!(target_vendor="apple");
        pub const FORTANIX: bool = cfg!(target_vendor="fortanix");
        pub const UNKNOWN:  bool = cfg!(target_vendor="unknown");
    }

    pub mod panic {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        pub struct cfg_panic_info {
            pub unwind: bool,
            pub abort:  bool,
        }
        pub const ALL_INFO: cfg_panic_info = all_info();
        pub const fn all_info() -> cfg_panic_info {
            cfg_panic_info {
                unwind: UNWIND,
                abort:  ABORT,
            }
        }

        pub const UNWIND: bool = cfg!(panic="unwind");
        pub const ABORT:  bool = cfg!(panic="abort");
    }

    pub const TEST:             bool = cfg!(test);
    pub const DEBUG_ASSERTIONS: bool = cfg!(debug_assertions);
    pub const PROC_MACRO:       bool = cfg!(proc_macro);
}

/// build info collects by [Vergen](https://docs.rs/vergen).
pub mod vergen {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    pub struct vergen_info {
        pub build_date:      &'static str,
        pub build_timestamp: &'static str,

        pub git_sha:                 &'static str,
        pub git_branch:              &'static str,
        pub git_commit_author_email: &'static str,
        pub git_commit_author_name:  &'static str,
        pub git_commit_count:        &'static str,
        pub git_commit_date:         &'static str,
        pub git_commit_message:      &'static str,
        pub git_commit_timestamp:    &'static str,
        pub git_describe:            &'static str,

        pub cargo_opt_level:     &'static str,
        pub cargo_features:      &'static str,
        pub cargo_debug:         &'static str,
        pub cargo_target_triple: &'static str,

        pub rustc_channel:       &'static str,
        pub rustc_commit_date:   &'static str,
        pub rustc_commit_hash:   &'static str,
        pub rustc_llvm_version:  &'static str,
        pub rustc_semver:        &'static str,
        pub rustc_host_triple:   &'static str,
        pub rustc_target_triple: &'static str,

        pub sysinfo_name:           &'static str,
        pub sysinfo_os_version:     &'static str,
        pub sysinfo_user:           &'static str,
        pub sysinfo_total_memory:   &'static str,
        pub sysinfo_cpu_vendor:     &'static str,
        pub sysinfo_cpu_core_count: &'static str,
        pub sysinfo_cpu_name:       &'static str,
        pub sysinfo_cpu_brand:      &'static str,
        pub sysinfo_cpu_frequency:  &'static str,
    }
    pub const ALL_INFO: vergen_info = all_info();
    pub const fn all_info() -> vergen_info {
        vergen_info {
            build_date:      BUILD_DATE,
            build_timestamp: BUILD_TIMESTAMP,

            git_sha:                 GIT_SHA,
            git_branch:              GIT_BRANCH,
            git_commit_author_email: GIT_COMMIT_AUTHOR_EMAIL,
            git_commit_author_name:  GIT_COMMIT_AUTHOR_NAME,
            git_commit_count:        GIT_COMMIT_COUNT,
            git_commit_date:         GIT_COMMIT_DATE,
            git_commit_message:      GIT_COMMIT_MESSAGE,
            git_commit_timestamp:    GIT_COMMIT_TIMESTAMP,
            git_describe:            GIT_DESCRIBE,

            cargo_opt_level:     CARGO_OPT_LEVEL,
            cargo_features:      CARGO_FEATURES,
            cargo_debug:         CARGO_DEBUG,
            cargo_target_triple: CARGO_TARGET_TRIPLE,

            rustc_channel:       RUSTC_CHANNEL,
            rustc_commit_date:   RUSTC_COMMIT_DATE,
            rustc_commit_hash:   RUSTC_COMMIT_HASH,
            rustc_llvm_version:  RUSTC_LLVM_VERSION,
            rustc_semver:        RUSTC_SEMVER,
            rustc_host_triple:   RUSTC_HOST_TRIPLE,
            rustc_target_triple: RUSTC_TARGET_TRIPLE,

            sysinfo_name:           SYSINFO_NAME,
            sysinfo_os_version:     SYSINFO_OS_VERSION,
            sysinfo_user:           SYSINFO_USER,
            sysinfo_total_memory:   SYSINFO_TOTAL_MEMORY,
            sysinfo_cpu_vendor:     SYSINFO_CPU_VENDOR,
            sysinfo_cpu_core_count: SYSINFO_CPU_CORE_COUNT,
            sysinfo_cpu_name:       SYSINFO_CPU_NAME,
            sysinfo_cpu_brand:      SYSINFO_CPU_BRAND,
            sysinfo_cpu_frequency:  SYSINFO_CPU_FREQUENCY,
        }
    }

    pub const BUILD_DATE:      &'static str = get_env!("VERGEN_BUILD_DATE");
    pub const BUILD_TIMESTAMP: &'static str = get_env!("VERGEN_BUILD_TIMESTAMP");

    pub const GIT_SHA:                 &'static str = get_env!("VERGEN_GIT_SHA");
    pub const GIT_BRANCH:              &'static str = get_env!("VERGEN_GIT_BRANCH");
    pub const GIT_COMMIT_AUTHOR_EMAIL: &'static str = get_env!("VERGEN_GIT_COMMIT_AUTHOR_EMAIL");
    pub const GIT_COMMIT_AUTHOR_NAME:  &'static str = get_env!("VERGEN_GIT_COMMIT_AUTHOR_NAME");
    pub const GIT_COMMIT_COUNT:        &'static str = get_env!("VERGEN_GIT_COMMIT_COUNT");
    pub const GIT_COMMIT_DATE:         &'static str = get_env!("VERGEN_GIT_COMMIT_DATE");
    pub const GIT_COMMIT_MESSAGE:      &'static str = get_env!("VERGEN_GIT_COMMIT_MESSAGE");
    pub const GIT_COMMIT_TIMESTAMP:    &'static str = get_env!("VERGEN_GIT_COMMIT_TIMESTAMP");
    pub const GIT_DESCRIBE:            &'static str = get_env!("VERGEN_GIT_DESCRIBE");

    pub const CARGO_OPT_LEVEL:     &'static str = get_env!("VERGEN_CARGO_OPT_LEVEL");
    pub const CARGO_FEATURES:      &'static str = get_env!("VERGEN_CARGO_FEATURES");
    pub const CARGO_DEBUG:         &'static str = get_env!("VERGEN_CARGO_DEBUG");
    pub const CARGO_TARGET_TRIPLE: &'static str = get_env!("VERGEN_CARGO_TARGET_TRIPLE");

    pub const RUSTC_CHANNEL:       &'static str = get_env!("VERGEN_RUSTC_CHANNEL");
    pub const RUSTC_COMMIT_DATE:   &'static str = get_env!("VERGEN_RUSTC_COMMIT_DATE");
    pub const RUSTC_COMMIT_HASH:   &'static str = get_env!("VERGEN_RUSTC_COMMIT_HASH");
    pub const RUSTC_LLVM_VERSION:  &'static str = get_env!("VERGEN_RUSTC_LLVM_VERSION");
    pub const RUSTC_SEMVER:        &'static str = get_env!("VERGEN_RUSTC_SEMVER");
    pub const RUSTC_HOST_TRIPLE:   &'static str = get_env!("VERGEN_RUSTC_HOST_TRIPLE");
    pub const RUSTC_TARGET_TRIPLE: &'static str = get_env!("VERGEN_CARGO_TARGET_TRIPLE");

    pub const SYSINFO_NAME:           &'static str = get_env!("VERGEN_SYSINFO_NAME");
    pub const SYSINFO_OS_VERSION:     &'static str = get_env!("VERGEN_SYSINFO_OS_VERSION");
    pub const SYSINFO_USER:           &'static str = get_env!("VERGEN_SYSINFO_USER");
    pub const SYSINFO_TOTAL_MEMORY:   &'static str = get_env!("VERGEN_SYSINFO_TOTAL_MEMORY");
    pub const SYSINFO_CPU_VENDOR:     &'static str = get_env!("VERGEN_SYSINFO_CPU_VENDOR");
    pub const SYSINFO_CPU_CORE_COUNT: &'static str = get_env!("VERGEN_SYSINFO_CPU_CORE_COUNT");
    pub const SYSINFO_CPU_NAME:       &'static str = get_env!("VERGEN_SYSINFO_CPU_NAME");
    pub const SYSINFO_CPU_BRAND:      &'static str = get_env!("VERGEN_SYSINFO_CPU_BRAND");
    pub const SYSINFO_CPU_FREQUENCY:  &'static str = get_env!("VERGEN_SYSINFO_CPU_FREQUENCY");
}

/// Environment variables set by Cargo package manager.
pub mod cargo {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    pub struct cargo_info {
        pub manifest_dir: &'static str,
        pub manifest_links: &'static str,
        pub makeflags: &'static str,
        pub pkg_version: &'static str,
        pub pkg_version_major: &'static str,
        pub pkg_version_minor: &'static str,
        pub pkg_version_patch: &'static str,
        pub pkg_version_pre: &'static str,
        pub pkg_authors: &'static str,
        pub pkg_name: &'static str,
        pub pkg_description: &'static str,
        pub pkg_homepage: &'static str,
        pub pkg_repository: &'static str,
        pub pkg_license: &'static str,
        pub pkg_license_file: &'static str,
        pub pkg_rust_version: &'static str,
        pub pkg_readme: &'static str,
        pub crate_name: &'static str,
        pub bin_name: &'static str,
        pub out_dir: &'static str,
        pub primary_package: &'static str,
        pub target_tmpdir: &'static str,
        pub rustc_current_dir: &'static str,
    }
    pub const ALL_INFO: cargo_info = all_info();
    pub const fn all_info() -> cargo_info {
        cargo_info {
            manifest_dir: MANIFEST_DIR,
            manifest_links: MANIFEST_LINKS,
            makeflags: MAKEFLAGS,
            pkg_version: PKG_VERSION,
            pkg_version_major: PKG_VERSION_MAJOR,
            pkg_version_minor: PKG_VERSION_MINOR,
            pkg_version_patch: PKG_VERSION_PATCH,
            pkg_version_pre: PKG_VERSION_PRE,
            pkg_authors: PKG_AUTHORS,
            pkg_name: PKG_NAME,
            pkg_description: PKG_DESCRIPTION,
            pkg_homepage: PKG_HOMEPAGE,
            pkg_repository: PKG_REPOSITORY,
            pkg_license: PKG_LICENSE,
            pkg_license_file: PKG_LICENSE_FILE,
            pkg_rust_version: PKG_RUST_VERSION,
            pkg_readme: PKG_README,
            crate_name: CRATE_NAME,
            bin_name: BIN_NAME,
            out_dir: OUT_DIR,
            primary_package: PRIMARY_PACKAGE,
            target_tmpdir: TARGET_TMPDIR,
            rustc_current_dir: RUSTC_CURRENT_DIR,
        }
    }

    pub const MANIFEST_DIR: &'static str = get_env!("CARGO_MANIFEST_DIR");
    pub const MANIFEST_LINKS: &'static str = get_env!("CARGO_MANIFEST_LINKS");
    pub const MAKEFLAGS: &'static str = get_env!("CARGO_MAKEFLAGS");

    pub const PKG_VERSION: &'static str = get_env!("CARGO_PKG_VERSION");

    pub const PKG_VERSION_MAJOR: &'static str = get_env!("CARGO_PKG_VERSION_MAJOR");
    pub const PKG_VERSION_MINOR: &'static str = get_env!("CARGO_PKG_VERSION_MINOR");
    pub const PKG_VERSION_PATCH: &'static str = get_env!("CARGO_PKG_VERSION_PATCH");
    pub const PKG_VERSION_PRE: &'static str = get_env!("CARGO_PKG_VERSION_PRE");

    pub const PKG_AUTHORS: &'static str = get_env!("CARGO_PKG_AUTHORS");
    pub const PKG_NAME: &'static str = get_env!("CARGO_PKG_NAME");
    pub const PKG_DESCRIPTION: &'static str = get_env!("CARGO_PKG_DESCRIPTION");
    pub const PKG_HOMEPAGE: &'static str = get_env!("CARGO_PKG_HOMEPAGE");
    pub const PKG_REPOSITORY: &'static str = get_env!("CARGO_PKG_REPOSITORY");
    pub const PKG_LICENSE: &'static str = get_env!("CARGO_PKG_LICENSE");
    pub const PKG_LICENSE_FILE: &'static str = get_env!("CARGO_PKG_LICENSE_FILE");
    pub const PKG_RUST_VERSION: &'static str = get_env!("CARGO_PKG_RUST_VERSION");
    pub const PKG_README: &'static str = get_env!("CARGO_PKG_README");
    pub const CRATE_NAME: &'static str = get_env!("CARGO_CRATE_NAME");
    pub const BIN_NAME: &'static str = get_env!("CARGO_BIN_NAME");

    pub const OUT_DIR: &'static str = get_env!("OUT_DIR");
    pub const PRIMARY_PACKAGE: &'static str = get_env!("CARGO_PRIMARY_PACKAGE");
    pub const TARGET_TMPDIR: &'static str = get_env!("CARGO_TARGET_TMPDIR");
    pub const RUSTC_CURRENT_DIR: &'static str = get_env!("CARGO_RUSTC_CURRENT_DIR");
}

/// Other info get from environment variables.
/// this is very platform-specified.
pub mod env {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    pub struct env_info {
        pub cargo: &'static str,
        pub rustc: &'static str,
        pub rustflags: &'static str,
        pub cc: &'static str,
        pub cflags: &'static str,
        pub path: &'static str,
        pub pwd: &'static str,
        pub home: &'static str,
        pub shell: &'static str,
        pub user: &'static str,
        pub prefix: &'static str,
        pub tmpdir: &'static str,
        pub ld_preload: &'static str,
        pub ld_library_path: &'static str,
        pub term: &'static str,
        pub colorterm: &'static str,
        pub android_root: &'static str,
        pub dex2oatbootclasspath: &'static str,
        pub dyld_fallback_library_path: &'static str,
        pub libpath: &'static str,
    }
    pub const ALL_INFO: env_info = all_info();
    pub const fn all_info() -> env_info {
        env_info {
            cargo: CARGO,
            rustc: RUSTC,
            rustflags: RUSTFLAGS,
            cc: CC,
            cflags: CFLAGS,
            path: PATH,
            pwd: PWD,
            home: HOME,
            shell: SHELL,
            user: USER,
            prefix: PREFIX,
            tmpdir: TMPDIR,
            ld_preload: LD_PRELOAD,
            ld_library_path: LD_LIBRARY_PATH,
            term: TERM,
            colorterm: COLORTERM,
            android_root: ANDROID_ROOT,
            dex2oatbootclasspath: DEX2OATBOOTCLASSPATH,
            dyld_fallback_library_path: DYLD_FALLBACK_LIBRARY_PATH,
            libpath: LIBPATH,
        }
    }

    /* Compiler Path & Flags */
    pub const CARGO: &'static str = get_env!("CARGO");
    pub const RUSTC: &'static str = get_env!("RUSTC");
    pub const RUSTFLAGS: &'static str = get_env!("RUSTFLAGS");
    pub const CC: &'static str = get_env!("CC");
    pub const CFLAGS: &'static str = get_env!("CFLAGS");

    /* General POSIX/UNIX environment */
    pub const PATH: &'static str = get_env!("PATH");
    pub const PWD: &'static str = get_env!("PWD");
    pub const HOME: &'static str = get_env!("HOME");
    pub const SHELL: &'static str = get_env!("SHELL");
    pub const USER: &'static str = get_env!("USER");
    pub const PREFIX: &'static str = get_env!("PREFIX");
    pub const TMPDIR: &'static str = get_env!("TMPDIR");
    pub const LD_PRELOAD: &'static str = get_env!("LD_PRELOAD");
    pub const LD_LIBRARY_PATH: &'static str = get_env!("LD_LIBRARY_PATH");
    pub const TERM: &'static str = get_env!("TERM");
    pub const COLORTERM: &'static str = get_env!("COLORTERM");

    // Android.
    pub const ANDROID_ROOT: &'static str = get_env!("ANDROID_ROOT");
    pub const DEX2OATBOOTCLASSPATH: &'static str = get_env!("DEX2OATBOOTCLASSPATH");

    // Mac OS X (darwin)
    pub const DYLD_FALLBACK_LIBRARY_PATH: &'static str = get_env!("DYLD_FALLBACK_LIBRARY_PATH");

    // AIX
    pub const LIBPATH: &'static str = get_env!("LIBPATH");
}

pub mod src {
    include!(env!("RSINFO_SOURCE_CODE_LIST"));
}

