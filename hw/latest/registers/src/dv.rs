// Licensed under the Apache-2.0 license.
//
// generated by caliptra_registers_generator with caliptra-rtl repo at 74e427e95bf1be19bb7a236ea1fcd1386f9fedf5
//
#![allow(clippy::erasing_op)]
#![allow(clippy::identity_op)]
/// A zero-sized type that represents ownership of this
/// peripheral, used to get access to a Register lock. Most
/// programs create one of these in unsafe code near the top of
/// main(), and pass it to the driver responsible for managing
/// all access to the hardware.
pub struct DvReg {
    _priv: (),
}
impl DvReg {
    pub const PTR: *mut u32 = 0x1001c000 as *mut u32;
    /// # Safety
    ///
    /// Caller must ensure that all concurrent use of this
    /// peripheral in the firmware is done so in a compatible
    /// way. The simplest way to enforce this is to only call
    /// this function once.
    #[inline(always)]
    pub unsafe fn new() -> Self {
        Self { _priv: () }
    }
    /// Returns a register block that can be used to read
    /// registers from this peripheral, but cannot write.
    #[inline(always)]
    pub fn regs(&self) -> RegisterBlock<ureg::RealMmio> {
        RegisterBlock {
            ptr: Self::PTR,
            mmio: core::default::Default::default(),
        }
    }
    /// Return a register block that can be used to read and
    /// write this peripheral's registers.
    #[inline(always)]
    pub fn regs_mut(&mut self) -> RegisterBlock<ureg::RealMmioMut> {
        RegisterBlock {
            ptr: Self::PTR,
            mmio: core::default::Default::default(),
        }
    }
}
#[derive(Clone, Copy)]
pub struct RegisterBlock<TMmio: ureg::Mmio + core::borrow::Borrow<TMmio>> {
    ptr: *mut u32,
    mmio: TMmio,
}
impl<TMmio: ureg::Mmio + core::default::Default> RegisterBlock<TMmio> {
    /// # Safety
    ///
    /// The caller is responsible for ensuring that ptr is valid for
    /// volatile reads and writes at any of the offsets in this register
    /// block.
    #[inline(always)]
    pub unsafe fn new(ptr: *mut u32) -> Self {
        Self {
            ptr,
            mmio: core::default::Default::default(),
        }
    }
}
impl<TMmio: ureg::Mmio> RegisterBlock<TMmio> {
    /// # Safety
    ///
    /// The caller is responsible for ensuring that ptr is valid for
    /// volatile reads and writes at any of the offsets in this register
    /// block.
    #[inline(always)]
    pub unsafe fn new_with_mmio(ptr: *mut u32, mmio: TMmio) -> Self {
        Self { ptr, mmio }
    }
    /// Controls for the Sticky Data Vault Entries (cleared on hard reset)
    ///
    /// Read value: [`dv::regs::DatavaultctrlReadVal`]; Write value: [`dv::regs::DatavaultctrlWriteVal`]
    #[inline(always)]
    pub fn sticky_data_vault_ctrl(
        &self,
    ) -> ureg::Array<10, ureg::RegRef<crate::dv::meta::Stickydatavaultctrl, &TMmio>> {
        unsafe {
            ureg::Array::new_with_mmio(
                self.ptr.wrapping_add(0 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Read value: [`u32`]; Write value: [`u32`]
    #[inline(always)]
    pub fn sticky_data_vault_entry(
        &self,
    ) -> ureg::Array<10, ureg::Array<12, ureg::RegRef<crate::dv::meta::StickyDataVaultEntry, &TMmio>>>
    {
        unsafe {
            ureg::Array::new_with_mmio(
                self.ptr.wrapping_add(0x28 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Controls for the Data Vault Entries (cleared on warm reset)
    ///
    /// Read value: [`dv::regs::DatavaultctrlReadVal`]; Write value: [`dv::regs::DatavaultctrlWriteVal`]
    #[inline(always)]
    pub fn data_vault_ctrl(
        &self,
    ) -> ureg::Array<10, ureg::RegRef<crate::dv::meta::Datavaultctrl, &TMmio>> {
        unsafe {
            ureg::Array::new_with_mmio(
                self.ptr.wrapping_add(0x208 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Read value: [`u32`]; Write value: [`u32`]
    #[inline(always)]
    pub fn data_vault_entry(
        &self,
    ) -> ureg::Array<10, ureg::Array<12, ureg::RegRef<crate::dv::meta::DataVaultEntry, &TMmio>>>
    {
        unsafe {
            ureg::Array::new_with_mmio(
                self.ptr.wrapping_add(0x230 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Scratch Register Controls (cleared on warm reset)
    ///
    /// Read value: [`dv::regs::LockablescratchregctrlReadVal`]; Write value: [`dv::regs::LockablescratchregctrlWriteVal`]
    #[inline(always)]
    pub fn lockable_scratch_reg_ctrl(
        &self,
    ) -> ureg::Array<10, ureg::RegRef<crate::dv::meta::Lockablescratchregctrl, &TMmio>> {
        unsafe {
            ureg::Array::new_with_mmio(
                self.ptr.wrapping_add(0x410 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Scratch Register Entrie (cleared on hard reset)
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    #[inline(always)]
    pub fn lockable_scratch_reg(
        &self,
    ) -> ureg::Array<10, ureg::RegRef<crate::dv::meta::Lockablescratchreg, &TMmio>> {
        unsafe {
            ureg::Array::new_with_mmio(
                self.ptr.wrapping_add(0x438 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Read value: [`u32`]; Write value: [`u32`]
    #[inline(always)]
    pub fn non_sticky_generic_scratch_reg(
        &self,
    ) -> ureg::Array<8, ureg::RegRef<crate::dv::meta::Nonstickygenericscratchreg, &TMmio>> {
        unsafe {
            ureg::Array::new_with_mmio(
                self.ptr.wrapping_add(0x460 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Sticky Scratch Register Controls (cleared on hard reset)
    ///
    /// Read value: [`dv::regs::LockablescratchregctrlReadVal`]; Write value: [`dv::regs::LockablescratchregctrlWriteVal`]
    #[inline(always)]
    pub fn sticky_lockable_scratch_reg_ctrl(
        &self,
    ) -> ureg::Array<8, ureg::RegRef<crate::dv::meta::Stickylockablescratchregctrl, &TMmio>> {
        unsafe {
            ureg::Array::new_with_mmio(
                self.ptr.wrapping_add(0x480 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Sticky Scratch Register Entries (cleared on hard reset)
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    #[inline(always)]
    pub fn sticky_lockable_scratch_reg(
        &self,
    ) -> ureg::Array<8, ureg::RegRef<crate::dv::meta::Stickylockablescratchreg, &TMmio>> {
        unsafe {
            ureg::Array::new_with_mmio(
                self.ptr.wrapping_add(0x4a0 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
}
pub mod regs {
    //! Types that represent the values held by registers.
    #[derive(Clone, Copy)]
    pub struct DatavaultctrlReadVal(u32);
    impl DatavaultctrlReadVal {
        /// Lock writes to this entry. Writes will be suppressed when locked.
        #[inline(always)]
        pub fn lock_entry(&self) -> bool {
            ((self.0 >> 0) & 1) != 0
        }
        /// Construct a WriteVal that can be used to modify the contents of this register value.
        #[inline(always)]
        pub fn modify(self) -> DatavaultctrlWriteVal {
            DatavaultctrlWriteVal(self.0)
        }
    }
    impl From<u32> for DatavaultctrlReadVal {
        #[inline(always)]
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<DatavaultctrlReadVal> for u32 {
        #[inline(always)]
        fn from(val: DatavaultctrlReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct DatavaultctrlWriteVal(u32);
    impl DatavaultctrlWriteVal {
        /// Lock writes to this entry. Writes will be suppressed when locked.
        #[inline(always)]
        pub fn lock_entry(self, val: bool) -> Self {
            Self((self.0 & !(1 << 0)) | (u32::from(val) << 0))
        }
    }
    impl From<u32> for DatavaultctrlWriteVal {
        #[inline(always)]
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<DatavaultctrlWriteVal> for u32 {
        #[inline(always)]
        fn from(val: DatavaultctrlWriteVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct LockablescratchregctrlReadVal(u32);
    impl LockablescratchregctrlReadVal {
        /// Lock writes to the Scratch registers. Writes will be suppressed when locked.
        #[inline(always)]
        pub fn lock_entry(&self) -> bool {
            ((self.0 >> 0) & 1) != 0
        }
        /// Construct a WriteVal that can be used to modify the contents of this register value.
        #[inline(always)]
        pub fn modify(self) -> LockablescratchregctrlWriteVal {
            LockablescratchregctrlWriteVal(self.0)
        }
    }
    impl From<u32> for LockablescratchregctrlReadVal {
        #[inline(always)]
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<LockablescratchregctrlReadVal> for u32 {
        #[inline(always)]
        fn from(val: LockablescratchregctrlReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct LockablescratchregctrlWriteVal(u32);
    impl LockablescratchregctrlWriteVal {
        /// Lock writes to the Scratch registers. Writes will be suppressed when locked.
        #[inline(always)]
        pub fn lock_entry(self, val: bool) -> Self {
            Self((self.0 & !(1 << 0)) | (u32::from(val) << 0))
        }
    }
    impl From<u32> for LockablescratchregctrlWriteVal {
        #[inline(always)]
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<LockablescratchregctrlWriteVal> for u32 {
        #[inline(always)]
        fn from(val: LockablescratchregctrlWriteVal) -> u32 {
            val.0
        }
    }
}
pub mod enums {
    //! Enumerations used by some register fields.
    pub mod selector {}
}
pub mod meta {
    //! Additional metadata needed by ureg.
    pub type Stickydatavaultctrl = ureg::ReadWriteReg32<
        0,
        crate::dv::regs::DatavaultctrlReadVal,
        crate::dv::regs::DatavaultctrlWriteVal,
    >;
    pub type StickyDataVaultEntry = ureg::ReadWriteReg32<0, u32, u32>;
    pub type Datavaultctrl = ureg::ReadWriteReg32<
        0,
        crate::dv::regs::DatavaultctrlReadVal,
        crate::dv::regs::DatavaultctrlWriteVal,
    >;
    pub type DataVaultEntry = ureg::ReadWriteReg32<0, u32, u32>;
    pub type Lockablescratchregctrl = ureg::ReadWriteReg32<
        0,
        crate::dv::regs::LockablescratchregctrlReadVal,
        crate::dv::regs::LockablescratchregctrlWriteVal,
    >;
    pub type Lockablescratchreg = ureg::ReadWriteReg32<0, u32, u32>;
    pub type Nonstickygenericscratchreg = ureg::ReadWriteReg32<0, u32, u32>;
    pub type Stickylockablescratchregctrl = ureg::ReadWriteReg32<
        0,
        crate::dv::regs::LockablescratchregctrlReadVal,
        crate::dv::regs::LockablescratchregctrlWriteVal,
    >;
    pub type Stickylockablescratchreg = ureg::ReadWriteReg32<0, u32, u32>;
}