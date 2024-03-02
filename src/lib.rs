#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct rsinfo {
    cfg:    cfg::cfg_info,
    vergen: vergen::vergen_info,
}

pub const ALL_INFO: rsinfo = all_info();
pub const all_info() -> rsinfo {
    rsinfo {
        cfg:    cfg::ALL_INFO,
        vergen: vergen::ALL_INFO,
    }
}

pub mod cfg {
    pub use target_arch::cfg_target_arch_info;
    pub use target_feature::cfg_target_feature_info;
    pub use target_os::cfg_target_os_info;
    pub use target_family::cfg_target_family_info;
    pub use target_env::cfg_target_env_info;
    pub use target_endian::cfg_target_endian_info;
    pub use target_pointer_width::cfg_target_pointer_width_info;
    pub use target_has_atomic::cfg_target_has_atomic_info;
    pub use target_vendor::cfg_target_vendor_info;
    pub use panic::cfg_panic_info;

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

pub mod vergen {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    pub struct vergen_info {
        pub build_date:      Option<&'static str>,
        pub build_timestamp: Option<&'static str>,

        pub git_sha:                 Option<&'static str>,
        pub git_branch:              Option<&'static str>,
        pub git_commit_author_email: Option<&'static str>,
        pub git_commit_author_name:  Option<&'static str>,
        pub git_commit_count:        Option<&'static str>,
        pub git_commit_date:         Option<&'static str>,
        pub git_commit_message:      Option<&'static str>,
        pub git_commit_timestamp:    Option<&'static str>,
        pub git_describe:            Option<&'static str>,

        pub cargo_opt_level:     Option<&'static str>,
        pub cargo_features:      Option<&'static str>,
        pub cargo_debug:         Option<&'static str>,
        pub cargo_target_triple: Option<&'static str>,

        pub rustc_channel:       Option<&'static str>,
        pub rustc_commit_date:   Option<&'static str>,
        pub rustc_commit_hash:   Option<&'static str>,
        pub rustc_llvm_version:  Option<&'static str>,
        pub rustc_semver:        Option<&'static str>,
        pub rustc_host_triple:   Option<&'static str>,
        pub rustc_target_triple: Option<&'static str>,

        pub sysinfo_name:           Option<&'static str>,
        pub sysinfo_os_version:     Option<&'static str>,
        pub sysinfo_user:           Option<&'static str>,
        pub sysinfo_total_memory:   Option<&'static str>,
        pub sysinfo_cpu_vendor:     Option<&'static str>,
        pub sysinfo_cpu_core_count: Option<&'static str>,
        pub sysinfo_cpu_name:       Option<&'static str>,
        pub sysinfo_cpu_brand:      Option<&'static str>,
        pub sysinfo_cpu_frequency:  Option<&'static str>,
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

    pub const BUILD_DATE:      Option<&'static str> = option_env!("VERGEN_BUILD_DATE");
    pub const BUILD_TIMESTAMP: Option<&'static str> = option_env!("VERGEN_BUILD_TIMESTAMP");

    pub const GIT_SHA:                 Option<&'static str> = option_env!("VERGEN_GIT_SHA");
    pub const GIT_BRANCH:              Option<&'static str> = option_env!("VERGEN_GIT_BRANCH");
    pub const GIT_COMMIT_AUTHOR_EMAIL: Option<&'static str> = option_env!("VERGEN_GIT_COMMIT_AUTHOR_EMAIL");
    pub const GIT_COMMIT_AUTHOR_NAME:  Option<&'static str> = option_env!("VERGEN_GIT_COMMIT_AUTHOR_NAME");
    pub const GIT_COMMIT_COUNT:        Option<&'static str> = option_env!("VERGEN_GIT_COMMIT_COUNT");
    pub const GIT_COMMIT_DATE:         Option<&'static str> = option_env!("VERGEN_GIT_COMMIT_DATE");
    pub const GIT_COMMIT_MESSAGE:      Option<&'static str> = option_env!("VERGEN_GIT_COMMIT_MESSAGE");
    pub const GIT_COMMIT_TIMESTAMP:    Option<&'static str> = option_env!("VERGEN_GIT_COMMIT_TIMESTAMP");
    pub const GIT_DESCRIBE:            Option<&'static str> = option_env!("VERGEN_GIT_DESCRIBE");

    pub const CARGO_OPT_LEVEL:     Option<&'static str> = option_env!("VERGEN_CARGO_OPT_LEVEL");
    pub const CARGO_FEATURES:      Option<&'static str> = option_env!("VERGEN_CARGO_FEATURES");
    pub const CARGO_DEBUG:         Option<&'static str> = option_env!("VERGEN_CARGO_DEBUG");
    pub const CARGO_TARGET_TRIPLE: Option<&'static str> = option_env!("VERGEN_CARGO_TARGET_TRIPLE");

    pub const RUSTC_CHANNEL:       Option<&'static str> = option_env!("VERGEN_RUSTC_CHANNEL");
    pub const RUSTC_COMMIT_DATE:   Option<&'static str> = option_env!("VERGEN_RUSTC_COMMIT_DATE");
    pub const RUSTC_COMMIT_HASH:   Option<&'static str> = option_env!("VERGEN_RUSTC_COMMIT_HASH");
    pub const RUSTC_LLVM_VERSION:  Option<&'static str> = option_env!("VERGEN_RUSTC_LLVM_VERSION");
    pub const RUSTC_SEMVER:        Option<&'static str> = option_env!("VERGEN_RUSTC_SEMVER");
    pub const RUSTC_HOST_TRIPLE:   Option<&'static str> = option_env!("VERGEN_RUSTC_HOST_TRIPLE");
    pub const RUSTC_TARGET_TRIPLE: Option<&'static str> = option_env!("VERGEN_CARGO_TARGET_TRIPLE");

    pub const SYSINFO_NAME:           Option<&'static str> = option_env!("VERGEN_SYSINFO_NAME");
    pub const SYSINFO_OS_VERSION:     Option<&'static str> = option_env!("VERGEN_SYSINFO_OS_VERSION");
    pub const SYSINFO_USER:           Option<&'static str> = option_env!("VERGEN_SYSINFO_USER");
    pub const SYSINFO_TOTAL_MEMORY:   Option<&'static str> = option_env!("VERGEN_SYSINFO_TOTAL_MEMORY");
    pub const SYSINFO_CPU_VENDOR:     Option<&'static str> = option_env!("VERGEN_SYSINFO_CPU_VENDOR");
    pub const SYSINFO_CPU_CORE_COUNT: Option<&'static str> = option_env!("VERGEN_SYSINFO_CPU_CORE_COUNT");
    pub const SYSINFO_CPU_NAME:       Option<&'static str> = option_env!("VERGEN_SYSINFO_CPU_NAME");
    pub const SYSINFO_CPU_BRAND:      Option<&'static str> = option_env!("VERGEN_SYSINFO_CPU_BRAND");
    pub const SYSINFO_CPU_FREQUENCY:  Option<&'static str> = option_env!("VERGEN_SYSINFO_CPU_FREQUENCY");
}

