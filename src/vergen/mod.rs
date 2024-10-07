use std::sync::LazyLock;

pub static BUILD_DATE:              LazyLock<&'static str> = LazyLock::new(||{ get_env!("VERGEN_BUILD_DATE") });
pub static BUILD_TIMESTAMP:         LazyLock<&'static str> = LazyLock::new(||{ get_env!("VERGEN_BUILD_TIMESTAMP") });

pub static GIT_SHA:                 LazyLock<&'static str> = LazyLock::new(||{ get_env!("VERGEN_GIT_SHA") });
pub static GIT_BRANCH:              LazyLock<&'static str> = LazyLock::new(||{ get_env!("VERGEN_GIT_BRANCH") });
pub static GIT_COMMIT_AUTHOR_EMAIL: LazyLock<&'static str> = LazyLock::new(||{ get_env!("VERGEN_GIT_COMMIT_AUTHOR_EMAIL") });
pub static GIT_COMMIT_AUTHOR_NAME:  LazyLock<&'static str> = LazyLock::new(||{ get_env!("VERGEN_GIT_COMMIT_AUTHOR_NAME") });
pub static GIT_COMMIT_COUNT:        LazyLock<&'static str> = LazyLock::new(||{ get_env!("VERGEN_GIT_COMMIT_COUNT") });
pub static GIT_COMMIT_DATE:         LazyLock<&'static str> = LazyLock::new(||{ get_env!("VERGEN_GIT_COMMIT_DATE") });
pub static GIT_COMMIT_MESSAGE:      LazyLock<&'static str> = LazyLock::new(||{ get_env!("VERGEN_GIT_COMMIT_MESSAGE") });
pub static GIT_COMMIT_TIMESTAMP:    LazyLock<&'static str> = LazyLock::new(||{ get_env!("VERGEN_GIT_COMMIT_TIMESTAMP") });
pub static GIT_DESCRIBE:            LazyLock<&'static str> = LazyLock::new(||{ get_env!("VERGEN_GIT_DESCRIBE") });

pub static CARGO_OPT_LEVEL:     LazyLock<&'static str> = LazyLock::new(||{ get_env!("VERGEN_CARGO_OPT_LEVEL") });
pub static CARGO_FEATURES:      LazyLock<&'static str> = LazyLock::new(||{ get_env!("VERGEN_CARGO_FEATURES") });
pub static CARGO_DEBUG:         LazyLock<&'static str> = LazyLock::new(||{ get_env!("VERGEN_CARGO_DEBUG") });
pub static CARGO_TARGET_TRIPLE: LazyLock<&'static str> = LazyLock::new(||{ get_env!("VERGEN_CARGO_TARGET_TRIPLE") });

pub static RUSTC_CHANNEL:       LazyLock<&'static str> = LazyLock::new(||{ get_env!("VERGEN_RUSTC_CHANNEL") });
pub static RUSTC_COMMIT_DATE:   LazyLock<&'static str> = LazyLock::new(||{ get_env!("VERGEN_RUSTC_COMMIT_DATE") });
pub static RUSTC_COMMIT_HASH:   LazyLock<&'static str> = LazyLock::new(||{ get_env!("VERGEN_RUSTC_COMMIT_HASH") });
pub static RUSTC_LLVM_VERSION:  LazyLock<&'static str> = LazyLock::new(||{ get_env!("VERGEN_RUSTC_LLVM_VERSION") });
pub static RUSTC_SEMVER:        LazyLock<&'static str> = LazyLock::new(||{ get_env!("VERGEN_RUSTC_SEMVER") });
pub static RUSTC_HOST_TRIPLE:   LazyLock<&'static str> = LazyLock::new(||{ get_env!("VERGEN_RUSTC_HOST_TRIPLE") });
pub static RUSTC_TARGET_TRIPLE: LazyLock<&'static str> = LazyLock::new(||{ get_env!("VERGEN_CARGO_TARGET_TRIPLE") });

