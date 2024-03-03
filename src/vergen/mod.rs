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