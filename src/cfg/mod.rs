pub mod target_arch;
pub use target_arch::cfg_target_arch_info;

pub mod target_feature;
pub use target_feature::cfg_target_feature_info;

pub mod target_os;
pub use target_os::cfg_target_os_info;

pub mod target_family;
pub use target_family::cfg_target_family_info;

pub mod target_env;
pub use target_env::cfg_target_env_info;

pub mod target_endian;
pub use target_endian::cfg_target_endian_info;

pub mod target_pointer_width;
pub use target_pointer_width::cfg_target_pointer_width_info;

pub mod target_has_atomic;
pub use target_has_atomic::cfg_target_has_atomic_info;

pub mod target_vendor;
pub use target_vendor::cfg_target_vendor_info;

pub mod panic;
pub use panic::cfg_panic_info;

pub const TEST:             bool = cfg!(test);
pub const DEBUG_ASSERTIONS: bool = cfg!(debug_assertions);
pub const PROC_MACRO:       bool = cfg!(proc_macro);

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