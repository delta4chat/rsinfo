pub const X86:       bool = cfg!(target_arch="x86");
pub const X86_64:    bool = cfg!(target_arch="x86_64");
pub const MIPS:      bool = cfg!(target_arch="mips");
pub const POWERPC:   bool = cfg!(target_arch="powerpc");
pub const POWERPC64: bool = cfg!(target_arch="powerpc64");
pub const ARM:       bool = cfg!(target_arch="arm");
pub const AARCH64:   bool = cfg!(target_arch="aarch64");

// [ANCHOR START] CONST-TO-STRUCT.PY //

/*
 * the following source code is automatic generated by const-to-struct.py
 * DO NOT EDIT.
*/

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct cfg_target_arch_info {
    pub x86: bool,
    pub x86_64: bool,
    pub mips: bool,
    pub powerpc: bool,
    pub powerpc64: bool,
    pub arm: bool,
    pub aarch64: bool,
}
pub const ALL_INFO: cfg_target_arch_info = all_info();
pub const fn all_info() -> cfg_target_arch_info {
    cfg_target_arch_info {
        x86: X86,
        x86_64: X86_64,
        mips: MIPS,
        powerpc: POWERPC,
        powerpc64: POWERPC64,
        arm: ARM,
        aarch64: AARCH64,
    }
}
// [ANCHOR ENDED] CONST-TO-STRUCT.PY //