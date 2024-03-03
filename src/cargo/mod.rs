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