pub static SYSINFO_NAME:           LazyLock<&'static str> = LazyLock::new(||{ get_env!("VERGEN_SYSINFO_NAME") });
pub static SYSINFO_OS_VERSION:     LazyLock<&'static str> = LazyLock::new(||{ get_env!("VERGEN_SYSINFO_OS_VERSION") });
pub static SYSINFO_USER:           LazyLock<&'static str> = LazyLock::new(||{ get_env!("VERGEN_SYSINFO_USER") });
pub static SYSINFO_TOTAL_MEMORY:   LazyLock<&'static str> = LazyLock::new(||{ get_env!("VERGEN_SYSINFO_TOTAL_MEMORY") });
pub static SYSINFO_CPU_VENDOR:     LazyLock<&'static str> = LazyLock::new(||{ get_env!("VERGEN_SYSINFO_CPU_VENDOR") });
pub static SYSINFO_CPU_CORE_COUNT: LazyLock<&'static str> = LazyLock::new(||{ get_env!("VERGEN_SYSINFO_CPU_CORE_COUNT") });
pub static SYSINFO_CPU_NAME:       LazyLock<&'static str> = LazyLock::new(||{ get_env!("VERGEN_SYSINFO_CPU_NAME") });
pub static SYSINFO_CPU_BRAND:      LazyLock<&'static str> = LazyLock::new(||{ get_env!("VERGEN_SYSINFO_CPU_BRAND") });
pub static SYSINFO_CPU_FREQUENCY:  LazyLock<&'static str> = LazyLock::new(||{ get_env!("VERGEN_SYSINFO_CPU_FREQUENCY") });

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct vergen_info {
    pub build_date: &'static str,
    pub build_timestamp: &'static str,
    pub git_sha: &'static str,
    pub git_branch: &'static str,
    pub git_commit_author_email: &'static str,
    pub git_commit_author_name: &'static str,
    pub git_commit_count: &'static str,
    pub git_commit_date: &'static str,
    pub git_commit_message: &'static str,
    pub git_commit_timestamp: &'static str,
    pub git_describe: &'static str,
    pub cargo_opt_level: &'static str,
    pub cargo_features: &'static str,
    pub cargo_debug: &'static str,
    pub cargo_target_triple: &'static str,
    pub rustc_channel: &'static str,
    pub rustc_commit_date: &'static str,
    pub rustc_commit_hash: &'static str,
    pub rustc_llvm_version: &'static str,
    pub rustc_semver: &'static str,
    pub rustc_host_triple: &'static str,
    pub rustc_target_triple: &'static str,
    pub sysinfo_name: &'static str,
    pub sysinfo_os_version: &'static str,
    pub sysinfo_user: &'static str,
    pub sysinfo_total_memory: &'static str,
    pub sysinfo_cpu_vendor: &'static str,
    pub sysinfo_cpu_core_count: &'static str,
    pub sysinfo_cpu_name: &'static str,
    pub sysinfo_cpu_brand: &'static str,
    pub sysinfo_cpu_frequency: &'static str,
}
pub fn all_info() -> vergen_info {
    vergen_info {
        build_date: &BUILD_DATE,
        build_timestamp: &BUILD_TIMESTAMP,
        git_sha: &GIT_SHA,
        git_branch: &GIT_BRANCH,
        git_commit_author_email: &GIT_COMMIT_AUTHOR_EMAIL,
        git_commit_author_name: &GIT_COMMIT_AUTHOR_NAME,
        git_commit_count: &GIT_COMMIT_COUNT,
        git_commit_date: &GIT_COMMIT_DATE,
        git_commit_message: &GIT_COMMIT_MESSAGE,
        git_commit_timestamp: &GIT_COMMIT_TIMESTAMP,
        git_describe: &GIT_DESCRIBE,
        cargo_opt_level: &CARGO_OPT_LEVEL,
        cargo_features: &CARGO_FEATURES,
        cargo_debug: &CARGO_DEBUG,
        cargo_target_triple: &CARGO_TARGET_TRIPLE,
        rustc_channel: &RUSTC_CHANNEL,
        rustc_commit_date: &RUSTC_COMMIT_DATE,
        rustc_commit_hash: &RUSTC_COMMIT_HASH,
        rustc_llvm_version: &RUSTC_LLVM_VERSION,
        rustc_semver: &RUSTC_SEMVER,
        rustc_host_triple: &RUSTC_HOST_TRIPLE,
        rustc_target_triple: &RUSTC_TARGET_TRIPLE,
        sysinfo_name: &SYSINFO_NAME,
        sysinfo_os_version: &SYSINFO_OS_VERSION,
        sysinfo_user: &SYSINFO_USER,
        sysinfo_total_memory: &SYSINFO_TOTAL_MEMORY,
        sysinfo_cpu_vendor: &SYSINFO_CPU_VENDOR,
        sysinfo_cpu_core_count: *SYSINFO_CPU_CORE_COUNT,
        sysinfo_cpu_name: &SYSINFO_CPU_NAME,
        sysinfo_cpu_brand: &SYSINFO_CPU_BRAND,
        sysinfo_cpu_frequency: &SYSINFO_CPU_FREQUENCY,
    }
}
#[cfg(feature = "json")]
impl vergen_info {
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "build_date": self.build_date,
            "build_timestamp": self.build_timestamp,
            "git_sha": self.git_sha,
            "git_branch": self.git_branch,
            "git_commit_author_email": self.git_commit_author_email,
            "git_commit_author_name": self.git_commit_author_name,
            "git_commit_count": self.git_commit_count,
            "git_commit_date": self.git_commit_date,
            "git_commit_message": self.git_commit_message,
            "git_commit_timestamp": self.git_commit_timestamp,
            "git_describe": self.git_describe,
            "cargo_opt_level": self.cargo_opt_level,
            "cargo_features": self.cargo_features,
            "cargo_debug": self.cargo_debug,
            "cargo_target_triple": self.cargo_target_triple,
            "rustc_channel": self.rustc_channel,
            "rustc_commit_date": self.rustc_commit_date,
            "rustc_commit_hash": self.rustc_commit_hash,
            "rustc_llvm_version": self.rustc_llvm_version,
            "rustc_semver": self.rustc_semver,
            "rustc_host_triple": self.rustc_host_triple,
            "rustc_target_triple": self.rustc_target_triple,
            "sysinfo_name": self.sysinfo_name,
            "sysinfo_os_version": self.sysinfo_os_version,
            "sysinfo_user": self.sysinfo_user,
            "sysinfo_total_memory": self.sysinfo_total_memory,
            "sysinfo_cpu_vendor": self.sysinfo_cpu_vendor,
            "sysinfo_cpu_core_count": self.sysinfo_cpu_core_count,
            "sysinfo_cpu_name": self.sysinfo_cpu_name,
            "sysinfo_cpu_brand": self.sysinfo_cpu_brand,
            "sysinfo_cpu_frequency": self.sysinfo_cpu_frequency,
        })
    }
}

