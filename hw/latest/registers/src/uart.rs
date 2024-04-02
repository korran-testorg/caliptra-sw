// Licensed under the Apache-2.0 license.
//
// generated by caliptra_registers_generator with caliptra-rtl repo at 648282f17ac13ad97e2551974db632d1acb76be2
//
#![allow(clippy::erasing_op)]
#![allow(clippy::identity_op)]
/// A zero-sized type that represents ownership of this
/// peripheral, used to get access to a Register lock. Most
/// programs create one of these in unsafe code near the top of
/// main(), and pass it to the driver responsible for managing
/// all access to the hardware.
pub struct Uart {
    _priv: (),
}
impl Uart {
    pub const PTR: *mut u32 = 0x20001000 as *mut u32;
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
    /// Read value: [`uart::regs::InterruptStateReadVal`]; Write value: [`uart::regs::InterruptStateWriteVal`]
    #[inline(always)]
    pub fn interrupt_state(&self) -> ureg::RegRef<crate::uart::meta::InterruptState, &TMmio> {
        unsafe {
            ureg::RegRef::new_with_mmio(
                self.ptr.wrapping_add(0 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Read value: [`uart::regs::InterruptEnableReadVal`]; Write value: [`uart::regs::InterruptEnableWriteVal`]
    #[inline(always)]
    pub fn interrupt_enable(&self) -> ureg::RegRef<crate::uart::meta::InterruptEnable, &TMmio> {
        unsafe {
            ureg::RegRef::new_with_mmio(
                self.ptr.wrapping_add(4 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Read value: [`uart::regs::InterruptTestReadVal`]; Write value: [`uart::regs::InterruptTestWriteVal`]
    #[inline(always)]
    pub fn interrupt_test(&self) -> ureg::RegRef<crate::uart::meta::InterruptTest, &TMmio> {
        unsafe {
            ureg::RegRef::new_with_mmio(
                self.ptr.wrapping_add(8 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Read value: [`uart::regs::AlertTestReadVal`]; Write value: [`uart::regs::AlertTestWriteVal`]
    #[inline(always)]
    pub fn alert_test(&self) -> ureg::RegRef<crate::uart::meta::AlertTest, &TMmio> {
        unsafe {
            ureg::RegRef::new_with_mmio(
                self.ptr.wrapping_add(0xc / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Read value: [`uart::regs::CtrlReadVal`]; Write value: [`uart::regs::CtrlWriteVal`]
    #[inline(always)]
    pub fn ctrl(&self) -> ureg::RegRef<crate::uart::meta::Ctrl, &TMmio> {
        unsafe {
            ureg::RegRef::new_with_mmio(
                self.ptr.wrapping_add(0x10 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Read value: [`uart::regs::StatusReadVal`]; Write value: [`uart::regs::StatusWriteVal`]
    #[inline(always)]
    pub fn status(&self) -> ureg::RegRef<crate::uart::meta::Status, &TMmio> {
        unsafe {
            ureg::RegRef::new_with_mmio(
                self.ptr.wrapping_add(0x14 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Read value: [`uart::regs::RdataReadVal`]; Write value: [`uart::regs::RdataWriteVal`]
    #[inline(always)]
    pub fn rdata(&self) -> ureg::RegRef<crate::uart::meta::Rdata, &TMmio> {
        unsafe {
            ureg::RegRef::new_with_mmio(
                self.ptr.wrapping_add(0x18 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Read value: [`uart::regs::WdataReadVal`]; Write value: [`uart::regs::WdataWriteVal`]
    #[inline(always)]
    pub fn wdata(&self) -> ureg::RegRef<crate::uart::meta::Wdata, &TMmio> {
        unsafe {
            ureg::RegRef::new_with_mmio(
                self.ptr.wrapping_add(0x1c / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Read value: [`uart::regs::FifoCtrlReadVal`]; Write value: [`uart::regs::FifoCtrlWriteVal`]
    #[inline(always)]
    pub fn fifo_ctrl(&self) -> ureg::RegRef<crate::uart::meta::FifoCtrl, &TMmio> {
        unsafe {
            ureg::RegRef::new_with_mmio(
                self.ptr.wrapping_add(0x20 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Read value: [`uart::regs::FifoStatusReadVal`]; Write value: [`uart::regs::FifoStatusWriteVal`]
    #[inline(always)]
    pub fn fifo_status(&self) -> ureg::RegRef<crate::uart::meta::FifoStatus, &TMmio> {
        unsafe {
            ureg::RegRef::new_with_mmio(
                self.ptr.wrapping_add(0x24 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Read value: [`uart::regs::OvrdReadVal`]; Write value: [`uart::regs::OvrdWriteVal`]
    #[inline(always)]
    pub fn ovrd(&self) -> ureg::RegRef<crate::uart::meta::Ovrd, &TMmio> {
        unsafe {
            ureg::RegRef::new_with_mmio(
                self.ptr.wrapping_add(0x28 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Read value: [`uart::regs::ValReadVal`]; Write value: [`uart::regs::ValWriteVal`]
    #[inline(always)]
    pub fn val(&self) -> ureg::RegRef<crate::uart::meta::Val, &TMmio> {
        unsafe {
            ureg::RegRef::new_with_mmio(
                self.ptr.wrapping_add(0x2c / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Read value: [`uart::regs::TimeoutCtrlReadVal`]; Write value: [`uart::regs::TimeoutCtrlWriteVal`]
    #[inline(always)]
    pub fn timeout_ctrl(&self) -> ureg::RegRef<crate::uart::meta::TimeoutCtrl, &TMmio> {
        unsafe {
            ureg::RegRef::new_with_mmio(
                self.ptr.wrapping_add(0x30 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
}
pub mod regs {
    //! Types that represent the values held by registers.
    #[derive(Clone, Copy)]
    pub struct AlertTestWriteVal(u32);
    impl AlertTestWriteVal {
        /// Write 1 to trigger one alert event of this kind.
        #[inline(always)]
        pub fn fatal_fault(self, val: bool) -> Self {
            Self((self.0 & !(1 << 0)) | (u32::from(val) << 0))
        }
    }
    impl From<u32> for AlertTestWriteVal {
        #[inline(always)]
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<AlertTestWriteVal> for u32 {
        #[inline(always)]
        fn from(val: AlertTestWriteVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct CtrlReadVal(u32);
    impl CtrlReadVal {
        /// TX enable
        #[inline(always)]
        pub fn tx(&self) -> bool {
            ((self.0 >> 0) & 1) != 0
        }
        /// RX enable
        #[inline(always)]
        pub fn rx(&self) -> bool {
            ((self.0 >> 1) & 1) != 0
        }
        /// RX noise filter enable.
        /// If the noise filter is enabled, RX line goes through the 3-tap
        /// repetition code. It ignores single IP clock period noise.
        #[inline(always)]
        pub fn nf(&self) -> bool {
            ((self.0 >> 2) & 1) != 0
        }
        /// System loopback enable.
        ///
        /// If this bit is turned on, any outgoing bits to TX are received through RX.
        /// See Block Diagram. Note that the TX line goes 1 if System loopback is enabled.
        #[inline(always)]
        pub fn slpbk(&self) -> bool {
            ((self.0 >> 4) & 1) != 0
        }
        /// Line loopback enable.
        ///
        /// If this bit is turned on, incoming bits are forwarded to TX for testing purpose.
        /// See Block Diagram. Note that the internal design sees RX value as 1 always if line
        /// loopback is enabled.
        #[inline(always)]
        pub fn llpbk(&self) -> bool {
            ((self.0 >> 5) & 1) != 0
        }
        /// If true, parity is enabled in both RX and TX directions.
        #[inline(always)]
        pub fn parity_en(&self) -> bool {
            ((self.0 >> 6) & 1) != 0
        }
        /// If PARITY_EN is true, this determines the type, 1 for odd parity, 0 for even.
        #[inline(always)]
        pub fn parity_odd(&self) -> bool {
            ((self.0 >> 7) & 1) != 0
        }
        /// Trigger level for RX break detection. Sets the number of character
        /// times the line must be low to detect a break.
        #[inline(always)]
        pub fn rxblvl(&self) -> u32 {
            (self.0 >> 8) & 3
        }
        /// BAUD clock rate control.
        #[inline(always)]
        pub fn nco(&self) -> u32 {
            (self.0 >> 16) & 0xffff
        }
        /// Construct a WriteVal that can be used to modify the contents of this register value.
        #[inline(always)]
        pub fn modify(self) -> CtrlWriteVal {
            CtrlWriteVal(self.0)
        }
    }
    impl From<u32> for CtrlReadVal {
        #[inline(always)]
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<CtrlReadVal> for u32 {
        #[inline(always)]
        fn from(val: CtrlReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct CtrlWriteVal(u32);
    impl CtrlWriteVal {
        /// TX enable
        #[inline(always)]
        pub fn tx(self, val: bool) -> Self {
            Self((self.0 & !(1 << 0)) | (u32::from(val) << 0))
        }
        /// RX enable
        #[inline(always)]
        pub fn rx(self, val: bool) -> Self {
            Self((self.0 & !(1 << 1)) | (u32::from(val) << 1))
        }
        /// RX noise filter enable.
        /// If the noise filter is enabled, RX line goes through the 3-tap
        /// repetition code. It ignores single IP clock period noise.
        #[inline(always)]
        pub fn nf(self, val: bool) -> Self {
            Self((self.0 & !(1 << 2)) | (u32::from(val) << 2))
        }
        /// System loopback enable.
        ///
        /// If this bit is turned on, any outgoing bits to TX are received through RX.
        /// See Block Diagram. Note that the TX line goes 1 if System loopback is enabled.
        #[inline(always)]
        pub fn slpbk(self, val: bool) -> Self {
            Self((self.0 & !(1 << 4)) | (u32::from(val) << 4))
        }
        /// Line loopback enable.
        ///
        /// If this bit is turned on, incoming bits are forwarded to TX for testing purpose.
        /// See Block Diagram. Note that the internal design sees RX value as 1 always if line
        /// loopback is enabled.
        #[inline(always)]
        pub fn llpbk(self, val: bool) -> Self {
            Self((self.0 & !(1 << 5)) | (u32::from(val) << 5))
        }
        /// If true, parity is enabled in both RX and TX directions.
        #[inline(always)]
        pub fn parity_en(self, val: bool) -> Self {
            Self((self.0 & !(1 << 6)) | (u32::from(val) << 6))
        }
        /// If PARITY_EN is true, this determines the type, 1 for odd parity, 0 for even.
        #[inline(always)]
        pub fn parity_odd(self, val: bool) -> Self {
            Self((self.0 & !(1 << 7)) | (u32::from(val) << 7))
        }
        /// Trigger level for RX break detection. Sets the number of character
        /// times the line must be low to detect a break.
        #[inline(always)]
        pub fn rxblvl(self, val: u32) -> Self {
            Self((self.0 & !(3 << 8)) | ((val & 3) << 8))
        }
        /// BAUD clock rate control.
        #[inline(always)]
        pub fn nco(self, val: u32) -> Self {
            Self((self.0 & !(0xffff << 16)) | ((val & 0xffff) << 16))
        }
    }
    impl From<u32> for CtrlWriteVal {
        #[inline(always)]
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<CtrlWriteVal> for u32 {
        #[inline(always)]
        fn from(val: CtrlWriteVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct FifoCtrlReadVal(u32);
    impl FifoCtrlReadVal {
        /// RX fifo reset. Write 1 to the register resets RX_FIFO. Read returns 0
        #[inline(always)]
        pub fn rxrst(&self) -> bool {
            ((self.0 >> 0) & 1) != 0
        }
        /// TX fifo reset. Write 1 to the register resets TX_FIFO. Read returns 0
        #[inline(always)]
        pub fn txrst(&self) -> bool {
            ((self.0 >> 1) & 1) != 0
        }
        /// Trigger level for RX interrupts. If the FIFO depth is greater than or equal to
        /// the setting, it raises rx_watermark interrupt.
        #[inline(always)]
        pub fn rxilvl(&self) -> u32 {
            (self.0 >> 2) & 7
        }
        /// Trigger level for TX interrupts. If the FIFO depth is less than the setting, it
        /// raises tx_watermark interrupt.
        #[inline(always)]
        pub fn txilvl(&self) -> u32 {
            (self.0 >> 5) & 3
        }
        /// Construct a WriteVal that can be used to modify the contents of this register value.
        #[inline(always)]
        pub fn modify(self) -> FifoCtrlWriteVal {
            FifoCtrlWriteVal(self.0)
        }
    }
    impl From<u32> for FifoCtrlReadVal {
        #[inline(always)]
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<FifoCtrlReadVal> for u32 {
        #[inline(always)]
        fn from(val: FifoCtrlReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct FifoCtrlWriteVal(u32);
    impl FifoCtrlWriteVal {
        /// RX fifo reset. Write 1 to the register resets RX_FIFO. Read returns 0
        #[inline(always)]
        pub fn rxrst(self, val: bool) -> Self {
            Self((self.0 & !(1 << 0)) | (u32::from(val) << 0))
        }
        /// TX fifo reset. Write 1 to the register resets TX_FIFO. Read returns 0
        #[inline(always)]
        pub fn txrst(self, val: bool) -> Self {
            Self((self.0 & !(1 << 1)) | (u32::from(val) << 1))
        }
        /// Trigger level for RX interrupts. If the FIFO depth is greater than or equal to
        /// the setting, it raises rx_watermark interrupt.
        #[inline(always)]
        pub fn rxilvl(self, val: u32) -> Self {
            Self((self.0 & !(7 << 2)) | ((val & 7) << 2))
        }
        /// Trigger level for TX interrupts. If the FIFO depth is less than the setting, it
        /// raises tx_watermark interrupt.
        #[inline(always)]
        pub fn txilvl(self, val: u32) -> Self {
            Self((self.0 & !(3 << 5)) | ((val & 3) << 5))
        }
    }
    impl From<u32> for FifoCtrlWriteVal {
        #[inline(always)]
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<FifoCtrlWriteVal> for u32 {
        #[inline(always)]
        fn from(val: FifoCtrlWriteVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct FifoStatusReadVal(u32);
    impl FifoStatusReadVal {
        /// Current fill level of TX fifo
        #[inline(always)]
        pub fn txlvl(&self) -> u32 {
            (self.0 >> 0) & 0x3f
        }
        /// Current fill level of RX fifo
        #[inline(always)]
        pub fn rxlvl(&self) -> u32 {
            (self.0 >> 16) & 0x3f
        }
    }
    impl From<u32> for FifoStatusReadVal {
        #[inline(always)]
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<FifoStatusReadVal> for u32 {
        #[inline(always)]
        fn from(val: FifoStatusReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct InterruptEnableReadVal(u32);
    impl InterruptEnableReadVal {
        /// Enable interrupt when tx_watermark is set.
        #[inline(always)]
        pub fn tx_watermark(&self) -> bool {
            ((self.0 >> 0) & 1) != 0
        }
        /// Enable interrupt when rx_watermark is set.
        #[inline(always)]
        pub fn rx_watermark(&self) -> bool {
            ((self.0 >> 1) & 1) != 0
        }
        /// Enable interrupt when tx_empty is set.
        #[inline(always)]
        pub fn tx_empty(&self) -> bool {
            ((self.0 >> 2) & 1) != 0
        }
        /// Enable interrupt when rx_overflow is set.
        #[inline(always)]
        pub fn rx_overflow(&self) -> bool {
            ((self.0 >> 3) & 1) != 0
        }
        /// Enable interrupt when rx_frame_err is set.
        #[inline(always)]
        pub fn rx_frame_err(&self) -> bool {
            ((self.0 >> 4) & 1) != 0
        }
        /// Enable interrupt when rx_break_err is set.
        #[inline(always)]
        pub fn rx_break_err(&self) -> bool {
            ((self.0 >> 5) & 1) != 0
        }
        /// Enable interrupt when rx_timeout is set.
        #[inline(always)]
        pub fn rx_timeout(&self) -> bool {
            ((self.0 >> 6) & 1) != 0
        }
        /// Enable interrupt when rx_parity_err is set.
        #[inline(always)]
        pub fn rx_parity_err(&self) -> bool {
            ((self.0 >> 7) & 1) != 0
        }
        /// Construct a WriteVal that can be used to modify the contents of this register value.
        #[inline(always)]
        pub fn modify(self) -> InterruptEnableWriteVal {
            InterruptEnableWriteVal(self.0)
        }
    }
    impl From<u32> for InterruptEnableReadVal {
        #[inline(always)]
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<InterruptEnableReadVal> for u32 {
        #[inline(always)]
        fn from(val: InterruptEnableReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct InterruptEnableWriteVal(u32);
    impl InterruptEnableWriteVal {
        /// Enable interrupt when tx_watermark is set.
        #[inline(always)]
        pub fn tx_watermark(self, val: bool) -> Self {
            Self((self.0 & !(1 << 0)) | (u32::from(val) << 0))
        }
        /// Enable interrupt when rx_watermark is set.
        #[inline(always)]
        pub fn rx_watermark(self, val: bool) -> Self {
            Self((self.0 & !(1 << 1)) | (u32::from(val) << 1))
        }
        /// Enable interrupt when tx_empty is set.
        #[inline(always)]
        pub fn tx_empty(self, val: bool) -> Self {
            Self((self.0 & !(1 << 2)) | (u32::from(val) << 2))
        }
        /// Enable interrupt when rx_overflow is set.
        #[inline(always)]
        pub fn rx_overflow(self, val: bool) -> Self {
            Self((self.0 & !(1 << 3)) | (u32::from(val) << 3))
        }
        /// Enable interrupt when rx_frame_err is set.
        #[inline(always)]
        pub fn rx_frame_err(self, val: bool) -> Self {
            Self((self.0 & !(1 << 4)) | (u32::from(val) << 4))
        }
        /// Enable interrupt when rx_break_err is set.
        #[inline(always)]
        pub fn rx_break_err(self, val: bool) -> Self {
            Self((self.0 & !(1 << 5)) | (u32::from(val) << 5))
        }
        /// Enable interrupt when rx_timeout is set.
        #[inline(always)]
        pub fn rx_timeout(self, val: bool) -> Self {
            Self((self.0 & !(1 << 6)) | (u32::from(val) << 6))
        }
        /// Enable interrupt when rx_parity_err is set.
        #[inline(always)]
        pub fn rx_parity_err(self, val: bool) -> Self {
            Self((self.0 & !(1 << 7)) | (u32::from(val) << 7))
        }
    }
    impl From<u32> for InterruptEnableWriteVal {
        #[inline(always)]
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<InterruptEnableWriteVal> for u32 {
        #[inline(always)]
        fn from(val: InterruptEnableWriteVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct InterruptStateReadVal(u32);
    impl InterruptStateReadVal {
        /// raised if the transmit FIFO is past the high-water mark.
        #[inline(always)]
        pub fn tx_watermark(&self) -> bool {
            ((self.0 >> 0) & 1) != 0
        }
        /// raised if the receive FIFO is past the high-water mark.
        #[inline(always)]
        pub fn rx_watermark(&self) -> bool {
            ((self.0 >> 1) & 1) != 0
        }
        /// raised if the transmit FIFO has emptied and no transmit is ongoing.
        #[inline(always)]
        pub fn tx_empty(&self) -> bool {
            ((self.0 >> 2) & 1) != 0
        }
        /// raised if the receive FIFO has overflowed.
        #[inline(always)]
        pub fn rx_overflow(&self) -> bool {
            ((self.0 >> 3) & 1) != 0
        }
        /// raised if a framing error has been detected on receive.
        #[inline(always)]
        pub fn rx_frame_err(&self) -> bool {
            ((self.0 >> 4) & 1) != 0
        }
        /// raised if break condition has been detected on receive.
        #[inline(always)]
        pub fn rx_break_err(&self) -> bool {
            ((self.0 >> 5) & 1) != 0
        }
        /// raised if RX FIFO has characters remaining in the FIFO without being
        /// retrieved for the programmed time period.
        #[inline(always)]
        pub fn rx_timeout(&self) -> bool {
            ((self.0 >> 6) & 1) != 0
        }
        /// raised if the receiver has detected a parity error.
        #[inline(always)]
        pub fn rx_parity_err(&self) -> bool {
            ((self.0 >> 7) & 1) != 0
        }
        /// Construct a WriteVal that can be used to modify the contents of this register value.
        #[inline(always)]
        pub fn modify(self) -> InterruptStateWriteVal {
            InterruptStateWriteVal(self.0)
        }
    }
    impl From<u32> for InterruptStateReadVal {
        #[inline(always)]
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<InterruptStateReadVal> for u32 {
        #[inline(always)]
        fn from(val: InterruptStateReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct InterruptStateWriteVal(u32);
    impl InterruptStateWriteVal {
        /// raised if the transmit FIFO is past the high-water mark.
        #[inline(always)]
        pub fn tx_watermark(self, val: bool) -> Self {
            Self((self.0 & !(1 << 0)) | (u32::from(val) << 0))
        }
        /// raised if the receive FIFO is past the high-water mark.
        #[inline(always)]
        pub fn rx_watermark(self, val: bool) -> Self {
            Self((self.0 & !(1 << 1)) | (u32::from(val) << 1))
        }
        /// raised if the transmit FIFO has emptied and no transmit is ongoing.
        #[inline(always)]
        pub fn tx_empty(self, val: bool) -> Self {
            Self((self.0 & !(1 << 2)) | (u32::from(val) << 2))
        }
        /// raised if the receive FIFO has overflowed.
        #[inline(always)]
        pub fn rx_overflow(self, val: bool) -> Self {
            Self((self.0 & !(1 << 3)) | (u32::from(val) << 3))
        }
        /// raised if a framing error has been detected on receive.
        #[inline(always)]
        pub fn rx_frame_err(self, val: bool) -> Self {
            Self((self.0 & !(1 << 4)) | (u32::from(val) << 4))
        }
        /// raised if break condition has been detected on receive.
        #[inline(always)]
        pub fn rx_break_err(self, val: bool) -> Self {
            Self((self.0 & !(1 << 5)) | (u32::from(val) << 5))
        }
        /// raised if RX FIFO has characters remaining in the FIFO without being
        /// retrieved for the programmed time period.
        #[inline(always)]
        pub fn rx_timeout(self, val: bool) -> Self {
            Self((self.0 & !(1 << 6)) | (u32::from(val) << 6))
        }
        /// raised if the receiver has detected a parity error.
        #[inline(always)]
        pub fn rx_parity_err(self, val: bool) -> Self {
            Self((self.0 & !(1 << 7)) | (u32::from(val) << 7))
        }
    }
    impl From<u32> for InterruptStateWriteVal {
        #[inline(always)]
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<InterruptStateWriteVal> for u32 {
        #[inline(always)]
        fn from(val: InterruptStateWriteVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct InterruptTestWriteVal(u32);
    impl InterruptTestWriteVal {
        /// Write 1 to force tx_watermark to 1.
        #[inline(always)]
        pub fn tx_watermark(self, val: bool) -> Self {
            Self((self.0 & !(1 << 0)) | (u32::from(val) << 0))
        }
        /// Write 1 to force rx_watermark to 1.
        #[inline(always)]
        pub fn rx_watermark(self, val: bool) -> Self {
            Self((self.0 & !(1 << 1)) | (u32::from(val) << 1))
        }
        /// Write 1 to force tx_empty to 1.
        #[inline(always)]
        pub fn tx_empty(self, val: bool) -> Self {
            Self((self.0 & !(1 << 2)) | (u32::from(val) << 2))
        }
        /// Write 1 to force rx_overflow to 1.
        #[inline(always)]
        pub fn rx_overflow(self, val: bool) -> Self {
            Self((self.0 & !(1 << 3)) | (u32::from(val) << 3))
        }
        /// Write 1 to force rx_frame_err to 1.
        #[inline(always)]
        pub fn rx_frame_err(self, val: bool) -> Self {
            Self((self.0 & !(1 << 4)) | (u32::from(val) << 4))
        }
        /// Write 1 to force rx_break_err to 1.
        #[inline(always)]
        pub fn rx_break_err(self, val: bool) -> Self {
            Self((self.0 & !(1 << 5)) | (u32::from(val) << 5))
        }
        /// Write 1 to force rx_timeout to 1.
        #[inline(always)]
        pub fn rx_timeout(self, val: bool) -> Self {
            Self((self.0 & !(1 << 6)) | (u32::from(val) << 6))
        }
        /// Write 1 to force rx_parity_err to 1.
        #[inline(always)]
        pub fn rx_parity_err(self, val: bool) -> Self {
            Self((self.0 & !(1 << 7)) | (u32::from(val) << 7))
        }
    }
    impl From<u32> for InterruptTestWriteVal {
        #[inline(always)]
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<InterruptTestWriteVal> for u32 {
        #[inline(always)]
        fn from(val: InterruptTestWriteVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct OvrdReadVal(u32);
    impl OvrdReadVal {
        /// Enable TX pin override control
        #[inline(always)]
        pub fn txen(&self) -> bool {
            ((self.0 >> 0) & 1) != 0
        }
        /// Write to set the value of the TX pin
        #[inline(always)]
        pub fn txval(&self) -> bool {
            ((self.0 >> 1) & 1) != 0
        }
        /// Construct a WriteVal that can be used to modify the contents of this register value.
        #[inline(always)]
        pub fn modify(self) -> OvrdWriteVal {
            OvrdWriteVal(self.0)
        }
    }
    impl From<u32> for OvrdReadVal {
        #[inline(always)]
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<OvrdReadVal> for u32 {
        #[inline(always)]
        fn from(val: OvrdReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct OvrdWriteVal(u32);
    impl OvrdWriteVal {
        /// Enable TX pin override control
        #[inline(always)]
        pub fn txen(self, val: bool) -> Self {
            Self((self.0 & !(1 << 0)) | (u32::from(val) << 0))
        }
        /// Write to set the value of the TX pin
        #[inline(always)]
        pub fn txval(self, val: bool) -> Self {
            Self((self.0 & !(1 << 1)) | (u32::from(val) << 1))
        }
    }
    impl From<u32> for OvrdWriteVal {
        #[inline(always)]
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<OvrdWriteVal> for u32 {
        #[inline(always)]
        fn from(val: OvrdWriteVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct RdataReadVal(u32);
    impl RdataReadVal {
        /// UART read data
        #[inline(always)]
        pub fn rdata(&self) -> u32 {
            (self.0 >> 0) & 0xff
        }
    }
    impl From<u32> for RdataReadVal {
        #[inline(always)]
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<RdataReadVal> for u32 {
        #[inline(always)]
        fn from(val: RdataReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct StatusReadVal(u32);
    impl StatusReadVal {
        /// TX buffer is full
        #[inline(always)]
        pub fn txfull(&self) -> bool {
            ((self.0 >> 0) & 1) != 0
        }
        /// RX buffer is full
        #[inline(always)]
        pub fn rxfull(&self) -> bool {
            ((self.0 >> 1) & 1) != 0
        }
        /// TX FIFO is empty
        #[inline(always)]
        pub fn txempty(&self) -> bool {
            ((self.0 >> 2) & 1) != 0
        }
        /// TX FIFO is empty and all bits have been transmitted
        #[inline(always)]
        pub fn txidle(&self) -> bool {
            ((self.0 >> 3) & 1) != 0
        }
        /// RX is idle
        #[inline(always)]
        pub fn rxidle(&self) -> bool {
            ((self.0 >> 4) & 1) != 0
        }
        /// RX FIFO is empty
        #[inline(always)]
        pub fn rxempty(&self) -> bool {
            ((self.0 >> 5) & 1) != 0
        }
    }
    impl From<u32> for StatusReadVal {
        #[inline(always)]
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<StatusReadVal> for u32 {
        #[inline(always)]
        fn from(val: StatusReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct TimeoutCtrlReadVal(u32);
    impl TimeoutCtrlReadVal {
        /// RX timeout value in UART bit times
        #[inline(always)]
        pub fn val(&self) -> u32 {
            (self.0 >> 0) & 0xffffff
        }
        /// Enable RX timeout feature
        #[inline(always)]
        pub fn en(&self) -> bool {
            ((self.0 >> 31) & 1) != 0
        }
        /// Construct a WriteVal that can be used to modify the contents of this register value.
        #[inline(always)]
        pub fn modify(self) -> TimeoutCtrlWriteVal {
            TimeoutCtrlWriteVal(self.0)
        }
    }
    impl From<u32> for TimeoutCtrlReadVal {
        #[inline(always)]
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<TimeoutCtrlReadVal> for u32 {
        #[inline(always)]
        fn from(val: TimeoutCtrlReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct TimeoutCtrlWriteVal(u32);
    impl TimeoutCtrlWriteVal {
        /// RX timeout value in UART bit times
        #[inline(always)]
        pub fn val(self, val: u32) -> Self {
            Self((self.0 & !(0xffffff << 0)) | ((val & 0xffffff) << 0))
        }
        /// Enable RX timeout feature
        #[inline(always)]
        pub fn en(self, val: bool) -> Self {
            Self((self.0 & !(1 << 31)) | (u32::from(val) << 31))
        }
    }
    impl From<u32> for TimeoutCtrlWriteVal {
        #[inline(always)]
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<TimeoutCtrlWriteVal> for u32 {
        #[inline(always)]
        fn from(val: TimeoutCtrlWriteVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct ValReadVal(u32);
    impl ValReadVal {
        /// Last 16 oversampled values of RX. Most recent bit is bit 0, oldest 15.
        #[inline(always)]
        pub fn rx(&self) -> u32 {
            (self.0 >> 0) & 0xffff
        }
    }
    impl From<u32> for ValReadVal {
        #[inline(always)]
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<ValReadVal> for u32 {
        #[inline(always)]
        fn from(val: ValReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct WdataWriteVal(u32);
    impl WdataWriteVal {
        /// UART write data
        #[inline(always)]
        pub fn wdata(self, val: u32) -> Self {
            Self((self.0 & !(0xff << 0)) | ((val & 0xff) << 0))
        }
    }
    impl From<u32> for WdataWriteVal {
        #[inline(always)]
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<WdataWriteVal> for u32 {
        #[inline(always)]
        fn from(val: WdataWriteVal) -> u32 {
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
    pub type InterruptState = ureg::ReadWriteReg32<
        0,
        crate::uart::regs::InterruptStateReadVal,
        crate::uart::regs::InterruptStateWriteVal,
    >;
    pub type InterruptEnable = ureg::ReadWriteReg32<
        0,
        crate::uart::regs::InterruptEnableReadVal,
        crate::uart::regs::InterruptEnableWriteVal,
    >;
    pub type InterruptTest = ureg::WriteOnlyReg32<0, crate::uart::regs::InterruptTestWriteVal>;
    pub type AlertTest = ureg::WriteOnlyReg32<0, crate::uart::regs::AlertTestWriteVal>;
    pub type Ctrl =
        ureg::ReadWriteReg32<0, crate::uart::regs::CtrlReadVal, crate::uart::regs::CtrlWriteVal>;
    pub type Status = ureg::ReadOnlyReg32<crate::uart::regs::StatusReadVal>;
    pub type Rdata = ureg::ReadOnlyReg32<crate::uart::regs::RdataReadVal>;
    pub type Wdata = ureg::WriteOnlyReg32<0, crate::uart::regs::WdataWriteVal>;
    pub type FifoCtrl = ureg::ReadWriteReg32<
        0,
        crate::uart::regs::FifoCtrlReadVal,
        crate::uart::regs::FifoCtrlWriteVal,
    >;
    pub type FifoStatus = ureg::ReadOnlyReg32<crate::uart::regs::FifoStatusReadVal>;
    pub type Ovrd =
        ureg::ReadWriteReg32<0, crate::uart::regs::OvrdReadVal, crate::uart::regs::OvrdWriteVal>;
    pub type Val = ureg::ReadOnlyReg32<crate::uart::regs::ValReadVal>;
    pub type TimeoutCtrl = ureg::ReadWriteReg32<
        0,
        crate::uart::regs::TimeoutCtrlReadVal,
        crate::uart::regs::TimeoutCtrlWriteVal,
    >;
}
