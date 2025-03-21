#![doc = "Peripheral access API (generated using chiptool v0.1.0 (26983da 2025-01-02))"]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - CLOCK_POWER"]
    CLOCK_POWER = 0,
    #[doc = "1 - RADIO"]
    RADIO = 1,
    #[doc = "2 - UARTE0"]
    UARTE0 = 2,
    #[doc = "3 - TWI0"]
    TWI0 = 3,
    #[doc = "4 - SPI0"]
    SPI0 = 4,
    #[doc = "6 - GPIOTE"]
    GPIOTE = 6,
    #[doc = "7 - SAADC"]
    SAADC = 7,
    #[doc = "8 - TIMER0"]
    TIMER0 = 8,
    #[doc = "9 - TIMER1"]
    TIMER1 = 9,
    #[doc = "10 - TIMER2"]
    TIMER2 = 10,
    #[doc = "11 - RTC0"]
    RTC0 = 11,
    #[doc = "12 - TEMP"]
    TEMP = 12,
    #[doc = "13 - RNG"]
    RNG = 13,
    #[doc = "14 - ECB"]
    ECB = 14,
    #[doc = "15 - AAR_CCM"]
    AAR_CCM = 15,
    #[doc = "16 - WDT"]
    WDT = 16,
    #[doc = "17 - RTC1"]
    RTC1 = 17,
    #[doc = "18 - QDEC"]
    QDEC = 18,
    #[doc = "20 - EGU0_SWI0"]
    EGU0_SWI0 = 20,
    #[doc = "21 - EGU1_SWI1"]
    EGU1_SWI1 = 21,
    #[doc = "22 - SWI2"]
    SWI2 = 22,
    #[doc = "23 - SWI3"]
    SWI3 = 23,
    #[doc = "24 - SWI4"]
    SWI4 = 24,
    #[doc = "25 - SWI5"]
    SWI5 = 25,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors {
    extern "C" {
        fn CLOCK_POWER();
        fn RADIO();
        fn UARTE0();
        fn TWI0();
        fn SPI0();
        fn GPIOTE();
        fn SAADC();
        fn TIMER0();
        fn TIMER1();
        fn TIMER2();
        fn RTC0();
        fn TEMP();
        fn RNG();
        fn ECB();
        fn AAR_CCM();
        fn WDT();
        fn RTC1();
        fn QDEC();
        fn EGU0_SWI0();
        fn EGU1_SWI1();
        fn SWI2();
        fn SWI3();
        fn SWI4();
        fn SWI5();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[link_section = ".vector_table.interrupts"]
    #[no_mangle]
    pub static __INTERRUPTS: [Vector; 26] = [
        Vector {
            _handler: CLOCK_POWER,
        },
        Vector { _handler: RADIO },
        Vector { _handler: UARTE0 },
        Vector { _handler: TWI0 },
        Vector { _handler: SPI0 },
        Vector { _reserved: 0 },
        Vector { _handler: GPIOTE },
        Vector { _handler: SAADC },
        Vector { _handler: TIMER0 },
        Vector { _handler: TIMER1 },
        Vector { _handler: TIMER2 },
        Vector { _handler: RTC0 },
        Vector { _handler: TEMP },
        Vector { _handler: RNG },
        Vector { _handler: ECB },
        Vector { _handler: AAR_CCM },
        Vector { _handler: WDT },
        Vector { _handler: RTC1 },
        Vector { _handler: QDEC },
        Vector { _reserved: 0 },
        Vector {
            _handler: EGU0_SWI0,
        },
        Vector {
            _handler: EGU1_SWI1,
        },
        Vector { _handler: SWI2 },
        Vector { _handler: SWI3 },
        Vector { _handler: SWI4 },
        Vector { _handler: SWI5 },
    ];
}
#[doc = "Factory information configuration registers"]
pub const FICR: ficr::Ficr = unsafe { ficr::Ficr::from_ptr(0x1000_0000usize as _) };
#[doc = "User information configuration registers"]
pub const UICR: uicr::Uicr = unsafe { uicr::Uicr::from_ptr(0x1000_1000usize as _) };
#[doc = "Block Protect"]
pub const BPROT: bprot::Bprot = unsafe { bprot::Bprot::from_ptr(0x4000_0000usize as _) };
#[doc = "Only for emulation on devices that support hardened AP-PROTECT."]
pub const APPROTECT: approtect::Approtect =
    unsafe { approtect::Approtect::from_ptr(0x4000_0000usize as _) };
#[doc = "Clock control"]
pub const CLOCK: clock::Clock = unsafe { clock::Clock::from_ptr(0x4000_0000usize as _) };
#[doc = "Power control"]
pub const POWER: power::Power = unsafe { power::Power::from_ptr(0x4000_0000usize as _) };
#[doc = "2.4 GHz radio"]
pub const RADIO: radio::Radio = unsafe { radio::Radio::from_ptr(0x4000_1000usize as _) };
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub const UART0: uart::Uart = unsafe { uart::Uart::from_ptr(0x4000_2000usize as _) };
#[doc = "UART with EasyDMA"]
pub const UARTE0: uarte::Uarte = unsafe { uarte::Uarte::from_ptr(0x4000_2000usize as _) };
#[doc = "I2C compatible Two-Wire Interface"]
pub const TWI0: twi::Twi = unsafe { twi::Twi::from_ptr(0x4000_3000usize as _) };
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA"]
pub const TWIM0: twim::Twim = unsafe { twim::Twim::from_ptr(0x4000_3000usize as _) };
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA"]
pub const TWIS0: twis::Twis = unsafe { twis::Twis::from_ptr(0x4000_3000usize as _) };
#[doc = "Serial Peripheral Interface"]
pub const SPI0: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_4000usize as _) };
#[doc = "Serial Peripheral Interface Master with EasyDMA"]
pub const SPIM0: spim::Spim = unsafe { spim::Spim::from_ptr(0x4000_4000usize as _) };
#[doc = "SPI Slave"]
pub const SPIS0: spis::Spis = unsafe { spis::Spis::from_ptr(0x4000_4000usize as _) };
#[doc = "GPIO Tasks and Events"]
pub const GPIOTE: gpiote::Gpiote = unsafe { gpiote::Gpiote::from_ptr(0x4000_6000usize as _) };
#[doc = "Analog to Digital Converter"]
pub const SAADC: saadc::Saadc = unsafe { saadc::Saadc::from_ptr(0x4000_7000usize as _) };
#[doc = "Timer/Counter 0"]
pub const TIMER0: timer::Timer = unsafe { timer::Timer::from_ptr(0x4000_8000usize as _) };
#[doc = "Timer/Counter 1"]
pub const TIMER1: timer::Timer = unsafe { timer::Timer::from_ptr(0x4000_9000usize as _) };
#[doc = "Timer/Counter 2"]
pub const TIMER2: timer::Timer = unsafe { timer::Timer::from_ptr(0x4000_a000usize as _) };
#[doc = "Real time counter 0"]
pub const RTC0: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4000_b000usize as _) };
#[doc = "Temperature Sensor"]
pub const TEMP: temp::Temp = unsafe { temp::Temp::from_ptr(0x4000_c000usize as _) };
#[doc = "Random Number Generator"]
pub const RNG: rng::Rng = unsafe { rng::Rng::from_ptr(0x4000_d000usize as _) };
#[doc = "AES ECB Mode Encryption"]
pub const ECB: ecb::Ecb = unsafe { ecb::Ecb::from_ptr(0x4000_e000usize as _) };
#[doc = "Accelerated Address Resolver"]
pub const AAR: aar::Aar = unsafe { aar::Aar::from_ptr(0x4000_f000usize as _) };
#[doc = "AES CCM Mode Encryption"]
pub const CCM: ccm::Ccm = unsafe { ccm::Ccm::from_ptr(0x4000_f000usize as _) };
#[doc = "Watchdog Timer"]
pub const WDT: wdt::Wdt = unsafe { wdt::Wdt::from_ptr(0x4001_0000usize as _) };
#[doc = "Real time counter 1"]
pub const RTC1: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4001_1000usize as _) };
#[doc = "Quadrature Decoder"]
pub const QDEC: qdec::Qdec = unsafe { qdec::Qdec::from_ptr(0x4001_2000usize as _) };
#[doc = "Event generator unit 0"]
pub const EGU0: egu::Egu = unsafe { egu::Egu::from_ptr(0x4001_4000usize as _) };
#[doc = "Software interrupt 0"]
pub const SWI0: swi::Swi = unsafe { swi::Swi::from_ptr(0x4001_4000usize as _) };
#[doc = "Event generator unit 1"]
pub const EGU1: egu::Egu = unsafe { egu::Egu::from_ptr(0x4001_5000usize as _) };
#[doc = "Software interrupt 1"]
pub const SWI1: swi::Swi = unsafe { swi::Swi::from_ptr(0x4001_5000usize as _) };
#[doc = "Software interrupt 2"]
pub const SWI2: swi::Swi = unsafe { swi::Swi::from_ptr(0x4001_6000usize as _) };
#[doc = "Software interrupt 3"]
pub const SWI3: swi::Swi = unsafe { swi::Swi::from_ptr(0x4001_7000usize as _) };
#[doc = "Software interrupt 4"]
pub const SWI4: swi::Swi = unsafe { swi::Swi::from_ptr(0x4001_8000usize as _) };
#[doc = "Software interrupt 5"]
pub const SWI5: swi::Swi = unsafe { swi::Swi::from_ptr(0x4001_9000usize as _) };
#[doc = "Non Volatile Memory Controller"]
pub const NVMC: nvmc::Nvmc = unsafe { nvmc::Nvmc::from_ptr(0x4001_e000usize as _) };
#[doc = "Programmable Peripheral Interconnect"]
pub const PPI: ppi::Ppi = unsafe { ppi::Ppi::from_ptr(0x4001_f000usize as _) };
#[doc = "GPIO Port"]
pub const P0: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5000_0000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
// #[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
pub mod aar {
    #[doc = "Accelerated Address Resolver"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Aar {
        ptr: *mut u8,
    }
    unsafe impl Send for Aar {}
    unsafe impl Sync for Aar {}
    impl Aar {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Start resolving addresses based on IRKs specified in the IRK data structure"]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Stop resolving addresses"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Address resolution procedure complete"]
        #[inline(always)]
        pub const fn events_end(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "Address resolved"]
        #[inline(always)]
        pub const fn events_resolved(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "Address not resolved"]
        #[inline(always)]
        pub const fn events_notresolved(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Resolution status"]
        #[inline(always)]
        pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
        }
        #[doc = "Enable AAR"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Number of IRKs"]
        #[inline(always)]
        pub const fn nirk(self) -> crate::common::Reg<regs::Nirk, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Pointer to IRK data structure"]
        #[inline(always)]
        pub const fn irkptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Pointer to the resolvable address"]
        #[inline(always)]
        pub const fn addrptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
        }
        #[doc = "Pointer to data area used for temporary storage"]
        #[inline(always)]
        pub const fn scratchptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Enable AAR"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable AAR"]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable AAR"]
            #[inline(always)]
            pub fn set_enable(&mut self, val: super::vals::Enable) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        impl core::fmt::Debug for Enable {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Enable")
                    .field("enable", &self.enable())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Enable {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Enable {
                    enable: super::vals::Enable,
                }
                let proxy = Enable {
                    enable: self.enable(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Write '1' to disable interrupt for event END"]
            #[inline(always)]
            pub const fn end(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event END"]
            #[inline(always)]
            pub fn set_end(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to disable interrupt for event RESOLVED"]
            #[inline(always)]
            pub const fn resolved(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event RESOLVED"]
            #[inline(always)]
            pub fn set_resolved(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to disable interrupt for event NOTRESOLVED"]
            #[inline(always)]
            pub const fn notresolved(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event NOTRESOLVED"]
            #[inline(always)]
            pub fn set_notresolved(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("end", &self.end())
                    .field("resolved", &self.resolved())
                    .field("notresolved", &self.notresolved())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    end: bool,
                    resolved: bool,
                    notresolved: bool,
                }
                let proxy = Int {
                    end: self.end(),
                    resolved: self.resolved(),
                    notresolved: self.notresolved(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Number of IRKs"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Nirk(pub u32);
        impl Nirk {
            #[doc = "Number of Identity root keys available in the IRK data structure"]
            #[inline(always)]
            pub const fn nirk(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x1f;
                val as u8
            }
            #[doc = "Number of Identity root keys available in the IRK data structure"]
            #[inline(always)]
            pub fn set_nirk(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
            }
        }
        impl Default for Nirk {
            #[inline(always)]
            fn default() -> Nirk {
                Nirk(0)
            }
        }
        impl core::fmt::Debug for Nirk {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Nirk").field("nirk", &self.nirk()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Nirk {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Nirk {
                    nirk: u8,
                }
                let proxy = Nirk { nirk: self.nirk() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Resolution status"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Status(pub u32);
        impl Status {
            #[doc = "The IRK that was used last time an address was resolved"]
            #[inline(always)]
            pub const fn status(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "The IRK that was used last time an address was resolved"]
            #[inline(always)]
            pub fn set_status(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Status {
            #[inline(always)]
            fn default() -> Status {
                Status(0)
            }
        }
        impl core::fmt::Debug for Status {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Status")
                    .field("status", &self.status())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Status {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Status {
                    status: u8,
                }
                let proxy = Status {
                    status: self.status(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Enable {
            #[doc = "Disable"]
            DISABLED = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            #[doc = "Enable"]
            ENABLED = 0x03,
        }
        impl Enable {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Enable {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Enable {
            #[inline(always)]
            fn from(val: u8) -> Enable {
                Enable::from_bits(val)
            }
        }
        impl From<Enable> for u8 {
            #[inline(always)]
            fn from(val: Enable) -> u8 {
                Enable::to_bits(val)
            }
        }
    }
}
pub mod approtect {
    #[doc = "Only for emulation on devices that support hardened AP-PROTECT."]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Approtect {
        ptr: *mut u8,
    }
    unsafe impl Send for Approtect {}
    unsafe impl Sync for Approtect {}
    impl Approtect {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Software force enable APPROTECT mechanism until next reset."]
        #[inline(always)]
        pub const fn forceprotect(
            self,
        ) -> crate::common::Reg<regs::Forceprotect, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0550usize) as _) }
        }
        #[doc = "Software disable APPROTECT mechanism"]
        #[inline(always)]
        pub const fn disable(self) -> crate::common::Reg<regs::Disable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0558usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Software disable APPROTECT mechanism"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Disable(pub u32);
        impl Disable {
            #[doc = "Software disable APPROTECT mechanism"]
            #[inline(always)]
            pub const fn disable(&self) -> super::vals::Disable {
                let val = (self.0 >> 0usize) & 0xff;
                super::vals::Disable::from_bits(val as u8)
            }
            #[doc = "Software disable APPROTECT mechanism"]
            #[inline(always)]
            pub fn set_disable(&mut self, val: super::vals::Disable) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Disable {
            #[inline(always)]
            fn default() -> Disable {
                Disable(0)
            }
        }
        impl core::fmt::Debug for Disable {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Disable")
                    .field("disable", &self.disable())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Disable {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Disable {
                    disable: super::vals::Disable,
                }
                let proxy = Disable {
                    disable: self.disable(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Software force enable APPROTECT mechanism until next reset."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Forceprotect(pub u32);
        impl Forceprotect {
            #[doc = "Write 0x0 to force enable APPROTECT mechanism"]
            #[inline(always)]
            pub const fn forceprotect(&self) -> super::vals::Forceprotect {
                let val = (self.0 >> 0usize) & 0xff;
                super::vals::Forceprotect::from_bits(val as u8)
            }
            #[doc = "Write 0x0 to force enable APPROTECT mechanism"]
            #[inline(always)]
            pub fn set_forceprotect(&mut self, val: super::vals::Forceprotect) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Forceprotect {
            #[inline(always)]
            fn default() -> Forceprotect {
                Forceprotect(0)
            }
        }
        impl core::fmt::Debug for Forceprotect {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Forceprotect")
                    .field("forceprotect", &self.forceprotect())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Forceprotect {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Forceprotect {
                    forceprotect: super::vals::Forceprotect,
                }
                let proxy = Forceprotect {
                    forceprotect: self.forceprotect(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Disable(pub u8);
        impl Disable {
            #[doc = "Software disable APPROTECT mechanism"]
            pub const SW_DISABLE: Self = Self(0x5a);
        }
        impl Disable {
            pub const fn from_bits(val: u8) -> Disable {
                Self(val & 0xff)
            }
            pub const fn to_bits(self) -> u8 {
                self.0
            }
        }
        impl core::fmt::Debug for Disable {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x5a => f.write_str("SW_DISABLE"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Disable {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x5a => defmt::write!(f, "SW_DISABLE"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u8> for Disable {
            #[inline(always)]
            fn from(val: u8) -> Disable {
                Disable::from_bits(val)
            }
        }
        impl From<Disable> for u8 {
            #[inline(always)]
            fn from(val: Disable) -> u8 {
                Disable::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Forceprotect(pub u8);
        impl Forceprotect {
            #[doc = "Software force enable APPROTECT mechanism"]
            pub const FORCE: Self = Self(0x0);
        }
        impl Forceprotect {
            pub const fn from_bits(val: u8) -> Forceprotect {
                Self(val & 0xff)
            }
            pub const fn to_bits(self) -> u8 {
                self.0
            }
        }
        impl core::fmt::Debug for Forceprotect {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0 => f.write_str("FORCE"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Forceprotect {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0 => defmt::write!(f, "FORCE"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u8> for Forceprotect {
            #[inline(always)]
            fn from(val: u8) -> Forceprotect {
                Forceprotect::from_bits(val)
            }
        }
        impl From<Forceprotect> for u8 {
            #[inline(always)]
            fn from(val: Forceprotect) -> u8 {
                Forceprotect::to_bits(val)
            }
        }
    }
}
pub mod bprot {
    #[doc = "Block Protect"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bprot {
        ptr: *mut u8,
    }
    unsafe impl Send for Bprot {}
    unsafe impl Sync for Bprot {}
    impl Bprot {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Block protect configuration register 0"]
        #[inline(always)]
        pub const fn config0(self) -> crate::common::Reg<regs::Config0, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0600usize) as _) }
        }
        #[doc = "Block protect configuration register 1"]
        #[inline(always)]
        pub const fn config1(self) -> crate::common::Reg<regs::Config1, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0604usize) as _) }
        }
        #[doc = "Disable protection mechanism in debug mode"]
        #[inline(always)]
        pub const fn disableindebug(
            self,
        ) -> crate::common::Reg<regs::Disableindebug, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0608usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Block protect configuration register 0"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config0(pub u32);
        impl Config0 {
            #[doc = "Enable protection for region 0. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region0(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 0. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable protection for region 1. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region1(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 1. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable protection for region 2. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region2(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 2. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable protection for region 3. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region3(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 3. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Enable protection for region 4. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region4(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 4. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region4(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Enable protection for region 5. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region5(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 5. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region5(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Enable protection for region 6. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region6(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 6. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region6(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Enable protection for region 7. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region7(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 7. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region7(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Enable protection for region 8. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region8(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 8. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region8(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Enable protection for region 9. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region9(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 9. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region9(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Enable protection for region 10. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region10(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 10. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region10(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "Enable protection for region 11. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region11(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 11. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region11(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "Enable protection for region 12. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region12(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 12. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region12(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "Enable protection for region 13. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region13(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 13. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region13(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "Enable protection for region 14. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region14(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 14. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region14(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "Enable protection for region 15. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region15(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 15. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region15(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
            #[doc = "Enable protection for region 16. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region16(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 16. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region16(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "Enable protection for region 17. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region17(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 17. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region17(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "Enable protection for region 18. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region18(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 18. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region18(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "Enable protection for region 19. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region19(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 19. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region19(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "Enable protection for region 20. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region20(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 20. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region20(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "Enable protection for region 21. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region21(&self) -> bool {
                let val = (self.0 >> 21usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 21. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region21(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
            }
            #[doc = "Enable protection for region 22. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region22(&self) -> bool {
                let val = (self.0 >> 22usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 22. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region22(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
            }
            #[doc = "Enable protection for region 23. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region23(&self) -> bool {
                let val = (self.0 >> 23usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 23. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region23(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
            }
            #[doc = "Enable protection for region 24. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region24(&self) -> bool {
                let val = (self.0 >> 24usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 24. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region24(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
            }
            #[doc = "Enable protection for region 25. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region25(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 25. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region25(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
            }
            #[doc = "Enable protection for region 26. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region26(&self) -> bool {
                let val = (self.0 >> 26usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 26. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region26(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
            }
            #[doc = "Enable protection for region 27. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region27(&self) -> bool {
                let val = (self.0 >> 27usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 27. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region27(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
            }
            #[doc = "Enable protection for region 28. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region28(&self) -> bool {
                let val = (self.0 >> 28usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 28. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region28(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
            }
            #[doc = "Enable protection for region 29. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region29(&self) -> bool {
                let val = (self.0 >> 29usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 29. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region29(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
            }
            #[doc = "Enable protection for region 30. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region30(&self) -> bool {
                let val = (self.0 >> 30usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 30. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region30(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
            }
            #[doc = "Enable protection for region 31. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region31(&self) -> bool {
                let val = (self.0 >> 31usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 31. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region31(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
            }
        }
        impl Default for Config0 {
            #[inline(always)]
            fn default() -> Config0 {
                Config0(0)
            }
        }
        impl core::fmt::Debug for Config0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Config0")
                    .field("region0", &self.region0())
                    .field("region1", &self.region1())
                    .field("region2", &self.region2())
                    .field("region3", &self.region3())
                    .field("region4", &self.region4())
                    .field("region5", &self.region5())
                    .field("region6", &self.region6())
                    .field("region7", &self.region7())
                    .field("region8", &self.region8())
                    .field("region9", &self.region9())
                    .field("region10", &self.region10())
                    .field("region11", &self.region11())
                    .field("region12", &self.region12())
                    .field("region13", &self.region13())
                    .field("region14", &self.region14())
                    .field("region15", &self.region15())
                    .field("region16", &self.region16())
                    .field("region17", &self.region17())
                    .field("region18", &self.region18())
                    .field("region19", &self.region19())
                    .field("region20", &self.region20())
                    .field("region21", &self.region21())
                    .field("region22", &self.region22())
                    .field("region23", &self.region23())
                    .field("region24", &self.region24())
                    .field("region25", &self.region25())
                    .field("region26", &self.region26())
                    .field("region27", &self.region27())
                    .field("region28", &self.region28())
                    .field("region29", &self.region29())
                    .field("region30", &self.region30())
                    .field("region31", &self.region31())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Config0 {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Config0 {
                    region0: bool,
                    region1: bool,
                    region2: bool,
                    region3: bool,
                    region4: bool,
                    region5: bool,
                    region6: bool,
                    region7: bool,
                    region8: bool,
                    region9: bool,
                    region10: bool,
                    region11: bool,
                    region12: bool,
                    region13: bool,
                    region14: bool,
                    region15: bool,
                    region16: bool,
                    region17: bool,
                    region18: bool,
                    region19: bool,
                    region20: bool,
                    region21: bool,
                    region22: bool,
                    region23: bool,
                    region24: bool,
                    region25: bool,
                    region26: bool,
                    region27: bool,
                    region28: bool,
                    region29: bool,
                    region30: bool,
                    region31: bool,
                }
                let proxy = Config0 {
                    region0: self.region0(),
                    region1: self.region1(),
                    region2: self.region2(),
                    region3: self.region3(),
                    region4: self.region4(),
                    region5: self.region5(),
                    region6: self.region6(),
                    region7: self.region7(),
                    region8: self.region8(),
                    region9: self.region9(),
                    region10: self.region10(),
                    region11: self.region11(),
                    region12: self.region12(),
                    region13: self.region13(),
                    region14: self.region14(),
                    region15: self.region15(),
                    region16: self.region16(),
                    region17: self.region17(),
                    region18: self.region18(),
                    region19: self.region19(),
                    region20: self.region20(),
                    region21: self.region21(),
                    region22: self.region22(),
                    region23: self.region23(),
                    region24: self.region24(),
                    region25: self.region25(),
                    region26: self.region26(),
                    region27: self.region27(),
                    region28: self.region28(),
                    region29: self.region29(),
                    region30: self.region30(),
                    region31: self.region31(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Block protect configuration register 1"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config1(pub u32);
        impl Config1 {
            #[doc = "Enable protection for region 32. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region32(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 32. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region32(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable protection for region 33. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region33(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 33. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region33(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable protection for region 34. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region34(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 34. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region34(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable protection for region 35. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region35(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 35. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region35(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Enable protection for region 36. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region36(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 36. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region36(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Enable protection for region 37. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region37(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 37. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region37(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Enable protection for region 38. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region38(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 38. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region38(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Enable protection for region 39. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region39(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 39. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region39(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Enable protection for region 40. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region40(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 40. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region40(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Enable protection for region 41. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region41(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 41. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region41(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Enable protection for region 42. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region42(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 42. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region42(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "Enable protection for region 43. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region43(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 43. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region43(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "Enable protection for region 44. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region44(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 44. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region44(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "Enable protection for region 45. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region45(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 45. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region45(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "Enable protection for region 46. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region46(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 46. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region46(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "Enable protection for region 47. Write '0' has no effect."]
            #[inline(always)]
            pub const fn region47(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "Enable protection for region 47. Write '0' has no effect."]
            #[inline(always)]
            pub fn set_region47(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
        }
        impl Default for Config1 {
            #[inline(always)]
            fn default() -> Config1 {
                Config1(0)
            }
        }
        impl core::fmt::Debug for Config1 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Config1")
                    .field("region32", &self.region32())
                    .field("region33", &self.region33())
                    .field("region34", &self.region34())
                    .field("region35", &self.region35())
                    .field("region36", &self.region36())
                    .field("region37", &self.region37())
                    .field("region38", &self.region38())
                    .field("region39", &self.region39())
                    .field("region40", &self.region40())
                    .field("region41", &self.region41())
                    .field("region42", &self.region42())
                    .field("region43", &self.region43())
                    .field("region44", &self.region44())
                    .field("region45", &self.region45())
                    .field("region46", &self.region46())
                    .field("region47", &self.region47())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Config1 {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Config1 {
                    region32: bool,
                    region33: bool,
                    region34: bool,
                    region35: bool,
                    region36: bool,
                    region37: bool,
                    region38: bool,
                    region39: bool,
                    region40: bool,
                    region41: bool,
                    region42: bool,
                    region43: bool,
                    region44: bool,
                    region45: bool,
                    region46: bool,
                    region47: bool,
                }
                let proxy = Config1 {
                    region32: self.region32(),
                    region33: self.region33(),
                    region34: self.region34(),
                    region35: self.region35(),
                    region36: self.region36(),
                    region37: self.region37(),
                    region38: self.region38(),
                    region39: self.region39(),
                    region40: self.region40(),
                    region41: self.region41(),
                    region42: self.region42(),
                    region43: self.region43(),
                    region44: self.region44(),
                    region45: self.region45(),
                    region46: self.region46(),
                    region47: self.region47(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Disable protection mechanism in debug mode"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Disableindebug(pub u32);
        impl Disableindebug {
            #[doc = "Disable the protection mechanism for NVM regions while in debug mode. This register will only disable the protection mechanism if the device is in debug mode."]
            #[inline(always)]
            pub const fn disableindebug(&self) -> super::vals::Disableindebug {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Disableindebug::from_bits(val as u8)
            }
            #[doc = "Disable the protection mechanism for NVM regions while in debug mode. This register will only disable the protection mechanism if the device is in debug mode."]
            #[inline(always)]
            pub fn set_disableindebug(&mut self, val: super::vals::Disableindebug) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Disableindebug {
            #[inline(always)]
            fn default() -> Disableindebug {
                Disableindebug(0)
            }
        }
        impl core::fmt::Debug for Disableindebug {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Disableindebug")
                    .field("disableindebug", &self.disableindebug())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Disableindebug {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Disableindebug {
                    disableindebug: super::vals::Disableindebug,
                }
                let proxy = Disableindebug {
                    disableindebug: self.disableindebug(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Disableindebug {
            #[doc = "Enabled in debug"]
            ENABLED = 0x0,
            #[doc = "Disabled in debug"]
            DISABLED = 0x01,
        }
        impl Disableindebug {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Disableindebug {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Disableindebug {
            #[inline(always)]
            fn from(val: u8) -> Disableindebug {
                Disableindebug::from_bits(val)
            }
        }
        impl From<Disableindebug> for u8 {
            #[inline(always)]
            fn from(val: Disableindebug) -> u8 {
                Disableindebug::to_bits(val)
            }
        }
    }
}
pub mod ccm {
    #[doc = "AES CCM Mode Encryption"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccm {
        ptr: *mut u8,
    }
    unsafe impl Send for Ccm {}
    unsafe impl Sync for Ccm {}
    impl Ccm {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Start generation of keystream. This operation will stop by itself when completed."]
        #[inline(always)]
        pub const fn tasks_ksgen(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Start encryption/decryption. This operation will stop by itself when completed."]
        #[inline(always)]
        pub const fn tasks_crypt(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Stop encryption/decryption"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Override DATARATE setting in MODE register with the contents of the RATEOVERRIDE register for any ongoing encryption/decryption"]
        #[inline(always)]
        pub const fn tasks_rateoverride(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Keystream generation complete"]
        #[inline(always)]
        pub const fn events_endksgen(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "Encrypt/decrypt complete"]
        #[inline(always)]
        pub const fn events_endcrypt(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "Deprecated register - CCM error event"]
        #[inline(always)]
        pub const fn events_error(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "MIC check result"]
        #[inline(always)]
        pub const fn micstatus(self) -> crate::common::Reg<regs::Micstatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
        }
        #[doc = "Enable"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Operation mode"]
        #[inline(always)]
        pub const fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Pointer to data structure holding AES key and NONCE vector"]
        #[inline(always)]
        pub const fn cnfptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Input pointer"]
        #[inline(always)]
        pub const fn inptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
        }
        #[doc = "Output pointer"]
        #[inline(always)]
        pub const fn outptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
        }
        #[doc = "Pointer to data area used for temporary storage"]
        #[inline(always)]
        pub const fn scratchptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
        }
        #[doc = "Length of keystream generated when MODE.LENGTH = Extended."]
        #[inline(always)]
        pub const fn maxpacketsize(
            self,
        ) -> crate::common::Reg<regs::Maxpacketsize, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
        }
        #[doc = "Data rate override setting."]
        #[inline(always)]
        pub const fn rateoverride(
            self,
        ) -> crate::common::Reg<regs::Rateoverride, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Enable"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable CCM"]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable CCM"]
            #[inline(always)]
            pub fn set_enable(&mut self, val: super::vals::Enable) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        impl core::fmt::Debug for Enable {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Enable")
                    .field("enable", &self.enable())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Enable {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Enable {
                    enable: super::vals::Enable,
                }
                let proxy = Enable {
                    enable: self.enable(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Write '1' to disable interrupt for event ENDKSGEN"]
            #[inline(always)]
            pub const fn endksgen(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event ENDKSGEN"]
            #[inline(always)]
            pub fn set_endksgen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to disable interrupt for event ENDCRYPT"]
            #[inline(always)]
            pub const fn endcrypt(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event ENDCRYPT"]
            #[inline(always)]
            pub fn set_endcrypt(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Deprecated intclrfield - Write '1' to disable interrupt for event ERROR"]
            #[inline(always)]
            pub const fn error(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Deprecated intclrfield - Write '1' to disable interrupt for event ERROR"]
            #[inline(always)]
            pub fn set_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("endksgen", &self.endksgen())
                    .field("endcrypt", &self.endcrypt())
                    .field("error", &self.error())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    endksgen: bool,
                    endcrypt: bool,
                    error: bool,
                }
                let proxy = Int {
                    endksgen: self.endksgen(),
                    endcrypt: self.endcrypt(),
                    error: self.error(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Length of keystream generated when MODE.LENGTH = Extended."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Maxpacketsize(pub u32);
        impl Maxpacketsize {
            #[doc = "Length of keystream generated when MODE.LENGTH = Extended. This value must be greater or equal to the subsequent packet payload to be encrypted/decrypted."]
            #[inline(always)]
            pub const fn maxpacketsize(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Length of keystream generated when MODE.LENGTH = Extended. This value must be greater or equal to the subsequent packet payload to be encrypted/decrypted."]
            #[inline(always)]
            pub fn set_maxpacketsize(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Maxpacketsize {
            #[inline(always)]
            fn default() -> Maxpacketsize {
                Maxpacketsize(0)
            }
        }
        impl core::fmt::Debug for Maxpacketsize {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Maxpacketsize")
                    .field("maxpacketsize", &self.maxpacketsize())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Maxpacketsize {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Maxpacketsize {
                    maxpacketsize: u8,
                }
                let proxy = Maxpacketsize {
                    maxpacketsize: self.maxpacketsize(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "MIC check result"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Micstatus(pub u32);
        impl Micstatus {
            #[doc = "The result of the MIC check performed during the previous decryption operation"]
            #[inline(always)]
            pub const fn micstatus(&self) -> super::vals::Micstatus {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Micstatus::from_bits(val as u8)
            }
            #[doc = "The result of the MIC check performed during the previous decryption operation"]
            #[inline(always)]
            pub fn set_micstatus(&mut self, val: super::vals::Micstatus) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Micstatus {
            #[inline(always)]
            fn default() -> Micstatus {
                Micstatus(0)
            }
        }
        impl core::fmt::Debug for Micstatus {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Micstatus")
                    .field("micstatus", &self.micstatus())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Micstatus {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Micstatus {
                    micstatus: super::vals::Micstatus,
                }
                let proxy = Micstatus {
                    micstatus: self.micstatus(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Operation mode"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Mode(pub u32);
        impl Mode {
            #[doc = "The mode of operation to be used. The settings in this register apply whenever either the KSGEN or CRYPT tasks are triggered."]
            #[inline(always)]
            pub const fn mode(&self) -> super::vals::Mode {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Mode::from_bits(val as u8)
            }
            #[doc = "The mode of operation to be used. The settings in this register apply whenever either the KSGEN or CRYPT tasks are triggered."]
            #[inline(always)]
            pub fn set_mode(&mut self, val: super::vals::Mode) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Radio data rate that the CCM shall run synchronous with"]
            #[inline(always)]
            pub const fn datarate(&self) -> super::vals::Datarate {
                let val = (self.0 >> 16usize) & 0x03;
                super::vals::Datarate::from_bits(val as u8)
            }
            #[doc = "Radio data rate that the CCM shall run synchronous with"]
            #[inline(always)]
            pub fn set_datarate(&mut self, val: super::vals::Datarate) {
                self.0 =
                    (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
            }
            #[doc = "Packet length configuration"]
            #[inline(always)]
            pub const fn length(&self) -> super::vals::Length {
                let val = (self.0 >> 24usize) & 0x01;
                super::vals::Length::from_bits(val as u8)
            }
            #[doc = "Packet length configuration"]
            #[inline(always)]
            pub fn set_length(&mut self, val: super::vals::Length) {
                self.0 =
                    (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
            }
        }
        impl Default for Mode {
            #[inline(always)]
            fn default() -> Mode {
                Mode(0)
            }
        }
        impl core::fmt::Debug for Mode {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Mode")
                    .field("mode", &self.mode())
                    .field("datarate", &self.datarate())
                    .field("length", &self.length())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Mode {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Mode {
                    mode: super::vals::Mode,
                    datarate: super::vals::Datarate,
                    length: super::vals::Length,
                }
                let proxy = Mode {
                    mode: self.mode(),
                    datarate: self.datarate(),
                    length: self.length(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Data rate override setting."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rateoverride(pub u32);
        impl Rateoverride {
            #[doc = "Data rate override setting."]
            #[inline(always)]
            pub const fn rateoverride(&self) -> super::vals::Rateoverride {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Rateoverride::from_bits(val as u8)
            }
            #[doc = "Data rate override setting."]
            #[inline(always)]
            pub fn set_rateoverride(&mut self, val: super::vals::Rateoverride) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Rateoverride {
            #[inline(always)]
            fn default() -> Rateoverride {
                Rateoverride(0)
            }
        }
        impl core::fmt::Debug for Rateoverride {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Rateoverride")
                    .field("rateoverride", &self.rateoverride())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rateoverride {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Rateoverride {
                    rateoverride: super::vals::Rateoverride,
                }
                let proxy = Rateoverride {
                    rateoverride: self.rateoverride(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event ENDKSGEN and task CRYPT"]
            #[inline(always)]
            pub const fn endksgen_crypt(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event ENDKSGEN and task CRYPT"]
            #[inline(always)]
            pub fn set_endksgen_crypt(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
        impl core::fmt::Debug for Shorts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Shorts")
                    .field("endksgen_crypt", &self.endksgen_crypt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Shorts {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Shorts {
                    endksgen_crypt: bool,
                }
                let proxy = Shorts {
                    endksgen_crypt: self.endksgen_crypt(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Datarate {
            #[doc = "1 Mbps"]
            _1MBIT = 0x0,
            #[doc = "2 Mbps"]
            _2MBIT = 0x01,
            #[doc = "125 Kbps"]
            _125KBPS = 0x02,
            #[doc = "500 Kbps"]
            _500KBPS = 0x03,
        }
        impl Datarate {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Datarate {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Datarate {
            #[inline(always)]
            fn from(val: u8) -> Datarate {
                Datarate::from_bits(val)
            }
        }
        impl From<Datarate> for u8 {
            #[inline(always)]
            fn from(val: Datarate) -> u8 {
                Datarate::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Enable {
            #[doc = "Disable"]
            DISABLED = 0x0,
            _RESERVED_1 = 0x01,
            #[doc = "Enable"]
            ENABLED = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl Enable {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Enable {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Enable {
            #[inline(always)]
            fn from(val: u8) -> Enable {
                Enable::from_bits(val)
            }
        }
        impl From<Enable> for u8 {
            #[inline(always)]
            fn from(val: Enable) -> u8 {
                Enable::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Length {
            #[doc = "Default length. Effective length of LENGTH field in encrypted/decrypted packet is 5 bits. A keystream for packet payloads up to 27 bytes will be generated."]
            DEFAULT = 0x0,
            #[doc = "Extended length. Effective length of LENGTH field in encrypted/decrypted packet is 8 bits. A keystream for packet payloads up to MAXPACKETSIZE bytes will be generated."]
            EXTENDED = 0x01,
        }
        impl Length {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Length {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Length {
            #[inline(always)]
            fn from(val: u8) -> Length {
                Length::from_bits(val)
            }
        }
        impl From<Length> for u8 {
            #[inline(always)]
            fn from(val: Length) -> u8 {
                Length::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Micstatus {
            #[doc = "MIC check failed"]
            CHECK_FAILED = 0x0,
            #[doc = "MIC check passed"]
            CHECK_PASSED = 0x01,
        }
        impl Micstatus {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Micstatus {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Micstatus {
            #[inline(always)]
            fn from(val: u8) -> Micstatus {
                Micstatus::from_bits(val)
            }
        }
        impl From<Micstatus> for u8 {
            #[inline(always)]
            fn from(val: Micstatus) -> u8 {
                Micstatus::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Mode {
            #[doc = "AES CCM packet encryption mode"]
            ENCRYPTION = 0x0,
            #[doc = "AES CCM packet decryption mode"]
            DECRYPTION = 0x01,
        }
        impl Mode {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Mode {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Mode {
            #[inline(always)]
            fn from(val: u8) -> Mode {
                Mode::from_bits(val)
            }
        }
        impl From<Mode> for u8 {
            #[inline(always)]
            fn from(val: Mode) -> u8 {
                Mode::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Rateoverride {
            #[doc = "1 Mbps"]
            _1MBIT = 0x0,
            #[doc = "2 Mbps"]
            _2MBIT = 0x01,
            #[doc = "125 Kbps"]
            _125KBPS = 0x02,
            #[doc = "500 Kbps"]
            _500KBPS = 0x03,
        }
        impl Rateoverride {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Rateoverride {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Rateoverride {
            #[inline(always)]
            fn from(val: u8) -> Rateoverride {
                Rateoverride::from_bits(val)
            }
        }
        impl From<Rateoverride> for u8 {
            #[inline(always)]
            fn from(val: Rateoverride) -> u8 {
                Rateoverride::to_bits(val)
            }
        }
    }
}
pub mod clock {
    #[doc = "Clock control"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clock {
        ptr: *mut u8,
    }
    unsafe impl Send for Clock {}
    unsafe impl Sync for Clock {}
    impl Clock {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Start HFCLK crystal oscillator"]
        #[inline(always)]
        pub const fn tasks_hfclkstart(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Stop HFCLK crystal oscillator"]
        #[inline(always)]
        pub const fn tasks_hfclkstop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Start LFCLK source"]
        #[inline(always)]
        pub const fn tasks_lfclkstart(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Stop LFCLK source"]
        #[inline(always)]
        pub const fn tasks_lfclkstop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Start calibration of LFRC oscillator"]
        #[inline(always)]
        pub const fn tasks_cal(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "Start calibration timer"]
        #[inline(always)]
        pub const fn tasks_ctstart(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "Stop calibration timer"]
        #[inline(always)]
        pub const fn tasks_ctstop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
        }
        #[doc = "HFCLK oscillator started"]
        #[inline(always)]
        pub const fn events_hfclkstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "LFCLK started"]
        #[inline(always)]
        pub const fn events_lfclkstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "Calibration of LFCLK RC oscillator complete event"]
        #[inline(always)]
        pub const fn events_done(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
        }
        #[doc = "Calibration timer timeout"]
        #[inline(always)]
        pub const fn events_ctto(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Status indicating that HFCLKSTART task has been triggered"]
        #[inline(always)]
        pub const fn hfclkrun(self) -> crate::common::Reg<regs::Hfclkrun, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0408usize) as _) }
        }
        #[doc = "HFCLK status"]
        #[inline(always)]
        pub const fn hfclkstat(self) -> crate::common::Reg<regs::Hfclkstat, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x040cusize) as _) }
        }
        #[doc = "Status indicating that LFCLKSTART task has been triggered"]
        #[inline(always)]
        pub const fn lfclkrun(self) -> crate::common::Reg<regs::Lfclkrun, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0414usize) as _) }
        }
        #[doc = "LFCLK status"]
        #[inline(always)]
        pub const fn lfclkstat(self) -> crate::common::Reg<regs::Lfclkstat, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0418usize) as _) }
        }
        #[doc = "Copy of LFCLKSRC register, set when LFCLKSTART task was triggered"]
        #[inline(always)]
        pub const fn lfclksrccopy(
            self,
        ) -> crate::common::Reg<regs::Lfclksrccopy, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x041cusize) as _) }
        }
        #[doc = "Clock source for the LFCLK"]
        #[inline(always)]
        pub const fn lfclksrc(self) -> crate::common::Reg<regs::Lfclksrc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
        }
        #[doc = "Calibration timer interval"]
        #[inline(always)]
        pub const fn ctiv(self) -> crate::common::Reg<regs::Ctiv, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0538usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Calibration timer interval"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctiv(pub u32);
        impl Ctiv {
            #[doc = "Calibration timer interval in multiple of 0.25 seconds. Range: 0.25 seconds to 31.75 seconds."]
            #[inline(always)]
            pub const fn ctiv(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x7f;
                val as u8
            }
            #[doc = "Calibration timer interval in multiple of 0.25 seconds. Range: 0.25 seconds to 31.75 seconds."]
            #[inline(always)]
            pub fn set_ctiv(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
            }
        }
        impl Default for Ctiv {
            #[inline(always)]
            fn default() -> Ctiv {
                Ctiv(0)
            }
        }
        impl core::fmt::Debug for Ctiv {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ctiv").field("ctiv", &self.ctiv()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ctiv {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Ctiv {
                    ctiv: u8,
                }
                let proxy = Ctiv { ctiv: self.ctiv() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Status indicating that HFCLKSTART task has been triggered"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Hfclkrun(pub u32);
        impl Hfclkrun {
            #[doc = "HFCLKSTART task triggered or not"]
            #[inline(always)]
            pub const fn status(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "HFCLKSTART task triggered or not"]
            #[inline(always)]
            pub fn set_status(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Hfclkrun {
            #[inline(always)]
            fn default() -> Hfclkrun {
                Hfclkrun(0)
            }
        }
        impl core::fmt::Debug for Hfclkrun {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Hfclkrun")
                    .field("status", &self.status())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Hfclkrun {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Hfclkrun {
                    status: bool,
                }
                let proxy = Hfclkrun {
                    status: self.status(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "HFCLK status"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Hfclkstat(pub u32);
        impl Hfclkstat {
            #[doc = "Source of HFCLK"]
            #[inline(always)]
            pub const fn src(&self) -> super::vals::HfclkstatSrc {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::HfclkstatSrc::from_bits(val as u8)
            }
            #[doc = "Source of HFCLK"]
            #[inline(always)]
            pub fn set_src(&mut self, val: super::vals::HfclkstatSrc) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "HFCLK state"]
            #[inline(always)]
            pub const fn state(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "HFCLK state"]
            #[inline(always)]
            pub fn set_state(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
        }
        impl Default for Hfclkstat {
            #[inline(always)]
            fn default() -> Hfclkstat {
                Hfclkstat(0)
            }
        }
        impl core::fmt::Debug for Hfclkstat {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Hfclkstat")
                    .field("src", &self.src())
                    .field("state", &self.state())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Hfclkstat {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Hfclkstat {
                    src: super::vals::HfclkstatSrc,
                    state: bool,
                }
                let proxy = Hfclkstat {
                    src: self.src(),
                    state: self.state(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Write '1' to disable interrupt for event HFCLKSTARTED"]
            #[inline(always)]
            pub const fn hfclkstarted(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event HFCLKSTARTED"]
            #[inline(always)]
            pub fn set_hfclkstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to disable interrupt for event LFCLKSTARTED"]
            #[inline(always)]
            pub const fn lfclkstarted(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event LFCLKSTARTED"]
            #[inline(always)]
            pub fn set_lfclkstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to disable interrupt for event DONE"]
            #[inline(always)]
            pub const fn done(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event DONE"]
            #[inline(always)]
            pub fn set_done(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Write '1' to disable interrupt for event CTTO"]
            #[inline(always)]
            pub const fn ctto(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event CTTO"]
            #[inline(always)]
            pub fn set_ctto(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("hfclkstarted", &self.hfclkstarted())
                    .field("lfclkstarted", &self.lfclkstarted())
                    .field("done", &self.done())
                    .field("ctto", &self.ctto())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    hfclkstarted: bool,
                    lfclkstarted: bool,
                    done: bool,
                    ctto: bool,
                }
                let proxy = Int {
                    hfclkstarted: self.hfclkstarted(),
                    lfclkstarted: self.lfclkstarted(),
                    done: self.done(),
                    ctto: self.ctto(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Status indicating that LFCLKSTART task has been triggered"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Lfclkrun(pub u32);
        impl Lfclkrun {
            #[doc = "LFCLKSTART task triggered or not"]
            #[inline(always)]
            pub const fn status(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "LFCLKSTART task triggered or not"]
            #[inline(always)]
            pub fn set_status(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Lfclkrun {
            #[inline(always)]
            fn default() -> Lfclkrun {
                Lfclkrun(0)
            }
        }
        impl core::fmt::Debug for Lfclkrun {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Lfclkrun")
                    .field("status", &self.status())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Lfclkrun {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Lfclkrun {
                    status: bool,
                }
                let proxy = Lfclkrun {
                    status: self.status(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Clock source for the LFCLK"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Lfclksrc(pub u32);
        impl Lfclksrc {
            #[doc = "Clock source"]
            #[inline(always)]
            pub const fn src(&self) -> super::vals::Lfclksrc {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Lfclksrc::from_bits(val as u8)
            }
            #[doc = "Clock source"]
            #[inline(always)]
            pub fn set_src(&mut self, val: super::vals::Lfclksrc) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
            #[doc = "Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
            #[inline(always)]
            pub const fn bypass(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
            #[inline(always)]
            pub fn set_bypass(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "Enable or disable external source for LFCLK"]
            #[inline(always)]
            pub const fn external(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable external source for LFCLK"]
            #[inline(always)]
            pub fn set_external(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
        }
        impl Default for Lfclksrc {
            #[inline(always)]
            fn default() -> Lfclksrc {
                Lfclksrc(0)
            }
        }
        impl core::fmt::Debug for Lfclksrc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Lfclksrc")
                    .field("src", &self.src())
                    .field("bypass", &self.bypass())
                    .field("external", &self.external())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Lfclksrc {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Lfclksrc {
                    src: super::vals::Lfclksrc,
                    bypass: bool,
                    external: bool,
                }
                let proxy = Lfclksrc {
                    src: self.src(),
                    bypass: self.bypass(),
                    external: self.external(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Copy of LFCLKSRC register, set when LFCLKSTART task was triggered"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Lfclksrccopy(pub u32);
        impl Lfclksrccopy {
            #[doc = "Clock source"]
            #[inline(always)]
            pub const fn src(&self) -> super::vals::Lfclksrc {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Lfclksrc::from_bits(val as u8)
            }
            #[doc = "Clock source"]
            #[inline(always)]
            pub fn set_src(&mut self, val: super::vals::Lfclksrc) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Lfclksrccopy {
            #[inline(always)]
            fn default() -> Lfclksrccopy {
                Lfclksrccopy(0)
            }
        }
        impl core::fmt::Debug for Lfclksrccopy {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Lfclksrccopy")
                    .field("src", &self.src())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Lfclksrccopy {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Lfclksrccopy {
                    src: super::vals::Lfclksrc,
                }
                let proxy = Lfclksrccopy { src: self.src() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "LFCLK status"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Lfclkstat(pub u32);
        impl Lfclkstat {
            #[doc = "Source of LFCLK"]
            #[inline(always)]
            pub const fn src(&self) -> super::vals::Lfclksrc {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Lfclksrc::from_bits(val as u8)
            }
            #[doc = "Source of LFCLK"]
            #[inline(always)]
            pub fn set_src(&mut self, val: super::vals::Lfclksrc) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
            #[doc = "LFCLK state"]
            #[inline(always)]
            pub const fn state(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "LFCLK state"]
            #[inline(always)]
            pub fn set_state(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
        }
        impl Default for Lfclkstat {
            #[inline(always)]
            fn default() -> Lfclkstat {
                Lfclkstat(0)
            }
        }
        impl core::fmt::Debug for Lfclkstat {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Lfclkstat")
                    .field("src", &self.src())
                    .field("state", &self.state())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Lfclkstat {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Lfclkstat {
                    src: super::vals::Lfclksrc,
                    state: bool,
                }
                let proxy = Lfclkstat {
                    src: self.src(),
                    state: self.state(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum HfclkstatSrc {
            #[doc = "64 MHz internal oscillator (HFINT)"]
            RC = 0x0,
            #[doc = "64 MHz crystal oscillator (HFXO)"]
            XTAL = 0x01,
        }
        impl HfclkstatSrc {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> HfclkstatSrc {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for HfclkstatSrc {
            #[inline(always)]
            fn from(val: u8) -> HfclkstatSrc {
                HfclkstatSrc::from_bits(val)
            }
        }
        impl From<HfclkstatSrc> for u8 {
            #[inline(always)]
            fn from(val: HfclkstatSrc) -> u8 {
                HfclkstatSrc::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Lfclksrc {
            #[doc = "32.768 kHz RC oscillator"]
            RC = 0x0,
            #[doc = "32.768 kHz crystal oscillator"]
            XTAL = 0x01,
            #[doc = "32.768 kHz synthesized from HFCLK"]
            SYNTH = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl Lfclksrc {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Lfclksrc {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Lfclksrc {
            #[inline(always)]
            fn from(val: u8) -> Lfclksrc {
                Lfclksrc::from_bits(val)
            }
        }
        impl From<Lfclksrc> for u8 {
            #[inline(always)]
            fn from(val: Lfclksrc) -> u8 {
                Lfclksrc::to_bits(val)
            }
        }
    }
}
pub mod common {
    use core::marker::PhantomData;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct RW;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct R;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct W;
    mod sealed {
        use super::*;
        pub trait Access {}
        impl Access for R {}
        impl Access for W {}
        impl Access for RW {}
    }
    pub trait Access: sealed::Access + Copy {}
    impl Access for R {}
    impl Access for W {}
    impl Access for RW {}
    pub trait Read: Access {}
    impl Read for RW {}
    impl Read for R {}
    pub trait Write: Access {}
    impl Write for RW {}
    impl Write for W {}
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct Reg<T: Copy, A: Access> {
        ptr: *mut u8,
        phantom: PhantomData<*mut (T, A)>,
    }
    unsafe impl<T: Copy, A: Access> Send for Reg<T, A> {}
    unsafe impl<T: Copy, A: Access> Sync for Reg<T, A> {}
    impl<T: Copy, A: Access> Reg<T, A> {
        #[allow(clippy::missing_safety_doc)]
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut T) -> Self {
            Self {
                ptr: ptr as _,
                phantom: PhantomData,
            }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut T {
            self.ptr as _
        }
    }
    impl<T: Copy, A: Read> Reg<T, A> {
        #[inline(always)]
        pub fn read(&self) -> T {
            unsafe { (self.ptr as *mut T).read_volatile() }
        }
    }
    impl<T: Copy, A: Write> Reg<T, A> {
        #[inline(always)]
        pub fn write_value(&self, val: T) {
            unsafe { (self.ptr as *mut T).write_volatile(val) }
        }
    }
    impl<T: Default + Copy, A: Write> Reg<T, A> {
        #[inline(always)]
        pub fn write<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            let mut val = Default::default();
            let res = f(&mut val);
            self.write_value(val);
            res
        }
    }
    impl<T: Copy, A: Read + Write> Reg<T, A> {
        #[inline(always)]
        pub fn modify<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            let mut val = self.read();
            let res = f(&mut val);
            self.write_value(val);
            res
        }
    }
}
pub mod ecb {
    #[doc = "AES ECB Mode Encryption"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ecb {
        ptr: *mut u8,
    }
    unsafe impl Send for Ecb {}
    unsafe impl Sync for Ecb {}
    impl Ecb {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Start ECB block encrypt"]
        #[inline(always)]
        pub const fn tasks_startecb(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Abort a possible executing ECB operation"]
        #[inline(always)]
        pub const fn tasks_stopecb(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "ECB block encrypt complete"]
        #[inline(always)]
        pub const fn events_endecb(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "ECB block encrypt aborted because of a STOPECB task or due to an error"]
        #[inline(always)]
        pub const fn events_errorecb(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "ECB block encrypt memory pointers"]
        #[inline(always)]
        pub const fn ecbdataptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Write '1' to disable interrupt for event ENDECB"]
            #[inline(always)]
            pub const fn endecb(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event ENDECB"]
            #[inline(always)]
            pub fn set_endecb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to disable interrupt for event ERRORECB"]
            #[inline(always)]
            pub const fn errorecb(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event ERRORECB"]
            #[inline(always)]
            pub fn set_errorecb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("endecb", &self.endecb())
                    .field("errorecb", &self.errorecb())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    endecb: bool,
                    errorecb: bool,
                }
                let proxy = Int {
                    endecb: self.endecb(),
                    errorecb: self.errorecb(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
}
pub mod egu {
    #[doc = "Event generator unit 0"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Egu {
        ptr: *mut u8,
    }
    unsafe impl Send for Egu {}
    unsafe impl Sync for Egu {}
    impl Egu {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description collection: Trigger n for triggering the corresponding TRIGGERED\\[n\\] event"]
        #[inline(always)]
        pub const fn tasks_trigger(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
            assert!(n < 16usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Event number n generated by triggering the corresponding TRIGGER\\[n\\] task"]
        #[inline(always)]
        pub const fn events_triggered(
            self,
            n: usize,
        ) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 16usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Enable or disable interrupt for event TRIGGERED\\[0\\]"]
            #[inline(always)]
            pub const fn triggered(&self, n: usize) -> bool {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event TRIGGERED\\[0\\]"]
            #[inline(always)]
            pub fn set_triggered(&mut self, n: usize, val: bool) {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field(
                        "triggered",
                        &[
                            self.triggered(0usize),
                            self.triggered(1usize),
                            self.triggered(2usize),
                            self.triggered(3usize),
                            self.triggered(4usize),
                            self.triggered(5usize),
                            self.triggered(6usize),
                            self.triggered(7usize),
                            self.triggered(8usize),
                            self.triggered(9usize),
                            self.triggered(10usize),
                            self.triggered(11usize),
                            self.triggered(12usize),
                            self.triggered(13usize),
                            self.triggered(14usize),
                            self.triggered(15usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    triggered: [bool; 16usize],
                }
                let proxy = Int {
                    triggered: [
                        self.triggered(0usize),
                        self.triggered(1usize),
                        self.triggered(2usize),
                        self.triggered(3usize),
                        self.triggered(4usize),
                        self.triggered(5usize),
                        self.triggered(6usize),
                        self.triggered(7usize),
                        self.triggered(8usize),
                        self.triggered(9usize),
                        self.triggered(10usize),
                        self.triggered(11usize),
                        self.triggered(12usize),
                        self.triggered(13usize),
                        self.triggered(14usize),
                        self.triggered(15usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
}
pub mod ficr {
    #[doc = "Factory information configuration registers"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ficr {
        ptr: *mut u8,
    }
    unsafe impl Send for Ficr {}
    unsafe impl Sync for Ficr {}
    impl Ficr {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Code memory page size"]
        #[inline(always)]
        pub const fn codepagesize(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "Code memory size"]
        #[inline(always)]
        pub const fn codesize(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "Description collection: Device identifier"]
        #[inline(always)]
        pub const fn deviceid(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
            assert!(n < 2usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Encryption root, word n"]
        #[inline(always)]
        pub const fn er(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
            assert!(n < 4usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Identity root, word n"]
        #[inline(always)]
        pub const fn ir(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
            assert!(n < 4usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize + n * 4usize) as _) }
        }
        #[doc = "Device address type"]
        #[inline(always)]
        pub const fn deviceaddrtype(
            self,
        ) -> crate::common::Reg<regs::Deviceaddrtype, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
        }
        #[doc = "Description collection: Device address n"]
        #[inline(always)]
        pub const fn deviceaddr(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
            assert!(n < 2usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize + n * 4usize) as _) }
        }
        #[doc = "Device info"]
        #[inline(always)]
        pub const fn info(self) -> Info {
            unsafe { Info::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "Registers storing factory TEMP module linearization coefficients"]
        #[inline(always)]
        pub const fn temp(self) -> Temp {
            unsafe { Temp::from_ptr(self.ptr.add(0x0404usize) as _) }
        }
    }
    #[doc = "Device info"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Info {
        ptr: *mut u8,
    }
    unsafe impl Send for Info {}
    unsafe impl Sync for Info {}
    impl Info {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Part code"]
        #[inline(always)]
        pub const fn part(self) -> crate::common::Reg<regs::Part, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Part variant, hardware version and production configuration"]
        #[inline(always)]
        pub const fn variant(self) -> crate::common::Reg<regs::Variant, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Package option"]
        #[inline(always)]
        pub const fn package(self) -> crate::common::Reg<regs::Package, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "RAM variant"]
        #[inline(always)]
        pub const fn ram(self) -> crate::common::Reg<regs::Ram, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Flash variant"]
        #[inline(always)]
        pub const fn flash(self) -> crate::common::Reg<regs::Flash, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
    }
    #[doc = "Registers storing factory TEMP module linearization coefficients"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Temp {
        ptr: *mut u8,
    }
    unsafe impl Send for Temp {}
    unsafe impl Sync for Temp {}
    impl Temp {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Slope definition A0"]
        #[inline(always)]
        pub const fn a0(self) -> crate::common::Reg<regs::A0, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Slope definition A1"]
        #[inline(always)]
        pub const fn a1(self) -> crate::common::Reg<regs::A1, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Slope definition A2"]
        #[inline(always)]
        pub const fn a2(self) -> crate::common::Reg<regs::A2, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Slope definition A3"]
        #[inline(always)]
        pub const fn a3(self) -> crate::common::Reg<regs::A3, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Slope definition A4"]
        #[inline(always)]
        pub const fn a4(self) -> crate::common::Reg<regs::A4, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "Slope definition A5"]
        #[inline(always)]
        pub const fn a5(self) -> crate::common::Reg<regs::A5, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "Y-intercept B0"]
        #[inline(always)]
        pub const fn b0(self) -> crate::common::Reg<regs::B0, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
        }
        #[doc = "Y-intercept B1"]
        #[inline(always)]
        pub const fn b1(self) -> crate::common::Reg<regs::B1, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
        #[doc = "Y-intercept B2"]
        #[inline(always)]
        pub const fn b2(self) -> crate::common::Reg<regs::B2, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }
        #[doc = "Y-intercept B3"]
        #[inline(always)]
        pub const fn b3(self) -> crate::common::Reg<regs::B3, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
        }
        #[doc = "Y-intercept B4"]
        #[inline(always)]
        pub const fn b4(self) -> crate::common::Reg<regs::B4, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
        }
        #[doc = "Y-intercept B5"]
        #[inline(always)]
        pub const fn b5(self) -> crate::common::Reg<regs::B5, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
        }
        #[doc = "Segment end T0"]
        #[inline(always)]
        pub const fn t0(self) -> crate::common::Reg<regs::T0, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
        }
        #[doc = "Segment end T1"]
        #[inline(always)]
        pub const fn t1(self) -> crate::common::Reg<regs::T1, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
        }
        #[doc = "Segment end T2"]
        #[inline(always)]
        pub const fn t2(self) -> crate::common::Reg<regs::T2, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
        }
        #[doc = "Segment end T3"]
        #[inline(always)]
        pub const fn t3(self) -> crate::common::Reg<regs::T3, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
        }
        #[doc = "Segment end T4"]
        #[inline(always)]
        pub const fn t4(self) -> crate::common::Reg<regs::T4, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Slope definition A0"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct A0(pub u32);
        impl A0 {
            #[doc = "A (slope definition) register"]
            #[inline(always)]
            pub const fn a(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "A (slope definition) register"]
            #[inline(always)]
            pub fn set_a(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
        }
        impl Default for A0 {
            #[inline(always)]
            fn default() -> A0 {
                A0(0)
            }
        }
        impl core::fmt::Debug for A0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("A0").field("a", &self.a()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for A0 {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct A0 {
                    a: u16,
                }
                let proxy = A0 { a: self.a() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Slope definition A1"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct A1(pub u32);
        impl A1 {
            #[doc = "A (slope definition) register"]
            #[inline(always)]
            pub const fn a(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "A (slope definition) register"]
            #[inline(always)]
            pub fn set_a(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
        }
        impl Default for A1 {
            #[inline(always)]
            fn default() -> A1 {
                A1(0)
            }
        }
        impl core::fmt::Debug for A1 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("A1").field("a", &self.a()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for A1 {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct A1 {
                    a: u16,
                }
                let proxy = A1 { a: self.a() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Slope definition A2"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct A2(pub u32);
        impl A2 {
            #[doc = "A (slope definition) register"]
            #[inline(always)]
            pub const fn a(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "A (slope definition) register"]
            #[inline(always)]
            pub fn set_a(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
        }
        impl Default for A2 {
            #[inline(always)]
            fn default() -> A2 {
                A2(0)
            }
        }
        impl core::fmt::Debug for A2 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("A2").field("a", &self.a()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for A2 {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct A2 {
                    a: u16,
                }
                let proxy = A2 { a: self.a() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Slope definition A3"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct A3(pub u32);
        impl A3 {
            #[doc = "A (slope definition) register"]
            #[inline(always)]
            pub const fn a(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "A (slope definition) register"]
            #[inline(always)]
            pub fn set_a(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
        }
        impl Default for A3 {
            #[inline(always)]
            fn default() -> A3 {
                A3(0)
            }
        }
        impl core::fmt::Debug for A3 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("A3").field("a", &self.a()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for A3 {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct A3 {
                    a: u16,
                }
                let proxy = A3 { a: self.a() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Slope definition A4"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct A4(pub u32);
        impl A4 {
            #[doc = "A (slope definition) register"]
            #[inline(always)]
            pub const fn a(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "A (slope definition) register"]
            #[inline(always)]
            pub fn set_a(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
        }
        impl Default for A4 {
            #[inline(always)]
            fn default() -> A4 {
                A4(0)
            }
        }
        impl core::fmt::Debug for A4 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("A4").field("a", &self.a()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for A4 {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct A4 {
                    a: u16,
                }
                let proxy = A4 { a: self.a() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Slope definition A5"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct A5(pub u32);
        impl A5 {
            #[doc = "A (slope definition) register"]
            #[inline(always)]
            pub const fn a(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "A (slope definition) register"]
            #[inline(always)]
            pub fn set_a(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
        }
        impl Default for A5 {
            #[inline(always)]
            fn default() -> A5 {
                A5(0)
            }
        }
        impl core::fmt::Debug for A5 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("A5").field("a", &self.a()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for A5 {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct A5 {
                    a: u16,
                }
                let proxy = A5 { a: self.a() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Y-intercept B0"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct B0(pub u32);
        impl B0 {
            #[doc = "B (y-intercept)"]
            #[inline(always)]
            pub const fn b(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "B (y-intercept)"]
            #[inline(always)]
            pub fn set_b(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for B0 {
            #[inline(always)]
            fn default() -> B0 {
                B0(0)
            }
        }
        impl core::fmt::Debug for B0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("B0").field("b", &self.b()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for B0 {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct B0 {
                    b: u16,
                }
                let proxy = B0 { b: self.b() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Y-intercept B1"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct B1(pub u32);
        impl B1 {
            #[doc = "B (y-intercept)"]
            #[inline(always)]
            pub const fn b(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "B (y-intercept)"]
            #[inline(always)]
            pub fn set_b(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for B1 {
            #[inline(always)]
            fn default() -> B1 {
                B1(0)
            }
        }
        impl core::fmt::Debug for B1 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("B1").field("b", &self.b()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for B1 {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct B1 {
                    b: u16,
                }
                let proxy = B1 { b: self.b() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Y-intercept B2"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct B2(pub u32);
        impl B2 {
            #[doc = "B (y-intercept)"]
            #[inline(always)]
            pub const fn b(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "B (y-intercept)"]
            #[inline(always)]
            pub fn set_b(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for B2 {
            #[inline(always)]
            fn default() -> B2 {
                B2(0)
            }
        }
        impl core::fmt::Debug for B2 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("B2").field("b", &self.b()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for B2 {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct B2 {
                    b: u16,
                }
                let proxy = B2 { b: self.b() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Y-intercept B3"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct B3(pub u32);
        impl B3 {
            #[doc = "B (y-intercept)"]
            #[inline(always)]
            pub const fn b(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "B (y-intercept)"]
            #[inline(always)]
            pub fn set_b(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for B3 {
            #[inline(always)]
            fn default() -> B3 {
                B3(0)
            }
        }
        impl core::fmt::Debug for B3 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("B3").field("b", &self.b()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for B3 {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct B3 {
                    b: u16,
                }
                let proxy = B3 { b: self.b() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Y-intercept B4"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct B4(pub u32);
        impl B4 {
            #[doc = "B (y-intercept)"]
            #[inline(always)]
            pub const fn b(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "B (y-intercept)"]
            #[inline(always)]
            pub fn set_b(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for B4 {
            #[inline(always)]
            fn default() -> B4 {
                B4(0)
            }
        }
        impl core::fmt::Debug for B4 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("B4").field("b", &self.b()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for B4 {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct B4 {
                    b: u16,
                }
                let proxy = B4 { b: self.b() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Y-intercept B5"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct B5(pub u32);
        impl B5 {
            #[doc = "B (y-intercept)"]
            #[inline(always)]
            pub const fn b(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "B (y-intercept)"]
            #[inline(always)]
            pub fn set_b(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for B5 {
            #[inline(always)]
            fn default() -> B5 {
                B5(0)
            }
        }
        impl core::fmt::Debug for B5 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("B5").field("b", &self.b()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for B5 {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct B5 {
                    b: u16,
                }
                let proxy = B5 { b: self.b() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Device address type"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Deviceaddrtype(pub u32);
        impl Deviceaddrtype {
            #[doc = "Device address type"]
            #[inline(always)]
            pub const fn deviceaddrtype(&self) -> super::vals::Deviceaddrtype {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Deviceaddrtype::from_bits(val as u8)
            }
            #[doc = "Device address type"]
            #[inline(always)]
            pub fn set_deviceaddrtype(&mut self, val: super::vals::Deviceaddrtype) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Deviceaddrtype {
            #[inline(always)]
            fn default() -> Deviceaddrtype {
                Deviceaddrtype(0)
            }
        }
        impl core::fmt::Debug for Deviceaddrtype {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Deviceaddrtype")
                    .field("deviceaddrtype", &self.deviceaddrtype())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Deviceaddrtype {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Deviceaddrtype {
                    deviceaddrtype: super::vals::Deviceaddrtype,
                }
                let proxy = Deviceaddrtype {
                    deviceaddrtype: self.deviceaddrtype(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Flash variant"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Flash(pub u32);
        impl Flash {
            #[doc = "Flash variant"]
            #[inline(always)]
            pub const fn flash(&self) -> super::vals::Flash {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Flash::from_bits(val as u32)
            }
            #[doc = "Flash variant"]
            #[inline(always)]
            pub fn set_flash(&mut self, val: super::vals::Flash) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize))
                    | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Flash {
            #[inline(always)]
            fn default() -> Flash {
                Flash(0)
            }
        }
        impl core::fmt::Debug for Flash {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Flash")
                    .field("flash", &self.flash())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Flash {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Flash {
                    flash: super::vals::Flash,
                }
                let proxy = Flash {
                    flash: self.flash(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Package option"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Package(pub u32);
        impl Package {
            #[doc = "Package option"]
            #[inline(always)]
            pub const fn package(&self) -> super::vals::Package {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Package::from_bits(val as u32)
            }
            #[doc = "Package option"]
            #[inline(always)]
            pub fn set_package(&mut self, val: super::vals::Package) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize))
                    | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Package {
            #[inline(always)]
            fn default() -> Package {
                Package(0)
            }
        }
        impl core::fmt::Debug for Package {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Package")
                    .field("package", &self.package())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Package {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Package {
                    package: super::vals::Package,
                }
                let proxy = Package {
                    package: self.package(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Part code"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Part(pub u32);
        impl Part {
            #[doc = "Part code"]
            #[inline(always)]
            pub const fn part(&self) -> super::vals::Part {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Part::from_bits(val as u32)
            }
            #[doc = "Part code"]
            #[inline(always)]
            pub fn set_part(&mut self, val: super::vals::Part) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize))
                    | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Part {
            #[inline(always)]
            fn default() -> Part {
                Part(0)
            }
        }
        impl core::fmt::Debug for Part {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Part").field("part", &self.part()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Part {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Part {
                    part: super::vals::Part,
                }
                let proxy = Part { part: self.part() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "RAM variant"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ram(pub u32);
        impl Ram {
            #[doc = "RAM variant"]
            #[inline(always)]
            pub const fn ram(&self) -> super::vals::Ram {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Ram::from_bits(val as u32)
            }
            #[doc = "RAM variant"]
            #[inline(always)]
            pub fn set_ram(&mut self, val: super::vals::Ram) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize))
                    | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Ram {
            #[inline(always)]
            fn default() -> Ram {
                Ram(0)
            }
        }
        impl core::fmt::Debug for Ram {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ram").field("ram", &self.ram()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ram {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Ram {
                    ram: super::vals::Ram,
                }
                let proxy = Ram { ram: self.ram() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Segment end T0"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct T0(pub u32);
        impl T0 {
            #[doc = "T (segment end) register"]
            #[inline(always)]
            pub const fn t(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "T (segment end) register"]
            #[inline(always)]
            pub fn set_t(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for T0 {
            #[inline(always)]
            fn default() -> T0 {
                T0(0)
            }
        }
        impl core::fmt::Debug for T0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("T0").field("t", &self.t()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for T0 {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct T0 {
                    t: u8,
                }
                let proxy = T0 { t: self.t() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Segment end T1"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct T1(pub u32);
        impl T1 {
            #[doc = "T (segment end) register"]
            #[inline(always)]
            pub const fn t(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "T (segment end) register"]
            #[inline(always)]
            pub fn set_t(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for T1 {
            #[inline(always)]
            fn default() -> T1 {
                T1(0)
            }
        }
        impl core::fmt::Debug for T1 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("T1").field("t", &self.t()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for T1 {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct T1 {
                    t: u8,
                }
                let proxy = T1 { t: self.t() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Segment end T2"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct T2(pub u32);
        impl T2 {
            #[doc = "T (segment end) register"]
            #[inline(always)]
            pub const fn t(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "T (segment end) register"]
            #[inline(always)]
            pub fn set_t(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for T2 {
            #[inline(always)]
            fn default() -> T2 {
                T2(0)
            }
        }
        impl core::fmt::Debug for T2 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("T2").field("t", &self.t()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for T2 {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct T2 {
                    t: u8,
                }
                let proxy = T2 { t: self.t() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Segment end T3"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct T3(pub u32);
        impl T3 {
            #[doc = "T (segment end) register"]
            #[inline(always)]
            pub const fn t(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "T (segment end) register"]
            #[inline(always)]
            pub fn set_t(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for T3 {
            #[inline(always)]
            fn default() -> T3 {
                T3(0)
            }
        }
        impl core::fmt::Debug for T3 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("T3").field("t", &self.t()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for T3 {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct T3 {
                    t: u8,
                }
                let proxy = T3 { t: self.t() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Segment end T4"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct T4(pub u32);
        impl T4 {
            #[doc = "T (segment end) register"]
            #[inline(always)]
            pub const fn t(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "T (segment end) register"]
            #[inline(always)]
            pub fn set_t(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for T4 {
            #[inline(always)]
            fn default() -> T4 {
                T4(0)
            }
        }
        impl core::fmt::Debug for T4 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("T4").field("t", &self.t()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for T4 {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct T4 {
                    t: u8,
                }
                let proxy = T4 { t: self.t() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Part variant, hardware version and production configuration"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Variant(pub u32);
        impl Variant {
            #[doc = "Part variant, hardware version and production configuration, encoded as ASCII"]
            #[inline(always)]
            pub const fn variant(&self) -> super::vals::Variant {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Variant::from_bits(val as u32)
            }
            #[doc = "Part variant, hardware version and production configuration, encoded as ASCII"]
            #[inline(always)]
            pub fn set_variant(&mut self, val: super::vals::Variant) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize))
                    | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Variant {
            #[inline(always)]
            fn default() -> Variant {
                Variant(0)
            }
        }
        impl core::fmt::Debug for Variant {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Variant")
                    .field("variant", &self.variant())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Variant {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Variant {
                    variant: super::vals::Variant,
                }
                let proxy = Variant {
                    variant: self.variant(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Deviceaddrtype {
            #[doc = "Public address"]
            PUBLIC = 0x0,
            #[doc = "Random address"]
            RANDOM = 0x01,
        }
        impl Deviceaddrtype {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Deviceaddrtype {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Deviceaddrtype {
            #[inline(always)]
            fn from(val: u8) -> Deviceaddrtype {
                Deviceaddrtype::from_bits(val)
            }
        }
        impl From<Deviceaddrtype> for u8 {
            #[inline(always)]
            fn from(val: Deviceaddrtype) -> u8 {
                Deviceaddrtype::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Flash(pub u32);
        impl Flash {
            #[doc = "192 kByte flash"]
            pub const K192: Self = Self(0xc0);
            #[doc = "Unspecified"]
            pub const UNSPECIFIED: Self = Self(0xffff_ffff);
        }
        impl Flash {
            pub const fn from_bits(val: u32) -> Flash {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl core::fmt::Debug for Flash {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0xc0 => f.write_str("K192"),
                    0xffff_ffff => f.write_str("UNSPECIFIED"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Flash {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0xc0 => defmt::write!(f, "K192"),
                    0xffff_ffff => defmt::write!(f, "UNSPECIFIED"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u32> for Flash {
            #[inline(always)]
            fn from(val: u32) -> Flash {
                Flash::from_bits(val)
            }
        }
        impl From<Flash> for u32 {
            #[inline(always)]
            fn from(val: Flash) -> u32 {
                Flash::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Package(pub u32);
        impl Package {
            #[doc = "CAxx - WLCSP"]
            pub const CA: Self = Self(0x2004);
            #[doc = "Unspecified"]
            pub const UNSPECIFIED: Self = Self(0xffff_ffff);
        }
        impl Package {
            pub const fn from_bits(val: u32) -> Package {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl core::fmt::Debug for Package {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x2004 => f.write_str("CA"),
                    0xffff_ffff => f.write_str("UNSPECIFIED"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Package {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x2004 => defmt::write!(f, "CA"),
                    0xffff_ffff => defmt::write!(f, "UNSPECIFIED"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u32> for Package {
            #[inline(always)]
            fn from(val: u32) -> Package {
                Package::from_bits(val)
            }
        }
        impl From<Package> for u32 {
            #[inline(always)]
            fn from(val: Package) -> u32 {
                Package::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Part(pub u32);
        impl Part {
            #[doc = "nRF52805"]
            pub const N52805: Self = Self(0x0005_2805);
            #[doc = "nRF52810"]
            pub const N52810: Self = Self(0x0005_2810);
            #[doc = "nRF52811"]
            pub const N52811: Self = Self(0x0005_2811);
            #[doc = "nRF52832"]
            pub const N52832: Self = Self(0x0005_2832);
            #[doc = "Unspecified"]
            pub const UNSPECIFIED: Self = Self(0xffff_ffff);
        }
        impl Part {
            pub const fn from_bits(val: u32) -> Part {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl core::fmt::Debug for Part {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0005_2805 => f.write_str("N52805"),
                    0x0005_2810 => f.write_str("N52810"),
                    0x0005_2811 => f.write_str("N52811"),
                    0x0005_2832 => f.write_str("N52832"),
                    0xffff_ffff => f.write_str("UNSPECIFIED"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Part {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0005_2805 => defmt::write!(f, "N52805"),
                    0x0005_2810 => defmt::write!(f, "N52810"),
                    0x0005_2811 => defmt::write!(f, "N52811"),
                    0x0005_2832 => defmt::write!(f, "N52832"),
                    0xffff_ffff => defmt::write!(f, "UNSPECIFIED"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u32> for Part {
            #[inline(always)]
            fn from(val: u32) -> Part {
                Part::from_bits(val)
            }
        }
        impl From<Part> for u32 {
            #[inline(always)]
            fn from(val: Part) -> u32 {
                Part::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Ram(pub u32);
        impl Ram {
            #[doc = "24 kByte RAM"]
            pub const K24: Self = Self(0x18);
            #[doc = "Unspecified"]
            pub const UNSPECIFIED: Self = Self(0xffff_ffff);
        }
        impl Ram {
            pub const fn from_bits(val: u32) -> Ram {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl core::fmt::Debug for Ram {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x18 => f.write_str("K24"),
                    0xffff_ffff => f.write_str("UNSPECIFIED"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ram {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x18 => defmt::write!(f, "K24"),
                    0xffff_ffff => defmt::write!(f, "UNSPECIFIED"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u32> for Ram {
            #[inline(always)]
            fn from(val: u32) -> Ram {
                Ram::from_bits(val)
            }
        }
        impl From<Ram> for u32 {
            #[inline(always)]
            fn from(val: Ram) -> u32 {
                Ram::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Variant(pub u32);
        impl Variant {
            #[doc = "AAA0"]
            pub const AAA0: Self = Self(0x4141_4130);
            #[doc = "AAAA"]
            pub const AAAA: Self = Self(0x4141_4141);
            #[doc = "AAB0"]
            pub const AAB0: Self = Self(0x4141_4230);
            #[doc = "AABA"]
            pub const AABA: Self = Self(0x4141_4241);
            #[doc = "AABB"]
            pub const AABB: Self = Self(0x4141_4242);
            #[doc = "AAC0"]
            pub const AAC0: Self = Self(0x4141_4330);
            #[doc = "AACA"]
            pub const AACA: Self = Self(0x4141_4341);
            #[doc = "AACB"]
            pub const AACB: Self = Self(0x4141_4342);
            #[doc = "Unspecified"]
            pub const UNSPECIFIED: Self = Self(0xffff_ffff);
        }
        impl Variant {
            pub const fn from_bits(val: u32) -> Variant {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl core::fmt::Debug for Variant {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x4141_4130 => f.write_str("AAA0"),
                    0x4141_4141 => f.write_str("AAAA"),
                    0x4141_4230 => f.write_str("AAB0"),
                    0x4141_4241 => f.write_str("AABA"),
                    0x4141_4242 => f.write_str("AABB"),
                    0x4141_4330 => f.write_str("AAC0"),
                    0x4141_4341 => f.write_str("AACA"),
                    0x4141_4342 => f.write_str("AACB"),
                    0xffff_ffff => f.write_str("UNSPECIFIED"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Variant {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x4141_4130 => defmt::write!(f, "AAA0"),
                    0x4141_4141 => defmt::write!(f, "AAAA"),
                    0x4141_4230 => defmt::write!(f, "AAB0"),
                    0x4141_4241 => defmt::write!(f, "AABA"),
                    0x4141_4242 => defmt::write!(f, "AABB"),
                    0x4141_4330 => defmt::write!(f, "AAC0"),
                    0x4141_4341 => defmt::write!(f, "AACA"),
                    0x4141_4342 => defmt::write!(f, "AACB"),
                    0xffff_ffff => defmt::write!(f, "UNSPECIFIED"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u32> for Variant {
            #[inline(always)]
            fn from(val: u32) -> Variant {
                Variant::from_bits(val)
            }
        }
        impl From<Variant> for u32 {
            #[inline(always)]
            fn from(val: Variant) -> u32 {
                Variant::to_bits(val)
            }
        }
    }
}
pub mod gpio {
    #[doc = "GPIO Port"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpio {
        ptr: *mut u8,
    }
    unsafe impl Send for Gpio {}
    unsafe impl Sync for Gpio {}
    impl Gpio {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Write GPIO port"]
        #[inline(always)]
        pub const fn out(self) -> crate::common::Reg<regs::Out, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Set individual bits in GPIO port"]
        #[inline(always)]
        pub const fn outset(self) -> crate::common::Reg<regs::Outset, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Clear individual bits in GPIO port"]
        #[inline(always)]
        pub const fn outclr(self) -> crate::common::Reg<regs::Outclr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
        }
        #[doc = "Read GPIO port"]
        #[inline(always)]
        pub const fn in_(self) -> crate::common::Reg<regs::In, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
        }
        #[doc = "Direction of GPIO pins"]
        #[inline(always)]
        pub const fn dir(self) -> crate::common::Reg<regs::Dir, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
        }
        #[doc = "DIR set register"]
        #[inline(always)]
        pub const fn dirset(self) -> crate::common::Reg<regs::Dirset, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
        }
        #[doc = "DIR clear register"]
        #[inline(always)]
        pub const fn dirclr(self) -> crate::common::Reg<regs::Dirclr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
        }
        #[doc = "Latch register indicating what GPIO pins that have met the criteria set in the PIN_CNF\\[n\\].SENSE registers"]
        #[inline(always)]
        pub const fn latch(self) -> crate::common::Reg<regs::Latch, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0520usize) as _) }
        }
        #[doc = "Select between default DETECT signal behavior and LDETECT mode"]
        #[inline(always)]
        pub const fn detectmode(self) -> crate::common::Reg<regs::Detectmode, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
        }
        #[doc = "Description collection: Configuration of GPIO pins"]
        #[inline(always)]
        pub const fn pin_cnf(
            self,
            n: usize,
        ) -> crate::common::Reg<regs::PinCnf, crate::common::RW> {
            assert!(n < 32usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0700usize + n * 4usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Select between default DETECT signal behavior and LDETECT mode"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Detectmode(pub u32);
        impl Detectmode {
            #[doc = "Select between default DETECT signal behavior and LDETECT mode"]
            #[inline(always)]
            pub const fn detectmode(&self) -> super::vals::Detectmode {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Detectmode::from_bits(val as u8)
            }
            #[doc = "Select between default DETECT signal behavior and LDETECT mode"]
            #[inline(always)]
            pub fn set_detectmode(&mut self, val: super::vals::Detectmode) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Detectmode {
            #[inline(always)]
            fn default() -> Detectmode {
                Detectmode(0)
            }
        }
        impl core::fmt::Debug for Detectmode {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Detectmode")
                    .field("detectmode", &self.detectmode())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Detectmode {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Detectmode {
                    detectmode: super::vals::Detectmode,
                }
                let proxy = Detectmode {
                    detectmode: self.detectmode(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Direction of GPIO pins"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dir(pub u32);
        impl Dir {
            #[doc = "Pin 0"]
            #[inline(always)]
            pub const fn pin(&self, n: usize) -> super::vals::Dir {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Dir::from_bits(val as u8)
            }
            #[doc = "Pin 0"]
            #[inline(always)]
            pub fn set_pin(&mut self, n: usize, val: super::vals::Dir) {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
            }
        }
        impl Default for Dir {
            #[inline(always)]
            fn default() -> Dir {
                Dir(0)
            }
        }
        impl core::fmt::Debug for Dir {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dir")
                    .field(
                        "pin",
                        &[
                            self.pin(0usize),
                            self.pin(1usize),
                            self.pin(2usize),
                            self.pin(3usize),
                            self.pin(4usize),
                            self.pin(5usize),
                            self.pin(6usize),
                            self.pin(7usize),
                            self.pin(8usize),
                            self.pin(9usize),
                            self.pin(10usize),
                            self.pin(11usize),
                            self.pin(12usize),
                            self.pin(13usize),
                            self.pin(14usize),
                            self.pin(15usize),
                            self.pin(16usize),
                            self.pin(17usize),
                            self.pin(18usize),
                            self.pin(19usize),
                            self.pin(20usize),
                            self.pin(21usize),
                            self.pin(22usize),
                            self.pin(23usize),
                            self.pin(24usize),
                            self.pin(25usize),
                            self.pin(26usize),
                            self.pin(27usize),
                            self.pin(28usize),
                            self.pin(29usize),
                            self.pin(30usize),
                            self.pin(31usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dir {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Dir {
                    pin: [super::vals::Dir; 32usize],
                }
                let proxy = Dir {
                    pin: [
                        self.pin(0usize),
                        self.pin(1usize),
                        self.pin(2usize),
                        self.pin(3usize),
                        self.pin(4usize),
                        self.pin(5usize),
                        self.pin(6usize),
                        self.pin(7usize),
                        self.pin(8usize),
                        self.pin(9usize),
                        self.pin(10usize),
                        self.pin(11usize),
                        self.pin(12usize),
                        self.pin(13usize),
                        self.pin(14usize),
                        self.pin(15usize),
                        self.pin(16usize),
                        self.pin(17usize),
                        self.pin(18usize),
                        self.pin(19usize),
                        self.pin(20usize),
                        self.pin(21usize),
                        self.pin(22usize),
                        self.pin(23usize),
                        self.pin(24usize),
                        self.pin(25usize),
                        self.pin(26usize),
                        self.pin(27usize),
                        self.pin(28usize),
                        self.pin(29usize),
                        self.pin(30usize),
                        self.pin(31usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "DIR clear register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dirclr(pub u32);
        impl Dirclr {
            #[doc = "Set as input pin 0"]
            #[inline(always)]
            pub const fn pin(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Set as input pin 0"]
            #[inline(always)]
            pub fn set_pin(&mut self, n: usize, val: bool) {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Dirclr {
            #[inline(always)]
            fn default() -> Dirclr {
                Dirclr(0)
            }
        }
        impl core::fmt::Debug for Dirclr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dirclr")
                    .field(
                        "pin",
                        &[
                            self.pin(0usize),
                            self.pin(1usize),
                            self.pin(2usize),
                            self.pin(3usize),
                            self.pin(4usize),
                            self.pin(5usize),
                            self.pin(6usize),
                            self.pin(7usize),
                            self.pin(8usize),
                            self.pin(9usize),
                            self.pin(10usize),
                            self.pin(11usize),
                            self.pin(12usize),
                            self.pin(13usize),
                            self.pin(14usize),
                            self.pin(15usize),
                            self.pin(16usize),
                            self.pin(17usize),
                            self.pin(18usize),
                            self.pin(19usize),
                            self.pin(20usize),
                            self.pin(21usize),
                            self.pin(22usize),
                            self.pin(23usize),
                            self.pin(24usize),
                            self.pin(25usize),
                            self.pin(26usize),
                            self.pin(27usize),
                            self.pin(28usize),
                            self.pin(29usize),
                            self.pin(30usize),
                            self.pin(31usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dirclr {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Dirclr {
                    pin: [bool; 32usize],
                }
                let proxy = Dirclr {
                    pin: [
                        self.pin(0usize),
                        self.pin(1usize),
                        self.pin(2usize),
                        self.pin(3usize),
                        self.pin(4usize),
                        self.pin(5usize),
                        self.pin(6usize),
                        self.pin(7usize),
                        self.pin(8usize),
                        self.pin(9usize),
                        self.pin(10usize),
                        self.pin(11usize),
                        self.pin(12usize),
                        self.pin(13usize),
                        self.pin(14usize),
                        self.pin(15usize),
                        self.pin(16usize),
                        self.pin(17usize),
                        self.pin(18usize),
                        self.pin(19usize),
                        self.pin(20usize),
                        self.pin(21usize),
                        self.pin(22usize),
                        self.pin(23usize),
                        self.pin(24usize),
                        self.pin(25usize),
                        self.pin(26usize),
                        self.pin(27usize),
                        self.pin(28usize),
                        self.pin(29usize),
                        self.pin(30usize),
                        self.pin(31usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "DIR set register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dirset(pub u32);
        impl Dirset {
            #[doc = "Set as output pin 0"]
            #[inline(always)]
            pub const fn pin(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Set as output pin 0"]
            #[inline(always)]
            pub fn set_pin(&mut self, n: usize, val: bool) {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Dirset {
            #[inline(always)]
            fn default() -> Dirset {
                Dirset(0)
            }
        }
        impl core::fmt::Debug for Dirset {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dirset")
                    .field(
                        "pin",
                        &[
                            self.pin(0usize),
                            self.pin(1usize),
                            self.pin(2usize),
                            self.pin(3usize),
                            self.pin(4usize),
                            self.pin(5usize),
                            self.pin(6usize),
                            self.pin(7usize),
                            self.pin(8usize),
                            self.pin(9usize),
                            self.pin(10usize),
                            self.pin(11usize),
                            self.pin(12usize),
                            self.pin(13usize),
                            self.pin(14usize),
                            self.pin(15usize),
                            self.pin(16usize),
                            self.pin(17usize),
                            self.pin(18usize),
                            self.pin(19usize),
                            self.pin(20usize),
                            self.pin(21usize),
                            self.pin(22usize),
                            self.pin(23usize),
                            self.pin(24usize),
                            self.pin(25usize),
                            self.pin(26usize),
                            self.pin(27usize),
                            self.pin(28usize),
                            self.pin(29usize),
                            self.pin(30usize),
                            self.pin(31usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dirset {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Dirset {
                    pin: [bool; 32usize],
                }
                let proxy = Dirset {
                    pin: [
                        self.pin(0usize),
                        self.pin(1usize),
                        self.pin(2usize),
                        self.pin(3usize),
                        self.pin(4usize),
                        self.pin(5usize),
                        self.pin(6usize),
                        self.pin(7usize),
                        self.pin(8usize),
                        self.pin(9usize),
                        self.pin(10usize),
                        self.pin(11usize),
                        self.pin(12usize),
                        self.pin(13usize),
                        self.pin(14usize),
                        self.pin(15usize),
                        self.pin(16usize),
                        self.pin(17usize),
                        self.pin(18usize),
                        self.pin(19usize),
                        self.pin(20usize),
                        self.pin(21usize),
                        self.pin(22usize),
                        self.pin(23usize),
                        self.pin(24usize),
                        self.pin(25usize),
                        self.pin(26usize),
                        self.pin(27usize),
                        self.pin(28usize),
                        self.pin(29usize),
                        self.pin(30usize),
                        self.pin(31usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Read GPIO port"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct In(pub u32);
        impl In {
            #[doc = "Pin 0"]
            #[inline(always)]
            pub const fn pin(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Pin 0"]
            #[inline(always)]
            pub fn set_pin(&mut self, n: usize, val: bool) {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for In {
            #[inline(always)]
            fn default() -> In {
                In(0)
            }
        }
        impl core::fmt::Debug for In {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("In")
                    .field(
                        "pin",
                        &[
                            self.pin(0usize),
                            self.pin(1usize),
                            self.pin(2usize),
                            self.pin(3usize),
                            self.pin(4usize),
                            self.pin(5usize),
                            self.pin(6usize),
                            self.pin(7usize),
                            self.pin(8usize),
                            self.pin(9usize),
                            self.pin(10usize),
                            self.pin(11usize),
                            self.pin(12usize),
                            self.pin(13usize),
                            self.pin(14usize),
                            self.pin(15usize),
                            self.pin(16usize),
                            self.pin(17usize),
                            self.pin(18usize),
                            self.pin(19usize),
                            self.pin(20usize),
                            self.pin(21usize),
                            self.pin(22usize),
                            self.pin(23usize),
                            self.pin(24usize),
                            self.pin(25usize),
                            self.pin(26usize),
                            self.pin(27usize),
                            self.pin(28usize),
                            self.pin(29usize),
                            self.pin(30usize),
                            self.pin(31usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for In {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct In {
                    pin: [bool; 32usize],
                }
                let proxy = In {
                    pin: [
                        self.pin(0usize),
                        self.pin(1usize),
                        self.pin(2usize),
                        self.pin(3usize),
                        self.pin(4usize),
                        self.pin(5usize),
                        self.pin(6usize),
                        self.pin(7usize),
                        self.pin(8usize),
                        self.pin(9usize),
                        self.pin(10usize),
                        self.pin(11usize),
                        self.pin(12usize),
                        self.pin(13usize),
                        self.pin(14usize),
                        self.pin(15usize),
                        self.pin(16usize),
                        self.pin(17usize),
                        self.pin(18usize),
                        self.pin(19usize),
                        self.pin(20usize),
                        self.pin(21usize),
                        self.pin(22usize),
                        self.pin(23usize),
                        self.pin(24usize),
                        self.pin(25usize),
                        self.pin(26usize),
                        self.pin(27usize),
                        self.pin(28usize),
                        self.pin(29usize),
                        self.pin(30usize),
                        self.pin(31usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Latch register indicating what GPIO pins that have met the criteria set in the PIN_CNF\\[n\\].SENSE registers"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Latch(pub u32);
        impl Latch {
            #[doc = "Status on whether PIN0 has met criteria set in PIN_CNF0.SENSE register. Write '1' to clear."]
            #[inline(always)]
            pub const fn pin(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Status on whether PIN0 has met criteria set in PIN_CNF0.SENSE register. Write '1' to clear."]
            #[inline(always)]
            pub fn set_pin(&mut self, n: usize, val: bool) {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Latch {
            #[inline(always)]
            fn default() -> Latch {
                Latch(0)
            }
        }
        impl core::fmt::Debug for Latch {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Latch")
                    .field(
                        "pin",
                        &[
                            self.pin(0usize),
                            self.pin(1usize),
                            self.pin(2usize),
                            self.pin(3usize),
                            self.pin(4usize),
                            self.pin(5usize),
                            self.pin(6usize),
                            self.pin(7usize),
                            self.pin(8usize),
                            self.pin(9usize),
                            self.pin(10usize),
                            self.pin(11usize),
                            self.pin(12usize),
                            self.pin(13usize),
                            self.pin(14usize),
                            self.pin(15usize),
                            self.pin(16usize),
                            self.pin(17usize),
                            self.pin(18usize),
                            self.pin(19usize),
                            self.pin(20usize),
                            self.pin(21usize),
                            self.pin(22usize),
                            self.pin(23usize),
                            self.pin(24usize),
                            self.pin(25usize),
                            self.pin(26usize),
                            self.pin(27usize),
                            self.pin(28usize),
                            self.pin(29usize),
                            self.pin(30usize),
                            self.pin(31usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Latch {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Latch {
                    pin: [bool; 32usize],
                }
                let proxy = Latch {
                    pin: [
                        self.pin(0usize),
                        self.pin(1usize),
                        self.pin(2usize),
                        self.pin(3usize),
                        self.pin(4usize),
                        self.pin(5usize),
                        self.pin(6usize),
                        self.pin(7usize),
                        self.pin(8usize),
                        self.pin(9usize),
                        self.pin(10usize),
                        self.pin(11usize),
                        self.pin(12usize),
                        self.pin(13usize),
                        self.pin(14usize),
                        self.pin(15usize),
                        self.pin(16usize),
                        self.pin(17usize),
                        self.pin(18usize),
                        self.pin(19usize),
                        self.pin(20usize),
                        self.pin(21usize),
                        self.pin(22usize),
                        self.pin(23usize),
                        self.pin(24usize),
                        self.pin(25usize),
                        self.pin(26usize),
                        self.pin(27usize),
                        self.pin(28usize),
                        self.pin(29usize),
                        self.pin(30usize),
                        self.pin(31usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Write GPIO port"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Out(pub u32);
        impl Out {
            #[doc = "Pin 0"]
            #[inline(always)]
            pub const fn pin(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Pin 0"]
            #[inline(always)]
            pub fn set_pin(&mut self, n: usize, val: bool) {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Out {
            #[inline(always)]
            fn default() -> Out {
                Out(0)
            }
        }
        impl core::fmt::Debug for Out {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Out")
                    .field(
                        "pin",
                        &[
                            self.pin(0usize),
                            self.pin(1usize),
                            self.pin(2usize),
                            self.pin(3usize),
                            self.pin(4usize),
                            self.pin(5usize),
                            self.pin(6usize),
                            self.pin(7usize),
                            self.pin(8usize),
                            self.pin(9usize),
                            self.pin(10usize),
                            self.pin(11usize),
                            self.pin(12usize),
                            self.pin(13usize),
                            self.pin(14usize),
                            self.pin(15usize),
                            self.pin(16usize),
                            self.pin(17usize),
                            self.pin(18usize),
                            self.pin(19usize),
                            self.pin(20usize),
                            self.pin(21usize),
                            self.pin(22usize),
                            self.pin(23usize),
                            self.pin(24usize),
                            self.pin(25usize),
                            self.pin(26usize),
                            self.pin(27usize),
                            self.pin(28usize),
                            self.pin(29usize),
                            self.pin(30usize),
                            self.pin(31usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Out {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Out {
                    pin: [bool; 32usize],
                }
                let proxy = Out {
                    pin: [
                        self.pin(0usize),
                        self.pin(1usize),
                        self.pin(2usize),
                        self.pin(3usize),
                        self.pin(4usize),
                        self.pin(5usize),
                        self.pin(6usize),
                        self.pin(7usize),
                        self.pin(8usize),
                        self.pin(9usize),
                        self.pin(10usize),
                        self.pin(11usize),
                        self.pin(12usize),
                        self.pin(13usize),
                        self.pin(14usize),
                        self.pin(15usize),
                        self.pin(16usize),
                        self.pin(17usize),
                        self.pin(18usize),
                        self.pin(19usize),
                        self.pin(20usize),
                        self.pin(21usize),
                        self.pin(22usize),
                        self.pin(23usize),
                        self.pin(24usize),
                        self.pin(25usize),
                        self.pin(26usize),
                        self.pin(27usize),
                        self.pin(28usize),
                        self.pin(29usize),
                        self.pin(30usize),
                        self.pin(31usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Clear individual bits in GPIO port"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Outclr(pub u32);
        impl Outclr {
            #[doc = "Pin 0"]
            #[inline(always)]
            pub const fn pin(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Pin 0"]
            #[inline(always)]
            pub fn set_pin(&mut self, n: usize, val: bool) {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Outclr {
            #[inline(always)]
            fn default() -> Outclr {
                Outclr(0)
            }
        }
        impl core::fmt::Debug for Outclr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Outclr")
                    .field(
                        "pin",
                        &[
                            self.pin(0usize),
                            self.pin(1usize),
                            self.pin(2usize),
                            self.pin(3usize),
                            self.pin(4usize),
                            self.pin(5usize),
                            self.pin(6usize),
                            self.pin(7usize),
                            self.pin(8usize),
                            self.pin(9usize),
                            self.pin(10usize),
                            self.pin(11usize),
                            self.pin(12usize),
                            self.pin(13usize),
                            self.pin(14usize),
                            self.pin(15usize),
                            self.pin(16usize),
                            self.pin(17usize),
                            self.pin(18usize),
                            self.pin(19usize),
                            self.pin(20usize),
                            self.pin(21usize),
                            self.pin(22usize),
                            self.pin(23usize),
                            self.pin(24usize),
                            self.pin(25usize),
                            self.pin(26usize),
                            self.pin(27usize),
                            self.pin(28usize),
                            self.pin(29usize),
                            self.pin(30usize),
                            self.pin(31usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Outclr {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Outclr {
                    pin: [bool; 32usize],
                }
                let proxy = Outclr {
                    pin: [
                        self.pin(0usize),
                        self.pin(1usize),
                        self.pin(2usize),
                        self.pin(3usize),
                        self.pin(4usize),
                        self.pin(5usize),
                        self.pin(6usize),
                        self.pin(7usize),
                        self.pin(8usize),
                        self.pin(9usize),
                        self.pin(10usize),
                        self.pin(11usize),
                        self.pin(12usize),
                        self.pin(13usize),
                        self.pin(14usize),
                        self.pin(15usize),
                        self.pin(16usize),
                        self.pin(17usize),
                        self.pin(18usize),
                        self.pin(19usize),
                        self.pin(20usize),
                        self.pin(21usize),
                        self.pin(22usize),
                        self.pin(23usize),
                        self.pin(24usize),
                        self.pin(25usize),
                        self.pin(26usize),
                        self.pin(27usize),
                        self.pin(28usize),
                        self.pin(29usize),
                        self.pin(30usize),
                        self.pin(31usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Set individual bits in GPIO port"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Outset(pub u32);
        impl Outset {
            #[doc = "Pin 0"]
            #[inline(always)]
            pub const fn pin(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Pin 0"]
            #[inline(always)]
            pub fn set_pin(&mut self, n: usize, val: bool) {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Outset {
            #[inline(always)]
            fn default() -> Outset {
                Outset(0)
            }
        }
        impl core::fmt::Debug for Outset {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Outset")
                    .field(
                        "pin",
                        &[
                            self.pin(0usize),
                            self.pin(1usize),
                            self.pin(2usize),
                            self.pin(3usize),
                            self.pin(4usize),
                            self.pin(5usize),
                            self.pin(6usize),
                            self.pin(7usize),
                            self.pin(8usize),
                            self.pin(9usize),
                            self.pin(10usize),
                            self.pin(11usize),
                            self.pin(12usize),
                            self.pin(13usize),
                            self.pin(14usize),
                            self.pin(15usize),
                            self.pin(16usize),
                            self.pin(17usize),
                            self.pin(18usize),
                            self.pin(19usize),
                            self.pin(20usize),
                            self.pin(21usize),
                            self.pin(22usize),
                            self.pin(23usize),
                            self.pin(24usize),
                            self.pin(25usize),
                            self.pin(26usize),
                            self.pin(27usize),
                            self.pin(28usize),
                            self.pin(29usize),
                            self.pin(30usize),
                            self.pin(31usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Outset {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Outset {
                    pin: [bool; 32usize],
                }
                let proxy = Outset {
                    pin: [
                        self.pin(0usize),
                        self.pin(1usize),
                        self.pin(2usize),
                        self.pin(3usize),
                        self.pin(4usize),
                        self.pin(5usize),
                        self.pin(6usize),
                        self.pin(7usize),
                        self.pin(8usize),
                        self.pin(9usize),
                        self.pin(10usize),
                        self.pin(11usize),
                        self.pin(12usize),
                        self.pin(13usize),
                        self.pin(14usize),
                        self.pin(15usize),
                        self.pin(16usize),
                        self.pin(17usize),
                        self.pin(18usize),
                        self.pin(19usize),
                        self.pin(20usize),
                        self.pin(21usize),
                        self.pin(22usize),
                        self.pin(23usize),
                        self.pin(24usize),
                        self.pin(25usize),
                        self.pin(26usize),
                        self.pin(27usize),
                        self.pin(28usize),
                        self.pin(29usize),
                        self.pin(30usize),
                        self.pin(31usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Description collection: Configuration of GPIO pins"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct PinCnf(pub u32);
        impl PinCnf {
            #[doc = "Pin direction. Same physical register as DIR register"]
            #[inline(always)]
            pub const fn dir(&self) -> super::vals::Dir {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Dir::from_bits(val as u8)
            }
            #[doc = "Pin direction. Same physical register as DIR register"]
            #[inline(always)]
            pub fn set_dir(&mut self, val: super::vals::Dir) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Connect or disconnect input buffer"]
            #[inline(always)]
            pub const fn input(&self) -> super::vals::Input {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Input::from_bits(val as u8)
            }
            #[doc = "Connect or disconnect input buffer"]
            #[inline(always)]
            pub fn set_input(&mut self, val: super::vals::Input) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Pull configuration"]
            #[inline(always)]
            pub const fn pull(&self) -> super::vals::Pull {
                let val = (self.0 >> 2usize) & 0x03;
                super::vals::Pull::from_bits(val as u8)
            }
            #[doc = "Pull configuration"]
            #[inline(always)]
            pub fn set_pull(&mut self, val: super::vals::Pull) {
                self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
            }
            #[doc = "Drive configuration"]
            #[inline(always)]
            pub const fn drive(&self) -> super::vals::Drive {
                let val = (self.0 >> 8usize) & 0x07;
                super::vals::Drive::from_bits(val as u8)
            }
            #[doc = "Drive configuration"]
            #[inline(always)]
            pub fn set_drive(&mut self, val: super::vals::Drive) {
                self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
            }
            #[doc = "Pin sensing mechanism"]
            #[inline(always)]
            pub const fn sense(&self) -> super::vals::Sense {
                let val = (self.0 >> 16usize) & 0x03;
                super::vals::Sense::from_bits(val as u8)
            }
            #[doc = "Pin sensing mechanism"]
            #[inline(always)]
            pub fn set_sense(&mut self, val: super::vals::Sense) {
                self.0 =
                    (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
            }
        }
        impl Default for PinCnf {
            #[inline(always)]
            fn default() -> PinCnf {
                PinCnf(0)
            }
        }
        impl core::fmt::Debug for PinCnf {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("PinCnf")
                    .field("dir", &self.dir())
                    .field("input", &self.input())
                    .field("pull", &self.pull())
                    .field("drive", &self.drive())
                    .field("sense", &self.sense())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for PinCnf {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct PinCnf {
                    dir: super::vals::Dir,
                    input: super::vals::Input,
                    pull: super::vals::Pull,
                    drive: super::vals::Drive,
                    sense: super::vals::Sense,
                }
                let proxy = PinCnf {
                    dir: self.dir(),
                    input: self.input(),
                    pull: self.pull(),
                    drive: self.drive(),
                    sense: self.sense(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Detectmode {
            #[doc = "DETECT directly connected to PIN DETECT signals"]
            DEFAULT = 0x0,
            #[doc = "Use the latched LDETECT behavior"]
            LDETECT = 0x01,
        }
        impl Detectmode {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Detectmode {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Detectmode {
            #[inline(always)]
            fn from(val: u8) -> Detectmode {
                Detectmode::from_bits(val)
            }
        }
        impl From<Detectmode> for u8 {
            #[inline(always)]
            fn from(val: Detectmode) -> u8 {
                Detectmode::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Dir {
            #[doc = "Configure pin as an input pin"]
            INPUT = 0x0,
            #[doc = "Configure pin as an output pin"]
            OUTPUT = 0x01,
        }
        impl Dir {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Dir {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Dir {
            #[inline(always)]
            fn from(val: u8) -> Dir {
                Dir::from_bits(val)
            }
        }
        impl From<Dir> for u8 {
            #[inline(always)]
            fn from(val: Dir) -> u8 {
                Dir::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Drive {
            #[doc = "Standard '0', standard '1'"]
            S0S1 = 0x0,
            #[doc = "High drive '0', standard '1'"]
            H0S1 = 0x01,
            #[doc = "Standard '0', high drive '1'"]
            S0H1 = 0x02,
            #[doc = "High drive '0', high 'drive '1''"]
            H0H1 = 0x03,
            #[doc = "Disconnect '0' standard '1' (normally used for wired-or connections)"]
            D0S1 = 0x04,
            #[doc = "Disconnect '0', high drive '1' (normally used for wired-or connections)"]
            D0H1 = 0x05,
            #[doc = "Standard '0'. disconnect '1' (normally used for wired-and connections)"]
            S0D1 = 0x06,
            #[doc = "High drive '0', disconnect '1' (normally used for wired-and connections)"]
            H0D1 = 0x07,
        }
        impl Drive {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Drive {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Drive {
            #[inline(always)]
            fn from(val: u8) -> Drive {
                Drive::from_bits(val)
            }
        }
        impl From<Drive> for u8 {
            #[inline(always)]
            fn from(val: Drive) -> u8 {
                Drive::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Input {
            #[doc = "Connect input buffer"]
            CONNECT = 0x0,
            #[doc = "Disconnect input buffer"]
            DISCONNECT = 0x01,
        }
        impl Input {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Input {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Input {
            #[inline(always)]
            fn from(val: u8) -> Input {
                Input::from_bits(val)
            }
        }
        impl From<Input> for u8 {
            #[inline(always)]
            fn from(val: Input) -> u8 {
                Input::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Pull {
            #[doc = "No pull"]
            DISABLED = 0x0,
            #[doc = "Pull down on pin"]
            PULLDOWN = 0x01,
            _RESERVED_2 = 0x02,
            #[doc = "Pull up on pin"]
            PULLUP = 0x03,
        }
        impl Pull {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Pull {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Pull {
            #[inline(always)]
            fn from(val: u8) -> Pull {
                Pull::from_bits(val)
            }
        }
        impl From<Pull> for u8 {
            #[inline(always)]
            fn from(val: Pull) -> u8 {
                Pull::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Sense {
            #[doc = "Disabled"]
            DISABLED = 0x0,
            _RESERVED_1 = 0x01,
            #[doc = "Sense for high level"]
            HIGH = 0x02,
            #[doc = "Sense for low level"]
            LOW = 0x03,
        }
        impl Sense {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Sense {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Sense {
            #[inline(always)]
            fn from(val: u8) -> Sense {
                Sense::from_bits(val)
            }
        }
        impl From<Sense> for u8 {
            #[inline(always)]
            fn from(val: Sense) -> u8 {
                Sense::to_bits(val)
            }
        }
    }
}
pub mod gpiote {
    #[doc = "GPIO Tasks and Events"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpiote {
        ptr: *mut u8,
    }
    unsafe impl Send for Gpiote {}
    unsafe impl Sync for Gpiote {}
    impl Gpiote {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is configured in CONFIG\\[n\\].POLARITY."]
        #[inline(always)]
        pub const fn tasks_out(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it high."]
        #[inline(always)]
        pub const fn tasks_set(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it low."]
        #[inline(always)]
        pub const fn tasks_clr(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Event generated from pin specified in CONFIG\\[n\\].PSEL"]
        #[inline(always)]
        pub const fn events_in(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
        }
        #[doc = "Event generated from multiple input GPIO pins with SENSE mechanism enabled"]
        #[inline(always)]
        pub const fn events_port(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x017cusize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Description collection: Configuration for OUT\\[n\\], SET\\[n\\], and CLR\\[n\\] tasks and IN\\[n\\] event"]
        #[inline(always)]
        pub const fn config(self, n: usize) -> crate::common::Reg<regs::Config, crate::common::RW> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize + n * 4usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Description collection: Configuration for OUT\\[n\\], SET\\[n\\], and CLR\\[n\\] tasks and IN\\[n\\] event"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Mode"]
            #[inline(always)]
            pub const fn mode(&self) -> super::vals::Mode {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Mode::from_bits(val as u8)
            }
            #[doc = "Mode"]
            #[inline(always)]
            pub fn set_mode(&mut self, val: super::vals::Mode) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
            #[doc = "GPIO number associated with SET\\[n\\], CLR\\[n\\], and OUT\\[n\\] tasks and IN\\[n\\] event"]
            #[inline(always)]
            pub const fn psel(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0x1f;
                val as u8
            }
            #[doc = "GPIO number associated with SET\\[n\\], CLR\\[n\\], and OUT\\[n\\] tasks and IN\\[n\\] event"]
            #[inline(always)]
            pub fn set_psel(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
            }
            #[doc = "When In task mode: Operation to be performed on output when OUT\\[n\\] task is triggered. When In event mode: Operation on input that shall trigger IN\\[n\\] event."]
            #[inline(always)]
            pub const fn polarity(&self) -> super::vals::Polarity {
                let val = (self.0 >> 16usize) & 0x03;
                super::vals::Polarity::from_bits(val as u8)
            }
            #[doc = "When In task mode: Operation to be performed on output when OUT\\[n\\] task is triggered. When In event mode: Operation on input that shall trigger IN\\[n\\] event."]
            #[inline(always)]
            pub fn set_polarity(&mut self, val: super::vals::Polarity) {
                self.0 =
                    (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
            }
            #[doc = "When in task mode: Initial value of the output when the GPIOTE channel is configured. When in event mode: No effect."]
            #[inline(always)]
            pub const fn outinit(&self) -> super::vals::Outinit {
                let val = (self.0 >> 20usize) & 0x01;
                super::vals::Outinit::from_bits(val as u8)
            }
            #[doc = "When in task mode: Initial value of the output when the GPIOTE channel is configured. When in event mode: No effect."]
            #[inline(always)]
            pub fn set_outinit(&mut self, val: super::vals::Outinit) {
                self.0 =
                    (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
            }
        }
        impl Default for Config {
            #[inline(always)]
            fn default() -> Config {
                Config(0)
            }
        }
        impl core::fmt::Debug for Config {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Config")
                    .field("mode", &self.mode())
                    .field("psel", &self.psel())
                    .field("polarity", &self.polarity())
                    .field("outinit", &self.outinit())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Config {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Config {
                    mode: super::vals::Mode,
                    psel: u8,
                    polarity: super::vals::Polarity,
                    outinit: super::vals::Outinit,
                }
                let proxy = Config {
                    mode: self.mode(),
                    psel: self.psel(),
                    polarity: self.polarity(),
                    outinit: self.outinit(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Write '1' to disable interrupt for event IN\\[0\\]"]
            #[inline(always)]
            pub const fn in_(&self, n: usize) -> bool {
                assert!(n < 8usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event IN\\[0\\]"]
            #[inline(always)]
            pub fn set_in_(&mut self, n: usize, val: bool) {
                assert!(n < 8usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Write '1' to disable interrupt for event PORT"]
            #[inline(always)]
            pub const fn port(&self) -> bool {
                let val = (self.0 >> 31usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event PORT"]
            #[inline(always)]
            pub fn set_port(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field(
                        "in_",
                        &[
                            self.in_(0usize),
                            self.in_(1usize),
                            self.in_(2usize),
                            self.in_(3usize),
                            self.in_(4usize),
                            self.in_(5usize),
                            self.in_(6usize),
                            self.in_(7usize),
                        ],
                    )
                    .field("port", &self.port())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    in_: [bool; 8usize],
                    port: bool,
                }
                let proxy = Int {
                    in_: [
                        self.in_(0usize),
                        self.in_(1usize),
                        self.in_(2usize),
                        self.in_(3usize),
                        self.in_(4usize),
                        self.in_(5usize),
                        self.in_(6usize),
                        self.in_(7usize),
                    ],
                    port: self.port(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Mode {
            #[doc = "Disabled. Pin specified by PSEL will not be acquired by the GPIOTE module."]
            DISABLED = 0x0,
            #[doc = "Event mode"]
            EVENT = 0x01,
            _RESERVED_2 = 0x02,
            #[doc = "Task mode"]
            TASK = 0x03,
        }
        impl Mode {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Mode {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Mode {
            #[inline(always)]
            fn from(val: u8) -> Mode {
                Mode::from_bits(val)
            }
        }
        impl From<Mode> for u8 {
            #[inline(always)]
            fn from(val: Mode) -> u8 {
                Mode::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Outinit {
            #[doc = "Task mode: Initial value of pin before task triggering is low"]
            LOW = 0x0,
            #[doc = "Task mode: Initial value of pin before task triggering is high"]
            HIGH = 0x01,
        }
        impl Outinit {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Outinit {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Outinit {
            #[inline(always)]
            fn from(val: u8) -> Outinit {
                Outinit::from_bits(val)
            }
        }
        impl From<Outinit> for u8 {
            #[inline(always)]
            fn from(val: Outinit) -> u8 {
                Outinit::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Polarity {
            #[doc = "Task mode: No effect on pin from OUT\\[n\\] task. Event mode: no IN\\[n\\] event generated on pin activity."]
            NONE = 0x0,
            #[doc = "Task mode: Set pin from OUT\\[n\\] task. Event mode: Generate IN\\[n\\] event when rising edge on pin."]
            LO_TO_HI = 0x01,
            #[doc = "Task mode: Clear pin from OUT\\[n\\] task. Event mode: Generate IN\\[n\\] event when falling edge on pin."]
            HI_TO_LO = 0x02,
            #[doc = "Task mode: Toggle pin from OUT\\[n\\]. Event mode: Generate IN\\[n\\] when any change on pin."]
            TOGGLE = 0x03,
        }
        impl Polarity {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Polarity {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Polarity {
            #[inline(always)]
            fn from(val: u8) -> Polarity {
                Polarity::from_bits(val)
            }
        }
        impl From<Polarity> for u8 {
            #[inline(always)]
            fn from(val: Polarity) -> u8 {
                Polarity::to_bits(val)
            }
        }
    }
}
pub mod nvmc {
    #[doc = "Non Volatile Memory Controller"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nvmc {
        ptr: *mut u8,
    }
    unsafe impl Send for Nvmc {}
    unsafe impl Sync for Nvmc {}
    impl Nvmc {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Ready flag"]
        #[inline(always)]
        pub const fn ready(self) -> crate::common::Reg<regs::Ready, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
        }
        #[doc = "Configuration register"]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Register for erasing a page in code area"]
        #[inline(always)]
        pub const fn erasepage(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Deprecated register - Register for erasing a page in code area, equivalent to ERASEPAGE"]
        #[inline(always)]
        pub const fn erasepcr1(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Register for erasing all non-volatile user memory"]
        #[inline(always)]
        pub const fn eraseall(self) -> crate::common::Reg<regs::Eraseall, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
        }
        #[doc = "Deprecated register - Register for erasing a page in code area, equivalent to ERASEPAGE"]
        #[inline(always)]
        pub const fn erasepcr0(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
        }
        #[doc = "Register for erasing user information configuration registers"]
        #[inline(always)]
        pub const fn eraseuicr(self) -> crate::common::Reg<regs::Eraseuicr, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
        }
        #[doc = "Register for partial erase of a page in code area"]
        #[inline(always)]
        pub const fn erasepagepartial(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
        }
        #[doc = "Register for partial erase configuration"]
        #[inline(always)]
        pub const fn erasepagepartialcfg(
            self,
        ) -> crate::common::Reg<regs::Erasepagepartialcfg, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Configuration register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used."]
            #[inline(always)]
            pub const fn wen(&self) -> super::vals::Wen {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Wen::from_bits(val as u8)
            }
            #[doc = "Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used."]
            #[inline(always)]
            pub fn set_wen(&mut self, val: super::vals::Wen) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Config {
            #[inline(always)]
            fn default() -> Config {
                Config(0)
            }
        }
        impl core::fmt::Debug for Config {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Config").field("wen", &self.wen()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Config {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Config {
                    wen: super::vals::Wen,
                }
                let proxy = Config { wen: self.wen() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Register for erasing all non-volatile user memory"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Eraseall(pub u32);
        impl Eraseall {
            #[doc = "Erase all non-volatile memory including UICR registers. The erase must be enabled using CONFIG.WEN before the non-volatile memory can be erased."]
            #[inline(always)]
            pub const fn eraseall(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Erase all non-volatile memory including UICR registers. The erase must be enabled using CONFIG.WEN before the non-volatile memory can be erased."]
            #[inline(always)]
            pub fn set_eraseall(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Eraseall {
            #[inline(always)]
            fn default() -> Eraseall {
                Eraseall(0)
            }
        }
        impl core::fmt::Debug for Eraseall {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Eraseall")
                    .field("eraseall", &self.eraseall())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Eraseall {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Eraseall {
                    eraseall: bool,
                }
                let proxy = Eraseall {
                    eraseall: self.eraseall(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Register for partial erase configuration"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Erasepagepartialcfg(pub u32);
        impl Erasepagepartialcfg {
            #[doc = "Duration of the partial erase in milliseconds"]
            #[inline(always)]
            pub const fn duration(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x7f;
                val as u8
            }
            #[doc = "Duration of the partial erase in milliseconds"]
            #[inline(always)]
            pub fn set_duration(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
            }
        }
        impl Default for Erasepagepartialcfg {
            #[inline(always)]
            fn default() -> Erasepagepartialcfg {
                Erasepagepartialcfg(0)
            }
        }
        impl core::fmt::Debug for Erasepagepartialcfg {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Erasepagepartialcfg")
                    .field("duration", &self.duration())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Erasepagepartialcfg {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Erasepagepartialcfg {
                    duration: u8,
                }
                let proxy = Erasepagepartialcfg {
                    duration: self.duration(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Register for erasing user information configuration registers"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Eraseuicr(pub u32);
        impl Eraseuicr {
            #[doc = "Register starting erase of all user information configuration registers. The erase must be enabled using CONFIG.WEN before the UICR can be erased."]
            #[inline(always)]
            pub const fn eraseuicr(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Register starting erase of all user information configuration registers. The erase must be enabled using CONFIG.WEN before the UICR can be erased."]
            #[inline(always)]
            pub fn set_eraseuicr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Eraseuicr {
            #[inline(always)]
            fn default() -> Eraseuicr {
                Eraseuicr(0)
            }
        }
        impl core::fmt::Debug for Eraseuicr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Eraseuicr")
                    .field("eraseuicr", &self.eraseuicr())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Eraseuicr {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Eraseuicr {
                    eraseuicr: bool,
                }
                let proxy = Eraseuicr {
                    eraseuicr: self.eraseuicr(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Ready flag"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ready(pub u32);
        impl Ready {
            #[doc = "NVMC is ready or busy"]
            #[inline(always)]
            pub const fn ready(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "NVMC is ready or busy"]
            #[inline(always)]
            pub fn set_ready(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Ready {
            #[inline(always)]
            fn default() -> Ready {
                Ready(0)
            }
        }
        impl core::fmt::Debug for Ready {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ready")
                    .field("ready", &self.ready())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ready {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Ready {
                    ready: bool,
                }
                let proxy = Ready {
                    ready: self.ready(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Wen {
            #[doc = "Read only access"]
            REN = 0x0,
            #[doc = "Write enabled"]
            WEN = 0x01,
            #[doc = "Erase enabled"]
            EEN = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl Wen {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Wen {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Wen {
            #[inline(always)]
            fn from(val: u8) -> Wen {
                Wen::from_bits(val)
            }
        }
        impl From<Wen> for u8 {
            #[inline(always)]
            fn from(val: Wen) -> u8 {
                Wen::to_bits(val)
            }
        }
    }
}
pub mod power {
    #[doc = "Power control"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Power {
        ptr: *mut u8,
    }
    unsafe impl Send for Power {}
    unsafe impl Sync for Power {}
    impl Power {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Enable Constant Latency mode"]
        #[inline(always)]
        pub const fn tasks_constlat(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
        }
        #[doc = "Enable Low-power mode (variable latency)"]
        #[inline(always)]
        pub const fn tasks_lowpwr(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
        }
        #[doc = "Power failure warning"]
        #[inline(always)]
        pub const fn events_pofwarn(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "CPU entered WFI/WFE sleep"]
        #[inline(always)]
        pub const fn events_sleepenter(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
        }
        #[doc = "CPU exited WFI/WFE sleep"]
        #[inline(always)]
        pub const fn events_sleepexit(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Reset reason"]
        #[inline(always)]
        pub const fn resetreas(self) -> crate::common::Reg<regs::Resetreas, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
        }
        #[doc = "System OFF register"]
        #[inline(always)]
        pub const fn systemoff(self) -> crate::common::Reg<regs::Systemoff, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Power failure comparator configuration"]
        #[inline(always)]
        pub const fn pofcon(self) -> crate::common::Reg<regs::Pofcon, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
        }
        #[doc = "General purpose retention register"]
        #[inline(always)]
        pub const fn gpregret(self) -> crate::common::Reg<regs::Gpregret, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
        }
        #[doc = "General purpose retention register"]
        #[inline(always)]
        pub const fn gpregret2(self) -> crate::common::Reg<regs::Gpregret2, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0520usize) as _) }
        }
        #[doc = "DC/DC enable register"]
        #[inline(always)]
        pub const fn dcdcen(self) -> crate::common::Reg<regs::Dcdcen, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0578usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn ram(self, n: usize) -> Ram {
            assert!(n < 8usize);
            unsafe { Ram::from_ptr(self.ptr.add(0x0900usize + n * 16usize) as _) }
        }
    }
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram {
        ptr: *mut u8,
    }
    unsafe impl Send for Ram {}
    unsafe impl Sync for Ram {}
    impl Ram {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description cluster: RAMn power control register. The RAM size will vary depending on product variant, and the RAMn register will only be present if the corresponding RAM AHB slave is present on the device."]
        #[inline(always)]
        pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Description cluster: RAMn power control set register"]
        #[inline(always)]
        pub const fn powerset(self) -> crate::common::Reg<regs::Power, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Description cluster: RAMn power control clear register"]
        #[inline(always)]
        pub const fn powerclr(self) -> crate::common::Reg<regs::Power, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "DC/DC enable register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dcdcen(pub u32);
        impl Dcdcen {
            #[doc = "Enable or disable DC/DC converter"]
            #[inline(always)]
            pub const fn dcdcen(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable DC/DC converter"]
            #[inline(always)]
            pub fn set_dcdcen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Dcdcen {
            #[inline(always)]
            fn default() -> Dcdcen {
                Dcdcen(0)
            }
        }
        impl core::fmt::Debug for Dcdcen {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dcdcen")
                    .field("dcdcen", &self.dcdcen())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dcdcen {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Dcdcen {
                    dcdcen: bool,
                }
                let proxy = Dcdcen {
                    dcdcen: self.dcdcen(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "General purpose retention register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Gpregret(pub u32);
        impl Gpregret {
            #[doc = "General purpose retention register"]
            #[inline(always)]
            pub const fn gpregret(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "General purpose retention register"]
            #[inline(always)]
            pub fn set_gpregret(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Gpregret {
            #[inline(always)]
            fn default() -> Gpregret {
                Gpregret(0)
            }
        }
        impl core::fmt::Debug for Gpregret {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Gpregret")
                    .field("gpregret", &self.gpregret())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Gpregret {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Gpregret {
                    gpregret: u8,
                }
                let proxy = Gpregret {
                    gpregret: self.gpregret(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "General purpose retention register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Gpregret2(pub u32);
        impl Gpregret2 {
            #[doc = "General purpose retention register"]
            #[inline(always)]
            pub const fn gpregret(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "General purpose retention register"]
            #[inline(always)]
            pub fn set_gpregret(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Gpregret2 {
            #[inline(always)]
            fn default() -> Gpregret2 {
                Gpregret2(0)
            }
        }
        impl core::fmt::Debug for Gpregret2 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Gpregret2")
                    .field("gpregret", &self.gpregret())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Gpregret2 {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Gpregret2 {
                    gpregret: u8,
                }
                let proxy = Gpregret2 {
                    gpregret: self.gpregret(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Write '1' to disable interrupt for event POFWARN"]
            #[inline(always)]
            pub const fn pofwarn(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event POFWARN"]
            #[inline(always)]
            pub fn set_pofwarn(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Write '1' to disable interrupt for event SLEEPENTER"]
            #[inline(always)]
            pub const fn sleepenter(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event SLEEPENTER"]
            #[inline(always)]
            pub fn set_sleepenter(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Write '1' to disable interrupt for event SLEEPEXIT"]
            #[inline(always)]
            pub const fn sleepexit(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event SLEEPEXIT"]
            #[inline(always)]
            pub fn set_sleepexit(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("pofwarn", &self.pofwarn())
                    .field("sleepenter", &self.sleepenter())
                    .field("sleepexit", &self.sleepexit())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    pofwarn: bool,
                    sleepenter: bool,
                    sleepexit: bool,
                }
                let proxy = Int {
                    pofwarn: self.pofwarn(),
                    sleepenter: self.sleepenter(),
                    sleepexit: self.sleepexit(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Power failure comparator configuration"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pofcon(pub u32);
        impl Pofcon {
            #[doc = "Enable or disable power failure comparator"]
            #[inline(always)]
            pub const fn pof(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable power failure comparator"]
            #[inline(always)]
            pub fn set_pof(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Power failure comparator threshold setting"]
            #[inline(always)]
            pub const fn threshold(&self) -> super::vals::Threshold {
                let val = (self.0 >> 1usize) & 0x0f;
                super::vals::Threshold::from_bits(val as u8)
            }
            #[doc = "Power failure comparator threshold setting"]
            #[inline(always)]
            pub fn set_threshold(&mut self, val: super::vals::Threshold) {
                self.0 = (self.0 & !(0x0f << 1usize)) | (((val.to_bits() as u32) & 0x0f) << 1usize);
            }
        }
        impl Default for Pofcon {
            #[inline(always)]
            fn default() -> Pofcon {
                Pofcon(0)
            }
        }
        impl core::fmt::Debug for Pofcon {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Pofcon")
                    .field("pof", &self.pof())
                    .field("threshold", &self.threshold())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Pofcon {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Pofcon {
                    pof: bool,
                    threshold: super::vals::Threshold,
                }
                let proxy = Pofcon {
                    pof: self.pof(),
                    threshold: self.threshold(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Description cluster: RAMn power control register. The RAM size will vary depending on product variant, and the RAMn register will only be present if the corresponding RAM AHB slave is present on the device."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Power(pub u32);
        impl Power {
            #[doc = "Keep RAM section S0 ON or OFF in System ON mode."]
            #[inline(always)]
            pub const fn s_power(&self, n: usize) -> bool {
                assert!(n < 2usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Keep RAM section S0 ON or OFF in System ON mode."]
            #[inline(always)]
            pub fn set_s_power(&mut self, n: usize, val: bool) {
                assert!(n < 2usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Keep retention on RAM section S0 when RAM section is in OFF"]
            #[inline(always)]
            pub const fn s_retention(&self, n: usize) -> bool {
                assert!(n < 2usize);
                let offs = 16usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Keep retention on RAM section S0 when RAM section is in OFF"]
            #[inline(always)]
            pub fn set_s_retention(&mut self, n: usize, val: bool) {
                assert!(n < 2usize);
                let offs = 16usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Power {
            #[inline(always)]
            fn default() -> Power {
                Power(0)
            }
        }
        impl core::fmt::Debug for Power {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Power")
                    .field("s_power", &[self.s_power(0usize), self.s_power(1usize)])
                    .field(
                        "s_retention",
                        &[self.s_retention(0usize), self.s_retention(1usize)],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Power {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Power {
                    s_power: [bool; 2usize],
                    s_retention: [bool; 2usize],
                }
                let proxy = Power {
                    s_power: [self.s_power(0usize), self.s_power(1usize)],
                    s_retention: [self.s_retention(0usize), self.s_retention(1usize)],
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Reset reason"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Resetreas(pub u32);
        impl Resetreas {
            #[doc = "Reset from pin-reset detected"]
            #[inline(always)]
            pub const fn resetpin(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Reset from pin-reset detected"]
            #[inline(always)]
            pub fn set_resetpin(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Reset from watchdog detected"]
            #[inline(always)]
            pub const fn dog(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Reset from watchdog detected"]
            #[inline(always)]
            pub fn set_dog(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Reset from soft reset detected"]
            #[inline(always)]
            pub const fn sreq(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Reset from soft reset detected"]
            #[inline(always)]
            pub fn set_sreq(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Reset from CPU lock-up detected"]
            #[inline(always)]
            pub const fn lockup(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Reset from CPU lock-up detected"]
            #[inline(always)]
            pub fn set_lockup(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Reset due to wake up from System OFF mode when wakeup is triggered from DETECT signal from GPIO"]
            #[inline(always)]
            pub const fn off(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Reset due to wake up from System OFF mode when wakeup is triggered from DETECT signal from GPIO"]
            #[inline(always)]
            pub fn set_off(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "Reset due to wake up from System OFF mode when wakeup is triggered from entering into debug interface mode"]
            #[inline(always)]
            pub const fn dif(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Reset due to wake up from System OFF mode when wakeup is triggered from entering into debug interface mode"]
            #[inline(always)]
            pub fn set_dif(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
        }
        impl Default for Resetreas {
            #[inline(always)]
            fn default() -> Resetreas {
                Resetreas(0)
            }
        }
        impl core::fmt::Debug for Resetreas {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Resetreas")
                    .field("resetpin", &self.resetpin())
                    .field("dog", &self.dog())
                    .field("sreq", &self.sreq())
                    .field("lockup", &self.lockup())
                    .field("off", &self.off())
                    .field("dif", &self.dif())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Resetreas {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Resetreas {
                    resetpin: bool,
                    dog: bool,
                    sreq: bool,
                    lockup: bool,
                    off: bool,
                    dif: bool,
                }
                let proxy = Resetreas {
                    resetpin: self.resetpin(),
                    dog: self.dog(),
                    sreq: self.sreq(),
                    lockup: self.lockup(),
                    off: self.off(),
                    dif: self.dif(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "System OFF register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Systemoff(pub u32);
        impl Systemoff {
            #[doc = "Enable System OFF mode"]
            #[inline(always)]
            pub const fn systemoff(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable System OFF mode"]
            #[inline(always)]
            pub fn set_systemoff(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Systemoff {
            #[inline(always)]
            fn default() -> Systemoff {
                Systemoff(0)
            }
        }
        impl core::fmt::Debug for Systemoff {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Systemoff")
                    .field("systemoff", &self.systemoff())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Systemoff {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Systemoff {
                    systemoff: bool,
                }
                let proxy = Systemoff {
                    systemoff: self.systemoff(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Threshold {
            _RESERVED_0 = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            #[doc = "Set threshold to 1.7 V"]
            V17 = 0x04,
            #[doc = "Set threshold to 1.8 V"]
            V18 = 0x05,
            #[doc = "Set threshold to 1.9 V"]
            V19 = 0x06,
            #[doc = "Set threshold to 2.0 V"]
            V20 = 0x07,
            #[doc = "Set threshold to 2.1 V"]
            V21 = 0x08,
            #[doc = "Set threshold to 2.2 V"]
            V22 = 0x09,
            #[doc = "Set threshold to 2.3 V"]
            V23 = 0x0a,
            #[doc = "Set threshold to 2.4 V"]
            V24 = 0x0b,
            #[doc = "Set threshold to 2.5 V"]
            V25 = 0x0c,
            #[doc = "Set threshold to 2.6 V"]
            V26 = 0x0d,
            #[doc = "Set threshold to 2.7 V"]
            V27 = 0x0e,
            #[doc = "Set threshold to 2.8 V"]
            V28 = 0x0f,
        }
        impl Threshold {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Threshold {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Threshold {
            #[inline(always)]
            fn from(val: u8) -> Threshold {
                Threshold::from_bits(val)
            }
        }
        impl From<Threshold> for u8 {
            #[inline(always)]
            fn from(val: Threshold) -> u8 {
                Threshold::to_bits(val)
            }
        }
    }
}
pub mod ppi {
    #[doc = "PPI Channel"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch {
        ptr: *mut u8,
    }
    unsafe impl Send for Ch {}
    unsafe impl Sync for Ch {}
    impl Ch {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description cluster: Channel n event endpoint"]
        #[inline(always)]
        pub const fn eep(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Description cluster: Channel n task endpoint"]
        #[inline(always)]
        pub const fn tep(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
    }
    #[doc = "Fork"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fork {
        ptr: *mut u8,
    }
    unsafe impl Send for Fork {}
    unsafe impl Sync for Fork {}
    impl Fork {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description cluster: Channel n task endpoint"]
        #[inline(always)]
        pub const fn tep(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
    }
    #[doc = "Programmable Peripheral Interconnect"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ppi {
        ptr: *mut u8,
    }
    unsafe impl Send for Ppi {}
    unsafe impl Sync for Ppi {}
    impl Ppi {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Channel group tasks"]
        #[inline(always)]
        pub const fn tasks_chg(self, n: usize) -> TasksChg {
            assert!(n < 6usize);
            unsafe { TasksChg::from_ptr(self.ptr.add(0x0usize + n * 8usize) as _) }
        }
        #[doc = "Channel enable register"]
        #[inline(always)]
        pub const fn chen(self) -> crate::common::Reg<regs::Chen, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Channel enable set register"]
        #[inline(always)]
        pub const fn chenset(self) -> crate::common::Reg<regs::Chen, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Channel enable clear register"]
        #[inline(always)]
        pub const fn chenclr(self) -> crate::common::Reg<regs::Chen, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "PPI Channel"]
        #[inline(always)]
        pub const fn ch(self, n: usize) -> Ch {
            assert!(n < 10usize);
            unsafe { Ch::from_ptr(self.ptr.add(0x0510usize + n * 8usize) as _) }
        }
        #[doc = "Description collection: Channel group n"]
        #[inline(always)]
        pub const fn chg(self, n: usize) -> crate::common::Reg<regs::Chg, crate::common::RW> {
            assert!(n < 6usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0800usize + n * 4usize) as _) }
        }
        #[doc = "Fork"]
        #[inline(always)]
        pub const fn fork(self, n: usize) -> Fork {
            assert!(n < 32usize);
            unsafe { Fork::from_ptr(self.ptr.add(0x0910usize + n * 4usize) as _) }
        }
    }
    #[doc = "Channel group tasks"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TasksChg {
        ptr: *mut u8,
    }
    unsafe impl Send for TasksChg {}
    unsafe impl Sync for TasksChg {}
    impl TasksChg {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description cluster: Enable channel group n"]
        #[inline(always)]
        pub const fn en(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Description cluster: Disable channel group n"]
        #[inline(always)]
        pub const fn dis(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Channel enable register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Chen(pub u32);
        impl Chen {
            #[doc = "Enable or disable channel 0"]
            #[inline(always)]
            pub const fn ch(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable channel 0"]
            #[inline(always)]
            pub fn set_ch(&mut self, n: usize, val: bool) {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Chen {
            #[inline(always)]
            fn default() -> Chen {
                Chen(0)
            }
        }
        impl core::fmt::Debug for Chen {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Chen")
                    .field(
                        "ch",
                        &[
                            self.ch(0usize),
                            self.ch(1usize),
                            self.ch(2usize),
                            self.ch(3usize),
                            self.ch(4usize),
                            self.ch(5usize),
                            self.ch(6usize),
                            self.ch(7usize),
                            self.ch(8usize),
                            self.ch(9usize),
                            self.ch(10usize),
                            self.ch(11usize),
                            self.ch(12usize),
                            self.ch(13usize),
                            self.ch(14usize),
                            self.ch(15usize),
                            self.ch(16usize),
                            self.ch(17usize),
                            self.ch(18usize),
                            self.ch(19usize),
                            self.ch(20usize),
                            self.ch(21usize),
                            self.ch(22usize),
                            self.ch(23usize),
                            self.ch(24usize),
                            self.ch(25usize),
                            self.ch(26usize),
                            self.ch(27usize),
                            self.ch(28usize),
                            self.ch(29usize),
                            self.ch(30usize),
                            self.ch(31usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Chen {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Chen {
                    ch: [bool; 32usize],
                }
                let proxy = Chen {
                    ch: [
                        self.ch(0usize),
                        self.ch(1usize),
                        self.ch(2usize),
                        self.ch(3usize),
                        self.ch(4usize),
                        self.ch(5usize),
                        self.ch(6usize),
                        self.ch(7usize),
                        self.ch(8usize),
                        self.ch(9usize),
                        self.ch(10usize),
                        self.ch(11usize),
                        self.ch(12usize),
                        self.ch(13usize),
                        self.ch(14usize),
                        self.ch(15usize),
                        self.ch(16usize),
                        self.ch(17usize),
                        self.ch(18usize),
                        self.ch(19usize),
                        self.ch(20usize),
                        self.ch(21usize),
                        self.ch(22usize),
                        self.ch(23usize),
                        self.ch(24usize),
                        self.ch(25usize),
                        self.ch(26usize),
                        self.ch(27usize),
                        self.ch(28usize),
                        self.ch(29usize),
                        self.ch(30usize),
                        self.ch(31usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Description collection: Channel group n"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Chg(pub u32);
        impl Chg {
            #[doc = "Include or exclude channel 0"]
            #[inline(always)]
            pub const fn ch(&self, n: usize) -> bool {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Include or exclude channel 0"]
            #[inline(always)]
            pub fn set_ch(&mut self, n: usize, val: bool) {
                assert!(n < 32usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Chg {
            #[inline(always)]
            fn default() -> Chg {
                Chg(0)
            }
        }
        impl core::fmt::Debug for Chg {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Chg")
                    .field(
                        "ch",
                        &[
                            self.ch(0usize),
                            self.ch(1usize),
                            self.ch(2usize),
                            self.ch(3usize),
                            self.ch(4usize),
                            self.ch(5usize),
                            self.ch(6usize),
                            self.ch(7usize),
                            self.ch(8usize),
                            self.ch(9usize),
                            self.ch(10usize),
                            self.ch(11usize),
                            self.ch(12usize),
                            self.ch(13usize),
                            self.ch(14usize),
                            self.ch(15usize),
                            self.ch(16usize),
                            self.ch(17usize),
                            self.ch(18usize),
                            self.ch(19usize),
                            self.ch(20usize),
                            self.ch(21usize),
                            self.ch(22usize),
                            self.ch(23usize),
                            self.ch(24usize),
                            self.ch(25usize),
                            self.ch(26usize),
                            self.ch(27usize),
                            self.ch(28usize),
                            self.ch(29usize),
                            self.ch(30usize),
                            self.ch(31usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Chg {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Chg {
                    ch: [bool; 32usize],
                }
                let proxy = Chg {
                    ch: [
                        self.ch(0usize),
                        self.ch(1usize),
                        self.ch(2usize),
                        self.ch(3usize),
                        self.ch(4usize),
                        self.ch(5usize),
                        self.ch(6usize),
                        self.ch(7usize),
                        self.ch(8usize),
                        self.ch(9usize),
                        self.ch(10usize),
                        self.ch(11usize),
                        self.ch(12usize),
                        self.ch(13usize),
                        self.ch(14usize),
                        self.ch(15usize),
                        self.ch(16usize),
                        self.ch(17usize),
                        self.ch(18usize),
                        self.ch(19usize),
                        self.ch(20usize),
                        self.ch(21usize),
                        self.ch(22usize),
                        self.ch(23usize),
                        self.ch(24usize),
                        self.ch(25usize),
                        self.ch(26usize),
                        self.ch(27usize),
                        self.ch(28usize),
                        self.ch(29usize),
                        self.ch(30usize),
                        self.ch(31usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
}
pub mod qdec {
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psel {
        ptr: *mut u8,
    }
    unsafe impl Send for Psel {}
    unsafe impl Sync for Psel {}
    impl Psel {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Pin select for LED signal"]
        #[inline(always)]
        pub const fn led(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Pin select for A signal"]
        #[inline(always)]
        pub const fn a(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Pin select for B signal"]
        #[inline(always)]
        pub const fn b(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
    }
    #[doc = "Quadrature Decoder"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qdec {
        ptr: *mut u8,
    }
    unsafe impl Send for Qdec {}
    unsafe impl Sync for Qdec {}
    impl Qdec {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Task starting the quadrature decoder"]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Task stopping the quadrature decoder"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Read and clear ACC and ACCDBL"]
        #[inline(always)]
        pub const fn tasks_readclracc(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Read and clear ACC"]
        #[inline(always)]
        pub const fn tasks_rdclracc(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Read and clear ACCDBL"]
        #[inline(always)]
        pub const fn tasks_rdclrdbl(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "Event being generated for every new sample value written to the SAMPLE register"]
        #[inline(always)]
        pub const fn events_samplerdy(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "Non-null report ready"]
        #[inline(always)]
        pub const fn events_reportrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "ACC or ACCDBL register overflow"]
        #[inline(always)]
        pub const fn events_accof(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "Double displacement(s) detected"]
        #[inline(always)]
        pub const fn events_dblrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
        }
        #[doc = "QDEC has been stopped"]
        #[inline(always)]
        pub const fn events_stopped(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Enable the quadrature decoder"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "LED output pin polarity"]
        #[inline(always)]
        pub const fn ledpol(self) -> crate::common::Reg<regs::Ledpol, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Sample period"]
        #[inline(always)]
        pub const fn sampleper(self) -> crate::common::Reg<regs::Sampleper, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Motion sample value"]
        #[inline(always)]
        pub const fn sample(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
        }
        #[doc = "Number of samples to be taken before REPORTRDY and DBLRDY events can be generated"]
        #[inline(always)]
        pub const fn reportper(self) -> crate::common::Reg<regs::Reportper, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
        }
        #[doc = "Register accumulating the valid transitions"]
        #[inline(always)]
        pub const fn acc(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
        }
        #[doc = "Snapshot of the ACC register, updated by the READCLRACC or RDCLRACC task"]
        #[inline(always)]
        pub const fn accread(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn psel(self) -> Psel {
            unsafe { Psel::from_ptr(self.ptr.add(0x051cusize) as _) }
        }
        #[doc = "Enable input debounce filters"]
        #[inline(always)]
        pub const fn dbfen(self) -> crate::common::Reg<regs::Dbfen, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0528usize) as _) }
        }
        #[doc = "Time period the LED is switched ON prior to sampling"]
        #[inline(always)]
        pub const fn ledpre(self) -> crate::common::Reg<regs::Ledpre, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0540usize) as _) }
        }
        #[doc = "Register accumulating the number of detected double transitions"]
        #[inline(always)]
        pub const fn accdbl(self) -> crate::common::Reg<regs::Accdbl, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0544usize) as _) }
        }
        #[doc = "Snapshot of the ACCDBL, updated by the READCLRACC or RDCLRDBL task"]
        #[inline(always)]
        pub const fn accdblread(self) -> crate::common::Reg<regs::Accdblread, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0548usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Register accumulating the number of detected double transitions"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Accdbl(pub u32);
        impl Accdbl {
            #[doc = "Register accumulating the number of detected double or illegal transitions. ( SAMPLE = 2 )."]
            #[inline(always)]
            pub const fn accdbl(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "Register accumulating the number of detected double or illegal transitions. ( SAMPLE = 2 )."]
            #[inline(always)]
            pub fn set_accdbl(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Accdbl {
            #[inline(always)]
            fn default() -> Accdbl {
                Accdbl(0)
            }
        }
        impl core::fmt::Debug for Accdbl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Accdbl")
                    .field("accdbl", &self.accdbl())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Accdbl {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Accdbl {
                    accdbl: u8,
                }
                let proxy = Accdbl {
                    accdbl: self.accdbl(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Snapshot of the ACCDBL, updated by the READCLRACC or RDCLRDBL task"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Accdblread(pub u32);
        impl Accdblread {
            #[doc = "Snapshot of the ACCDBL register. This field is updated when the READCLRACC or RDCLRDBL task is triggered."]
            #[inline(always)]
            pub const fn accdblread(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "Snapshot of the ACCDBL register. This field is updated when the READCLRACC or RDCLRDBL task is triggered."]
            #[inline(always)]
            pub fn set_accdblread(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Accdblread {
            #[inline(always)]
            fn default() -> Accdblread {
                Accdblread(0)
            }
        }
        impl core::fmt::Debug for Accdblread {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Accdblread")
                    .field("accdblread", &self.accdblread())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Accdblread {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Accdblread {
                    accdblread: u8,
                }
                let proxy = Accdblread {
                    accdblread: self.accdblread(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Enable input debounce filters"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dbfen(pub u32);
        impl Dbfen {
            #[doc = "Enable input debounce filters"]
            #[inline(always)]
            pub const fn dbfen(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable input debounce filters"]
            #[inline(always)]
            pub fn set_dbfen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Dbfen {
            #[inline(always)]
            fn default() -> Dbfen {
                Dbfen(0)
            }
        }
        impl core::fmt::Debug for Dbfen {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dbfen")
                    .field("dbfen", &self.dbfen())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dbfen {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Dbfen {
                    dbfen: bool,
                }
                let proxy = Dbfen {
                    dbfen: self.dbfen(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Enable the quadrature decoder"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable the quadrature decoder"]
            #[inline(always)]
            pub const fn enable(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable the quadrature decoder"]
            #[inline(always)]
            pub fn set_enable(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        impl core::fmt::Debug for Enable {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Enable")
                    .field("enable", &self.enable())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Enable {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Enable {
                    enable: bool,
                }
                let proxy = Enable {
                    enable: self.enable(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Write '1' to disable interrupt for event SAMPLERDY"]
            #[inline(always)]
            pub const fn samplerdy(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event SAMPLERDY"]
            #[inline(always)]
            pub fn set_samplerdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to disable interrupt for event REPORTRDY"]
            #[inline(always)]
            pub const fn reportrdy(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event REPORTRDY"]
            #[inline(always)]
            pub fn set_reportrdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to disable interrupt for event ACCOF"]
            #[inline(always)]
            pub const fn accof(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event ACCOF"]
            #[inline(always)]
            pub fn set_accof(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Write '1' to disable interrupt for event DBLRDY"]
            #[inline(always)]
            pub const fn dblrdy(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event DBLRDY"]
            #[inline(always)]
            pub fn set_dblrdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Write '1' to disable interrupt for event STOPPED"]
            #[inline(always)]
            pub const fn stopped(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event STOPPED"]
            #[inline(always)]
            pub fn set_stopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("samplerdy", &self.samplerdy())
                    .field("reportrdy", &self.reportrdy())
                    .field("accof", &self.accof())
                    .field("dblrdy", &self.dblrdy())
                    .field("stopped", &self.stopped())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    samplerdy: bool,
                    reportrdy: bool,
                    accof: bool,
                    dblrdy: bool,
                    stopped: bool,
                }
                let proxy = Int {
                    samplerdy: self.samplerdy(),
                    reportrdy: self.reportrdy(),
                    accof: self.accof(),
                    dblrdy: self.dblrdy(),
                    stopped: self.stopped(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "LED output pin polarity"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ledpol(pub u32);
        impl Ledpol {
            #[doc = "LED output pin polarity"]
            #[inline(always)]
            pub const fn ledpol(&self) -> super::vals::Ledpol {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Ledpol::from_bits(val as u8)
            }
            #[doc = "LED output pin polarity"]
            #[inline(always)]
            pub fn set_ledpol(&mut self, val: super::vals::Ledpol) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Ledpol {
            #[inline(always)]
            fn default() -> Ledpol {
                Ledpol(0)
            }
        }
        impl core::fmt::Debug for Ledpol {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ledpol")
                    .field("ledpol", &self.ledpol())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ledpol {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Ledpol {
                    ledpol: super::vals::Ledpol,
                }
                let proxy = Ledpol {
                    ledpol: self.ledpol(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Time period the LED is switched ON prior to sampling"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ledpre(pub u32);
        impl Ledpre {
            #[doc = "Period in us the LED is switched on prior to sampling"]
            #[inline(always)]
            pub const fn ledpre(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x01ff;
                val as u16
            }
            #[doc = "Period in us the LED is switched on prior to sampling"]
            #[inline(always)]
            pub fn set_ledpre(&mut self, val: u16) {
                self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
            }
        }
        impl Default for Ledpre {
            #[inline(always)]
            fn default() -> Ledpre {
                Ledpre(0)
            }
        }
        impl core::fmt::Debug for Ledpre {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ledpre")
                    .field("ledpre", &self.ledpre())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ledpre {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Ledpre {
                    ledpre: u16,
                }
                let proxy = Ledpre {
                    ledpre: self.ledpre(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Number of samples to be taken before REPORTRDY and DBLRDY events can be generated"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Reportper(pub u32);
        impl Reportper {
            #[doc = "Specifies the number of samples to be accumulated in the ACC register before the REPORTRDY and DBLRDY events can be generated."]
            #[inline(always)]
            pub const fn reportper(&self) -> super::vals::Reportper {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Reportper::from_bits(val as u8)
            }
            #[doc = "Specifies the number of samples to be accumulated in the ACC register before the REPORTRDY and DBLRDY events can be generated."]
            #[inline(always)]
            pub fn set_reportper(&mut self, val: super::vals::Reportper) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Reportper {
            #[inline(always)]
            fn default() -> Reportper {
                Reportper(0)
            }
        }
        impl core::fmt::Debug for Reportper {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Reportper")
                    .field("reportper", &self.reportper())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Reportper {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Reportper {
                    reportper: super::vals::Reportper,
                }
                let proxy = Reportper {
                    reportper: self.reportper(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Sample period"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sampleper(pub u32);
        impl Sampleper {
            #[doc = "Sample period. The SAMPLE register will be updated for every new sample"]
            #[inline(always)]
            pub const fn sampleper(&self) -> super::vals::Sampleper {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Sampleper::from_bits(val as u8)
            }
            #[doc = "Sample period. The SAMPLE register will be updated for every new sample"]
            #[inline(always)]
            pub fn set_sampleper(&mut self, val: super::vals::Sampleper) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Sampleper {
            #[inline(always)]
            fn default() -> Sampleper {
                Sampleper(0)
            }
        }
        impl core::fmt::Debug for Sampleper {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Sampleper")
                    .field("sampleper", &self.sampleper())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Sampleper {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Sampleper {
                    sampleper: super::vals::Sampleper,
                }
                let proxy = Sampleper {
                    sampleper: self.sampleper(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event REPORTRDY and task READCLRACC"]
            #[inline(always)]
            pub const fn reportrdy_readclracc(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event REPORTRDY and task READCLRACC"]
            #[inline(always)]
            pub fn set_reportrdy_readclracc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Shortcut between event SAMPLERDY and task STOP"]
            #[inline(always)]
            pub const fn samplerdy_stop(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event SAMPLERDY and task STOP"]
            #[inline(always)]
            pub fn set_samplerdy_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Shortcut between event REPORTRDY and task RDCLRACC"]
            #[inline(always)]
            pub const fn reportrdy_rdclracc(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event REPORTRDY and task RDCLRACC"]
            #[inline(always)]
            pub fn set_reportrdy_rdclracc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Shortcut between event REPORTRDY and task STOP"]
            #[inline(always)]
            pub const fn reportrdy_stop(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event REPORTRDY and task STOP"]
            #[inline(always)]
            pub fn set_reportrdy_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Shortcut between event DBLRDY and task RDCLRDBL"]
            #[inline(always)]
            pub const fn dblrdy_rdclrdbl(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event DBLRDY and task RDCLRDBL"]
            #[inline(always)]
            pub fn set_dblrdy_rdclrdbl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Shortcut between event DBLRDY and task STOP"]
            #[inline(always)]
            pub const fn dblrdy_stop(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event DBLRDY and task STOP"]
            #[inline(always)]
            pub fn set_dblrdy_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Shortcut between event SAMPLERDY and task READCLRACC"]
            #[inline(always)]
            pub const fn samplerdy_readclracc(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event SAMPLERDY and task READCLRACC"]
            #[inline(always)]
            pub fn set_samplerdy_readclracc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
        impl core::fmt::Debug for Shorts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Shorts")
                    .field("reportrdy_readclracc", &self.reportrdy_readclracc())
                    .field("samplerdy_stop", &self.samplerdy_stop())
                    .field("reportrdy_rdclracc", &self.reportrdy_rdclracc())
                    .field("reportrdy_stop", &self.reportrdy_stop())
                    .field("dblrdy_rdclrdbl", &self.dblrdy_rdclrdbl())
                    .field("dblrdy_stop", &self.dblrdy_stop())
                    .field("samplerdy_readclracc", &self.samplerdy_readclracc())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Shorts {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Shorts {
                    reportrdy_readclracc: bool,
                    samplerdy_stop: bool,
                    reportrdy_rdclracc: bool,
                    reportrdy_stop: bool,
                    dblrdy_rdclrdbl: bool,
                    dblrdy_stop: bool,
                    samplerdy_readclracc: bool,
                }
                let proxy = Shorts {
                    reportrdy_readclracc: self.reportrdy_readclracc(),
                    samplerdy_stop: self.samplerdy_stop(),
                    reportrdy_rdclracc: self.reportrdy_rdclracc(),
                    reportrdy_stop: self.reportrdy_stop(),
                    dblrdy_rdclrdbl: self.dblrdy_rdclrdbl(),
                    dblrdy_stop: self.dblrdy_stop(),
                    samplerdy_readclracc: self.samplerdy_readclracc(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Ledpol {
            #[doc = "Led active on output pin low"]
            ACTIVE_LOW = 0x0,
            #[doc = "Led active on output pin high"]
            ACTIVE_HIGH = 0x01,
        }
        impl Ledpol {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Ledpol {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Ledpol {
            #[inline(always)]
            fn from(val: u8) -> Ledpol {
                Ledpol::from_bits(val)
            }
        }
        impl From<Ledpol> for u8 {
            #[inline(always)]
            fn from(val: Ledpol) -> u8 {
                Ledpol::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Reportper {
            #[doc = "10 samples/report"]
            _10SMPL = 0x0,
            #[doc = "40 samples/report"]
            _40SMPL = 0x01,
            #[doc = "80 samples/report"]
            _80SMPL = 0x02,
            #[doc = "120 samples/report"]
            _120SMPL = 0x03,
            #[doc = "160 samples/report"]
            _160SMPL = 0x04,
            #[doc = "200 samples/report"]
            _200SMPL = 0x05,
            #[doc = "240 samples/report"]
            _240SMPL = 0x06,
            #[doc = "280 samples/report"]
            _280SMPL = 0x07,
            #[doc = "1 sample/report"]
            _1SMPL = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl Reportper {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Reportper {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Reportper {
            #[inline(always)]
            fn from(val: u8) -> Reportper {
                Reportper::from_bits(val)
            }
        }
        impl From<Reportper> for u8 {
            #[inline(always)]
            fn from(val: Reportper) -> u8 {
                Reportper::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Sampleper {
            #[doc = "128 us"]
            _128US = 0x0,
            #[doc = "256 us"]
            _256US = 0x01,
            #[doc = "512 us"]
            _512US = 0x02,
            #[doc = "1024 us"]
            _1024US = 0x03,
            #[doc = "2048 us"]
            _2048US = 0x04,
            #[doc = "4096 us"]
            _4096US = 0x05,
            #[doc = "8192 us"]
            _8192US = 0x06,
            #[doc = "16384 us"]
            _16384US = 0x07,
            #[doc = "32768 us"]
            _32MS = 0x08,
            #[doc = "65536 us"]
            _65MS = 0x09,
            #[doc = "131072 us"]
            _131MS = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl Sampleper {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Sampleper {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Sampleper {
            #[inline(always)]
            fn from(val: u8) -> Sampleper {
                Sampleper::from_bits(val)
            }
        }
        impl From<Sampleper> for u8 {
            #[inline(always)]
            fn from(val: Sampleper) -> u8 {
                Sampleper::to_bits(val)
            }
        }
    }
}
pub mod radio {
    #[doc = "2.4 GHz radio"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Radio {
        ptr: *mut u8,
    }
    unsafe impl Send for Radio {}
    unsafe impl Sync for Radio {}
    impl Radio {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Enable RADIO in TX mode"]
        #[inline(always)]
        pub const fn tasks_txen(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Enable RADIO in RX mode"]
        #[inline(always)]
        pub const fn tasks_rxen(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Start RADIO"]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Stop RADIO"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Disable RADIO"]
        #[inline(always)]
        pub const fn tasks_disable(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "Start the RSSI and take one single sample of the receive signal strength"]
        #[inline(always)]
        pub const fn tasks_rssistart(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "Stop the RSSI measurement"]
        #[inline(always)]
        pub const fn tasks_rssistop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
        }
        #[doc = "Start the bit counter"]
        #[inline(always)]
        pub const fn tasks_bcstart(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
        #[doc = "Stop the bit counter"]
        #[inline(always)]
        pub const fn tasks_bcstop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }
        #[doc = "RADIO has ramped up and is ready to be started"]
        #[inline(always)]
        pub const fn events_ready(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "Address sent or received"]
        #[inline(always)]
        pub const fn events_address(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "Packet payload sent or received"]
        #[inline(always)]
        pub const fn events_payload(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "Packet sent or received"]
        #[inline(always)]
        pub const fn events_end(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
        }
        #[doc = "RADIO has been disabled"]
        #[inline(always)]
        pub const fn events_disabled(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
        }
        #[doc = "A device address match occurred on the last received packet"]
        #[inline(always)]
        pub const fn events_devmatch(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
        }
        #[doc = "No device address match occurred on the last received packet"]
        #[inline(always)]
        pub const fn events_devmiss(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
        }
        #[doc = "Sampling of receive signal strength complete"]
        #[inline(always)]
        pub const fn events_rssiend(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
        }
        #[doc = "Bit counter reached bit count value"]
        #[inline(always)]
        pub const fn events_bcmatch(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
        }
        #[doc = "Packet received with CRC ok"]
        #[inline(always)]
        pub const fn events_crcok(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
        }
        #[doc = "Packet received with CRC error"]
        #[inline(always)]
        pub const fn events_crcerror(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
        }
        #[doc = "RADIO has ramped up and is ready to be started TX path"]
        #[inline(always)]
        pub const fn events_txready(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0154usize) as _) }
        }
        #[doc = "RADIO has ramped up and is ready to be started RX path"]
        #[inline(always)]
        pub const fn events_rxready(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0158usize) as _) }
        }
        #[doc = "Generated when last bit is sent on air, or received from air"]
        #[inline(always)]
        pub const fn events_phyend(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x016cusize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "CRC status"]
        #[inline(always)]
        pub const fn crcstatus(self) -> crate::common::Reg<regs::Crcstatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
        }
        #[doc = "Received address"]
        #[inline(always)]
        pub const fn rxmatch(self) -> crate::common::Reg<regs::Rxmatch, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0408usize) as _) }
        }
        #[doc = "CRC field of previously received packet"]
        #[inline(always)]
        pub const fn rxcrc(self) -> crate::common::Reg<regs::Rxcrc, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x040cusize) as _) }
        }
        #[doc = "Device address match index"]
        #[inline(always)]
        pub const fn dai(self) -> crate::common::Reg<regs::Dai, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0410usize) as _) }
        }
        #[doc = "Payload status"]
        #[inline(always)]
        pub const fn pdustat(self) -> crate::common::Reg<regs::Pdustat, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0414usize) as _) }
        }
        #[doc = "Packet pointer"]
        #[inline(always)]
        pub const fn packetptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Frequency"]
        #[inline(always)]
        pub const fn frequency(self) -> crate::common::Reg<regs::Frequency, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Output power"]
        #[inline(always)]
        pub const fn txpower(self) -> crate::common::Reg<regs::Txpower, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
        }
        #[doc = "Data rate and modulation"]
        #[inline(always)]
        pub const fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
        }
        #[doc = "Packet configuration register 0"]
        #[inline(always)]
        pub const fn pcnf0(self) -> crate::common::Reg<regs::Pcnf0, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
        }
        #[doc = "Packet configuration register 1"]
        #[inline(always)]
        pub const fn pcnf1(self) -> crate::common::Reg<regs::Pcnf1, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
        }
        #[doc = "Base address 0"]
        #[inline(always)]
        pub const fn base0(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
        }
        #[doc = "Base address 1"]
        #[inline(always)]
        pub const fn base1(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0520usize) as _) }
        }
        #[doc = "Prefixes bytes for logical addresses 0-3"]
        #[inline(always)]
        pub const fn prefix0(self) -> crate::common::Reg<regs::Prefix0, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
        }
        #[doc = "Prefixes bytes for logical addresses 4-7"]
        #[inline(always)]
        pub const fn prefix1(self) -> crate::common::Reg<regs::Prefix1, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0528usize) as _) }
        }
        #[doc = "Transmit address select"]
        #[inline(always)]
        pub const fn txaddress(self) -> crate::common::Reg<regs::Txaddress, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x052cusize) as _) }
        }
        #[doc = "Receive address select"]
        #[inline(always)]
        pub const fn rxaddresses(self) -> crate::common::Reg<regs::Rxaddresses, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0530usize) as _) }
        }
        #[doc = "CRC configuration"]
        #[inline(always)]
        pub const fn crccnf(self) -> crate::common::Reg<regs::Crccnf, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0534usize) as _) }
        }
        #[doc = "CRC polynomial"]
        #[inline(always)]
        pub const fn crcpoly(self) -> crate::common::Reg<regs::Crcpoly, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0538usize) as _) }
        }
        #[doc = "CRC initial value"]
        #[inline(always)]
        pub const fn crcinit(self) -> crate::common::Reg<regs::Crcinit, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x053cusize) as _) }
        }
        #[doc = "Interframe spacing in us"]
        #[inline(always)]
        pub const fn tifs(self) -> crate::common::Reg<regs::Tifs, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0544usize) as _) }
        }
        #[doc = "RSSI sample"]
        #[inline(always)]
        pub const fn rssisample(self) -> crate::common::Reg<regs::Rssisample, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0548usize) as _) }
        }
        #[doc = "Current radio state"]
        #[inline(always)]
        pub const fn state(self) -> crate::common::Reg<regs::State, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0550usize) as _) }
        }
        #[doc = "Data whitening initial value"]
        #[inline(always)]
        pub const fn datawhiteiv(self) -> crate::common::Reg<regs::Datawhiteiv, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0554usize) as _) }
        }
        #[doc = "Bit counter compare"]
        #[inline(always)]
        pub const fn bcc(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0560usize) as _) }
        }
        #[doc = "Description collection: Device address base segment n"]
        #[inline(always)]
        pub const fn dab(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0600usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Device address prefix n"]
        #[inline(always)]
        pub const fn dap(self, n: usize) -> crate::common::Reg<regs::Dap, crate::common::RW> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0620usize + n * 4usize) as _) }
        }
        #[doc = "Device address match configuration"]
        #[inline(always)]
        pub const fn dacnf(self) -> crate::common::Reg<regs::Dacnf, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0640usize) as _) }
        }
        #[doc = "Radio mode configuration register 0"]
        #[inline(always)]
        pub const fn modecnf0(self) -> crate::common::Reg<regs::Modecnf0, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0650usize) as _) }
        }
        #[doc = "Peripheral power control"]
        #[inline(always)]
        pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ffcusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "CRC configuration"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Crccnf(pub u32);
        impl Crccnf {
            #[doc = "CRC length in number of bytes"]
            #[inline(always)]
            pub const fn len(&self) -> super::vals::Len {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Len::from_bits(val as u8)
            }
            #[doc = "CRC length in number of bytes"]
            #[inline(always)]
            pub fn set_len(&mut self, val: super::vals::Len) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
            #[doc = "Include or exclude packet address field out of CRC calculation."]
            #[inline(always)]
            pub const fn skipaddr(&self) -> super::vals::Skipaddr {
                let val = (self.0 >> 8usize) & 0x03;
                super::vals::Skipaddr::from_bits(val as u8)
            }
            #[doc = "Include or exclude packet address field out of CRC calculation."]
            #[inline(always)]
            pub fn set_skipaddr(&mut self, val: super::vals::Skipaddr) {
                self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
            }
        }
        impl Default for Crccnf {
            #[inline(always)]
            fn default() -> Crccnf {
                Crccnf(0)
            }
        }
        impl core::fmt::Debug for Crccnf {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Crccnf")
                    .field("len", &self.len())
                    .field("skipaddr", &self.skipaddr())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Crccnf {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Crccnf {
                    len: super::vals::Len,
                    skipaddr: super::vals::Skipaddr,
                }
                let proxy = Crccnf {
                    len: self.len(),
                    skipaddr: self.skipaddr(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "CRC initial value"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Crcinit(pub u32);
        impl Crcinit {
            #[doc = "CRC initial value"]
            #[inline(always)]
            pub const fn crcinit(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x00ff_ffff;
                val as u32
            }
            #[doc = "CRC initial value"]
            #[inline(always)]
            pub fn set_crcinit(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
            }
        }
        impl Default for Crcinit {
            #[inline(always)]
            fn default() -> Crcinit {
                Crcinit(0)
            }
        }
        impl core::fmt::Debug for Crcinit {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Crcinit")
                    .field("crcinit", &self.crcinit())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Crcinit {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Crcinit {
                    crcinit: u32,
                }
                let proxy = Crcinit {
                    crcinit: self.crcinit(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "CRC polynomial"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Crcpoly(pub u32);
        impl Crcpoly {
            #[doc = "CRC polynomial"]
            #[inline(always)]
            pub const fn crcpoly(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x00ff_ffff;
                val as u32
            }
            #[doc = "CRC polynomial"]
            #[inline(always)]
            pub fn set_crcpoly(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
            }
        }
        impl Default for Crcpoly {
            #[inline(always)]
            fn default() -> Crcpoly {
                Crcpoly(0)
            }
        }
        impl core::fmt::Debug for Crcpoly {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Crcpoly")
                    .field("crcpoly", &self.crcpoly())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Crcpoly {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Crcpoly {
                    crcpoly: u32,
                }
                let proxy = Crcpoly {
                    crcpoly: self.crcpoly(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "CRC status"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Crcstatus(pub u32);
        impl Crcstatus {
            #[doc = "CRC status of packet received"]
            #[inline(always)]
            pub const fn crcstatus(&self) -> super::vals::Crcstatus {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Crcstatus::from_bits(val as u8)
            }
            #[doc = "CRC status of packet received"]
            #[inline(always)]
            pub fn set_crcstatus(&mut self, val: super::vals::Crcstatus) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Crcstatus {
            #[inline(always)]
            fn default() -> Crcstatus {
                Crcstatus(0)
            }
        }
        impl core::fmt::Debug for Crcstatus {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Crcstatus")
                    .field("crcstatus", &self.crcstatus())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Crcstatus {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Crcstatus {
                    crcstatus: super::vals::Crcstatus,
                }
                let proxy = Crcstatus {
                    crcstatus: self.crcstatus(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Device address match configuration"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dacnf(pub u32);
        impl Dacnf {
            #[doc = "Enable or disable device address matching using device address 0"]
            #[inline(always)]
            pub const fn ena0(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable device address matching using device address 0"]
            #[inline(always)]
            pub fn set_ena0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable device address matching using device address 1"]
            #[inline(always)]
            pub const fn ena1(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable device address matching using device address 1"]
            #[inline(always)]
            pub fn set_ena1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable device address matching using device address 2"]
            #[inline(always)]
            pub const fn ena2(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable device address matching using device address 2"]
            #[inline(always)]
            pub fn set_ena2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable or disable device address matching using device address 3"]
            #[inline(always)]
            pub const fn ena3(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable device address matching using device address 3"]
            #[inline(always)]
            pub fn set_ena3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Enable or disable device address matching using device address 4"]
            #[inline(always)]
            pub const fn ena4(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable device address matching using device address 4"]
            #[inline(always)]
            pub fn set_ena4(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Enable or disable device address matching using device address 5"]
            #[inline(always)]
            pub const fn ena5(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable device address matching using device address 5"]
            #[inline(always)]
            pub fn set_ena5(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Enable or disable device address matching using device address 6"]
            #[inline(always)]
            pub const fn ena6(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable device address matching using device address 6"]
            #[inline(always)]
            pub fn set_ena6(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Enable or disable device address matching using device address 7"]
            #[inline(always)]
            pub const fn ena7(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable device address matching using device address 7"]
            #[inline(always)]
            pub fn set_ena7(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "TxAdd for device address 0"]
            #[inline(always)]
            pub const fn txadd0(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "TxAdd for device address 0"]
            #[inline(always)]
            pub fn set_txadd0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "TxAdd for device address 1"]
            #[inline(always)]
            pub const fn txadd1(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "TxAdd for device address 1"]
            #[inline(always)]
            pub fn set_txadd1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "TxAdd for device address 2"]
            #[inline(always)]
            pub const fn txadd2(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "TxAdd for device address 2"]
            #[inline(always)]
            pub fn set_txadd2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "TxAdd for device address 3"]
            #[inline(always)]
            pub const fn txadd3(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "TxAdd for device address 3"]
            #[inline(always)]
            pub fn set_txadd3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "TxAdd for device address 4"]
            #[inline(always)]
            pub const fn txadd4(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "TxAdd for device address 4"]
            #[inline(always)]
            pub fn set_txadd4(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "TxAdd for device address 5"]
            #[inline(always)]
            pub const fn txadd5(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "TxAdd for device address 5"]
            #[inline(always)]
            pub fn set_txadd5(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "TxAdd for device address 6"]
            #[inline(always)]
            pub const fn txadd6(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "TxAdd for device address 6"]
            #[inline(always)]
            pub fn set_txadd6(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "TxAdd for device address 7"]
            #[inline(always)]
            pub const fn txadd7(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "TxAdd for device address 7"]
            #[inline(always)]
            pub fn set_txadd7(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
        }
        impl Default for Dacnf {
            #[inline(always)]
            fn default() -> Dacnf {
                Dacnf(0)
            }
        }
        impl core::fmt::Debug for Dacnf {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dacnf")
                    .field("ena0", &self.ena0())
                    .field("ena1", &self.ena1())
                    .field("ena2", &self.ena2())
                    .field("ena3", &self.ena3())
                    .field("ena4", &self.ena4())
                    .field("ena5", &self.ena5())
                    .field("ena6", &self.ena6())
                    .field("ena7", &self.ena7())
                    .field("txadd0", &self.txadd0())
                    .field("txadd1", &self.txadd1())
                    .field("txadd2", &self.txadd2())
                    .field("txadd3", &self.txadd3())
                    .field("txadd4", &self.txadd4())
                    .field("txadd5", &self.txadd5())
                    .field("txadd6", &self.txadd6())
                    .field("txadd7", &self.txadd7())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dacnf {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Dacnf {
                    ena0: bool,
                    ena1: bool,
                    ena2: bool,
                    ena3: bool,
                    ena4: bool,
                    ena5: bool,
                    ena6: bool,
                    ena7: bool,
                    txadd0: bool,
                    txadd1: bool,
                    txadd2: bool,
                    txadd3: bool,
                    txadd4: bool,
                    txadd5: bool,
                    txadd6: bool,
                    txadd7: bool,
                }
                let proxy = Dacnf {
                    ena0: self.ena0(),
                    ena1: self.ena1(),
                    ena2: self.ena2(),
                    ena3: self.ena3(),
                    ena4: self.ena4(),
                    ena5: self.ena5(),
                    ena6: self.ena6(),
                    ena7: self.ena7(),
                    txadd0: self.txadd0(),
                    txadd1: self.txadd1(),
                    txadd2: self.txadd2(),
                    txadd3: self.txadd3(),
                    txadd4: self.txadd4(),
                    txadd5: self.txadd5(),
                    txadd6: self.txadd6(),
                    txadd7: self.txadd7(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Device address match index"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dai(pub u32);
        impl Dai {
            #[doc = "Device address match index"]
            #[inline(always)]
            pub const fn dai(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x07;
                val as u8
            }
            #[doc = "Device address match index"]
            #[inline(always)]
            pub fn set_dai(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
            }
        }
        impl Default for Dai {
            #[inline(always)]
            fn default() -> Dai {
                Dai(0)
            }
        }
        impl core::fmt::Debug for Dai {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dai").field("dai", &self.dai()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dai {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Dai {
                    dai: u8,
                }
                let proxy = Dai { dai: self.dai() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Description collection: Device address prefix n"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dap(pub u32);
        impl Dap {
            #[doc = "Device address prefix n"]
            #[inline(always)]
            pub const fn dap(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Device address prefix n"]
            #[inline(always)]
            pub fn set_dap(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Dap {
            #[inline(always)]
            fn default() -> Dap {
                Dap(0)
            }
        }
        impl core::fmt::Debug for Dap {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dap").field("dap", &self.dap()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dap {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Dap {
                    dap: u16,
                }
                let proxy = Dap { dap: self.dap() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Data whitening initial value"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Datawhiteiv(pub u32);
        impl Datawhiteiv {
            #[doc = "Data whitening initial value. Bit 6 is hardwired to '1', writing '0' to it has no effect, and it will always be read back and used by the device as '1'."]
            #[inline(always)]
            pub const fn datawhiteiv(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x7f;
                val as u8
            }
            #[doc = "Data whitening initial value. Bit 6 is hardwired to '1', writing '0' to it has no effect, and it will always be read back and used by the device as '1'."]
            #[inline(always)]
            pub fn set_datawhiteiv(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
            }
        }
        impl Default for Datawhiteiv {
            #[inline(always)]
            fn default() -> Datawhiteiv {
                Datawhiteiv(0)
            }
        }
        impl core::fmt::Debug for Datawhiteiv {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Datawhiteiv")
                    .field("datawhiteiv", &self.datawhiteiv())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Datawhiteiv {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Datawhiteiv {
                    datawhiteiv: u8,
                }
                let proxy = Datawhiteiv {
                    datawhiteiv: self.datawhiteiv(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Frequency"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Frequency(pub u32);
        impl Frequency {
            #[doc = "Radio channel frequency"]
            #[inline(always)]
            pub const fn frequency(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x7f;
                val as u8
            }
            #[doc = "Radio channel frequency"]
            #[inline(always)]
            pub fn set_frequency(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
            }
            #[doc = "Channel map selection"]
            #[inline(always)]
            pub const fn map(&self) -> super::vals::Map {
                let val = (self.0 >> 8usize) & 0x01;
                super::vals::Map::from_bits(val as u8)
            }
            #[doc = "Channel map selection"]
            #[inline(always)]
            pub fn set_map(&mut self, val: super::vals::Map) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
            }
        }
        impl Default for Frequency {
            #[inline(always)]
            fn default() -> Frequency {
                Frequency(0)
            }
        }
        impl core::fmt::Debug for Frequency {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Frequency")
                    .field("frequency", &self.frequency())
                    .field("map", &self.map())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Frequency {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Frequency {
                    frequency: u8,
                    map: super::vals::Map,
                }
                let proxy = Frequency {
                    frequency: self.frequency(),
                    map: self.map(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Write '1' to disable interrupt for event READY"]
            #[inline(always)]
            pub const fn ready(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event READY"]
            #[inline(always)]
            pub fn set_ready(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to disable interrupt for event ADDRESS"]
            #[inline(always)]
            pub const fn address(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event ADDRESS"]
            #[inline(always)]
            pub fn set_address(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to disable interrupt for event PAYLOAD"]
            #[inline(always)]
            pub const fn payload(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event PAYLOAD"]
            #[inline(always)]
            pub fn set_payload(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Write '1' to disable interrupt for event END"]
            #[inline(always)]
            pub const fn end(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event END"]
            #[inline(always)]
            pub fn set_end(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Write '1' to disable interrupt for event DISABLED"]
            #[inline(always)]
            pub const fn disabled(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event DISABLED"]
            #[inline(always)]
            pub fn set_disabled(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Write '1' to disable interrupt for event DEVMATCH"]
            #[inline(always)]
            pub const fn devmatch(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event DEVMATCH"]
            #[inline(always)]
            pub fn set_devmatch(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Write '1' to disable interrupt for event DEVMISS"]
            #[inline(always)]
            pub const fn devmiss(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event DEVMISS"]
            #[inline(always)]
            pub fn set_devmiss(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Write '1' to disable interrupt for event RSSIEND"]
            #[inline(always)]
            pub const fn rssiend(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event RSSIEND"]
            #[inline(always)]
            pub fn set_rssiend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Write '1' to disable interrupt for event BCMATCH"]
            #[inline(always)]
            pub const fn bcmatch(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event BCMATCH"]
            #[inline(always)]
            pub fn set_bcmatch(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "Write '1' to disable interrupt for event CRCOK"]
            #[inline(always)]
            pub const fn crcok(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event CRCOK"]
            #[inline(always)]
            pub fn set_crcok(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "Write '1' to disable interrupt for event CRCERROR"]
            #[inline(always)]
            pub const fn crcerror(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event CRCERROR"]
            #[inline(always)]
            pub fn set_crcerror(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "Write '1' to disable interrupt for event TXREADY"]
            #[inline(always)]
            pub const fn txready(&self) -> bool {
                let val = (self.0 >> 21usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event TXREADY"]
            #[inline(always)]
            pub fn set_txready(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
            }
            #[doc = "Write '1' to disable interrupt for event RXREADY"]
            #[inline(always)]
            pub const fn rxready(&self) -> bool {
                let val = (self.0 >> 22usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event RXREADY"]
            #[inline(always)]
            pub fn set_rxready(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
            }
            #[doc = "Write '1' to disable interrupt for event PHYEND"]
            #[inline(always)]
            pub const fn phyend(&self) -> bool {
                let val = (self.0 >> 27usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event PHYEND"]
            #[inline(always)]
            pub fn set_phyend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("ready", &self.ready())
                    .field("address", &self.address())
                    .field("payload", &self.payload())
                    .field("end", &self.end())
                    .field("disabled", &self.disabled())
                    .field("devmatch", &self.devmatch())
                    .field("devmiss", &self.devmiss())
                    .field("rssiend", &self.rssiend())
                    .field("bcmatch", &self.bcmatch())
                    .field("crcok", &self.crcok())
                    .field("crcerror", &self.crcerror())
                    .field("txready", &self.txready())
                    .field("rxready", &self.rxready())
                    .field("phyend", &self.phyend())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    ready: bool,
                    address: bool,
                    payload: bool,
                    end: bool,
                    disabled: bool,
                    devmatch: bool,
                    devmiss: bool,
                    rssiend: bool,
                    bcmatch: bool,
                    crcok: bool,
                    crcerror: bool,
                    txready: bool,
                    rxready: bool,
                    phyend: bool,
                }
                let proxy = Int {
                    ready: self.ready(),
                    address: self.address(),
                    payload: self.payload(),
                    end: self.end(),
                    disabled: self.disabled(),
                    devmatch: self.devmatch(),
                    devmiss: self.devmiss(),
                    rssiend: self.rssiend(),
                    bcmatch: self.bcmatch(),
                    crcok: self.crcok(),
                    crcerror: self.crcerror(),
                    txready: self.txready(),
                    rxready: self.rxready(),
                    phyend: self.phyend(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Data rate and modulation"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Mode(pub u32);
        impl Mode {
            #[doc = "Radio data rate and modulation setting. The radio supports frequency-shift keying (FSK) modulation."]
            #[inline(always)]
            pub const fn mode(&self) -> super::vals::Mode {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Mode::from_bits(val as u8)
            }
            #[doc = "Radio data rate and modulation setting. The radio supports frequency-shift keying (FSK) modulation."]
            #[inline(always)]
            pub fn set_mode(&mut self, val: super::vals::Mode) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Mode {
            #[inline(always)]
            fn default() -> Mode {
                Mode(0)
            }
        }
        impl core::fmt::Debug for Mode {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Mode").field("mode", &self.mode()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Mode {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Mode {
                    mode: super::vals::Mode,
                }
                let proxy = Mode { mode: self.mode() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Radio mode configuration register 0"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Modecnf0(pub u32);
        impl Modecnf0 {
            #[doc = "Radio ramp-up time"]
            #[inline(always)]
            pub const fn ru(&self) -> super::vals::Ru {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Ru::from_bits(val as u8)
            }
            #[doc = "Radio ramp-up time"]
            #[inline(always)]
            pub fn set_ru(&mut self, val: super::vals::Ru) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Default TX value"]
            #[inline(always)]
            pub const fn dtx(&self) -> super::vals::Dtx {
                let val = (self.0 >> 8usize) & 0x03;
                super::vals::Dtx::from_bits(val as u8)
            }
            #[doc = "Default TX value"]
            #[inline(always)]
            pub fn set_dtx(&mut self, val: super::vals::Dtx) {
                self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
            }
        }
        impl Default for Modecnf0 {
            #[inline(always)]
            fn default() -> Modecnf0 {
                Modecnf0(0)
            }
        }
        impl core::fmt::Debug for Modecnf0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Modecnf0")
                    .field("ru", &self.ru())
                    .field("dtx", &self.dtx())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Modecnf0 {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Modecnf0 {
                    ru: super::vals::Ru,
                    dtx: super::vals::Dtx,
                }
                let proxy = Modecnf0 {
                    ru: self.ru(),
                    dtx: self.dtx(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Packet configuration register 0"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pcnf0(pub u32);
        impl Pcnf0 {
            #[doc = "Length on air of LENGTH field in number of bits"]
            #[inline(always)]
            pub const fn lflen(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "Length on air of LENGTH field in number of bits"]
            #[inline(always)]
            pub fn set_lflen(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
            #[doc = "Length on air of S0 field in number of bytes"]
            #[inline(always)]
            pub const fn s0len(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Length on air of S0 field in number of bytes"]
            #[inline(always)]
            pub fn set_s0len(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Length on air of S1 field in number of bits"]
            #[inline(always)]
            pub const fn s1len(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0x0f;
                val as u8
            }
            #[doc = "Length on air of S1 field in number of bits"]
            #[inline(always)]
            pub fn set_s1len(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
            }
            #[doc = "Include or exclude S1 field in RAM"]
            #[inline(always)]
            pub const fn s1incl(&self) -> super::vals::S1incl {
                let val = (self.0 >> 20usize) & 0x01;
                super::vals::S1incl::from_bits(val as u8)
            }
            #[doc = "Include or exclude S1 field in RAM"]
            #[inline(always)]
            pub fn set_s1incl(&mut self, val: super::vals::S1incl) {
                self.0 =
                    (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
            }
            #[doc = "Length of preamble on air. Decision point: TASKS_START task"]
            #[inline(always)]
            pub const fn plen(&self) -> super::vals::Plen {
                let val = (self.0 >> 24usize) & 0x03;
                super::vals::Plen::from_bits(val as u8)
            }
            #[doc = "Length of preamble on air. Decision point: TASKS_START task"]
            #[inline(always)]
            pub fn set_plen(&mut self, val: super::vals::Plen) {
                self.0 =
                    (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
            }
            #[doc = "Indicates if LENGTH field contains CRC or not"]
            #[inline(always)]
            pub const fn crcinc(&self) -> super::vals::Crcinc {
                let val = (self.0 >> 26usize) & 0x01;
                super::vals::Crcinc::from_bits(val as u8)
            }
            #[doc = "Indicates if LENGTH field contains CRC or not"]
            #[inline(always)]
            pub fn set_crcinc(&mut self, val: super::vals::Crcinc) {
                self.0 =
                    (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
            }
        }
        impl Default for Pcnf0 {
            #[inline(always)]
            fn default() -> Pcnf0 {
                Pcnf0(0)
            }
        }
        impl core::fmt::Debug for Pcnf0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Pcnf0")
                    .field("lflen", &self.lflen())
                    .field("s0len", &self.s0len())
                    .field("s1len", &self.s1len())
                    .field("s1incl", &self.s1incl())
                    .field("plen", &self.plen())
                    .field("crcinc", &self.crcinc())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Pcnf0 {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Pcnf0 {
                    lflen: u8,
                    s0len: bool,
                    s1len: u8,
                    s1incl: super::vals::S1incl,
                    plen: super::vals::Plen,
                    crcinc: super::vals::Crcinc,
                }
                let proxy = Pcnf0 {
                    lflen: self.lflen(),
                    s0len: self.s0len(),
                    s1len: self.s1len(),
                    s1incl: self.s1incl(),
                    plen: self.plen(),
                    crcinc: self.crcinc(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Packet configuration register 1"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pcnf1(pub u32);
        impl Pcnf1 {
            #[doc = "Maximum length of packet payload. If the packet payload is larger than MAXLEN, the radio will truncate the payload to MAXLEN."]
            #[inline(always)]
            pub const fn maxlen(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Maximum length of packet payload. If the packet payload is larger than MAXLEN, the radio will truncate the payload to MAXLEN."]
            #[inline(always)]
            pub fn set_maxlen(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "Static length in number of bytes"]
            #[inline(always)]
            pub const fn statlen(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "Static length in number of bytes"]
            #[inline(always)]
            pub fn set_statlen(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
            #[doc = "Base address length in number of bytes"]
            #[inline(always)]
            pub const fn balen(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0x07;
                val as u8
            }
            #[doc = "Base address length in number of bytes"]
            #[inline(always)]
            pub fn set_balen(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
            }
            #[doc = "On-air endianness of packet, this applies to the S0, LENGTH, S1, and the PAYLOAD fields."]
            #[inline(always)]
            pub const fn endian(&self) -> super::vals::Endian {
                let val = (self.0 >> 24usize) & 0x01;
                super::vals::Endian::from_bits(val as u8)
            }
            #[doc = "On-air endianness of packet, this applies to the S0, LENGTH, S1, and the PAYLOAD fields."]
            #[inline(always)]
            pub fn set_endian(&mut self, val: super::vals::Endian) {
                self.0 =
                    (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
            }
            #[doc = "Enable or disable packet whitening"]
            #[inline(always)]
            pub const fn whiteen(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable packet whitening"]
            #[inline(always)]
            pub fn set_whiteen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
            }
        }
        impl Default for Pcnf1 {
            #[inline(always)]
            fn default() -> Pcnf1 {
                Pcnf1(0)
            }
        }
        impl core::fmt::Debug for Pcnf1 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Pcnf1")
                    .field("maxlen", &self.maxlen())
                    .field("statlen", &self.statlen())
                    .field("balen", &self.balen())
                    .field("endian", &self.endian())
                    .field("whiteen", &self.whiteen())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Pcnf1 {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Pcnf1 {
                    maxlen: u8,
                    statlen: u8,
                    balen: u8,
                    endian: super::vals::Endian,
                    whiteen: bool,
                }
                let proxy = Pcnf1 {
                    maxlen: self.maxlen(),
                    statlen: self.statlen(),
                    balen: self.balen(),
                    endian: self.endian(),
                    whiteen: self.whiteen(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Payload status"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pdustat(pub u32);
        impl Pdustat {
            #[doc = "Status on payload length vs. PCNF1.MAXLEN"]
            #[inline(always)]
            pub const fn pdustat(&self) -> super::vals::Pdustat {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Pdustat::from_bits(val as u8)
            }
            #[doc = "Status on payload length vs. PCNF1.MAXLEN"]
            #[inline(always)]
            pub fn set_pdustat(&mut self, val: super::vals::Pdustat) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Pdustat {
            #[inline(always)]
            fn default() -> Pdustat {
                Pdustat(0)
            }
        }
        impl core::fmt::Debug for Pdustat {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Pdustat")
                    .field("pdustat", &self.pdustat())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Pdustat {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Pdustat {
                    pdustat: super::vals::Pdustat,
                }
                let proxy = Pdustat {
                    pdustat: self.pdustat(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Peripheral power control"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Power(pub u32);
        impl Power {
            #[doc = "Peripheral power control. The peripheral and its registers will be reset to its initial state by switching the peripheral off and then back on again."]
            #[inline(always)]
            pub const fn power(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Peripheral power control. The peripheral and its registers will be reset to its initial state by switching the peripheral off and then back on again."]
            #[inline(always)]
            pub fn set_power(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Power {
            #[inline(always)]
            fn default() -> Power {
                Power(0)
            }
        }
        impl core::fmt::Debug for Power {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Power")
                    .field("power", &self.power())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Power {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Power {
                    power: bool,
                }
                let proxy = Power {
                    power: self.power(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Prefixes bytes for logical addresses 0-3"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Prefix0(pub u32);
        impl Prefix0 {
            #[doc = "Address prefix 0."]
            #[inline(always)]
            pub const fn ap0(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Address prefix 0."]
            #[inline(always)]
            pub fn set_ap0(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "Address prefix 1."]
            #[inline(always)]
            pub const fn ap1(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "Address prefix 1."]
            #[inline(always)]
            pub fn set_ap1(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
            #[doc = "Address prefix 2."]
            #[inline(always)]
            pub const fn ap2(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0xff;
                val as u8
            }
            #[doc = "Address prefix 2."]
            #[inline(always)]
            pub fn set_ap2(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
            }
            #[doc = "Address prefix 3."]
            #[inline(always)]
            pub const fn ap3(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0xff;
                val as u8
            }
            #[doc = "Address prefix 3."]
            #[inline(always)]
            pub fn set_ap3(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
            }
        }
        impl Default for Prefix0 {
            #[inline(always)]
            fn default() -> Prefix0 {
                Prefix0(0)
            }
        }
        impl core::fmt::Debug for Prefix0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Prefix0")
                    .field("ap0", &self.ap0())
                    .field("ap1", &self.ap1())
                    .field("ap2", &self.ap2())
                    .field("ap3", &self.ap3())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Prefix0 {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Prefix0 {
                    ap0: u8,
                    ap1: u8,
                    ap2: u8,
                    ap3: u8,
                }
                let proxy = Prefix0 {
                    ap0: self.ap0(),
                    ap1: self.ap1(),
                    ap2: self.ap2(),
                    ap3: self.ap3(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Prefixes bytes for logical addresses 4-7"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Prefix1(pub u32);
        impl Prefix1 {
            #[doc = "Address prefix 4."]
            #[inline(always)]
            pub const fn ap4(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Address prefix 4."]
            #[inline(always)]
            pub fn set_ap4(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "Address prefix 5."]
            #[inline(always)]
            pub const fn ap5(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "Address prefix 5."]
            #[inline(always)]
            pub fn set_ap5(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
            #[doc = "Address prefix 6."]
            #[inline(always)]
            pub const fn ap6(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0xff;
                val as u8
            }
            #[doc = "Address prefix 6."]
            #[inline(always)]
            pub fn set_ap6(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
            }
            #[doc = "Address prefix 7."]
            #[inline(always)]
            pub const fn ap7(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0xff;
                val as u8
            }
            #[doc = "Address prefix 7."]
            #[inline(always)]
            pub fn set_ap7(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
            }
        }
        impl Default for Prefix1 {
            #[inline(always)]
            fn default() -> Prefix1 {
                Prefix1(0)
            }
        }
        impl core::fmt::Debug for Prefix1 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Prefix1")
                    .field("ap4", &self.ap4())
                    .field("ap5", &self.ap5())
                    .field("ap6", &self.ap6())
                    .field("ap7", &self.ap7())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Prefix1 {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Prefix1 {
                    ap4: u8,
                    ap5: u8,
                    ap6: u8,
                    ap7: u8,
                }
                let proxy = Prefix1 {
                    ap4: self.ap4(),
                    ap5: self.ap5(),
                    ap6: self.ap6(),
                    ap7: self.ap7(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "RSSI sample"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rssisample(pub u32);
        impl Rssisample {
            #[doc = "RSSI sample."]
            #[inline(always)]
            pub const fn rssisample(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x7f;
                val as u8
            }
            #[doc = "RSSI sample."]
            #[inline(always)]
            pub fn set_rssisample(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
            }
        }
        impl Default for Rssisample {
            #[inline(always)]
            fn default() -> Rssisample {
                Rssisample(0)
            }
        }
        impl core::fmt::Debug for Rssisample {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Rssisample")
                    .field("rssisample", &self.rssisample())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rssisample {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Rssisample {
                    rssisample: u8,
                }
                let proxy = Rssisample {
                    rssisample: self.rssisample(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Receive address select"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxaddresses(pub u32);
        impl Rxaddresses {
            #[doc = "Enable or disable reception on logical address 0."]
            #[inline(always)]
            pub const fn addr0(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable reception on logical address 0."]
            #[inline(always)]
            pub fn set_addr0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable reception on logical address 1."]
            #[inline(always)]
            pub const fn addr1(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable reception on logical address 1."]
            #[inline(always)]
            pub fn set_addr1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable reception on logical address 2."]
            #[inline(always)]
            pub const fn addr2(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable reception on logical address 2."]
            #[inline(always)]
            pub fn set_addr2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable or disable reception on logical address 3."]
            #[inline(always)]
            pub const fn addr3(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable reception on logical address 3."]
            #[inline(always)]
            pub fn set_addr3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Enable or disable reception on logical address 4."]
            #[inline(always)]
            pub const fn addr4(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable reception on logical address 4."]
            #[inline(always)]
            pub fn set_addr4(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Enable or disable reception on logical address 5."]
            #[inline(always)]
            pub const fn addr5(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable reception on logical address 5."]
            #[inline(always)]
            pub fn set_addr5(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Enable or disable reception on logical address 6."]
            #[inline(always)]
            pub const fn addr6(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable reception on logical address 6."]
            #[inline(always)]
            pub fn set_addr6(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Enable or disable reception on logical address 7."]
            #[inline(always)]
            pub const fn addr7(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable reception on logical address 7."]
            #[inline(always)]
            pub fn set_addr7(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
        }
        impl Default for Rxaddresses {
            #[inline(always)]
            fn default() -> Rxaddresses {
                Rxaddresses(0)
            }
        }
        impl core::fmt::Debug for Rxaddresses {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Rxaddresses")
                    .field("addr0", &self.addr0())
                    .field("addr1", &self.addr1())
                    .field("addr2", &self.addr2())
                    .field("addr3", &self.addr3())
                    .field("addr4", &self.addr4())
                    .field("addr5", &self.addr5())
                    .field("addr6", &self.addr6())
                    .field("addr7", &self.addr7())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rxaddresses {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Rxaddresses {
                    addr0: bool,
                    addr1: bool,
                    addr2: bool,
                    addr3: bool,
                    addr4: bool,
                    addr5: bool,
                    addr6: bool,
                    addr7: bool,
                }
                let proxy = Rxaddresses {
                    addr0: self.addr0(),
                    addr1: self.addr1(),
                    addr2: self.addr2(),
                    addr3: self.addr3(),
                    addr4: self.addr4(),
                    addr5: self.addr5(),
                    addr6: self.addr6(),
                    addr7: self.addr7(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "CRC field of previously received packet"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxcrc(pub u32);
        impl Rxcrc {
            #[doc = "CRC field of previously received packet"]
            #[inline(always)]
            pub const fn rxcrc(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x00ff_ffff;
                val as u32
            }
            #[doc = "CRC field of previously received packet"]
            #[inline(always)]
            pub fn set_rxcrc(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
            }
        }
        impl Default for Rxcrc {
            #[inline(always)]
            fn default() -> Rxcrc {
                Rxcrc(0)
            }
        }
        impl core::fmt::Debug for Rxcrc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Rxcrc")
                    .field("rxcrc", &self.rxcrc())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rxcrc {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Rxcrc {
                    rxcrc: u32,
                }
                let proxy = Rxcrc {
                    rxcrc: self.rxcrc(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Received address"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxmatch(pub u32);
        impl Rxmatch {
            #[doc = "Received address"]
            #[inline(always)]
            pub const fn rxmatch(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x07;
                val as u8
            }
            #[doc = "Received address"]
            #[inline(always)]
            pub fn set_rxmatch(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
            }
        }
        impl Default for Rxmatch {
            #[inline(always)]
            fn default() -> Rxmatch {
                Rxmatch(0)
            }
        }
        impl core::fmt::Debug for Rxmatch {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Rxmatch")
                    .field("rxmatch", &self.rxmatch())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rxmatch {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Rxmatch {
                    rxmatch: u8,
                }
                let proxy = Rxmatch {
                    rxmatch: self.rxmatch(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event READY and task START"]
            #[inline(always)]
            pub const fn ready_start(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event READY and task START"]
            #[inline(always)]
            pub fn set_ready_start(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Shortcut between event END and task DISABLE"]
            #[inline(always)]
            pub const fn end_disable(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event END and task DISABLE"]
            #[inline(always)]
            pub fn set_end_disable(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Shortcut between event DISABLED and task TXEN"]
            #[inline(always)]
            pub const fn disabled_txen(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event DISABLED and task TXEN"]
            #[inline(always)]
            pub fn set_disabled_txen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Shortcut between event DISABLED and task RXEN"]
            #[inline(always)]
            pub const fn disabled_rxen(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event DISABLED and task RXEN"]
            #[inline(always)]
            pub fn set_disabled_rxen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Shortcut between event ADDRESS and task RSSISTART"]
            #[inline(always)]
            pub const fn address_rssistart(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event ADDRESS and task RSSISTART"]
            #[inline(always)]
            pub fn set_address_rssistart(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Shortcut between event END and task START"]
            #[inline(always)]
            pub const fn end_start(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event END and task START"]
            #[inline(always)]
            pub fn set_end_start(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Shortcut between event ADDRESS and task BCSTART"]
            #[inline(always)]
            pub const fn address_bcstart(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event ADDRESS and task BCSTART"]
            #[inline(always)]
            pub fn set_address_bcstart(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Shortcut between event DISABLED and task RSSISTOP"]
            #[inline(always)]
            pub const fn disabled_rssistop(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event DISABLED and task RSSISTOP"]
            #[inline(always)]
            pub fn set_disabled_rssistop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Shortcut between event TXREADY and task START"]
            #[inline(always)]
            pub const fn txready_start(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event TXREADY and task START"]
            #[inline(always)]
            pub fn set_txready_start(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "Shortcut between event RXREADY and task START"]
            #[inline(always)]
            pub const fn rxready_start(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event RXREADY and task START"]
            #[inline(always)]
            pub fn set_rxready_start(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "Shortcut between event PHYEND and task DISABLE"]
            #[inline(always)]
            pub const fn phyend_disable(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event PHYEND and task DISABLE"]
            #[inline(always)]
            pub fn set_phyend_disable(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "Shortcut between event PHYEND and task START"]
            #[inline(always)]
            pub const fn phyend_start(&self) -> bool {
                let val = (self.0 >> 21usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event PHYEND and task START"]
            #[inline(always)]
            pub fn set_phyend_start(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
        impl core::fmt::Debug for Shorts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Shorts")
                    .field("ready_start", &self.ready_start())
                    .field("end_disable", &self.end_disable())
                    .field("disabled_txen", &self.disabled_txen())
                    .field("disabled_rxen", &self.disabled_rxen())
                    .field("address_rssistart", &self.address_rssistart())
                    .field("end_start", &self.end_start())
                    .field("address_bcstart", &self.address_bcstart())
                    .field("disabled_rssistop", &self.disabled_rssistop())
                    .field("txready_start", &self.txready_start())
                    .field("rxready_start", &self.rxready_start())
                    .field("phyend_disable", &self.phyend_disable())
                    .field("phyend_start", &self.phyend_start())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Shorts {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Shorts {
                    ready_start: bool,
                    end_disable: bool,
                    disabled_txen: bool,
                    disabled_rxen: bool,
                    address_rssistart: bool,
                    end_start: bool,
                    address_bcstart: bool,
                    disabled_rssistop: bool,
                    txready_start: bool,
                    rxready_start: bool,
                    phyend_disable: bool,
                    phyend_start: bool,
                }
                let proxy = Shorts {
                    ready_start: self.ready_start(),
                    end_disable: self.end_disable(),
                    disabled_txen: self.disabled_txen(),
                    disabled_rxen: self.disabled_rxen(),
                    address_rssistart: self.address_rssistart(),
                    end_start: self.end_start(),
                    address_bcstart: self.address_bcstart(),
                    disabled_rssistop: self.disabled_rssistop(),
                    txready_start: self.txready_start(),
                    rxready_start: self.rxready_start(),
                    phyend_disable: self.phyend_disable(),
                    phyend_start: self.phyend_start(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Current radio state"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct State(pub u32);
        impl State {
            #[doc = "Current radio state"]
            #[inline(always)]
            pub const fn state(&self) -> super::vals::State {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::State::from_bits(val as u8)
            }
            #[doc = "Current radio state"]
            #[inline(always)]
            pub fn set_state(&mut self, val: super::vals::State) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for State {
            #[inline(always)]
            fn default() -> State {
                State(0)
            }
        }
        impl core::fmt::Debug for State {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("State")
                    .field("state", &self.state())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for State {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct State {
                    state: super::vals::State,
                }
                let proxy = State {
                    state: self.state(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Interframe spacing in us"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Tifs(pub u32);
        impl Tifs {
            #[doc = "Interframe spacing in us."]
            #[inline(always)]
            pub const fn tifs(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x03ff;
                val as u16
            }
            #[doc = "Interframe spacing in us."]
            #[inline(always)]
            pub fn set_tifs(&mut self, val: u16) {
                self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
            }
        }
        impl Default for Tifs {
            #[inline(always)]
            fn default() -> Tifs {
                Tifs(0)
            }
        }
        impl core::fmt::Debug for Tifs {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Tifs").field("tifs", &self.tifs()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Tifs {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Tifs {
                    tifs: u16,
                }
                let proxy = Tifs { tifs: self.tifs() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Transmit address select"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Txaddress(pub u32);
        impl Txaddress {
            #[doc = "Transmit address select"]
            #[inline(always)]
            pub const fn txaddress(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x07;
                val as u8
            }
            #[doc = "Transmit address select"]
            #[inline(always)]
            pub fn set_txaddress(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
            }
        }
        impl Default for Txaddress {
            #[inline(always)]
            fn default() -> Txaddress {
                Txaddress(0)
            }
        }
        impl core::fmt::Debug for Txaddress {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Txaddress")
                    .field("txaddress", &self.txaddress())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Txaddress {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Txaddress {
                    txaddress: u8,
                }
                let proxy = Txaddress {
                    txaddress: self.txaddress(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Output power"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Txpower(pub u32);
        impl Txpower {
            #[doc = "RADIO output power"]
            #[inline(always)]
            pub const fn txpower(&self) -> super::vals::Txpower {
                let val = (self.0 >> 0usize) & 0xff;
                super::vals::Txpower::from_bits(val as u8)
            }
            #[doc = "RADIO output power"]
            #[inline(always)]
            pub fn set_txpower(&mut self, val: super::vals::Txpower) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Txpower {
            #[inline(always)]
            fn default() -> Txpower {
                Txpower(0)
            }
        }
        impl core::fmt::Debug for Txpower {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Txpower")
                    .field("txpower", &self.txpower())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Txpower {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Txpower {
                    txpower: super::vals::Txpower,
                }
                let proxy = Txpower {
                    txpower: self.txpower(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Crcinc {
            #[doc = "LENGTH does not contain CRC"]
            EXCLUDE = 0x0,
            #[doc = "LENGTH includes CRC"]
            INCLUDE = 0x01,
        }
        impl Crcinc {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Crcinc {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Crcinc {
            #[inline(always)]
            fn from(val: u8) -> Crcinc {
                Crcinc::from_bits(val)
            }
        }
        impl From<Crcinc> for u8 {
            #[inline(always)]
            fn from(val: Crcinc) -> u8 {
                Crcinc::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Crcstatus {
            #[doc = "Packet received with CRC error"]
            CRCERROR = 0x0,
            #[doc = "Packet received with CRC ok"]
            CRCOK = 0x01,
        }
        impl Crcstatus {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Crcstatus {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Crcstatus {
            #[inline(always)]
            fn from(val: u8) -> Crcstatus {
                Crcstatus::from_bits(val)
            }
        }
        impl From<Crcstatus> for u8 {
            #[inline(always)]
            fn from(val: Crcstatus) -> u8 {
                Crcstatus::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Dtx {
            #[doc = "Transmit '1'"]
            B1 = 0x0,
            #[doc = "Transmit '0'"]
            B0 = 0x01,
            #[doc = "Transmit center frequency"]
            CENTER = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl Dtx {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Dtx {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Dtx {
            #[inline(always)]
            fn from(val: u8) -> Dtx {
                Dtx::from_bits(val)
            }
        }
        impl From<Dtx> for u8 {
            #[inline(always)]
            fn from(val: Dtx) -> u8 {
                Dtx::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Endian {
            #[doc = "Least significant bit on air first"]
            LITTLE = 0x0,
            #[doc = "Most significant bit on air first"]
            BIG = 0x01,
        }
        impl Endian {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Endian {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Endian {
            #[inline(always)]
            fn from(val: u8) -> Endian {
                Endian::from_bits(val)
            }
        }
        impl From<Endian> for u8 {
            #[inline(always)]
            fn from(val: Endian) -> u8 {
                Endian::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Len {
            #[doc = "CRC length is zero and CRC calculation is disabled"]
            DISABLED = 0x0,
            #[doc = "CRC length is one byte and CRC calculation is enabled"]
            ONE = 0x01,
            #[doc = "CRC length is two bytes and CRC calculation is enabled"]
            TWO = 0x02,
            #[doc = "CRC length is three bytes and CRC calculation is enabled"]
            THREE = 0x03,
        }
        impl Len {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Len {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Len {
            #[inline(always)]
            fn from(val: u8) -> Len {
                Len::from_bits(val)
            }
        }
        impl From<Len> for u8 {
            #[inline(always)]
            fn from(val: Len) -> u8 {
                Len::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Map {
            #[doc = "Channel map between 2400 MHZ .. 2500 MHz"]
            DEFAULT = 0x0,
            #[doc = "Channel map between 2360 MHZ .. 2460 MHz"]
            LOW = 0x01,
        }
        impl Map {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Map {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Map {
            #[inline(always)]
            fn from(val: u8) -> Map {
                Map::from_bits(val)
            }
        }
        impl From<Map> for u8 {
            #[inline(always)]
            fn from(val: Map) -> u8 {
                Map::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Mode {
            #[doc = "1 Mbps Nordic proprietary radio mode"]
            NRF_1MBIT = 0x0,
            #[doc = "2 Mbps Nordic proprietary radio mode"]
            NRF_2MBIT = 0x01,
            _RESERVED_2 = 0x02,
            #[doc = "1 Mbps BLE"]
            BLE_1MBIT = 0x03,
            #[doc = "2 Mbps BLE"]
            BLE_2MBIT = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
            _RESERVED_8 = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl Mode {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Mode {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Mode {
            #[inline(always)]
            fn from(val: u8) -> Mode {
                Mode::from_bits(val)
            }
        }
        impl From<Mode> for u8 {
            #[inline(always)]
            fn from(val: Mode) -> u8 {
                Mode::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Pdustat {
            #[doc = "Payload less than PCNF1.MAXLEN"]
            LESS_THAN = 0x0,
            #[doc = "Payload greater than PCNF1.MAXLEN"]
            GREATER_THAN = 0x01,
        }
        impl Pdustat {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Pdustat {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Pdustat {
            #[inline(always)]
            fn from(val: u8) -> Pdustat {
                Pdustat::from_bits(val)
            }
        }
        impl From<Pdustat> for u8 {
            #[inline(always)]
            fn from(val: Pdustat) -> u8 {
                Pdustat::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Plen {
            #[doc = "8-bit preamble"]
            _8BIT = 0x0,
            #[doc = "16-bit preamble"]
            _16BIT = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl Plen {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Plen {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Plen {
            #[inline(always)]
            fn from(val: u8) -> Plen {
                Plen::from_bits(val)
            }
        }
        impl From<Plen> for u8 {
            #[inline(always)]
            fn from(val: Plen) -> u8 {
                Plen::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Ru {
            #[doc = "Default ramp-up time (tRXEN and tTXEN), compatible with firmware written for nRF51"]
            DEFAULT = 0x0,
            #[doc = "Fast ramp-up (tRXEN,FAST and tTXEN,FAST), see electrical specifications for more information"]
            FAST = 0x01,
        }
        impl Ru {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Ru {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Ru {
            #[inline(always)]
            fn from(val: u8) -> Ru {
                Ru::from_bits(val)
            }
        }
        impl From<Ru> for u8 {
            #[inline(always)]
            fn from(val: Ru) -> u8 {
                Ru::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum S1incl {
            #[doc = "Include S1 field in RAM only if S1LEN &gt; 0"]
            AUTOMATIC = 0x0,
            #[doc = "Always include S1 field in RAM independent of S1LEN"]
            INCLUDE = 0x01,
        }
        impl S1incl {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> S1incl {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for S1incl {
            #[inline(always)]
            fn from(val: u8) -> S1incl {
                S1incl::from_bits(val)
            }
        }
        impl From<S1incl> for u8 {
            #[inline(always)]
            fn from(val: S1incl) -> u8 {
                S1incl::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Skipaddr {
            #[doc = "CRC calculation includes address field"]
            INCLUDE = 0x0,
            #[doc = "CRC calculation does not include address field. The CRC calculation will start at the first byte after the address."]
            SKIP = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl Skipaddr {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Skipaddr {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Skipaddr {
            #[inline(always)]
            fn from(val: u8) -> Skipaddr {
                Skipaddr::from_bits(val)
            }
        }
        impl From<Skipaddr> for u8 {
            #[inline(always)]
            fn from(val: Skipaddr) -> u8 {
                Skipaddr::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum State {
            #[doc = "RADIO is in the Disabled state"]
            DISABLED = 0x0,
            #[doc = "RADIO is in the RXRU state"]
            RX_RU = 0x01,
            #[doc = "RADIO is in the RXIDLE state"]
            RX_IDLE = 0x02,
            #[doc = "RADIO is in the RX state"]
            RX = 0x03,
            #[doc = "RADIO is in the RXDISABLED state"]
            RX_DISABLE = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
            _RESERVED_8 = 0x08,
            #[doc = "RADIO is in the TXRU state"]
            TX_RU = 0x09,
            #[doc = "RADIO is in the TXIDLE state"]
            TX_IDLE = 0x0a,
            #[doc = "RADIO is in the TX state"]
            TX = 0x0b,
            #[doc = "RADIO is in the TXDISABLED state"]
            TX_DISABLE = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl State {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> State {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for State {
            #[inline(always)]
            fn from(val: u8) -> State {
                State::from_bits(val)
            }
        }
        impl From<State> for u8 {
            #[inline(always)]
            fn from(val: State) -> u8 {
                State::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Txpower {
            #[doc = "0 dBm"]
            _0_DBM = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            #[doc = "+3 dBm"]
            POS3_DBM = 0x03,
            #[doc = "+4 dBm"]
            POS4_DBM = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
            _RESERVED_8 = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
            _RESERVED_10 = 0x10,
            _RESERVED_11 = 0x11,
            _RESERVED_12 = 0x12,
            _RESERVED_13 = 0x13,
            _RESERVED_14 = 0x14,
            _RESERVED_15 = 0x15,
            _RESERVED_16 = 0x16,
            _RESERVED_17 = 0x17,
            _RESERVED_18 = 0x18,
            _RESERVED_19 = 0x19,
            _RESERVED_1a = 0x1a,
            _RESERVED_1b = 0x1b,
            _RESERVED_1c = 0x1c,
            _RESERVED_1d = 0x1d,
            _RESERVED_1e = 0x1e,
            _RESERVED_1f = 0x1f,
            _RESERVED_20 = 0x20,
            _RESERVED_21 = 0x21,
            _RESERVED_22 = 0x22,
            _RESERVED_23 = 0x23,
            _RESERVED_24 = 0x24,
            _RESERVED_25 = 0x25,
            _RESERVED_26 = 0x26,
            _RESERVED_27 = 0x27,
            _RESERVED_28 = 0x28,
            _RESERVED_29 = 0x29,
            _RESERVED_2a = 0x2a,
            _RESERVED_2b = 0x2b,
            _RESERVED_2c = 0x2c,
            _RESERVED_2d = 0x2d,
            _RESERVED_2e = 0x2e,
            _RESERVED_2f = 0x2f,
            _RESERVED_30 = 0x30,
            _RESERVED_31 = 0x31,
            _RESERVED_32 = 0x32,
            _RESERVED_33 = 0x33,
            _RESERVED_34 = 0x34,
            _RESERVED_35 = 0x35,
            _RESERVED_36 = 0x36,
            _RESERVED_37 = 0x37,
            _RESERVED_38 = 0x38,
            _RESERVED_39 = 0x39,
            _RESERVED_3a = 0x3a,
            _RESERVED_3b = 0x3b,
            _RESERVED_3c = 0x3c,
            _RESERVED_3d = 0x3d,
            _RESERVED_3e = 0x3e,
            _RESERVED_3f = 0x3f,
            _RESERVED_40 = 0x40,
            _RESERVED_41 = 0x41,
            _RESERVED_42 = 0x42,
            _RESERVED_43 = 0x43,
            _RESERVED_44 = 0x44,
            _RESERVED_45 = 0x45,
            _RESERVED_46 = 0x46,
            _RESERVED_47 = 0x47,
            _RESERVED_48 = 0x48,
            _RESERVED_49 = 0x49,
            _RESERVED_4a = 0x4a,
            _RESERVED_4b = 0x4b,
            _RESERVED_4c = 0x4c,
            _RESERVED_4d = 0x4d,
            _RESERVED_4e = 0x4e,
            _RESERVED_4f = 0x4f,
            _RESERVED_50 = 0x50,
            _RESERVED_51 = 0x51,
            _RESERVED_52 = 0x52,
            _RESERVED_53 = 0x53,
            _RESERVED_54 = 0x54,
            _RESERVED_55 = 0x55,
            _RESERVED_56 = 0x56,
            _RESERVED_57 = 0x57,
            _RESERVED_58 = 0x58,
            _RESERVED_59 = 0x59,
            _RESERVED_5a = 0x5a,
            _RESERVED_5b = 0x5b,
            _RESERVED_5c = 0x5c,
            _RESERVED_5d = 0x5d,
            _RESERVED_5e = 0x5e,
            _RESERVED_5f = 0x5f,
            _RESERVED_60 = 0x60,
            _RESERVED_61 = 0x61,
            _RESERVED_62 = 0x62,
            _RESERVED_63 = 0x63,
            _RESERVED_64 = 0x64,
            _RESERVED_65 = 0x65,
            _RESERVED_66 = 0x66,
            _RESERVED_67 = 0x67,
            _RESERVED_68 = 0x68,
            _RESERVED_69 = 0x69,
            _RESERVED_6a = 0x6a,
            _RESERVED_6b = 0x6b,
            _RESERVED_6c = 0x6c,
            _RESERVED_6d = 0x6d,
            _RESERVED_6e = 0x6e,
            _RESERVED_6f = 0x6f,
            _RESERVED_70 = 0x70,
            _RESERVED_71 = 0x71,
            _RESERVED_72 = 0x72,
            _RESERVED_73 = 0x73,
            _RESERVED_74 = 0x74,
            _RESERVED_75 = 0x75,
            _RESERVED_76 = 0x76,
            _RESERVED_77 = 0x77,
            _RESERVED_78 = 0x78,
            _RESERVED_79 = 0x79,
            _RESERVED_7a = 0x7a,
            _RESERVED_7b = 0x7b,
            _RESERVED_7c = 0x7c,
            _RESERVED_7d = 0x7d,
            _RESERVED_7e = 0x7e,
            _RESERVED_7f = 0x7f,
            _RESERVED_80 = 0x80,
            _RESERVED_81 = 0x81,
            _RESERVED_82 = 0x82,
            _RESERVED_83 = 0x83,
            _RESERVED_84 = 0x84,
            _RESERVED_85 = 0x85,
            _RESERVED_86 = 0x86,
            _RESERVED_87 = 0x87,
            _RESERVED_88 = 0x88,
            _RESERVED_89 = 0x89,
            _RESERVED_8a = 0x8a,
            _RESERVED_8b = 0x8b,
            _RESERVED_8c = 0x8c,
            _RESERVED_8d = 0x8d,
            _RESERVED_8e = 0x8e,
            _RESERVED_8f = 0x8f,
            _RESERVED_90 = 0x90,
            _RESERVED_91 = 0x91,
            _RESERVED_92 = 0x92,
            _RESERVED_93 = 0x93,
            _RESERVED_94 = 0x94,
            _RESERVED_95 = 0x95,
            _RESERVED_96 = 0x96,
            _RESERVED_97 = 0x97,
            _RESERVED_98 = 0x98,
            _RESERVED_99 = 0x99,
            _RESERVED_9a = 0x9a,
            _RESERVED_9b = 0x9b,
            _RESERVED_9c = 0x9c,
            _RESERVED_9d = 0x9d,
            _RESERVED_9e = 0x9e,
            _RESERVED_9f = 0x9f,
            _RESERVED_a0 = 0xa0,
            _RESERVED_a1 = 0xa1,
            _RESERVED_a2 = 0xa2,
            _RESERVED_a3 = 0xa3,
            _RESERVED_a4 = 0xa4,
            _RESERVED_a5 = 0xa5,
            _RESERVED_a6 = 0xa6,
            _RESERVED_a7 = 0xa7,
            _RESERVED_a8 = 0xa8,
            _RESERVED_a9 = 0xa9,
            _RESERVED_aa = 0xaa,
            _RESERVED_ab = 0xab,
            _RESERVED_ac = 0xac,
            _RESERVED_ad = 0xad,
            _RESERVED_ae = 0xae,
            _RESERVED_af = 0xaf,
            _RESERVED_b0 = 0xb0,
            _RESERVED_b1 = 0xb1,
            _RESERVED_b2 = 0xb2,
            _RESERVED_b3 = 0xb3,
            _RESERVED_b4 = 0xb4,
            _RESERVED_b5 = 0xb5,
            _RESERVED_b6 = 0xb6,
            _RESERVED_b7 = 0xb7,
            _RESERVED_b8 = 0xb8,
            _RESERVED_b9 = 0xb9,
            _RESERVED_ba = 0xba,
            _RESERVED_bb = 0xbb,
            _RESERVED_bc = 0xbc,
            _RESERVED_bd = 0xbd,
            _RESERVED_be = 0xbe,
            _RESERVED_bf = 0xbf,
            _RESERVED_c0 = 0xc0,
            _RESERVED_c1 = 0xc1,
            _RESERVED_c2 = 0xc2,
            _RESERVED_c3 = 0xc3,
            _RESERVED_c4 = 0xc4,
            _RESERVED_c5 = 0xc5,
            _RESERVED_c6 = 0xc6,
            _RESERVED_c7 = 0xc7,
            _RESERVED_c8 = 0xc8,
            _RESERVED_c9 = 0xc9,
            _RESERVED_ca = 0xca,
            _RESERVED_cb = 0xcb,
            _RESERVED_cc = 0xcc,
            _RESERVED_cd = 0xcd,
            _RESERVED_ce = 0xce,
            _RESERVED_cf = 0xcf,
            _RESERVED_d0 = 0xd0,
            _RESERVED_d1 = 0xd1,
            _RESERVED_d2 = 0xd2,
            _RESERVED_d3 = 0xd3,
            _RESERVED_d4 = 0xd4,
            _RESERVED_d5 = 0xd5,
            _RESERVED_d6 = 0xd6,
            _RESERVED_d7 = 0xd7,
            #[doc = "-40 dBm"]
            NEG40_DBM = 0xd8,
            _RESERVED_d9 = 0xd9,
            _RESERVED_da = 0xda,
            _RESERVED_db = 0xdb,
            _RESERVED_dc = 0xdc,
            _RESERVED_dd = 0xdd,
            _RESERVED_de = 0xde,
            _RESERVED_df = 0xdf,
            _RESERVED_e0 = 0xe0,
            _RESERVED_e1 = 0xe1,
            #[doc = "Deprecated enumerator - -40 dBm"]
            NEG30_DBM = 0xe2,
            _RESERVED_e3 = 0xe3,
            _RESERVED_e4 = 0xe4,
            _RESERVED_e5 = 0xe5,
            _RESERVED_e6 = 0xe6,
            _RESERVED_e7 = 0xe7,
            _RESERVED_e8 = 0xe8,
            _RESERVED_e9 = 0xe9,
            _RESERVED_ea = 0xea,
            _RESERVED_eb = 0xeb,
            #[doc = "-20 dBm"]
            NEG20_DBM = 0xec,
            _RESERVED_ed = 0xed,
            _RESERVED_ee = 0xee,
            _RESERVED_ef = 0xef,
            #[doc = "-16 dBm"]
            NEG16_DBM = 0xf0,
            _RESERVED_f1 = 0xf1,
            _RESERVED_f2 = 0xf2,
            _RESERVED_f3 = 0xf3,
            #[doc = "-12 dBm"]
            NEG12_DBM = 0xf4,
            _RESERVED_f5 = 0xf5,
            _RESERVED_f6 = 0xf6,
            _RESERVED_f7 = 0xf7,
            #[doc = "-8 dBm"]
            NEG8_DBM = 0xf8,
            _RESERVED_f9 = 0xf9,
            _RESERVED_fa = 0xfa,
            _RESERVED_fb = 0xfb,
            #[doc = "-4 dBm"]
            NEG4_DBM = 0xfc,
            _RESERVED_fd = 0xfd,
            _RESERVED_fe = 0xfe,
            _RESERVED_ff = 0xff,
        }
        impl Txpower {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Txpower {
                unsafe { core::mem::transmute(val & 0xff) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Txpower {
            #[inline(always)]
            fn from(val: u8) -> Txpower {
                Txpower::from_bits(val)
            }
        }
        impl From<Txpower> for u8 {
            #[inline(always)]
            fn from(val: Txpower) -> u8 {
                Txpower::to_bits(val)
            }
        }
    }
}
pub mod rng {
    #[doc = "Random Number Generator"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rng {
        ptr: *mut u8,
    }
    unsafe impl Send for Rng {}
    unsafe impl Sync for Rng {}
    impl Rng {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Task starting the random number generator"]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Task stopping the random number generator"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Event being generated for every new random number written to the VALUE register"]
        #[inline(always)]
        pub const fn events_valrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Configuration register"]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Output random number"]
        #[inline(always)]
        pub const fn value(self) -> crate::common::Reg<regs::Value, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Configuration register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Bias correction"]
            #[inline(always)]
            pub const fn dercen(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Bias correction"]
            #[inline(always)]
            pub fn set_dercen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Config {
            #[inline(always)]
            fn default() -> Config {
                Config(0)
            }
        }
        impl core::fmt::Debug for Config {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Config")
                    .field("dercen", &self.dercen())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Config {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Config {
                    dercen: bool,
                }
                let proxy = Config {
                    dercen: self.dercen(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Write '1' to disable interrupt for event VALRDY"]
            #[inline(always)]
            pub const fn valrdy(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event VALRDY"]
            #[inline(always)]
            pub fn set_valrdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("valrdy", &self.valrdy())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    valrdy: bool,
                }
                let proxy = Int {
                    valrdy: self.valrdy(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event VALRDY and task STOP"]
            #[inline(always)]
            pub const fn valrdy_stop(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event VALRDY and task STOP"]
            #[inline(always)]
            pub fn set_valrdy_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
        impl core::fmt::Debug for Shorts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Shorts")
                    .field("valrdy_stop", &self.valrdy_stop())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Shorts {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Shorts {
                    valrdy_stop: bool,
                }
                let proxy = Shorts {
                    valrdy_stop: self.valrdy_stop(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Output random number"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Value(pub u32);
        impl Value {
            #[doc = "Generated random number"]
            #[inline(always)]
            pub const fn value(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Generated random number"]
            #[inline(always)]
            pub fn set_value(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Value {
            #[inline(always)]
            fn default() -> Value {
                Value(0)
            }
        }
        impl core::fmt::Debug for Value {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Value")
                    .field("value", &self.value())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Value {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Value {
                    value: u8,
                }
                let proxy = Value {
                    value: self.value(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
}
pub mod rtc {
    #[doc = "Real time counter 0"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rtc {
        ptr: *mut u8,
    }
    unsafe impl Send for Rtc {}
    unsafe impl Sync for Rtc {}
    impl Rtc {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Start RTC COUNTER"]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Stop RTC COUNTER"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Clear RTC COUNTER"]
        #[inline(always)]
        pub const fn tasks_clear(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Set COUNTER to 0xFFFFF0"]
        #[inline(always)]
        pub const fn tasks_trigovrflw(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Event on COUNTER increment"]
        #[inline(always)]
        pub const fn events_tick(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "Event on COUNTER overflow"]
        #[inline(always)]
        pub const fn events_ovrflw(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "Description collection: Compare event on CC\\[n\\] match"]
        #[inline(always)]
        pub const fn events_compare(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 4usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize + n * 4usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Enable or disable event routing"]
        #[inline(always)]
        pub const fn evten(self) -> crate::common::Reg<regs::Evt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0340usize) as _) }
        }
        #[doc = "Enable event routing"]
        #[inline(always)]
        pub const fn evtenset(self) -> crate::common::Reg<regs::Evt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0344usize) as _) }
        }
        #[doc = "Disable event routing"]
        #[inline(always)]
        pub const fn evtenclr(self) -> crate::common::Reg<regs::Evt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0348usize) as _) }
        }
        #[doc = "Current COUNTER value"]
        #[inline(always)]
        pub const fn counter(self) -> crate::common::Reg<regs::Counter, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "12 bit prescaler for COUNTER frequency (32768/(PRESCALER+1)).Must be written when RTC is stopped"]
        #[inline(always)]
        pub const fn prescaler(self) -> crate::common::Reg<regs::Prescaler, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Description collection: Compare register n"]
        #[inline(always)]
        pub const fn cc(self, n: usize) -> crate::common::Reg<regs::Cc, crate::common::RW> {
            assert!(n < 4usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0540usize + n * 4usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Description collection: Compare register n"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cc(pub u32);
        impl Cc {
            #[doc = "Compare value"]
            #[inline(always)]
            pub const fn compare(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x00ff_ffff;
                val as u32
            }
            #[doc = "Compare value"]
            #[inline(always)]
            pub fn set_compare(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
            }
        }
        impl Default for Cc {
            #[inline(always)]
            fn default() -> Cc {
                Cc(0)
            }
        }
        impl core::fmt::Debug for Cc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Cc")
                    .field("compare", &self.compare())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Cc {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Cc {
                    compare: u32,
                }
                let proxy = Cc {
                    compare: self.compare(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Current COUNTER value"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Counter(pub u32);
        impl Counter {
            #[doc = "Counter value"]
            #[inline(always)]
            pub const fn counter(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x00ff_ffff;
                val as u32
            }
            #[doc = "Counter value"]
            #[inline(always)]
            pub fn set_counter(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
            }
        }
        impl Default for Counter {
            #[inline(always)]
            fn default() -> Counter {
                Counter(0)
            }
        }
        impl core::fmt::Debug for Counter {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Counter")
                    .field("counter", &self.counter())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Counter {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Counter {
                    counter: u32,
                }
                let proxy = Counter {
                    counter: self.counter(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Enable or disable event routing"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Evt(pub u32);
        impl Evt {
            #[doc = "Enable or disable event routing for event TICK"]
            #[inline(always)]
            pub const fn tick(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable event routing for event TICK"]
            #[inline(always)]
            pub fn set_tick(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable event routing for event OVRFLW"]
            #[inline(always)]
            pub const fn ovrflw(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable event routing for event OVRFLW"]
            #[inline(always)]
            pub fn set_ovrflw(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable event routing for event COMPARE\\[0\\]"]
            #[inline(always)]
            pub const fn compare(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 16usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable event routing for event COMPARE\\[0\\]"]
            #[inline(always)]
            pub fn set_compare(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 16usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Evt {
            #[inline(always)]
            fn default() -> Evt {
                Evt(0)
            }
        }
        impl core::fmt::Debug for Evt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Evt")
                    .field("tick", &self.tick())
                    .field("ovrflw", &self.ovrflw())
                    .field(
                        "compare",
                        &[
                            self.compare(0usize),
                            self.compare(1usize),
                            self.compare(2usize),
                            self.compare(3usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Evt {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Evt {
                    tick: bool,
                    ovrflw: bool,
                    compare: [bool; 4usize],
                }
                let proxy = Evt {
                    tick: self.tick(),
                    ovrflw: self.ovrflw(),
                    compare: [
                        self.compare(0usize),
                        self.compare(1usize),
                        self.compare(2usize),
                        self.compare(3usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Write '1' to disable interrupt for event TICK"]
            #[inline(always)]
            pub const fn tick(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event TICK"]
            #[inline(always)]
            pub fn set_tick(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to disable interrupt for event OVRFLW"]
            #[inline(always)]
            pub const fn ovrflw(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event OVRFLW"]
            #[inline(always)]
            pub fn set_ovrflw(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to disable interrupt for event COMPARE\\[0\\]"]
            #[inline(always)]
            pub const fn compare(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 16usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event COMPARE\\[0\\]"]
            #[inline(always)]
            pub fn set_compare(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 16usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("tick", &self.tick())
                    .field("ovrflw", &self.ovrflw())
                    .field(
                        "compare",
                        &[
                            self.compare(0usize),
                            self.compare(1usize),
                            self.compare(2usize),
                            self.compare(3usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    tick: bool,
                    ovrflw: bool,
                    compare: [bool; 4usize],
                }
                let proxy = Int {
                    tick: self.tick(),
                    ovrflw: self.ovrflw(),
                    compare: [
                        self.compare(0usize),
                        self.compare(1usize),
                        self.compare(2usize),
                        self.compare(3usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "12 bit prescaler for COUNTER frequency (32768/(PRESCALER+1)).Must be written when RTC is stopped"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Prescaler(pub u32);
        impl Prescaler {
            #[doc = "Prescaler value"]
            #[inline(always)]
            pub const fn prescaler(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "Prescaler value"]
            #[inline(always)]
            pub fn set_prescaler(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
        }
        impl Default for Prescaler {
            #[inline(always)]
            fn default() -> Prescaler {
                Prescaler(0)
            }
        }
        impl core::fmt::Debug for Prescaler {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Prescaler")
                    .field("prescaler", &self.prescaler())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Prescaler {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Prescaler {
                    prescaler: u16,
                }
                let proxy = Prescaler {
                    prescaler: self.prescaler(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
}
pub mod saadc {
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch {
        ptr: *mut u8,
    }
    unsafe impl Send for Ch {}
    unsafe impl Sync for Ch {}
    impl Ch {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description cluster: Input positive pin selection for CH\\[n\\]"]
        #[inline(always)]
        pub const fn pselp(self) -> crate::common::Reg<regs::Pselp, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Description cluster: Input negative pin selection for CH\\[n\\]"]
        #[inline(always)]
        pub const fn pseln(self) -> crate::common::Reg<regs::Pseln, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Description cluster: Input configuration for CH\\[n\\]"]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Description cluster: High/low limits for event monitoring a channel"]
        #[inline(always)]
        pub const fn limit(self) -> crate::common::Reg<regs::Limit, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
    }
    #[doc = "Peripheral events."]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EventsCh {
        ptr: *mut u8,
    }
    unsafe impl Send for EventsCh {}
    unsafe impl Sync for EventsCh {}
    impl EventsCh {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description cluster: Last results is equal or above CH\\[n\\].LIMIT.HIGH"]
        #[inline(always)]
        pub const fn limith(self) -> crate::common::Reg<regs::Limith, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Description cluster: Last results is equal or below CH\\[n\\].LIMIT.LOW"]
        #[inline(always)]
        pub const fn limitl(self) -> crate::common::Reg<regs::Limitl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
    }
    #[doc = "RESULT EasyDMA channel"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Result {
        ptr: *mut u8,
    }
    unsafe impl Send for Result {}
    unsafe impl Sync for Result {}
    impl Result {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Data pointer"]
        #[inline(always)]
        pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Maximum number of buffer words to transfer"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::Maxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Number of buffer words transferred since last START"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::Amount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
    }
    #[doc = "Analog to Digital Converter"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Saadc {
        ptr: *mut u8,
    }
    unsafe impl Send for Saadc {}
    unsafe impl Sync for Saadc {}
    impl Saadc {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Start the ADC and prepare the result buffer in RAM"]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Take one ADC sample, if scan is enabled all channels are sampled"]
        #[inline(always)]
        pub const fn tasks_sample(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Stop the ADC and terminate any on-going conversion"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Starts offset auto-calibration"]
        #[inline(always)]
        pub const fn tasks_calibrateoffset(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "The ADC has started"]
        #[inline(always)]
        pub const fn events_started(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "The ADC has filled up the Result buffer"]
        #[inline(always)]
        pub const fn events_end(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "A conversion task has been completed. Depending on the mode, multiple conversions might be needed for a result to be transferred to RAM."]
        #[inline(always)]
        pub const fn events_done(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "A result is ready to get transferred to RAM."]
        #[inline(always)]
        pub const fn events_resultdone(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
        }
        #[doc = "Calibration is complete"]
        #[inline(always)]
        pub const fn events_calibratedone(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
        }
        #[doc = "The ADC has stopped"]
        #[inline(always)]
        pub const fn events_stopped(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
        }
        #[doc = "Peripheral events."]
        #[inline(always)]
        pub const fn events_ch(self, n: usize) -> EventsCh {
            assert!(n < 8usize);
            unsafe { EventsCh::from_ptr(self.ptr.add(0x0118usize + n * 8usize) as _) }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Status"]
        #[inline(always)]
        pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
        }
        #[doc = "Enable or disable ADC"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn ch(self, n: usize) -> Ch {
            assert!(n < 8usize);
            unsafe { Ch::from_ptr(self.ptr.add(0x0510usize + n * 16usize) as _) }
        }
        #[doc = "Resolution configuration"]
        #[inline(always)]
        pub const fn resolution(self) -> crate::common::Reg<regs::Resolution, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05f0usize) as _) }
        }
        #[doc = "Oversampling configuration. OVERSAMPLE should not be combined with SCAN. The RESOLUTION is applied before averaging, thus for high OVERSAMPLE a higher RESOLUTION should be used."]
        #[inline(always)]
        pub const fn oversample(self) -> crate::common::Reg<regs::Oversample, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05f4usize) as _) }
        }
        #[doc = "Controls normal or continuous sample rate"]
        #[inline(always)]
        pub const fn samplerate(self) -> crate::common::Reg<regs::Samplerate, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05f8usize) as _) }
        }
        #[doc = "RESULT EasyDMA channel"]
        #[inline(always)]
        pub const fn result(self) -> Result {
            unsafe { Result::from_ptr(self.ptr.add(0x062cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Number of buffer words transferred since last START"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Amount(pub u32);
        impl Amount {
            #[doc = "Number of buffer words transferred since last START. This register can be read after an END or STOPPED event."]
            #[inline(always)]
            pub const fn amount(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x7fff;
                val as u16
            }
            #[doc = "Number of buffer words transferred since last START. This register can be read after an END or STOPPED event."]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
            }
        }
        impl Default for Amount {
            #[inline(always)]
            fn default() -> Amount {
                Amount(0)
            }
        }
        impl core::fmt::Debug for Amount {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Amount")
                    .field("amount", &self.amount())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Amount {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Amount {
                    amount: u16,
                }
                let proxy = Amount {
                    amount: self.amount(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Description cluster: Input configuration for CH\\[n\\]"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Positive channel resistor control"]
            #[inline(always)]
            pub const fn resp(&self) -> super::vals::Resp {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Resp::from_bits(val as u8)
            }
            #[doc = "Positive channel resistor control"]
            #[inline(always)]
            pub fn set_resp(&mut self, val: super::vals::Resp) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
            #[doc = "Negative channel resistor control"]
            #[inline(always)]
            pub const fn resn(&self) -> super::vals::Resn {
                let val = (self.0 >> 4usize) & 0x03;
                super::vals::Resn::from_bits(val as u8)
            }
            #[doc = "Negative channel resistor control"]
            #[inline(always)]
            pub fn set_resn(&mut self, val: super::vals::Resn) {
                self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
            }
            #[doc = "Gain control"]
            #[inline(always)]
            pub const fn gain(&self) -> super::vals::Gain {
                let val = (self.0 >> 8usize) & 0x07;
                super::vals::Gain::from_bits(val as u8)
            }
            #[doc = "Gain control"]
            #[inline(always)]
            pub fn set_gain(&mut self, val: super::vals::Gain) {
                self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
            }
            #[doc = "Reference control"]
            #[inline(always)]
            pub const fn refsel(&self) -> super::vals::Refsel {
                let val = (self.0 >> 12usize) & 0x01;
                super::vals::Refsel::from_bits(val as u8)
            }
            #[doc = "Reference control"]
            #[inline(always)]
            pub fn set_refsel(&mut self, val: super::vals::Refsel) {
                self.0 =
                    (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
            }
            #[doc = "Acquisition time, the time the ADC uses to sample the input voltage"]
            #[inline(always)]
            pub const fn tacq(&self) -> super::vals::Tacq {
                let val = (self.0 >> 16usize) & 0x07;
                super::vals::Tacq::from_bits(val as u8)
            }
            #[doc = "Acquisition time, the time the ADC uses to sample the input voltage"]
            #[inline(always)]
            pub fn set_tacq(&mut self, val: super::vals::Tacq) {
                self.0 =
                    (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
            }
            #[doc = "Enable differential mode"]
            #[inline(always)]
            pub const fn mode(&self) -> super::vals::ConfigMode {
                let val = (self.0 >> 20usize) & 0x01;
                super::vals::ConfigMode::from_bits(val as u8)
            }
            #[doc = "Enable differential mode"]
            #[inline(always)]
            pub fn set_mode(&mut self, val: super::vals::ConfigMode) {
                self.0 =
                    (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
            }
            #[doc = "Enable burst mode"]
            #[inline(always)]
            pub const fn burst(&self) -> bool {
                let val = (self.0 >> 24usize) & 0x01;
                val != 0
            }
            #[doc = "Enable burst mode"]
            #[inline(always)]
            pub fn set_burst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
            }
        }
        impl Default for Config {
            #[inline(always)]
            fn default() -> Config {
                Config(0)
            }
        }
        impl core::fmt::Debug for Config {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Config")
                    .field("resp", &self.resp())
                    .field("resn", &self.resn())
                    .field("gain", &self.gain())
                    .field("refsel", &self.refsel())
                    .field("tacq", &self.tacq())
                    .field("mode", &self.mode())
                    .field("burst", &self.burst())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Config {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Config {
                    resp: super::vals::Resp,
                    resn: super::vals::Resn,
                    gain: super::vals::Gain,
                    refsel: super::vals::Refsel,
                    tacq: super::vals::Tacq,
                    mode: super::vals::ConfigMode,
                    burst: bool,
                }
                let proxy = Config {
                    resp: self.resp(),
                    resn: self.resn(),
                    gain: self.gain(),
                    refsel: self.refsel(),
                    tacq: self.tacq(),
                    mode: self.mode(),
                    burst: self.burst(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Enable or disable ADC"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable ADC"]
            #[inline(always)]
            pub const fn enable(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable ADC"]
            #[inline(always)]
            pub fn set_enable(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        impl core::fmt::Debug for Enable {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Enable")
                    .field("enable", &self.enable())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Enable {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Enable {
                    enable: bool,
                }
                let proxy = Enable {
                    enable: self.enable(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Enable or disable interrupt for event STARTED"]
            #[inline(always)]
            pub const fn started(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event STARTED"]
            #[inline(always)]
            pub fn set_started(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable interrupt for event END"]
            #[inline(always)]
            pub const fn end(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event END"]
            #[inline(always)]
            pub fn set_end(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable interrupt for event DONE"]
            #[inline(always)]
            pub const fn done(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event DONE"]
            #[inline(always)]
            pub fn set_done(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable or disable interrupt for event RESULTDONE"]
            #[inline(always)]
            pub const fn resultdone(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RESULTDONE"]
            #[inline(always)]
            pub fn set_resultdone(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Enable or disable interrupt for event CALIBRATEDONE"]
            #[inline(always)]
            pub const fn calibratedone(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event CALIBRATEDONE"]
            #[inline(always)]
            pub fn set_calibratedone(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Enable or disable interrupt for event STOPPED"]
            #[inline(always)]
            pub const fn stopped(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event STOPPED"]
            #[inline(always)]
            pub fn set_stopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Enable or disable interrupt for event CH0LIMITH"]
            #[inline(always)]
            pub const fn chlimith(&self, n: usize) -> bool {
                assert!(n < 8usize);
                let offs = 6usize + n * 2usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event CH0LIMITH"]
            #[inline(always)]
            pub fn set_chlimith(&mut self, n: usize, val: bool) {
                assert!(n < 8usize);
                let offs = 6usize + n * 2usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Enable or disable interrupt for event CH0LIMITL"]
            #[inline(always)]
            pub const fn chlimitl(&self, n: usize) -> bool {
                assert!(n < 8usize);
                let offs = 7usize + n * 2usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event CH0LIMITL"]
            #[inline(always)]
            pub fn set_chlimitl(&mut self, n: usize, val: bool) {
                assert!(n < 8usize);
                let offs = 7usize + n * 2usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("started", &self.started())
                    .field("end", &self.end())
                    .field("done", &self.done())
                    .field("resultdone", &self.resultdone())
                    .field("calibratedone", &self.calibratedone())
                    .field("stopped", &self.stopped())
                    .field(
                        "chlimith",
                        &[
                            self.chlimith(0usize),
                            self.chlimith(1usize),
                            self.chlimith(2usize),
                            self.chlimith(3usize),
                            self.chlimith(4usize),
                            self.chlimith(5usize),
                            self.chlimith(6usize),
                            self.chlimith(7usize),
                        ],
                    )
                    .field(
                        "chlimitl",
                        &[
                            self.chlimitl(0usize),
                            self.chlimitl(1usize),
                            self.chlimitl(2usize),
                            self.chlimitl(3usize),
                            self.chlimitl(4usize),
                            self.chlimitl(5usize),
                            self.chlimitl(6usize),
                            self.chlimitl(7usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    started: bool,
                    end: bool,
                    done: bool,
                    resultdone: bool,
                    calibratedone: bool,
                    stopped: bool,
                    chlimith: [bool; 8usize],
                    chlimitl: [bool; 8usize],
                }
                let proxy = Int {
                    started: self.started(),
                    end: self.end(),
                    done: self.done(),
                    resultdone: self.resultdone(),
                    calibratedone: self.calibratedone(),
                    stopped: self.stopped(),
                    chlimith: [
                        self.chlimith(0usize),
                        self.chlimith(1usize),
                        self.chlimith(2usize),
                        self.chlimith(3usize),
                        self.chlimith(4usize),
                        self.chlimith(5usize),
                        self.chlimith(6usize),
                        self.chlimith(7usize),
                    ],
                    chlimitl: [
                        self.chlimitl(0usize),
                        self.chlimitl(1usize),
                        self.chlimitl(2usize),
                        self.chlimitl(3usize),
                        self.chlimitl(4usize),
                        self.chlimitl(5usize),
                        self.chlimitl(6usize),
                        self.chlimitl(7usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Description cluster: High/low limits for event monitoring a channel"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Limit(pub u32);
        impl Limit {
            #[doc = "Low level limit"]
            #[inline(always)]
            pub const fn low(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Low level limit"]
            #[inline(always)]
            pub fn set_low(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
            #[doc = "High level limit"]
            #[inline(always)]
            pub const fn high(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0xffff;
                val as u16
            }
            #[doc = "High level limit"]
            #[inline(always)]
            pub fn set_high(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
            }
        }
        impl Default for Limit {
            #[inline(always)]
            fn default() -> Limit {
                Limit(0)
            }
        }
        impl core::fmt::Debug for Limit {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Limit")
                    .field("low", &self.low())
                    .field("high", &self.high())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Limit {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Limit {
                    low: u16,
                    high: u16,
                }
                let proxy = Limit {
                    low: self.low(),
                    high: self.high(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Description cluster: Last results is equal or above CH\\[n\\].LIMIT.HIGH"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Limith(pub u32);
        impl Limith {
            #[doc = "Last results is equal or above CH\\[n\\].LIMIT.HIGH"]
            #[inline(always)]
            pub const fn limith(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Last results is equal or above CH\\[n\\].LIMIT.HIGH"]
            #[inline(always)]
            pub fn set_limith(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Limith {
            #[inline(always)]
            fn default() -> Limith {
                Limith(0)
            }
        }
        impl core::fmt::Debug for Limith {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Limith")
                    .field("limith", &self.limith())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Limith {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Limith {
                    limith: bool,
                }
                let proxy = Limith {
                    limith: self.limith(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Description cluster: Last results is equal or below CH\\[n\\].LIMIT.LOW"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Limitl(pub u32);
        impl Limitl {
            #[doc = "Last results is equal or below CH\\[n\\].LIMIT.LOW"]
            #[inline(always)]
            pub const fn limitl(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Last results is equal or below CH\\[n\\].LIMIT.LOW"]
            #[inline(always)]
            pub fn set_limitl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Limitl {
            #[inline(always)]
            fn default() -> Limitl {
                Limitl(0)
            }
        }
        impl core::fmt::Debug for Limitl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Limitl")
                    .field("limitl", &self.limitl())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Limitl {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Limitl {
                    limitl: bool,
                }
                let proxy = Limitl {
                    limitl: self.limitl(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Maximum number of buffer words to transfer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Maxcnt(pub u32);
        impl Maxcnt {
            #[doc = "Maximum number of buffer words to transfer"]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x7fff;
                val as u16
            }
            #[doc = "Maximum number of buffer words to transfer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
            }
        }
        impl Default for Maxcnt {
            #[inline(always)]
            fn default() -> Maxcnt {
                Maxcnt(0)
            }
        }
        impl core::fmt::Debug for Maxcnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Maxcnt")
                    .field("maxcnt", &self.maxcnt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Maxcnt {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Maxcnt {
                    maxcnt: u16,
                }
                let proxy = Maxcnt {
                    maxcnt: self.maxcnt(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Oversampling configuration. OVERSAMPLE should not be combined with SCAN. The RESOLUTION is applied before averaging, thus for high OVERSAMPLE a higher RESOLUTION should be used."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Oversample(pub u32);
        impl Oversample {
            #[doc = "Oversample control"]
            #[inline(always)]
            pub const fn oversample(&self) -> super::vals::Oversample {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Oversample::from_bits(val as u8)
            }
            #[doc = "Oversample control"]
            #[inline(always)]
            pub fn set_oversample(&mut self, val: super::vals::Oversample) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Oversample {
            #[inline(always)]
            fn default() -> Oversample {
                Oversample(0)
            }
        }
        impl core::fmt::Debug for Oversample {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Oversample")
                    .field("oversample", &self.oversample())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Oversample {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Oversample {
                    oversample: super::vals::Oversample,
                }
                let proxy = Oversample {
                    oversample: self.oversample(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Description cluster: Input negative pin selection for CH\\[n\\]"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pseln(pub u32);
        impl Pseln {
            #[doc = "Analog negative input, enables differential channel"]
            #[inline(always)]
            pub const fn pseln(&self) -> super::vals::Psel {
                let val = (self.0 >> 0usize) & 0x1f;
                super::vals::Psel::from_bits(val as u8)
            }
            #[doc = "Analog negative input, enables differential channel"]
            #[inline(always)]
            pub fn set_pseln(&mut self, val: super::vals::Psel) {
                self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
            }
        }
        impl Default for Pseln {
            #[inline(always)]
            fn default() -> Pseln {
                Pseln(0)
            }
        }
        impl core::fmt::Debug for Pseln {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Pseln")
                    .field("pseln", &self.pseln())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Pseln {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Pseln {
                    pseln: super::vals::Psel,
                }
                let proxy = Pseln {
                    pseln: self.pseln(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Description cluster: Input positive pin selection for CH\\[n\\]"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pselp(pub u32);
        impl Pselp {
            #[doc = "Analog positive input channel"]
            #[inline(always)]
            pub const fn pselp(&self) -> super::vals::Psel {
                let val = (self.0 >> 0usize) & 0x1f;
                super::vals::Psel::from_bits(val as u8)
            }
            #[doc = "Analog positive input channel"]
            #[inline(always)]
            pub fn set_pselp(&mut self, val: super::vals::Psel) {
                self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
            }
        }
        impl Default for Pselp {
            #[inline(always)]
            fn default() -> Pselp {
                Pselp(0)
            }
        }
        impl core::fmt::Debug for Pselp {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Pselp")
                    .field("pselp", &self.pselp())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Pselp {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Pselp {
                    pselp: super::vals::Psel,
                }
                let proxy = Pselp {
                    pselp: self.pselp(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Resolution configuration"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Resolution(pub u32);
        impl Resolution {
            #[doc = "Set the resolution"]
            #[inline(always)]
            pub const fn val(&self) -> super::vals::Val {
                let val = (self.0 >> 0usize) & 0x07;
                super::vals::Val::from_bits(val as u8)
            }
            #[doc = "Set the resolution"]
            #[inline(always)]
            pub fn set_val(&mut self, val: super::vals::Val) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
            }
        }
        impl Default for Resolution {
            #[inline(always)]
            fn default() -> Resolution {
                Resolution(0)
            }
        }
        impl core::fmt::Debug for Resolution {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Resolution")
                    .field("val", &self.val())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Resolution {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Resolution {
                    val: super::vals::Val,
                }
                let proxy = Resolution { val: self.val() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Controls normal or continuous sample rate"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Samplerate(pub u32);
        impl Samplerate {
            #[doc = "Capture and compare value. Sample rate is 16 MHz/CC"]
            #[inline(always)]
            pub const fn cc(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x07ff;
                val as u16
            }
            #[doc = "Capture and compare value. Sample rate is 16 MHz/CC"]
            #[inline(always)]
            pub fn set_cc(&mut self, val: u16) {
                self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
            }
            #[doc = "Select mode for sample rate control"]
            #[inline(always)]
            pub const fn mode(&self) -> super::vals::SamplerateMode {
                let val = (self.0 >> 12usize) & 0x01;
                super::vals::SamplerateMode::from_bits(val as u8)
            }
            #[doc = "Select mode for sample rate control"]
            #[inline(always)]
            pub fn set_mode(&mut self, val: super::vals::SamplerateMode) {
                self.0 =
                    (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
            }
        }
        impl Default for Samplerate {
            #[inline(always)]
            fn default() -> Samplerate {
                Samplerate(0)
            }
        }
        impl core::fmt::Debug for Samplerate {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Samplerate")
                    .field("cc", &self.cc())
                    .field("mode", &self.mode())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Samplerate {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Samplerate {
                    cc: u16,
                    mode: super::vals::SamplerateMode,
                }
                let proxy = Samplerate {
                    cc: self.cc(),
                    mode: self.mode(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Status"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Status(pub u32);
        impl Status {
            #[doc = "Status"]
            #[inline(always)]
            pub const fn status(&self) -> super::vals::Status {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Status::from_bits(val as u8)
            }
            #[doc = "Status"]
            #[inline(always)]
            pub fn set_status(&mut self, val: super::vals::Status) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Status {
            #[inline(always)]
            fn default() -> Status {
                Status(0)
            }
        }
        impl core::fmt::Debug for Status {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Status")
                    .field("status", &self.status())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Status {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Status {
                    status: super::vals::Status,
                }
                let proxy = Status {
                    status: self.status(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum ConfigMode {
            #[doc = "Single ended, PSELN will be ignored, negative input to ADC shorted to GND"]
            SE = 0x0,
            #[doc = "Differential"]
            DIFF = 0x01,
        }
        impl ConfigMode {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> ConfigMode {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for ConfigMode {
            #[inline(always)]
            fn from(val: u8) -> ConfigMode {
                ConfigMode::from_bits(val)
            }
        }
        impl From<ConfigMode> for u8 {
            #[inline(always)]
            fn from(val: ConfigMode) -> u8 {
                ConfigMode::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Gain {
            #[doc = "1/6"]
            GAIN1_6 = 0x0,
            #[doc = "1/5"]
            GAIN1_5 = 0x01,
            #[doc = "1/4"]
            GAIN1_4 = 0x02,
            #[doc = "1/3"]
            GAIN1_3 = 0x03,
            #[doc = "1/2"]
            GAIN1_2 = 0x04,
            #[doc = "1"]
            GAIN1 = 0x05,
            #[doc = "2"]
            GAIN2 = 0x06,
            #[doc = "4"]
            GAIN4 = 0x07,
        }
        impl Gain {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Gain {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Gain {
            #[inline(always)]
            fn from(val: u8) -> Gain {
                Gain::from_bits(val)
            }
        }
        impl From<Gain> for u8 {
            #[inline(always)]
            fn from(val: Gain) -> u8 {
                Gain::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Oversample {
            #[doc = "Bypass oversampling"]
            BYPASS = 0x0,
            #[doc = "Oversample 2x"]
            OVER2X = 0x01,
            #[doc = "Oversample 4x"]
            OVER4X = 0x02,
            #[doc = "Oversample 8x"]
            OVER8X = 0x03,
            #[doc = "Oversample 16x"]
            OVER16X = 0x04,
            #[doc = "Oversample 32x"]
            OVER32X = 0x05,
            #[doc = "Oversample 64x"]
            OVER64X = 0x06,
            #[doc = "Oversample 128x"]
            OVER128X = 0x07,
            #[doc = "Oversample 256x"]
            OVER256X = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl Oversample {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Oversample {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Oversample {
            #[inline(always)]
            fn from(val: u8) -> Oversample {
                Oversample::from_bits(val)
            }
        }
        impl From<Oversample> for u8 {
            #[inline(always)]
            fn from(val: Oversample) -> u8 {
                Oversample::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Psel {
            #[doc = "Not connected"]
            NC = 0x0,
            #[doc = "AIN0"]
            ANALOG_INPUT0 = 0x01,
            #[doc = "AIN1"]
            ANALOG_INPUT1 = 0x02,
            #[doc = "AIN2"]
            ANALOG_INPUT2 = 0x03,
            #[doc = "AIN3"]
            ANALOG_INPUT3 = 0x04,
            #[doc = "AIN4"]
            ANALOG_INPUT4 = 0x05,
            #[doc = "AIN5"]
            ANALOG_INPUT5 = 0x06,
            #[doc = "AIN6"]
            ANALOG_INPUT6 = 0x07,
            #[doc = "AIN7"]
            ANALOG_INPUT7 = 0x08,
            #[doc = "VDD"]
            VDD = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
            _RESERVED_10 = 0x10,
            _RESERVED_11 = 0x11,
            _RESERVED_12 = 0x12,
            _RESERVED_13 = 0x13,
            _RESERVED_14 = 0x14,
            _RESERVED_15 = 0x15,
            _RESERVED_16 = 0x16,
            _RESERVED_17 = 0x17,
            _RESERVED_18 = 0x18,
            _RESERVED_19 = 0x19,
            _RESERVED_1a = 0x1a,
            _RESERVED_1b = 0x1b,
            _RESERVED_1c = 0x1c,
            _RESERVED_1d = 0x1d,
            _RESERVED_1e = 0x1e,
            _RESERVED_1f = 0x1f,
        }
        impl Psel {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Psel {
                unsafe { core::mem::transmute(val & 0x1f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Psel {
            #[inline(always)]
            fn from(val: u8) -> Psel {
                Psel::from_bits(val)
            }
        }
        impl From<Psel> for u8 {
            #[inline(always)]
            fn from(val: Psel) -> u8 {
                Psel::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Refsel {
            #[doc = "Internal reference (0.6 V)"]
            INTERNAL = 0x0,
            #[doc = "VDD/4 as reference"]
            VDD1_4 = 0x01,
        }
        impl Refsel {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Refsel {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Refsel {
            #[inline(always)]
            fn from(val: u8) -> Refsel {
                Refsel::from_bits(val)
            }
        }
        impl From<Refsel> for u8 {
            #[inline(always)]
            fn from(val: Refsel) -> u8 {
                Refsel::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Resn {
            #[doc = "Bypass resistor ladder"]
            BYPASS = 0x0,
            #[doc = "Pull-down to GND"]
            PULLDOWN = 0x01,
            #[doc = "Pull-up to VDD"]
            PULLUP = 0x02,
            #[doc = "Set input at VDD/2"]
            VDD1_2 = 0x03,
        }
        impl Resn {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Resn {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Resn {
            #[inline(always)]
            fn from(val: u8) -> Resn {
                Resn::from_bits(val)
            }
        }
        impl From<Resn> for u8 {
            #[inline(always)]
            fn from(val: Resn) -> u8 {
                Resn::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Resp {
            #[doc = "Bypass resistor ladder"]
            BYPASS = 0x0,
            #[doc = "Pull-down to GND"]
            PULLDOWN = 0x01,
            #[doc = "Pull-up to VDD"]
            PULLUP = 0x02,
            #[doc = "Set input at VDD/2"]
            VDD1_2 = 0x03,
        }
        impl Resp {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Resp {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Resp {
            #[inline(always)]
            fn from(val: u8) -> Resp {
                Resp::from_bits(val)
            }
        }
        impl From<Resp> for u8 {
            #[inline(always)]
            fn from(val: Resp) -> u8 {
                Resp::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum SamplerateMode {
            #[doc = "Rate is controlled from SAMPLE task"]
            TASK = 0x0,
            #[doc = "Rate is controlled from local timer (use CC to control the rate)"]
            TIMERS = 0x01,
        }
        impl SamplerateMode {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> SamplerateMode {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for SamplerateMode {
            #[inline(always)]
            fn from(val: u8) -> SamplerateMode {
                SamplerateMode::from_bits(val)
            }
        }
        impl From<SamplerateMode> for u8 {
            #[inline(always)]
            fn from(val: SamplerateMode) -> u8 {
                SamplerateMode::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Status {
            #[doc = "ADC is ready. No on-going conversion."]
            READY = 0x0,
            #[doc = "ADC is busy. Conversion in progress."]
            BUSY = 0x01,
        }
        impl Status {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Status {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Status {
            #[inline(always)]
            fn from(val: u8) -> Status {
                Status::from_bits(val)
            }
        }
        impl From<Status> for u8 {
            #[inline(always)]
            fn from(val: Status) -> u8 {
                Status::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Tacq {
            #[doc = "3 us"]
            _3US = 0x0,
            #[doc = "5 us"]
            _5US = 0x01,
            #[doc = "10 us"]
            _10US = 0x02,
            #[doc = "15 us"]
            _15US = 0x03,
            #[doc = "20 us"]
            _20US = 0x04,
            #[doc = "40 us"]
            _40US = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
        }
        impl Tacq {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Tacq {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Tacq {
            #[inline(always)]
            fn from(val: u8) -> Tacq {
                Tacq::from_bits(val)
            }
        }
        impl From<Tacq> for u8 {
            #[inline(always)]
            fn from(val: Tacq) -> u8 {
                Tacq::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Val {
            #[doc = "8 bit"]
            _8BIT = 0x0,
            #[doc = "10 bit"]
            _10BIT = 0x01,
            #[doc = "12 bit"]
            _12BIT = 0x02,
            #[doc = "14 bit"]
            _14BIT = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
        }
        impl Val {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Val {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Val {
            #[inline(always)]
            fn from(val: u8) -> Val {
                Val::from_bits(val)
            }
        }
        impl From<Val> for u8 {
            #[inline(always)]
            fn from(val: Val) -> u8 {
                Val::to_bits(val)
            }
        }
    }
}
pub mod shared {
    pub mod regs {
        #[doc = "Pin select for A signal"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Psel(pub u32);
        impl Psel {
            #[doc = "Pin number"]
            #[inline(always)]
            pub const fn pin(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x1f;
                val as u8
            }
            #[doc = "Pin number"]
            #[inline(always)]
            pub fn set_pin(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
            }
            #[doc = "Connection"]
            #[inline(always)]
            pub const fn connect(&self) -> super::vals::Connect {
                let val = (self.0 >> 31usize) & 0x01;
                super::vals::Connect::from_bits(val as u8)
            }
            #[doc = "Connection"]
            #[inline(always)]
            pub fn set_connect(&mut self, val: super::vals::Connect) {
                self.0 =
                    (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
            }
        }
        impl Default for Psel {
            #[inline(always)]
            fn default() -> Psel {
                Psel(0)
            }
        }
        impl core::fmt::Debug for Psel {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Psel")
                    .field("pin", &self.pin())
                    .field("connect", &self.connect())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Psel {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Psel {
                    pin: u8,
                    connect: super::vals::Connect,
                }
                let proxy = Psel {
                    pin: self.pin(),
                    connect: self.connect(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Connect {
            #[doc = "Connect"]
            CONNECTED = 0x0,
            #[doc = "Disconnect"]
            DISCONNECTED = 0x01,
        }
        impl Connect {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Connect {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Connect {
            #[inline(always)]
            fn from(val: u8) -> Connect {
                Connect::from_bits(val)
            }
        }
        impl From<Connect> for u8 {
            #[inline(always)]
            fn from(val: Connect) -> u8 {
                Connect::to_bits(val)
            }
        }
    }
}
pub mod spi {
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psel {
        ptr: *mut u8,
    }
    unsafe impl Send for Psel {}
    unsafe impl Sync for Psel {}
    impl Psel {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Pin select for SCK"]
        #[inline(always)]
        pub const fn sck(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Pin select for MOSI signal"]
        #[inline(always)]
        pub const fn mosi(
            self,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Pin select for MISO signal"]
        #[inline(always)]
        pub const fn miso(
            self,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
    }
    #[doc = "Serial Peripheral Interface"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Spi {
        ptr: *mut u8,
    }
    unsafe impl Send for Spi {}
    unsafe impl Sync for Spi {}
    impl Spi {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "TXD byte sent and RXD byte received"]
        #[inline(always)]
        pub const fn events_ready(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Enable SPI"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn psel(self) -> Psel {
            unsafe { Psel::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "RXD register"]
        #[inline(always)]
        pub const fn rxd(self) -> crate::common::Reg<regs::Rxd, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
        }
        #[doc = "TXD register"]
        #[inline(always)]
        pub const fn txd(self) -> crate::common::Reg<regs::Txd, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
        }
        #[doc = "SPI frequency. Accuracy depends on the HFCLK source selected."]
        #[inline(always)]
        pub const fn frequency(self) -> crate::common::Reg<regs::Frequency, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
        }
        #[doc = "Configuration register"]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0554usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Configuration register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Bit order"]
            #[inline(always)]
            pub const fn order(&self) -> super::vals::Order {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Order::from_bits(val as u8)
            }
            #[doc = "Bit order"]
            #[inline(always)]
            pub fn set_order(&mut self, val: super::vals::Order) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Serial clock (SCK) phase"]
            #[inline(always)]
            pub const fn cpha(&self) -> super::vals::Cpha {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Cpha::from_bits(val as u8)
            }
            #[doc = "Serial clock (SCK) phase"]
            #[inline(always)]
            pub fn set_cpha(&mut self, val: super::vals::Cpha) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Serial clock (SCK) polarity"]
            #[inline(always)]
            pub const fn cpol(&self) -> super::vals::Cpol {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Cpol::from_bits(val as u8)
            }
            #[doc = "Serial clock (SCK) polarity"]
            #[inline(always)]
            pub fn set_cpol(&mut self, val: super::vals::Cpol) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
            }
        }
        impl Default for Config {
            #[inline(always)]
            fn default() -> Config {
                Config(0)
            }
        }
        impl core::fmt::Debug for Config {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Config")
                    .field("order", &self.order())
                    .field("cpha", &self.cpha())
                    .field("cpol", &self.cpol())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Config {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Config {
                    order: super::vals::Order,
                    cpha: super::vals::Cpha,
                    cpol: super::vals::Cpol,
                }
                let proxy = Config {
                    order: self.order(),
                    cpha: self.cpha(),
                    cpol: self.cpol(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Enable SPI"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable SPI"]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable SPI"]
            #[inline(always)]
            pub fn set_enable(&mut self, val: super::vals::Enable) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        impl core::fmt::Debug for Enable {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Enable")
                    .field("enable", &self.enable())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Enable {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Enable {
                    enable: super::vals::Enable,
                }
                let proxy = Enable {
                    enable: self.enable(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "SPI frequency. Accuracy depends on the HFCLK source selected."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Frequency(pub u32);
        impl Frequency {
            #[doc = "SPI master data rate"]
            #[inline(always)]
            pub const fn frequency(&self) -> super::vals::Frequency {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Frequency::from_bits(val as u32)
            }
            #[doc = "SPI master data rate"]
            #[inline(always)]
            pub fn set_frequency(&mut self, val: super::vals::Frequency) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize))
                    | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Frequency {
            #[inline(always)]
            fn default() -> Frequency {
                Frequency(0)
            }
        }
        impl core::fmt::Debug for Frequency {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Frequency")
                    .field("frequency", &self.frequency())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Frequency {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Frequency {
                    frequency: super::vals::Frequency,
                }
                let proxy = Frequency {
                    frequency: self.frequency(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Write '1' to disable interrupt for event READY"]
            #[inline(always)]
            pub const fn ready(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event READY"]
            #[inline(always)]
            pub fn set_ready(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int").field("ready", &self.ready()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    ready: bool,
                }
                let proxy = Int {
                    ready: self.ready(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "RXD register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxd(pub u32);
        impl Rxd {
            #[doc = "RX data received. Double buffered"]
            #[inline(always)]
            pub const fn rxd(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "RX data received. Double buffered"]
            #[inline(always)]
            pub fn set_rxd(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Rxd {
            #[inline(always)]
            fn default() -> Rxd {
                Rxd(0)
            }
        }
        impl core::fmt::Debug for Rxd {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Rxd").field("rxd", &self.rxd()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rxd {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Rxd {
                    rxd: u8,
                }
                let proxy = Rxd { rxd: self.rxd() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "TXD register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Txd(pub u32);
        impl Txd {
            #[doc = "TX data to send. Double buffered."]
            #[inline(always)]
            pub const fn txd(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "TX data to send. Double buffered."]
            #[inline(always)]
            pub fn set_txd(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Txd {
            #[inline(always)]
            fn default() -> Txd {
                Txd(0)
            }
        }
        impl core::fmt::Debug for Txd {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Txd").field("txd", &self.txd()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Txd {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Txd {
                    txd: u8,
                }
                let proxy = Txd { txd: self.txd() };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Cpha {
            #[doc = "Sample on leading edge of clock, shift serial data on trailing edge"]
            LEADING = 0x0,
            #[doc = "Sample on trailing edge of clock, shift serial data on leading edge"]
            TRAILING = 0x01,
        }
        impl Cpha {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Cpha {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Cpha {
            #[inline(always)]
            fn from(val: u8) -> Cpha {
                Cpha::from_bits(val)
            }
        }
        impl From<Cpha> for u8 {
            #[inline(always)]
            fn from(val: Cpha) -> u8 {
                Cpha::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Cpol {
            #[doc = "Active high"]
            ACTIVE_HIGH = 0x0,
            #[doc = "Active low"]
            ACTIVE_LOW = 0x01,
        }
        impl Cpol {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Cpol {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Cpol {
            #[inline(always)]
            fn from(val: u8) -> Cpol {
                Cpol::from_bits(val)
            }
        }
        impl From<Cpol> for u8 {
            #[inline(always)]
            fn from(val: Cpol) -> u8 {
                Cpol::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Enable {
            #[doc = "Disable SPI"]
            DISABLED = 0x0,
            #[doc = "Enable SPI"]
            ENABLED = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
            _RESERVED_8 = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl Enable {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Enable {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Enable {
            #[inline(always)]
            fn from(val: u8) -> Enable {
                Enable::from_bits(val)
            }
        }
        impl From<Enable> for u8 {
            #[inline(always)]
            fn from(val: Enable) -> u8 {
                Enable::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Frequency(pub u32);
        impl Frequency {
            #[doc = "125 kbps"]
            pub const K125: Self = Self(0x0200_0000);
            #[doc = "250 kbps"]
            pub const K250: Self = Self(0x0400_0000);
            #[doc = "500 kbps"]
            pub const K500: Self = Self(0x0800_0000);
            #[doc = "1 Mbps"]
            pub const M1: Self = Self(0x1000_0000);
            #[doc = "2 Mbps"]
            pub const M2: Self = Self(0x2000_0000);
            #[doc = "4 Mbps"]
            pub const M4: Self = Self(0x4000_0000);
            #[doc = "8 Mbps"]
            pub const M8: Self = Self(0x8000_0000);
        }
        impl Frequency {
            pub const fn from_bits(val: u32) -> Frequency {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl core::fmt::Debug for Frequency {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0200_0000 => f.write_str("K125"),
                    0x0400_0000 => f.write_str("K250"),
                    0x0800_0000 => f.write_str("K500"),
                    0x1000_0000 => f.write_str("M1"),
                    0x2000_0000 => f.write_str("M2"),
                    0x4000_0000 => f.write_str("M4"),
                    0x8000_0000 => f.write_str("M8"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Frequency {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0200_0000 => defmt::write!(f, "K125"),
                    0x0400_0000 => defmt::write!(f, "K250"),
                    0x0800_0000 => defmt::write!(f, "K500"),
                    0x1000_0000 => defmt::write!(f, "M1"),
                    0x2000_0000 => defmt::write!(f, "M2"),
                    0x4000_0000 => defmt::write!(f, "M4"),
                    0x8000_0000 => defmt::write!(f, "M8"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u32> for Frequency {
            #[inline(always)]
            fn from(val: u32) -> Frequency {
                Frequency::from_bits(val)
            }
        }
        impl From<Frequency> for u32 {
            #[inline(always)]
            fn from(val: Frequency) -> u32 {
                Frequency::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Order {
            #[doc = "Most significant bit shifted out first"]
            MSB_FIRST = 0x0,
            #[doc = "Least significant bit shifted out first"]
            LSB_FIRST = 0x01,
        }
        impl Order {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Order {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Order {
            #[inline(always)]
            fn from(val: u8) -> Order {
                Order::from_bits(val)
            }
        }
        impl From<Order> for u8 {
            #[inline(always)]
            fn from(val: Order) -> u8 {
                Order::to_bits(val)
            }
        }
    }
}
pub mod spim {
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psel {
        ptr: *mut u8,
    }
    unsafe impl Send for Psel {}
    unsafe impl Sync for Psel {}
    impl Psel {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Pin select for SCK"]
        #[inline(always)]
        pub const fn sck(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Pin select for MOSI signal"]
        #[inline(always)]
        pub const fn mosi(
            self,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Pin select for MISO signal"]
        #[inline(always)]
        pub const fn miso(
            self,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
    }
    #[doc = "RXD EasyDMA channel"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxd {
        ptr: *mut u8,
    }
    unsafe impl Send for Rxd {}
    unsafe impl Sync for Rxd {}
    impl Rxd {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Data pointer"]
        #[inline(always)]
        pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Maximum number of bytes in receive buffer"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::RxdMaxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::RxdAmount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "EasyDMA list type"]
        #[inline(always)]
        pub const fn list(self) -> crate::common::Reg<regs::RxdList, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
    }
    #[doc = "Serial Peripheral Interface Master with EasyDMA"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Spim {
        ptr: *mut u8,
    }
    unsafe impl Send for Spim {}
    unsafe impl Sync for Spim {}
    impl Spim {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Start SPI transaction"]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "Stop SPI transaction"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "Suspend SPI transaction"]
        #[inline(always)]
        pub const fn tasks_suspend(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
        #[doc = "Resume SPI transaction"]
        #[inline(always)]
        pub const fn tasks_resume(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }
        #[doc = "SPI transaction has stopped"]
        #[inline(always)]
        pub const fn events_stopped(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "End of RXD buffer reached"]
        #[inline(always)]
        pub const fn events_endrx(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
        }
        #[doc = "End of RXD buffer and TXD buffer reached"]
        #[inline(always)]
        pub const fn events_end(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
        }
        #[doc = "End of TXD buffer reached"]
        #[inline(always)]
        pub const fn events_endtx(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
        }
        #[doc = "Transaction started"]
        #[inline(always)]
        pub const fn events_started(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Enable SPIM"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn psel(self) -> Psel {
            unsafe { Psel::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "SPI frequency. Accuracy depends on the HFCLK source selected."]
        #[inline(always)]
        pub const fn frequency(self) -> crate::common::Reg<regs::Frequency, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
        }
        #[doc = "RXD EasyDMA channel"]
        #[inline(always)]
        pub const fn rxd(self) -> Rxd {
            unsafe { Rxd::from_ptr(self.ptr.add(0x0534usize) as _) }
        }
        #[doc = "TXD EasyDMA channel"]
        #[inline(always)]
        pub const fn txd(self) -> Txd {
            unsafe { Txd::from_ptr(self.ptr.add(0x0544usize) as _) }
        }
        #[doc = "Configuration register"]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0554usize) as _) }
        }
        #[doc = "Over-read character. Character clocked out in case and over-read of the TXD buffer."]
        #[inline(always)]
        pub const fn orc(self) -> crate::common::Reg<regs::Orc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05c0usize) as _) }
        }
    }
    #[doc = "TXD EasyDMA channel"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txd {
        ptr: *mut u8,
    }
    unsafe impl Send for Txd {}
    unsafe impl Sync for Txd {}
    impl Txd {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Data pointer"]
        #[inline(always)]
        pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Maximum number of bytes in transmit buffer"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::TxdMaxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::TxdAmount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "EasyDMA list type"]
        #[inline(always)]
        pub const fn list(self) -> crate::common::Reg<regs::TxdList, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Configuration register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Bit order"]
            #[inline(always)]
            pub const fn order(&self) -> super::vals::Order {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Order::from_bits(val as u8)
            }
            #[doc = "Bit order"]
            #[inline(always)]
            pub fn set_order(&mut self, val: super::vals::Order) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Serial clock (SCK) phase"]
            #[inline(always)]
            pub const fn cpha(&self) -> super::vals::Cpha {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Cpha::from_bits(val as u8)
            }
            #[doc = "Serial clock (SCK) phase"]
            #[inline(always)]
            pub fn set_cpha(&mut self, val: super::vals::Cpha) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Serial clock (SCK) polarity"]
            #[inline(always)]
            pub const fn cpol(&self) -> super::vals::Cpol {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Cpol::from_bits(val as u8)
            }
            #[doc = "Serial clock (SCK) polarity"]
            #[inline(always)]
            pub fn set_cpol(&mut self, val: super::vals::Cpol) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
            }
        }
        impl Default for Config {
            #[inline(always)]
            fn default() -> Config {
                Config(0)
            }
        }
        impl core::fmt::Debug for Config {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Config")
                    .field("order", &self.order())
                    .field("cpha", &self.cpha())
                    .field("cpol", &self.cpol())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Config {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Config {
                    order: super::vals::Order,
                    cpha: super::vals::Cpha,
                    cpol: super::vals::Cpol,
                }
                let proxy = Config {
                    order: self.order(),
                    cpha: self.cpha(),
                    cpol: self.cpol(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Enable SPIM"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable SPIM"]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable SPIM"]
            #[inline(always)]
            pub fn set_enable(&mut self, val: super::vals::Enable) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        impl core::fmt::Debug for Enable {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Enable")
                    .field("enable", &self.enable())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Enable {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Enable {
                    enable: super::vals::Enable,
                }
                let proxy = Enable {
                    enable: self.enable(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "SPI frequency. Accuracy depends on the HFCLK source selected."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Frequency(pub u32);
        impl Frequency {
            #[doc = "SPI master data rate"]
            #[inline(always)]
            pub const fn frequency(&self) -> super::vals::Frequency {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Frequency::from_bits(val as u32)
            }
            #[doc = "SPI master data rate"]
            #[inline(always)]
            pub fn set_frequency(&mut self, val: super::vals::Frequency) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize))
                    | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Frequency {
            #[inline(always)]
            fn default() -> Frequency {
                Frequency(0)
            }
        }
        impl core::fmt::Debug for Frequency {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Frequency")
                    .field("frequency", &self.frequency())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Frequency {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Frequency {
                    frequency: super::vals::Frequency,
                }
                let proxy = Frequency {
                    frequency: self.frequency(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Write '1' to disable interrupt for event STOPPED"]
            #[inline(always)]
            pub const fn stopped(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event STOPPED"]
            #[inline(always)]
            pub fn set_stopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to disable interrupt for event ENDRX"]
            #[inline(always)]
            pub const fn endrx(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event ENDRX"]
            #[inline(always)]
            pub fn set_endrx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Write '1' to disable interrupt for event END"]
            #[inline(always)]
            pub const fn end(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event END"]
            #[inline(always)]
            pub fn set_end(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Write '1' to disable interrupt for event ENDTX"]
            #[inline(always)]
            pub const fn endtx(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event ENDTX"]
            #[inline(always)]
            pub fn set_endtx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Write '1' to disable interrupt for event STARTED"]
            #[inline(always)]
            pub const fn started(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event STARTED"]
            #[inline(always)]
            pub fn set_started(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("stopped", &self.stopped())
                    .field("endrx", &self.endrx())
                    .field("end", &self.end())
                    .field("endtx", &self.endtx())
                    .field("started", &self.started())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    stopped: bool,
                    endrx: bool,
                    end: bool,
                    endtx: bool,
                    started: bool,
                }
                let proxy = Int {
                    stopped: self.stopped(),
                    endrx: self.endrx(),
                    end: self.end(),
                    endtx: self.endtx(),
                    started: self.started(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Over-read character. Character clocked out in case and over-read of the TXD buffer."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Orc(pub u32);
        impl Orc {
            #[doc = "Over-read character. Character clocked out in case and over-read of the TXD buffer."]
            #[inline(always)]
            pub const fn orc(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Over-read character. Character clocked out in case and over-read of the TXD buffer."]
            #[inline(always)]
            pub fn set_orc(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Orc {
            #[inline(always)]
            fn default() -> Orc {
                Orc(0)
            }
        }
        impl core::fmt::Debug for Orc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Orc").field("orc", &self.orc()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Orc {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Orc {
                    orc: u8,
                }
                let proxy = Orc { orc: self.orc() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdAmount(pub u32);
        impl RxdAmount {
            #[doc = "Number of bytes transferred in the last transaction"]
            #[inline(always)]
            pub const fn amount(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "Number of bytes transferred in the last transaction"]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for RxdAmount {
            #[inline(always)]
            fn default() -> RxdAmount {
                RxdAmount(0)
            }
        }
        impl core::fmt::Debug for RxdAmount {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RxdAmount")
                    .field("amount", &self.amount())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RxdAmount {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct RxdAmount {
                    amount: u16,
                }
                let proxy = RxdAmount {
                    amount: self.amount(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "EasyDMA list type"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdList(pub u32);
        impl RxdList {
            #[doc = "List type"]
            #[inline(always)]
            pub const fn list(&self) -> super::vals::RxdListList {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::RxdListList::from_bits(val as u8)
            }
            #[doc = "List type"]
            #[inline(always)]
            pub fn set_list(&mut self, val: super::vals::RxdListList) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for RxdList {
            #[inline(always)]
            fn default() -> RxdList {
                RxdList(0)
            }
        }
        impl core::fmt::Debug for RxdList {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RxdList")
                    .field("list", &self.list())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RxdList {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct RxdList {
                    list: super::vals::RxdListList,
                }
                let proxy = RxdList { list: self.list() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Maximum number of bytes in receive buffer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdMaxcnt(pub u32);
        impl RxdMaxcnt {
            #[doc = "Maximum number of bytes in receive buffer"]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "Maximum number of bytes in receive buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for RxdMaxcnt {
            #[inline(always)]
            fn default() -> RxdMaxcnt {
                RxdMaxcnt(0)
            }
        }
        impl core::fmt::Debug for RxdMaxcnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RxdMaxcnt")
                    .field("maxcnt", &self.maxcnt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RxdMaxcnt {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct RxdMaxcnt {
                    maxcnt: u16,
                }
                let proxy = RxdMaxcnt {
                    maxcnt: self.maxcnt(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event END and task START"]
            #[inline(always)]
            pub const fn end_start(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event END and task START"]
            #[inline(always)]
            pub fn set_end_start(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
        impl core::fmt::Debug for Shorts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Shorts")
                    .field("end_start", &self.end_start())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Shorts {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Shorts {
                    end_start: bool,
                }
                let proxy = Shorts {
                    end_start: self.end_start(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdAmount(pub u32);
        impl TxdAmount {
            #[doc = "Number of bytes transferred in the last transaction"]
            #[inline(always)]
            pub const fn amount(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "Number of bytes transferred in the last transaction"]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for TxdAmount {
            #[inline(always)]
            fn default() -> TxdAmount {
                TxdAmount(0)
            }
        }
        impl core::fmt::Debug for TxdAmount {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("TxdAmount")
                    .field("amount", &self.amount())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for TxdAmount {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct TxdAmount {
                    amount: u16,
                }
                let proxy = TxdAmount {
                    amount: self.amount(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "EasyDMA list type"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdList(pub u32);
        impl TxdList {
            #[doc = "List type"]
            #[inline(always)]
            pub const fn list(&self) -> super::vals::TxdListList {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::TxdListList::from_bits(val as u8)
            }
            #[doc = "List type"]
            #[inline(always)]
            pub fn set_list(&mut self, val: super::vals::TxdListList) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for TxdList {
            #[inline(always)]
            fn default() -> TxdList {
                TxdList(0)
            }
        }
        impl core::fmt::Debug for TxdList {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("TxdList")
                    .field("list", &self.list())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for TxdList {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct TxdList {
                    list: super::vals::TxdListList,
                }
                let proxy = TxdList { list: self.list() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Maximum number of bytes in transmit buffer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdMaxcnt(pub u32);
        impl TxdMaxcnt {
            #[doc = "Maximum number of bytes in transmit buffer"]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "Maximum number of bytes in transmit buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for TxdMaxcnt {
            #[inline(always)]
            fn default() -> TxdMaxcnt {
                TxdMaxcnt(0)
            }
        }
        impl core::fmt::Debug for TxdMaxcnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("TxdMaxcnt")
                    .field("maxcnt", &self.maxcnt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for TxdMaxcnt {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct TxdMaxcnt {
                    maxcnt: u16,
                }
                let proxy = TxdMaxcnt {
                    maxcnt: self.maxcnt(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Cpha {
            #[doc = "Sample on leading edge of clock, shift serial data on trailing edge"]
            LEADING = 0x0,
            #[doc = "Sample on trailing edge of clock, shift serial data on leading edge"]
            TRAILING = 0x01,
        }
        impl Cpha {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Cpha {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Cpha {
            #[inline(always)]
            fn from(val: u8) -> Cpha {
                Cpha::from_bits(val)
            }
        }
        impl From<Cpha> for u8 {
            #[inline(always)]
            fn from(val: Cpha) -> u8 {
                Cpha::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Cpol {
            #[doc = "Active high"]
            ACTIVE_HIGH = 0x0,
            #[doc = "Active low"]
            ACTIVE_LOW = 0x01,
        }
        impl Cpol {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Cpol {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Cpol {
            #[inline(always)]
            fn from(val: u8) -> Cpol {
                Cpol::from_bits(val)
            }
        }
        impl From<Cpol> for u8 {
            #[inline(always)]
            fn from(val: Cpol) -> u8 {
                Cpol::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Enable {
            #[doc = "Disable SPIM"]
            DISABLED = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            #[doc = "Enable SPIM"]
            ENABLED = 0x07,
            _RESERVED_8 = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl Enable {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Enable {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Enable {
            #[inline(always)]
            fn from(val: u8) -> Enable {
                Enable::from_bits(val)
            }
        }
        impl From<Enable> for u8 {
            #[inline(always)]
            fn from(val: Enable) -> u8 {
                Enable::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Frequency(pub u32);
        impl Frequency {
            #[doc = "125 kbps"]
            pub const K125: Self = Self(0x0200_0000);
            #[doc = "250 kbps"]
            pub const K250: Self = Self(0x0400_0000);
            #[doc = "500 kbps"]
            pub const K500: Self = Self(0x0800_0000);
            #[doc = "1 Mbps"]
            pub const M1: Self = Self(0x1000_0000);
            #[doc = "2 Mbps"]
            pub const M2: Self = Self(0x2000_0000);
            #[doc = "4 Mbps"]
            pub const M4: Self = Self(0x4000_0000);
            #[doc = "8 Mbps"]
            pub const M8: Self = Self(0x8000_0000);
        }
        impl Frequency {
            pub const fn from_bits(val: u32) -> Frequency {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl core::fmt::Debug for Frequency {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0200_0000 => f.write_str("K125"),
                    0x0400_0000 => f.write_str("K250"),
                    0x0800_0000 => f.write_str("K500"),
                    0x1000_0000 => f.write_str("M1"),
                    0x2000_0000 => f.write_str("M2"),
                    0x4000_0000 => f.write_str("M4"),
                    0x8000_0000 => f.write_str("M8"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Frequency {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0200_0000 => defmt::write!(f, "K125"),
                    0x0400_0000 => defmt::write!(f, "K250"),
                    0x0800_0000 => defmt::write!(f, "K500"),
                    0x1000_0000 => defmt::write!(f, "M1"),
                    0x2000_0000 => defmt::write!(f, "M2"),
                    0x4000_0000 => defmt::write!(f, "M4"),
                    0x8000_0000 => defmt::write!(f, "M8"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u32> for Frequency {
            #[inline(always)]
            fn from(val: u32) -> Frequency {
                Frequency::from_bits(val)
            }
        }
        impl From<Frequency> for u32 {
            #[inline(always)]
            fn from(val: Frequency) -> u32 {
                Frequency::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Order {
            #[doc = "Most significant bit shifted out first"]
            MSB_FIRST = 0x0,
            #[doc = "Least significant bit shifted out first"]
            LSB_FIRST = 0x01,
        }
        impl Order {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Order {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Order {
            #[inline(always)]
            fn from(val: u8) -> Order {
                Order::from_bits(val)
            }
        }
        impl From<Order> for u8 {
            #[inline(always)]
            fn from(val: Order) -> u8 {
                Order::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum RxdListList {
            #[doc = "Disable EasyDMA list"]
            DISABLED = 0x0,
            #[doc = "Use array list"]
            ARRAY_LIST = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl RxdListList {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> RxdListList {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for RxdListList {
            #[inline(always)]
            fn from(val: u8) -> RxdListList {
                RxdListList::from_bits(val)
            }
        }
        impl From<RxdListList> for u8 {
            #[inline(always)]
            fn from(val: RxdListList) -> u8 {
                RxdListList::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum TxdListList {
            #[doc = "Disable EasyDMA list"]
            DISABLED = 0x0,
            #[doc = "Use array list"]
            ARRAY_LIST = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl TxdListList {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> TxdListList {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for TxdListList {
            #[inline(always)]
            fn from(val: u8) -> TxdListList {
                TxdListList::from_bits(val)
            }
        }
        impl From<TxdListList> for u8 {
            #[inline(always)]
            fn from(val: TxdListList) -> u8 {
                TxdListList::to_bits(val)
            }
        }
    }
}
pub mod spis {
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psel {
        ptr: *mut u8,
    }
    unsafe impl Send for Psel {}
    unsafe impl Sync for Psel {}
    impl Psel {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Pin select for SCK"]
        #[inline(always)]
        pub const fn sck(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Pin select for MISO signal"]
        #[inline(always)]
        pub const fn miso(
            self,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Pin select for MOSI signal"]
        #[inline(always)]
        pub const fn mosi(
            self,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Pin select for CSN signal"]
        #[inline(always)]
        pub const fn csn(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
    }
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxd {
        ptr: *mut u8,
    }
    unsafe impl Send for Rxd {}
    unsafe impl Sync for Rxd {}
    impl Rxd {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "RXD data pointer"]
        #[inline(always)]
        pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Maximum number of bytes in receive buffer"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::RxdMaxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Number of bytes received in last granted transaction"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::RxdAmount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "EasyDMA list type"]
        #[inline(always)]
        pub const fn list(self) -> crate::common::Reg<regs::RxdList, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
    }
    #[doc = "SPI Slave"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Spis {
        ptr: *mut u8,
    }
    unsafe impl Send for Spis {}
    unsafe impl Sync for Spis {}
    impl Spis {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Acquire SPI semaphore"]
        #[inline(always)]
        pub const fn tasks_acquire(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
        }
        #[doc = "Release SPI semaphore, enabling the SPI slave to acquire it"]
        #[inline(always)]
        pub const fn tasks_release(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
        }
        #[doc = "Granted transaction completed"]
        #[inline(always)]
        pub const fn events_end(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "End of RXD buffer reached"]
        #[inline(always)]
        pub const fn events_endrx(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
        }
        #[doc = "Semaphore acquired"]
        #[inline(always)]
        pub const fn events_acquired(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Semaphore status register"]
        #[inline(always)]
        pub const fn semstat(self) -> crate::common::Reg<regs::Semstat, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
        }
        #[doc = "Status from last transaction"]
        #[inline(always)]
        pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0440usize) as _) }
        }
        #[doc = "Enable SPI slave"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn psel(self) -> Psel {
            unsafe { Psel::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn rxd(self) -> Rxd {
            unsafe { Rxd::from_ptr(self.ptr.add(0x0534usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn txd(self) -> Txd {
            unsafe { Txd::from_ptr(self.ptr.add(0x0544usize) as _) }
        }
        #[doc = "Configuration register"]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0554usize) as _) }
        }
        #[doc = "Default character. Character clocked out in case of an ignored transaction."]
        #[inline(always)]
        pub const fn def(self) -> crate::common::Reg<regs::Def, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x055cusize) as _) }
        }
        #[doc = "Over-read character"]
        #[inline(always)]
        pub const fn orc(self) -> crate::common::Reg<regs::Orc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05c0usize) as _) }
        }
    }
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txd {
        ptr: *mut u8,
    }
    unsafe impl Send for Txd {}
    unsafe impl Sync for Txd {}
    impl Txd {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "TXD data pointer"]
        #[inline(always)]
        pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Maximum number of bytes in transmit buffer"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::TxdMaxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Number of bytes transmitted in last granted transaction"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::TxdAmount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "EasyDMA list type"]
        #[inline(always)]
        pub const fn list(self) -> crate::common::Reg<regs::TxdList, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Configuration register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Bit order"]
            #[inline(always)]
            pub const fn order(&self) -> super::vals::Order {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Order::from_bits(val as u8)
            }
            #[doc = "Bit order"]
            #[inline(always)]
            pub fn set_order(&mut self, val: super::vals::Order) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Serial clock (SCK) phase"]
            #[inline(always)]
            pub const fn cpha(&self) -> super::vals::Cpha {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Cpha::from_bits(val as u8)
            }
            #[doc = "Serial clock (SCK) phase"]
            #[inline(always)]
            pub fn set_cpha(&mut self, val: super::vals::Cpha) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
            }
            #[doc = "Serial clock (SCK) polarity"]
            #[inline(always)]
            pub const fn cpol(&self) -> super::vals::Cpol {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Cpol::from_bits(val as u8)
            }
            #[doc = "Serial clock (SCK) polarity"]
            #[inline(always)]
            pub fn set_cpol(&mut self, val: super::vals::Cpol) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
            }
        }
        impl Default for Config {
            #[inline(always)]
            fn default() -> Config {
                Config(0)
            }
        }
        impl core::fmt::Debug for Config {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Config")
                    .field("order", &self.order())
                    .field("cpha", &self.cpha())
                    .field("cpol", &self.cpol())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Config {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Config {
                    order: super::vals::Order,
                    cpha: super::vals::Cpha,
                    cpol: super::vals::Cpol,
                }
                let proxy = Config {
                    order: self.order(),
                    cpha: self.cpha(),
                    cpol: self.cpol(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Default character. Character clocked out in case of an ignored transaction."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Def(pub u32);
        impl Def {
            #[doc = "Default character. Character clocked out in case of an ignored transaction."]
            #[inline(always)]
            pub const fn def(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Default character. Character clocked out in case of an ignored transaction."]
            #[inline(always)]
            pub fn set_def(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Def {
            #[inline(always)]
            fn default() -> Def {
                Def(0)
            }
        }
        impl core::fmt::Debug for Def {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Def").field("def", &self.def()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Def {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Def {
                    def: u8,
                }
                let proxy = Def { def: self.def() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Enable SPI slave"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable SPI slave"]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable SPI slave"]
            #[inline(always)]
            pub fn set_enable(&mut self, val: super::vals::Enable) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        impl core::fmt::Debug for Enable {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Enable")
                    .field("enable", &self.enable())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Enable {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Enable {
                    enable: super::vals::Enable,
                }
                let proxy = Enable {
                    enable: self.enable(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Write '1' to disable interrupt for event END"]
            #[inline(always)]
            pub const fn end(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event END"]
            #[inline(always)]
            pub fn set_end(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to disable interrupt for event ENDRX"]
            #[inline(always)]
            pub const fn endrx(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event ENDRX"]
            #[inline(always)]
            pub fn set_endrx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Write '1' to disable interrupt for event ACQUIRED"]
            #[inline(always)]
            pub const fn acquired(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event ACQUIRED"]
            #[inline(always)]
            pub fn set_acquired(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("end", &self.end())
                    .field("endrx", &self.endrx())
                    .field("acquired", &self.acquired())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    end: bool,
                    endrx: bool,
                    acquired: bool,
                }
                let proxy = Int {
                    end: self.end(),
                    endrx: self.endrx(),
                    acquired: self.acquired(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Over-read character"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Orc(pub u32);
        impl Orc {
            #[doc = "Over-read character. Character clocked out after an over-read of the transmit buffer."]
            #[inline(always)]
            pub const fn orc(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Over-read character. Character clocked out after an over-read of the transmit buffer."]
            #[inline(always)]
            pub fn set_orc(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Orc {
            #[inline(always)]
            fn default() -> Orc {
                Orc(0)
            }
        }
        impl core::fmt::Debug for Orc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Orc").field("orc", &self.orc()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Orc {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Orc {
                    orc: u8,
                }
                let proxy = Orc { orc: self.orc() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Number of bytes received in last granted transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdAmount(pub u32);
        impl RxdAmount {
            #[doc = "Number of bytes received in the last granted transaction"]
            #[inline(always)]
            pub const fn amount(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "Number of bytes received in the last granted transaction"]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for RxdAmount {
            #[inline(always)]
            fn default() -> RxdAmount {
                RxdAmount(0)
            }
        }
        impl core::fmt::Debug for RxdAmount {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RxdAmount")
                    .field("amount", &self.amount())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RxdAmount {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct RxdAmount {
                    amount: u16,
                }
                let proxy = RxdAmount {
                    amount: self.amount(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "EasyDMA list type"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdList(pub u32);
        impl RxdList {
            #[doc = "List type"]
            #[inline(always)]
            pub const fn list(&self) -> super::vals::RxdListList {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::RxdListList::from_bits(val as u8)
            }
            #[doc = "List type"]
            #[inline(always)]
            pub fn set_list(&mut self, val: super::vals::RxdListList) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for RxdList {
            #[inline(always)]
            fn default() -> RxdList {
                RxdList(0)
            }
        }
        impl core::fmt::Debug for RxdList {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RxdList")
                    .field("list", &self.list())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RxdList {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct RxdList {
                    list: super::vals::RxdListList,
                }
                let proxy = RxdList { list: self.list() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Maximum number of bytes in receive buffer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdMaxcnt(pub u32);
        impl RxdMaxcnt {
            #[doc = "Maximum number of bytes in receive buffer"]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "Maximum number of bytes in receive buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for RxdMaxcnt {
            #[inline(always)]
            fn default() -> RxdMaxcnt {
                RxdMaxcnt(0)
            }
        }
        impl core::fmt::Debug for RxdMaxcnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RxdMaxcnt")
                    .field("maxcnt", &self.maxcnt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RxdMaxcnt {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct RxdMaxcnt {
                    maxcnt: u16,
                }
                let proxy = RxdMaxcnt {
                    maxcnt: self.maxcnt(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Semaphore status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Semstat(pub u32);
        impl Semstat {
            #[doc = "Semaphore status"]
            #[inline(always)]
            pub const fn semstat(&self) -> super::vals::Semstat {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Semstat::from_bits(val as u8)
            }
            #[doc = "Semaphore status"]
            #[inline(always)]
            pub fn set_semstat(&mut self, val: super::vals::Semstat) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Semstat {
            #[inline(always)]
            fn default() -> Semstat {
                Semstat(0)
            }
        }
        impl core::fmt::Debug for Semstat {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Semstat")
                    .field("semstat", &self.semstat())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Semstat {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Semstat {
                    semstat: super::vals::Semstat,
                }
                let proxy = Semstat {
                    semstat: self.semstat(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event END and task ACQUIRE"]
            #[inline(always)]
            pub const fn end_acquire(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event END and task ACQUIRE"]
            #[inline(always)]
            pub fn set_end_acquire(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
        impl core::fmt::Debug for Shorts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Shorts")
                    .field("end_acquire", &self.end_acquire())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Shorts {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Shorts {
                    end_acquire: bool,
                }
                let proxy = Shorts {
                    end_acquire: self.end_acquire(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Status from last transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Status(pub u32);
        impl Status {
            #[doc = "TX buffer over-read detected, and prevented"]
            #[inline(always)]
            pub const fn overread(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "TX buffer over-read detected, and prevented"]
            #[inline(always)]
            pub fn set_overread(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "RX buffer overflow detected, and prevented"]
            #[inline(always)]
            pub const fn overflow(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "RX buffer overflow detected, and prevented"]
            #[inline(always)]
            pub fn set_overflow(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
        }
        impl Default for Status {
            #[inline(always)]
            fn default() -> Status {
                Status(0)
            }
        }
        impl core::fmt::Debug for Status {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Status")
                    .field("overread", &self.overread())
                    .field("overflow", &self.overflow())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Status {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Status {
                    overread: bool,
                    overflow: bool,
                }
                let proxy = Status {
                    overread: self.overread(),
                    overflow: self.overflow(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Number of bytes transmitted in last granted transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdAmount(pub u32);
        impl TxdAmount {
            #[doc = "Number of bytes transmitted in last granted transaction"]
            #[inline(always)]
            pub const fn amount(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "Number of bytes transmitted in last granted transaction"]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for TxdAmount {
            #[inline(always)]
            fn default() -> TxdAmount {
                TxdAmount(0)
            }
        }
        impl core::fmt::Debug for TxdAmount {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("TxdAmount")
                    .field("amount", &self.amount())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for TxdAmount {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct TxdAmount {
                    amount: u16,
                }
                let proxy = TxdAmount {
                    amount: self.amount(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "EasyDMA list type"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdList(pub u32);
        impl TxdList {
            #[doc = "List type"]
            #[inline(always)]
            pub const fn list(&self) -> super::vals::TxdListList {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::TxdListList::from_bits(val as u8)
            }
            #[doc = "List type"]
            #[inline(always)]
            pub fn set_list(&mut self, val: super::vals::TxdListList) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for TxdList {
            #[inline(always)]
            fn default() -> TxdList {
                TxdList(0)
            }
        }
        impl core::fmt::Debug for TxdList {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("TxdList")
                    .field("list", &self.list())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for TxdList {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct TxdList {
                    list: super::vals::TxdListList,
                }
                let proxy = TxdList { list: self.list() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Maximum number of bytes in transmit buffer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdMaxcnt(pub u32);
        impl TxdMaxcnt {
            #[doc = "Maximum number of bytes in transmit buffer"]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "Maximum number of bytes in transmit buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for TxdMaxcnt {
            #[inline(always)]
            fn default() -> TxdMaxcnt {
                TxdMaxcnt(0)
            }
        }
        impl core::fmt::Debug for TxdMaxcnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("TxdMaxcnt")
                    .field("maxcnt", &self.maxcnt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for TxdMaxcnt {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct TxdMaxcnt {
                    maxcnt: u16,
                }
                let proxy = TxdMaxcnt {
                    maxcnt: self.maxcnt(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Cpha {
            #[doc = "Sample on leading edge of clock, shift serial data on trailing edge"]
            LEADING = 0x0,
            #[doc = "Sample on trailing edge of clock, shift serial data on leading edge"]
            TRAILING = 0x01,
        }
        impl Cpha {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Cpha {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Cpha {
            #[inline(always)]
            fn from(val: u8) -> Cpha {
                Cpha::from_bits(val)
            }
        }
        impl From<Cpha> for u8 {
            #[inline(always)]
            fn from(val: Cpha) -> u8 {
                Cpha::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Cpol {
            #[doc = "Active high"]
            ACTIVE_HIGH = 0x0,
            #[doc = "Active low"]
            ACTIVE_LOW = 0x01,
        }
        impl Cpol {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Cpol {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Cpol {
            #[inline(always)]
            fn from(val: u8) -> Cpol {
                Cpol::from_bits(val)
            }
        }
        impl From<Cpol> for u8 {
            #[inline(always)]
            fn from(val: Cpol) -> u8 {
                Cpol::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Enable {
            #[doc = "Disable SPI slave"]
            DISABLED = 0x0,
            _RESERVED_1 = 0x01,
            #[doc = "Enable SPI slave"]
            ENABLED = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
            _RESERVED_8 = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl Enable {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Enable {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Enable {
            #[inline(always)]
            fn from(val: u8) -> Enable {
                Enable::from_bits(val)
            }
        }
        impl From<Enable> for u8 {
            #[inline(always)]
            fn from(val: Enable) -> u8 {
                Enable::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Order {
            #[doc = "Most significant bit shifted out first"]
            MSB_FIRST = 0x0,
            #[doc = "Least significant bit shifted out first"]
            LSB_FIRST = 0x01,
        }
        impl Order {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Order {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Order {
            #[inline(always)]
            fn from(val: u8) -> Order {
                Order::from_bits(val)
            }
        }
        impl From<Order> for u8 {
            #[inline(always)]
            fn from(val: Order) -> u8 {
                Order::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum RxdListList {
            #[doc = "Disable EasyDMA list"]
            DISABLED = 0x0,
            #[doc = "Use array list"]
            ARRAY_LIST = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl RxdListList {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> RxdListList {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for RxdListList {
            #[inline(always)]
            fn from(val: u8) -> RxdListList {
                RxdListList::from_bits(val)
            }
        }
        impl From<RxdListList> for u8 {
            #[inline(always)]
            fn from(val: RxdListList) -> u8 {
                RxdListList::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Semstat {
            #[doc = "Semaphore is free"]
            FREE = 0x0,
            #[doc = "Semaphore is assigned to CPU"]
            CPU = 0x01,
            #[doc = "Semaphore is assigned to SPI slave"]
            SPIS = 0x02,
            #[doc = "Semaphore is assigned to SPI but a handover to the CPU is pending"]
            CPUPENDING = 0x03,
        }
        impl Semstat {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Semstat {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Semstat {
            #[inline(always)]
            fn from(val: u8) -> Semstat {
                Semstat::from_bits(val)
            }
        }
        impl From<Semstat> for u8 {
            #[inline(always)]
            fn from(val: Semstat) -> u8 {
                Semstat::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum TxdListList {
            #[doc = "Disable EasyDMA list"]
            DISABLED = 0x0,
            #[doc = "Use array list"]
            ARRAY_LIST = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl TxdListList {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> TxdListList {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for TxdListList {
            #[inline(always)]
            fn from(val: u8) -> TxdListList {
                TxdListList::from_bits(val)
            }
        }
        impl From<TxdListList> for u8 {
            #[inline(always)]
            fn from(val: TxdListList) -> u8 {
                TxdListList::to_bits(val)
            }
        }
    }
}
pub mod swi {
    #[doc = "Software interrupt 0"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swi {
        ptr: *mut u8,
    }
    unsafe impl Send for Swi {}
    unsafe impl Sync for Swi {}
    impl Swi {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Unused."]
        #[inline(always)]
        pub const fn unused(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
    }
}
pub mod temp {
    #[doc = "Temperature Sensor"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Temp {
        ptr: *mut u8,
    }
    unsafe impl Send for Temp {}
    unsafe impl Sync for Temp {}
    impl Temp {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Start temperature measurement"]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Stop temperature measurement"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Temperature measurement complete, data ready"]
        #[inline(always)]
        pub const fn events_datardy(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Temperature in degC (0.25deg steps)"]
        #[inline(always)]
        pub const fn temp(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Slope of first piecewise linear function"]
        #[inline(always)]
        pub const fn a(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 6usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0520usize + n * 4usize) as _) }
        }
        #[doc = "y-intercept of first piecewise linear function"]
        #[inline(always)]
        pub const fn b(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 6usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0540usize + n * 4usize) as _) }
        }
        #[doc = "End point of first piecewise linear function"]
        #[inline(always)]
        pub const fn t(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 5usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0560usize + n * 4usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Write '1' to disable interrupt for event DATARDY"]
            #[inline(always)]
            pub const fn datardy(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event DATARDY"]
            #[inline(always)]
            pub fn set_datardy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("datardy", &self.datardy())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    datardy: bool,
                }
                let proxy = Int {
                    datardy: self.datardy(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
}
pub mod timer {
    #[doc = "Timer/Counter 0"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer {
        ptr: *mut u8,
    }
    unsafe impl Send for Timer {}
    unsafe impl Sync for Timer {}
    impl Timer {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Start Timer"]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Stop Timer"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Increment Timer (Counter mode only)"]
        #[inline(always)]
        pub const fn tasks_count(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Clear time"]
        #[inline(always)]
        pub const fn tasks_clear(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Deprecated register - Shut down timer"]
        #[inline(always)]
        pub const fn tasks_shutdown(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "Description collection: Capture Timer value to CC\\[n\\] register"]
        #[inline(always)]
        pub const fn tasks_capture(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
            assert!(n < 6usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Compare event on CC\\[n\\] match"]
        #[inline(always)]
        pub const fn events_compare(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 6usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize + n * 4usize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Timer mode selection"]
        #[inline(always)]
        pub const fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Configure the number of bits used by the TIMER"]
        #[inline(always)]
        pub const fn bitmode(self) -> crate::common::Reg<regs::Bitmode, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Timer prescaler register"]
        #[inline(always)]
        pub const fn prescaler(self) -> crate::common::Reg<regs::Prescaler, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
        }
        #[doc = "Description collection: Capture/Compare register n"]
        #[inline(always)]
        pub const fn cc(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 6usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0540usize + n * 4usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Configure the number of bits used by the TIMER"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Bitmode(pub u32);
        impl Bitmode {
            #[doc = "Timer bit width"]
            #[inline(always)]
            pub const fn bitmode(&self) -> super::vals::Bitmode {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Bitmode::from_bits(val as u8)
            }
            #[doc = "Timer bit width"]
            #[inline(always)]
            pub fn set_bitmode(&mut self, val: super::vals::Bitmode) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Bitmode {
            #[inline(always)]
            fn default() -> Bitmode {
                Bitmode(0)
            }
        }
        impl core::fmt::Debug for Bitmode {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Bitmode")
                    .field("bitmode", &self.bitmode())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Bitmode {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Bitmode {
                    bitmode: super::vals::Bitmode,
                }
                let proxy = Bitmode {
                    bitmode: self.bitmode(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Write '1' to disable interrupt for event COMPARE\\[0\\]"]
            #[inline(always)]
            pub const fn compare(&self, n: usize) -> bool {
                assert!(n < 6usize);
                let offs = 16usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event COMPARE\\[0\\]"]
            #[inline(always)]
            pub fn set_compare(&mut self, n: usize, val: bool) {
                assert!(n < 6usize);
                let offs = 16usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field(
                        "compare",
                        &[
                            self.compare(0usize),
                            self.compare(1usize),
                            self.compare(2usize),
                            self.compare(3usize),
                            self.compare(4usize),
                            self.compare(5usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    compare: [bool; 6usize],
                }
                let proxy = Int {
                    compare: [
                        self.compare(0usize),
                        self.compare(1usize),
                        self.compare(2usize),
                        self.compare(3usize),
                        self.compare(4usize),
                        self.compare(5usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Timer mode selection"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Mode(pub u32);
        impl Mode {
            #[doc = "Timer mode"]
            #[inline(always)]
            pub const fn mode(&self) -> super::vals::Mode {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Mode::from_bits(val as u8)
            }
            #[doc = "Timer mode"]
            #[inline(always)]
            pub fn set_mode(&mut self, val: super::vals::Mode) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for Mode {
            #[inline(always)]
            fn default() -> Mode {
                Mode(0)
            }
        }
        impl core::fmt::Debug for Mode {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Mode").field("mode", &self.mode()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Mode {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Mode {
                    mode: super::vals::Mode,
                }
                let proxy = Mode { mode: self.mode() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Timer prescaler register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Prescaler(pub u32);
        impl Prescaler {
            #[doc = "Prescaler value"]
            #[inline(always)]
            pub const fn prescaler(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "Prescaler value"]
            #[inline(always)]
            pub fn set_prescaler(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Prescaler {
            #[inline(always)]
            fn default() -> Prescaler {
                Prescaler(0)
            }
        }
        impl core::fmt::Debug for Prescaler {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Prescaler")
                    .field("prescaler", &self.prescaler())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Prescaler {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Prescaler {
                    prescaler: u8,
                }
                let proxy = Prescaler {
                    prescaler: self.prescaler(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event COMPARE\\[0\\] and task CLEAR"]
            #[inline(always)]
            pub const fn compare_clear(&self, n: usize) -> bool {
                assert!(n < 6usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event COMPARE\\[0\\] and task CLEAR"]
            #[inline(always)]
            pub fn set_compare_clear(&mut self, n: usize, val: bool) {
                assert!(n < 6usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Shortcut between event COMPARE\\[0\\] and task STOP"]
            #[inline(always)]
            pub const fn compare_stop(&self, n: usize) -> bool {
                assert!(n < 6usize);
                let offs = 8usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event COMPARE\\[0\\] and task STOP"]
            #[inline(always)]
            pub fn set_compare_stop(&mut self, n: usize, val: bool) {
                assert!(n < 6usize);
                let offs = 8usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
        impl core::fmt::Debug for Shorts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Shorts")
                    .field(
                        "compare_clear",
                        &[
                            self.compare_clear(0usize),
                            self.compare_clear(1usize),
                            self.compare_clear(2usize),
                            self.compare_clear(3usize),
                            self.compare_clear(4usize),
                            self.compare_clear(5usize),
                        ],
                    )
                    .field(
                        "compare_stop",
                        &[
                            self.compare_stop(0usize),
                            self.compare_stop(1usize),
                            self.compare_stop(2usize),
                            self.compare_stop(3usize),
                            self.compare_stop(4usize),
                            self.compare_stop(5usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Shorts {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Shorts {
                    compare_clear: [bool; 6usize],
                    compare_stop: [bool; 6usize],
                }
                let proxy = Shorts {
                    compare_clear: [
                        self.compare_clear(0usize),
                        self.compare_clear(1usize),
                        self.compare_clear(2usize),
                        self.compare_clear(3usize),
                        self.compare_clear(4usize),
                        self.compare_clear(5usize),
                    ],
                    compare_stop: [
                        self.compare_stop(0usize),
                        self.compare_stop(1usize),
                        self.compare_stop(2usize),
                        self.compare_stop(3usize),
                        self.compare_stop(4usize),
                        self.compare_stop(5usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Bitmode {
            #[doc = "16 bit timer bit width"]
            _16BIT = 0x0,
            #[doc = "8 bit timer bit width"]
            _08BIT = 0x01,
            #[doc = "24 bit timer bit width"]
            _24BIT = 0x02,
            #[doc = "32 bit timer bit width"]
            _32BIT = 0x03,
        }
        impl Bitmode {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Bitmode {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Bitmode {
            #[inline(always)]
            fn from(val: u8) -> Bitmode {
                Bitmode::from_bits(val)
            }
        }
        impl From<Bitmode> for u8 {
            #[inline(always)]
            fn from(val: Bitmode) -> u8 {
                Bitmode::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Mode {
            #[doc = "Select Timer mode"]
            TIMER = 0x0,
            #[doc = "Deprecated enumerator - Select Counter mode"]
            COUNTER = 0x01,
            #[doc = "Select Low Power Counter mode"]
            LOW_POWER_COUNTER = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl Mode {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Mode {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Mode {
            #[inline(always)]
            fn from(val: u8) -> Mode {
                Mode::from_bits(val)
            }
        }
        impl From<Mode> for u8 {
            #[inline(always)]
            fn from(val: Mode) -> u8 {
                Mode::to_bits(val)
            }
        }
    }
}
pub mod twi {
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psel {
        ptr: *mut u8,
    }
    unsafe impl Send for Psel {}
    unsafe impl Sync for Psel {}
    impl Psel {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Pin select for SCL"]
        #[inline(always)]
        pub const fn scl(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Pin select for SDA"]
        #[inline(always)]
        pub const fn sda(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
    }
    #[doc = "I2C compatible Two-Wire Interface"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Twi {
        ptr: *mut u8,
    }
    unsafe impl Send for Twi {}
    unsafe impl Sync for Twi {}
    impl Twi {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Start TWI receive sequence"]
        #[inline(always)]
        pub const fn tasks_startrx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Start TWI transmit sequence"]
        #[inline(always)]
        pub const fn tasks_starttx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Stop TWI transaction"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "Suspend TWI transaction"]
        #[inline(always)]
        pub const fn tasks_suspend(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
        #[doc = "Resume TWI transaction"]
        #[inline(always)]
        pub const fn tasks_resume(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }
        #[doc = "TWI stopped"]
        #[inline(always)]
        pub const fn events_stopped(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "TWI RXD byte received"]
        #[inline(always)]
        pub const fn events_rxdready(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "TWI TXD byte sent"]
        #[inline(always)]
        pub const fn events_txdsent(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
        }
        #[doc = "TWI error"]
        #[inline(always)]
        pub const fn events_error(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
        }
        #[doc = "TWI byte boundary, generated before each byte that is sent or received"]
        #[inline(always)]
        pub const fn events_bb(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
        }
        #[doc = "TWI entered the suspended state"]
        #[inline(always)]
        pub const fn events_suspended(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Error source"]
        #[inline(always)]
        pub const fn errorsrc(self) -> crate::common::Reg<regs::Errorsrc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04c4usize) as _) }
        }
        #[doc = "Enable TWI"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn psel(self) -> Psel {
            unsafe { Psel::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "RXD register"]
        #[inline(always)]
        pub const fn rxd(self) -> crate::common::Reg<regs::Rxd, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
        }
        #[doc = "TXD register"]
        #[inline(always)]
        pub const fn txd(self) -> crate::common::Reg<regs::Txd, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
        }
        #[doc = "TWI frequency. Accuracy depends on the HFCLK source selected."]
        #[inline(always)]
        pub const fn frequency(self) -> crate::common::Reg<regs::Frequency, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
        }
        #[doc = "Address used in the TWI transfer"]
        #[inline(always)]
        pub const fn address(self) -> crate::common::Reg<regs::Address, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0588usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Address used in the TWI transfer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Address(pub u32);
        impl Address {
            #[doc = "Address used in the TWI transfer"]
            #[inline(always)]
            pub const fn address(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x7f;
                val as u8
            }
            #[doc = "Address used in the TWI transfer"]
            #[inline(always)]
            pub fn set_address(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
            }
        }
        impl Default for Address {
            #[inline(always)]
            fn default() -> Address {
                Address(0)
            }
        }
        impl core::fmt::Debug for Address {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Address")
                    .field("address", &self.address())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Address {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Address {
                    address: u8,
                }
                let proxy = Address {
                    address: self.address(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Enable TWI"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable TWI"]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable TWI"]
            #[inline(always)]
            pub fn set_enable(&mut self, val: super::vals::Enable) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        impl core::fmt::Debug for Enable {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Enable")
                    .field("enable", &self.enable())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Enable {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Enable {
                    enable: super::vals::Enable,
                }
                let proxy = Enable {
                    enable: self.enable(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Error source"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Errorsrc(pub u32);
        impl Errorsrc {
            #[doc = "Overrun error"]
            #[inline(always)]
            pub const fn overrun(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Overrun error"]
            #[inline(always)]
            pub fn set_overrun(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "NACK received after sending the address (write '1' to clear)"]
            #[inline(always)]
            pub const fn anack(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "NACK received after sending the address (write '1' to clear)"]
            #[inline(always)]
            pub fn set_anack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "NACK received after sending a data byte (write '1' to clear)"]
            #[inline(always)]
            pub const fn dnack(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "NACK received after sending a data byte (write '1' to clear)"]
            #[inline(always)]
            pub fn set_dnack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
        }
        impl Default for Errorsrc {
            #[inline(always)]
            fn default() -> Errorsrc {
                Errorsrc(0)
            }
        }
        impl core::fmt::Debug for Errorsrc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Errorsrc")
                    .field("overrun", &self.overrun())
                    .field("anack", &self.anack())
                    .field("dnack", &self.dnack())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Errorsrc {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Errorsrc {
                    overrun: bool,
                    anack: bool,
                    dnack: bool,
                }
                let proxy = Errorsrc {
                    overrun: self.overrun(),
                    anack: self.anack(),
                    dnack: self.dnack(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "TWI frequency. Accuracy depends on the HFCLK source selected."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Frequency(pub u32);
        impl Frequency {
            #[doc = "TWI master clock frequency"]
            #[inline(always)]
            pub const fn frequency(&self) -> super::vals::Frequency {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Frequency::from_bits(val as u32)
            }
            #[doc = "TWI master clock frequency"]
            #[inline(always)]
            pub fn set_frequency(&mut self, val: super::vals::Frequency) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize))
                    | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Frequency {
            #[inline(always)]
            fn default() -> Frequency {
                Frequency(0)
            }
        }
        impl core::fmt::Debug for Frequency {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Frequency")
                    .field("frequency", &self.frequency())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Frequency {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Frequency {
                    frequency: super::vals::Frequency,
                }
                let proxy = Frequency {
                    frequency: self.frequency(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Write '1' to disable interrupt for event STOPPED"]
            #[inline(always)]
            pub const fn stopped(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event STOPPED"]
            #[inline(always)]
            pub fn set_stopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to disable interrupt for event RXDREADY"]
            #[inline(always)]
            pub const fn rxdready(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event RXDREADY"]
            #[inline(always)]
            pub fn set_rxdready(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Write '1' to disable interrupt for event TXDSENT"]
            #[inline(always)]
            pub const fn txdsent(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event TXDSENT"]
            #[inline(always)]
            pub fn set_txdsent(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Write '1' to disable interrupt for event ERROR"]
            #[inline(always)]
            pub const fn error(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event ERROR"]
            #[inline(always)]
            pub fn set_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Write '1' to disable interrupt for event BB"]
            #[inline(always)]
            pub const fn bb(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event BB"]
            #[inline(always)]
            pub fn set_bb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "Write '1' to disable interrupt for event SUSPENDED"]
            #[inline(always)]
            pub const fn suspended(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event SUSPENDED"]
            #[inline(always)]
            pub fn set_suspended(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("stopped", &self.stopped())
                    .field("rxdready", &self.rxdready())
                    .field("txdsent", &self.txdsent())
                    .field("error", &self.error())
                    .field("bb", &self.bb())
                    .field("suspended", &self.suspended())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    stopped: bool,
                    rxdready: bool,
                    txdsent: bool,
                    error: bool,
                    bb: bool,
                    suspended: bool,
                }
                let proxy = Int {
                    stopped: self.stopped(),
                    rxdready: self.rxdready(),
                    txdsent: self.txdsent(),
                    error: self.error(),
                    bb: self.bb(),
                    suspended: self.suspended(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "RXD register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxd(pub u32);
        impl Rxd {
            #[doc = "RXD register"]
            #[inline(always)]
            pub const fn rxd(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "RXD register"]
            #[inline(always)]
            pub fn set_rxd(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Rxd {
            #[inline(always)]
            fn default() -> Rxd {
                Rxd(0)
            }
        }
        impl core::fmt::Debug for Rxd {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Rxd").field("rxd", &self.rxd()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rxd {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Rxd {
                    rxd: u8,
                }
                let proxy = Rxd { rxd: self.rxd() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event BB and task SUSPEND"]
            #[inline(always)]
            pub const fn bb_suspend(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event BB and task SUSPEND"]
            #[inline(always)]
            pub fn set_bb_suspend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Shortcut between event BB and task STOP"]
            #[inline(always)]
            pub const fn bb_stop(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event BB and task STOP"]
            #[inline(always)]
            pub fn set_bb_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
        impl core::fmt::Debug for Shorts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Shorts")
                    .field("bb_suspend", &self.bb_suspend())
                    .field("bb_stop", &self.bb_stop())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Shorts {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Shorts {
                    bb_suspend: bool,
                    bb_stop: bool,
                }
                let proxy = Shorts {
                    bb_suspend: self.bb_suspend(),
                    bb_stop: self.bb_stop(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "TXD register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Txd(pub u32);
        impl Txd {
            #[doc = "TXD register"]
            #[inline(always)]
            pub const fn txd(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "TXD register"]
            #[inline(always)]
            pub fn set_txd(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Txd {
            #[inline(always)]
            fn default() -> Txd {
                Txd(0)
            }
        }
        impl core::fmt::Debug for Txd {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Txd").field("txd", &self.txd()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Txd {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Txd {
                    txd: u8,
                }
                let proxy = Txd { txd: self.txd() };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Enable {
            #[doc = "Disable TWI"]
            DISABLED = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            #[doc = "Enable TWI"]
            ENABLED = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
            _RESERVED_8 = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl Enable {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Enable {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Enable {
            #[inline(always)]
            fn from(val: u8) -> Enable {
                Enable::from_bits(val)
            }
        }
        impl From<Enable> for u8 {
            #[inline(always)]
            fn from(val: Enable) -> u8 {
                Enable::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Frequency(pub u32);
        impl Frequency {
            #[doc = "100 kbps"]
            pub const K100: Self = Self(0x0198_0000);
            #[doc = "250 kbps"]
            pub const K250: Self = Self(0x0400_0000);
            #[doc = "400 kbps (actual rate 410.256 kbps)"]
            pub const K400: Self = Self(0x0668_0000);
        }
        impl Frequency {
            pub const fn from_bits(val: u32) -> Frequency {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl core::fmt::Debug for Frequency {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0198_0000 => f.write_str("K100"),
                    0x0400_0000 => f.write_str("K250"),
                    0x0668_0000 => f.write_str("K400"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Frequency {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0198_0000 => defmt::write!(f, "K100"),
                    0x0400_0000 => defmt::write!(f, "K250"),
                    0x0668_0000 => defmt::write!(f, "K400"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u32> for Frequency {
            #[inline(always)]
            fn from(val: u32) -> Frequency {
                Frequency::from_bits(val)
            }
        }
        impl From<Frequency> for u32 {
            #[inline(always)]
            fn from(val: Frequency) -> u32 {
                Frequency::to_bits(val)
            }
        }
    }
}
pub mod twim {
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psel {
        ptr: *mut u8,
    }
    unsafe impl Send for Psel {}
    unsafe impl Sync for Psel {}
    impl Psel {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Pin select for SCL signal"]
        #[inline(always)]
        pub const fn scl(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Pin select for SDA signal"]
        #[inline(always)]
        pub const fn sda(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
    }
    #[doc = "RXD EasyDMA channel"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxd {
        ptr: *mut u8,
    }
    unsafe impl Send for Rxd {}
    unsafe impl Sync for Rxd {}
    impl Rxd {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Data pointer"]
        #[inline(always)]
        pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Maximum number of bytes in receive buffer"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::RxdMaxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::RxdAmount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "EasyDMA list type"]
        #[inline(always)]
        pub const fn list(self) -> crate::common::Reg<regs::RxdList, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
    }
    #[doc = "I2C compatible Two-Wire Master Interface with EasyDMA"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Twim {
        ptr: *mut u8,
    }
    unsafe impl Send for Twim {}
    unsafe impl Sync for Twim {}
    impl Twim {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Start TWI receive sequence"]
        #[inline(always)]
        pub const fn tasks_startrx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Start TWI transmit sequence"]
        #[inline(always)]
        pub const fn tasks_starttx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Stop TWI transaction. Must be issued while the TWI master is not suspended."]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "Suspend TWI transaction"]
        #[inline(always)]
        pub const fn tasks_suspend(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
        #[doc = "Resume TWI transaction"]
        #[inline(always)]
        pub const fn tasks_resume(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }
        #[doc = "TWI stopped"]
        #[inline(always)]
        pub const fn events_stopped(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "TWI error"]
        #[inline(always)]
        pub const fn events_error(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
        }
        #[doc = "SUSPEND task has been issued, TWI traffic is now suspended."]
        #[inline(always)]
        pub const fn events_suspended(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
        }
        #[doc = "Receive sequence started"]
        #[inline(always)]
        pub const fn events_rxstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
        }
        #[doc = "Transmit sequence started"]
        #[inline(always)]
        pub const fn events_txstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
        }
        #[doc = "Byte boundary, starting to receive the last byte"]
        #[inline(always)]
        pub const fn events_lastrx(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x015cusize) as _) }
        }
        #[doc = "Byte boundary, starting to transmit the last byte"]
        #[inline(always)]
        pub const fn events_lasttx(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0160usize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Error source"]
        #[inline(always)]
        pub const fn errorsrc(self) -> crate::common::Reg<regs::Errorsrc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04c4usize) as _) }
        }
        #[doc = "Enable TWIM"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn psel(self) -> Psel {
            unsafe { Psel::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "TWI frequency. Accuracy depends on the HFCLK source selected."]
        #[inline(always)]
        pub const fn frequency(self) -> crate::common::Reg<regs::Frequency, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
        }
        #[doc = "RXD EasyDMA channel"]
        #[inline(always)]
        pub const fn rxd(self) -> Rxd {
            unsafe { Rxd::from_ptr(self.ptr.add(0x0534usize) as _) }
        }
        #[doc = "TXD EasyDMA channel"]
        #[inline(always)]
        pub const fn txd(self) -> Txd {
            unsafe { Txd::from_ptr(self.ptr.add(0x0544usize) as _) }
        }
        #[doc = "Address used in the TWI transfer"]
        #[inline(always)]
        pub const fn address(self) -> crate::common::Reg<regs::Address, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0588usize) as _) }
        }
    }
    #[doc = "TXD EasyDMA channel"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txd {
        ptr: *mut u8,
    }
    unsafe impl Send for Txd {}
    unsafe impl Sync for Txd {}
    impl Txd {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Data pointer"]
        #[inline(always)]
        pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Maximum number of bytes in transmit buffer"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::TxdMaxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::TxdAmount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "EasyDMA list type"]
        #[inline(always)]
        pub const fn list(self) -> crate::common::Reg<regs::TxdList, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Address used in the TWI transfer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Address(pub u32);
        impl Address {
            #[doc = "Address used in the TWI transfer"]
            #[inline(always)]
            pub const fn address(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x7f;
                val as u8
            }
            #[doc = "Address used in the TWI transfer"]
            #[inline(always)]
            pub fn set_address(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
            }
        }
        impl Default for Address {
            #[inline(always)]
            fn default() -> Address {
                Address(0)
            }
        }
        impl core::fmt::Debug for Address {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Address")
                    .field("address", &self.address())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Address {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Address {
                    address: u8,
                }
                let proxy = Address {
                    address: self.address(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Enable TWIM"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable TWIM"]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable TWIM"]
            #[inline(always)]
            pub fn set_enable(&mut self, val: super::vals::Enable) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        impl core::fmt::Debug for Enable {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Enable")
                    .field("enable", &self.enable())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Enable {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Enable {
                    enable: super::vals::Enable,
                }
                let proxy = Enable {
                    enable: self.enable(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Error source"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Errorsrc(pub u32);
        impl Errorsrc {
            #[doc = "Overrun error"]
            #[inline(always)]
            pub const fn overrun(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Overrun error"]
            #[inline(always)]
            pub fn set_overrun(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "NACK received after sending the address (write '1' to clear)"]
            #[inline(always)]
            pub const fn anack(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "NACK received after sending the address (write '1' to clear)"]
            #[inline(always)]
            pub fn set_anack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "NACK received after sending a data byte (write '1' to clear)"]
            #[inline(always)]
            pub const fn dnack(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "NACK received after sending a data byte (write '1' to clear)"]
            #[inline(always)]
            pub fn set_dnack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
        }
        impl Default for Errorsrc {
            #[inline(always)]
            fn default() -> Errorsrc {
                Errorsrc(0)
            }
        }
        impl core::fmt::Debug for Errorsrc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Errorsrc")
                    .field("overrun", &self.overrun())
                    .field("anack", &self.anack())
                    .field("dnack", &self.dnack())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Errorsrc {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Errorsrc {
                    overrun: bool,
                    anack: bool,
                    dnack: bool,
                }
                let proxy = Errorsrc {
                    overrun: self.overrun(),
                    anack: self.anack(),
                    dnack: self.dnack(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "TWI frequency. Accuracy depends on the HFCLK source selected."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Frequency(pub u32);
        impl Frequency {
            #[doc = "TWI master clock frequency"]
            #[inline(always)]
            pub const fn frequency(&self) -> super::vals::Frequency {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Frequency::from_bits(val as u32)
            }
            #[doc = "TWI master clock frequency"]
            #[inline(always)]
            pub fn set_frequency(&mut self, val: super::vals::Frequency) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize))
                    | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Frequency {
            #[inline(always)]
            fn default() -> Frequency {
                Frequency(0)
            }
        }
        impl core::fmt::Debug for Frequency {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Frequency")
                    .field("frequency", &self.frequency())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Frequency {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Frequency {
                    frequency: super::vals::Frequency,
                }
                let proxy = Frequency {
                    frequency: self.frequency(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Enable or disable interrupt for event STOPPED"]
            #[inline(always)]
            pub const fn stopped(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event STOPPED"]
            #[inline(always)]
            pub fn set_stopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable interrupt for event ERROR"]
            #[inline(always)]
            pub const fn error(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event ERROR"]
            #[inline(always)]
            pub fn set_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Enable or disable interrupt for event SUSPENDED"]
            #[inline(always)]
            pub const fn suspended(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event SUSPENDED"]
            #[inline(always)]
            pub fn set_suspended(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "Enable or disable interrupt for event RXSTARTED"]
            #[inline(always)]
            pub const fn rxstarted(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RXSTARTED"]
            #[inline(always)]
            pub fn set_rxstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "Enable or disable interrupt for event TXSTARTED"]
            #[inline(always)]
            pub const fn txstarted(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event TXSTARTED"]
            #[inline(always)]
            pub fn set_txstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "Enable or disable interrupt for event LASTRX"]
            #[inline(always)]
            pub const fn lastrx(&self) -> bool {
                let val = (self.0 >> 23usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event LASTRX"]
            #[inline(always)]
            pub fn set_lastrx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
            }
            #[doc = "Enable or disable interrupt for event LASTTX"]
            #[inline(always)]
            pub const fn lasttx(&self) -> bool {
                let val = (self.0 >> 24usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event LASTTX"]
            #[inline(always)]
            pub fn set_lasttx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("stopped", &self.stopped())
                    .field("error", &self.error())
                    .field("suspended", &self.suspended())
                    .field("rxstarted", &self.rxstarted())
                    .field("txstarted", &self.txstarted())
                    .field("lastrx", &self.lastrx())
                    .field("lasttx", &self.lasttx())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    stopped: bool,
                    error: bool,
                    suspended: bool,
                    rxstarted: bool,
                    txstarted: bool,
                    lastrx: bool,
                    lasttx: bool,
                }
                let proxy = Int {
                    stopped: self.stopped(),
                    error: self.error(),
                    suspended: self.suspended(),
                    rxstarted: self.rxstarted(),
                    txstarted: self.txstarted(),
                    lastrx: self.lastrx(),
                    lasttx: self.lasttx(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdAmount(pub u32);
        impl RxdAmount {
            #[doc = "Number of bytes transferred in the last transaction. In case of NACK error, includes the NACK'ed byte."]
            #[inline(always)]
            pub const fn amount(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "Number of bytes transferred in the last transaction. In case of NACK error, includes the NACK'ed byte."]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for RxdAmount {
            #[inline(always)]
            fn default() -> RxdAmount {
                RxdAmount(0)
            }
        }
        impl core::fmt::Debug for RxdAmount {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RxdAmount")
                    .field("amount", &self.amount())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RxdAmount {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct RxdAmount {
                    amount: u16,
                }
                let proxy = RxdAmount {
                    amount: self.amount(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "EasyDMA list type"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdList(pub u32);
        impl RxdList {
            #[doc = "List type"]
            #[inline(always)]
            pub const fn list(&self) -> super::vals::RxdListList {
                let val = (self.0 >> 0usize) & 0x07;
                super::vals::RxdListList::from_bits(val as u8)
            }
            #[doc = "List type"]
            #[inline(always)]
            pub fn set_list(&mut self, val: super::vals::RxdListList) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
            }
        }
        impl Default for RxdList {
            #[inline(always)]
            fn default() -> RxdList {
                RxdList(0)
            }
        }
        impl core::fmt::Debug for RxdList {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RxdList")
                    .field("list", &self.list())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RxdList {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct RxdList {
                    list: super::vals::RxdListList,
                }
                let proxy = RxdList { list: self.list() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Maximum number of bytes in receive buffer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdMaxcnt(pub u32);
        impl RxdMaxcnt {
            #[doc = "Maximum number of bytes in receive buffer"]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "Maximum number of bytes in receive buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for RxdMaxcnt {
            #[inline(always)]
            fn default() -> RxdMaxcnt {
                RxdMaxcnt(0)
            }
        }
        impl core::fmt::Debug for RxdMaxcnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RxdMaxcnt")
                    .field("maxcnt", &self.maxcnt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RxdMaxcnt {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct RxdMaxcnt {
                    maxcnt: u16,
                }
                let proxy = RxdMaxcnt {
                    maxcnt: self.maxcnt(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event LASTTX and task STARTRX"]
            #[inline(always)]
            pub const fn lasttx_startrx(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event LASTTX and task STARTRX"]
            #[inline(always)]
            pub fn set_lasttx_startrx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Shortcut between event LASTTX and task SUSPEND"]
            #[inline(always)]
            pub const fn lasttx_suspend(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event LASTTX and task SUSPEND"]
            #[inline(always)]
            pub fn set_lasttx_suspend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Shortcut between event LASTTX and task STOP"]
            #[inline(always)]
            pub const fn lasttx_stop(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event LASTTX and task STOP"]
            #[inline(always)]
            pub fn set_lasttx_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Shortcut between event LASTRX and task STARTTX"]
            #[inline(always)]
            pub const fn lastrx_starttx(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event LASTRX and task STARTTX"]
            #[inline(always)]
            pub fn set_lastrx_starttx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "Shortcut between event LASTRX and task SUSPEND"]
            #[inline(always)]
            pub const fn lastrx_suspend(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event LASTRX and task SUSPEND"]
            #[inline(always)]
            pub fn set_lastrx_suspend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "Shortcut between event LASTRX and task STOP"]
            #[inline(always)]
            pub const fn lastrx_stop(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event LASTRX and task STOP"]
            #[inline(always)]
            pub fn set_lastrx_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
        impl core::fmt::Debug for Shorts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Shorts")
                    .field("lasttx_startrx", &self.lasttx_startrx())
                    .field("lasttx_suspend", &self.lasttx_suspend())
                    .field("lasttx_stop", &self.lasttx_stop())
                    .field("lastrx_starttx", &self.lastrx_starttx())
                    .field("lastrx_suspend", &self.lastrx_suspend())
                    .field("lastrx_stop", &self.lastrx_stop())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Shorts {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Shorts {
                    lasttx_startrx: bool,
                    lasttx_suspend: bool,
                    lasttx_stop: bool,
                    lastrx_starttx: bool,
                    lastrx_suspend: bool,
                    lastrx_stop: bool,
                }
                let proxy = Shorts {
                    lasttx_startrx: self.lasttx_startrx(),
                    lasttx_suspend: self.lasttx_suspend(),
                    lasttx_stop: self.lasttx_stop(),
                    lastrx_starttx: self.lastrx_starttx(),
                    lastrx_suspend: self.lastrx_suspend(),
                    lastrx_stop: self.lastrx_stop(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdAmount(pub u32);
        impl TxdAmount {
            #[doc = "Number of bytes transferred in the last transaction. In case of NACK error, includes the NACK'ed byte."]
            #[inline(always)]
            pub const fn amount(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "Number of bytes transferred in the last transaction. In case of NACK error, includes the NACK'ed byte."]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for TxdAmount {
            #[inline(always)]
            fn default() -> TxdAmount {
                TxdAmount(0)
            }
        }
        impl core::fmt::Debug for TxdAmount {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("TxdAmount")
                    .field("amount", &self.amount())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for TxdAmount {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct TxdAmount {
                    amount: u16,
                }
                let proxy = TxdAmount {
                    amount: self.amount(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "EasyDMA list type"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdList(pub u32);
        impl TxdList {
            #[doc = "List type"]
            #[inline(always)]
            pub const fn list(&self) -> super::vals::TxdListList {
                let val = (self.0 >> 0usize) & 0x07;
                super::vals::TxdListList::from_bits(val as u8)
            }
            #[doc = "List type"]
            #[inline(always)]
            pub fn set_list(&mut self, val: super::vals::TxdListList) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
            }
        }
        impl Default for TxdList {
            #[inline(always)]
            fn default() -> TxdList {
                TxdList(0)
            }
        }
        impl core::fmt::Debug for TxdList {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("TxdList")
                    .field("list", &self.list())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for TxdList {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct TxdList {
                    list: super::vals::TxdListList,
                }
                let proxy = TxdList { list: self.list() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Maximum number of bytes in transmit buffer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdMaxcnt(pub u32);
        impl TxdMaxcnt {
            #[doc = "Maximum number of bytes in transmit buffer"]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "Maximum number of bytes in transmit buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for TxdMaxcnt {
            #[inline(always)]
            fn default() -> TxdMaxcnt {
                TxdMaxcnt(0)
            }
        }
        impl core::fmt::Debug for TxdMaxcnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("TxdMaxcnt")
                    .field("maxcnt", &self.maxcnt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for TxdMaxcnt {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct TxdMaxcnt {
                    maxcnt: u16,
                }
                let proxy = TxdMaxcnt {
                    maxcnt: self.maxcnt(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Enable {
            #[doc = "Disable TWIM"]
            DISABLED = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            #[doc = "Enable TWIM"]
            ENABLED = 0x06,
            _RESERVED_7 = 0x07,
            _RESERVED_8 = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl Enable {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Enable {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Enable {
            #[inline(always)]
            fn from(val: u8) -> Enable {
                Enable::from_bits(val)
            }
        }
        impl From<Enable> for u8 {
            #[inline(always)]
            fn from(val: Enable) -> u8 {
                Enable::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Frequency(pub u32);
        impl Frequency {
            #[doc = "100 kbps"]
            pub const K100: Self = Self(0x0198_0000);
            #[doc = "250 kbps"]
            pub const K250: Self = Self(0x0400_0000);
            #[doc = "400 kbps"]
            pub const K400: Self = Self(0x0640_0000);
        }
        impl Frequency {
            pub const fn from_bits(val: u32) -> Frequency {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl core::fmt::Debug for Frequency {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0198_0000 => f.write_str("K100"),
                    0x0400_0000 => f.write_str("K250"),
                    0x0640_0000 => f.write_str("K400"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Frequency {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0198_0000 => defmt::write!(f, "K100"),
                    0x0400_0000 => defmt::write!(f, "K250"),
                    0x0640_0000 => defmt::write!(f, "K400"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u32> for Frequency {
            #[inline(always)]
            fn from(val: u32) -> Frequency {
                Frequency::from_bits(val)
            }
        }
        impl From<Frequency> for u32 {
            #[inline(always)]
            fn from(val: Frequency) -> u32 {
                Frequency::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum RxdListList {
            #[doc = "Disable EasyDMA list"]
            DISABLED = 0x0,
            #[doc = "Use array list"]
            ARRAY_LIST = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
        }
        impl RxdListList {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> RxdListList {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for RxdListList {
            #[inline(always)]
            fn from(val: u8) -> RxdListList {
                RxdListList::from_bits(val)
            }
        }
        impl From<RxdListList> for u8 {
            #[inline(always)]
            fn from(val: RxdListList) -> u8 {
                RxdListList::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum TxdListList {
            #[doc = "Disable EasyDMA list"]
            DISABLED = 0x0,
            #[doc = "Use array list"]
            ARRAY_LIST = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
        }
        impl TxdListList {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> TxdListList {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for TxdListList {
            #[inline(always)]
            fn from(val: u8) -> TxdListList {
                TxdListList::from_bits(val)
            }
        }
        impl From<TxdListList> for u8 {
            #[inline(always)]
            fn from(val: TxdListList) -> u8 {
                TxdListList::to_bits(val)
            }
        }
    }
}
pub mod twis {
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psel {
        ptr: *mut u8,
    }
    unsafe impl Send for Psel {}
    unsafe impl Sync for Psel {}
    impl Psel {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Pin select for SCL signal"]
        #[inline(always)]
        pub const fn scl(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Pin select for SDA signal"]
        #[inline(always)]
        pub const fn sda(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
    }
    #[doc = "RXD EasyDMA channel"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxd {
        ptr: *mut u8,
    }
    unsafe impl Send for Rxd {}
    unsafe impl Sync for Rxd {}
    impl Rxd {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "RXD Data pointer"]
        #[inline(always)]
        pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Maximum number of bytes in RXD buffer"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::RxdMaxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Number of bytes transferred in the last RXD transaction"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::RxdAmount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "EasyDMA list type"]
        #[inline(always)]
        pub const fn list(self) -> crate::common::Reg<regs::RxdList, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
    }
    #[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Twis {
        ptr: *mut u8,
    }
    unsafe impl Send for Twis {}
    unsafe impl Sync for Twis {}
    impl Twis {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Stop TWI transaction"]
        #[inline(always)]
        pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "Suspend TWI transaction"]
        #[inline(always)]
        pub const fn tasks_suspend(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
        #[doc = "Resume TWI transaction"]
        #[inline(always)]
        pub const fn tasks_resume(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }
        #[doc = "Prepare the TWI slave to respond to a write command"]
        #[inline(always)]
        pub const fn tasks_preparerx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
        }
        #[doc = "Prepare the TWI slave to respond to a read command"]
        #[inline(always)]
        pub const fn tasks_preparetx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
        }
        #[doc = "TWI stopped"]
        #[inline(always)]
        pub const fn events_stopped(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "TWI error"]
        #[inline(always)]
        pub const fn events_error(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
        }
        #[doc = "Receive sequence started"]
        #[inline(always)]
        pub const fn events_rxstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
        }
        #[doc = "Transmit sequence started"]
        #[inline(always)]
        pub const fn events_txstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
        }
        #[doc = "Write command received"]
        #[inline(always)]
        pub const fn events_write(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0164usize) as _) }
        }
        #[doc = "Read command received"]
        #[inline(always)]
        pub const fn events_read(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0168usize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Error source"]
        #[inline(always)]
        pub const fn errorsrc(self) -> crate::common::Reg<regs::Errorsrc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04d0usize) as _) }
        }
        #[doc = "Status register indicating which address had a match"]
        #[inline(always)]
        pub const fn match_(self) -> crate::common::Reg<regs::Match, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04d4usize) as _) }
        }
        #[doc = "Enable TWIS"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn psel(self) -> Psel {
            unsafe { Psel::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "RXD EasyDMA channel"]
        #[inline(always)]
        pub const fn rxd(self) -> Rxd {
            unsafe { Rxd::from_ptr(self.ptr.add(0x0534usize) as _) }
        }
        #[doc = "TXD EasyDMA channel"]
        #[inline(always)]
        pub const fn txd(self) -> Txd {
            unsafe { Txd::from_ptr(self.ptr.add(0x0544usize) as _) }
        }
        #[doc = "Description collection: TWI slave address n"]
        #[inline(always)]
        pub const fn address(
            self,
            n: usize,
        ) -> crate::common::Reg<regs::Address, crate::common::RW> {
            assert!(n < 2usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0588usize + n * 4usize) as _) }
        }
        #[doc = "Configuration register for the address match mechanism"]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0594usize) as _) }
        }
        #[doc = "Over-read character. Character sent out in case of an over-read of the transmit buffer."]
        #[inline(always)]
        pub const fn orc(self) -> crate::common::Reg<regs::Orc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05c0usize) as _) }
        }
    }
    #[doc = "TXD EasyDMA channel"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txd {
        ptr: *mut u8,
    }
    unsafe impl Send for Txd {}
    unsafe impl Sync for Txd {}
    impl Txd {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "TXD Data pointer"]
        #[inline(always)]
        pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Maximum number of bytes in TXD buffer"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::TxdMaxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Number of bytes transferred in the last TXD transaction"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::TxdAmount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "EasyDMA list type"]
        #[inline(always)]
        pub const fn list(self) -> crate::common::Reg<regs::TxdList, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Description collection: TWI slave address n"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Address(pub u32);
        impl Address {
            #[doc = "TWI slave address"]
            #[inline(always)]
            pub const fn address(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x7f;
                val as u8
            }
            #[doc = "TWI slave address"]
            #[inline(always)]
            pub fn set_address(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
            }
        }
        impl Default for Address {
            #[inline(always)]
            fn default() -> Address {
                Address(0)
            }
        }
        impl core::fmt::Debug for Address {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Address")
                    .field("address", &self.address())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Address {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Address {
                    address: u8,
                }
                let proxy = Address {
                    address: self.address(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Configuration register for the address match mechanism"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Enable or disable address matching on ADDRESS\\[0\\]"]
            #[inline(always)]
            pub const fn address0(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable address matching on ADDRESS\\[0\\]"]
            #[inline(always)]
            pub fn set_address0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable address matching on ADDRESS\\[1\\]"]
            #[inline(always)]
            pub const fn address1(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable address matching on ADDRESS\\[1\\]"]
            #[inline(always)]
            pub fn set_address1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
        }
        impl Default for Config {
            #[inline(always)]
            fn default() -> Config {
                Config(0)
            }
        }
        impl core::fmt::Debug for Config {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Config")
                    .field("address0", &self.address0())
                    .field("address1", &self.address1())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Config {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Config {
                    address0: bool,
                    address1: bool,
                }
                let proxy = Config {
                    address0: self.address0(),
                    address1: self.address1(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Enable TWIS"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable TWIS"]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable TWIS"]
            #[inline(always)]
            pub fn set_enable(&mut self, val: super::vals::Enable) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        impl core::fmt::Debug for Enable {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Enable")
                    .field("enable", &self.enable())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Enable {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Enable {
                    enable: super::vals::Enable,
                }
                let proxy = Enable {
                    enable: self.enable(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Error source"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Errorsrc(pub u32);
        impl Errorsrc {
            #[doc = "RX buffer overflow detected, and prevented"]
            #[inline(always)]
            pub const fn overflow(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "RX buffer overflow detected, and prevented"]
            #[inline(always)]
            pub fn set_overflow(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "NACK sent after receiving a data byte"]
            #[inline(always)]
            pub const fn dnack(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "NACK sent after receiving a data byte"]
            #[inline(always)]
            pub fn set_dnack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "TX buffer over-read detected, and prevented"]
            #[inline(always)]
            pub const fn overread(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "TX buffer over-read detected, and prevented"]
            #[inline(always)]
            pub fn set_overread(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Errorsrc {
            #[inline(always)]
            fn default() -> Errorsrc {
                Errorsrc(0)
            }
        }
        impl core::fmt::Debug for Errorsrc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Errorsrc")
                    .field("overflow", &self.overflow())
                    .field("dnack", &self.dnack())
                    .field("overread", &self.overread())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Errorsrc {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Errorsrc {
                    overflow: bool,
                    dnack: bool,
                    overread: bool,
                }
                let proxy = Errorsrc {
                    overflow: self.overflow(),
                    dnack: self.dnack(),
                    overread: self.overread(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Enable or disable interrupt for event STOPPED"]
            #[inline(always)]
            pub const fn stopped(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event STOPPED"]
            #[inline(always)]
            pub fn set_stopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable interrupt for event ERROR"]
            #[inline(always)]
            pub const fn error(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event ERROR"]
            #[inline(always)]
            pub fn set_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Enable or disable interrupt for event RXSTARTED"]
            #[inline(always)]
            pub const fn rxstarted(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RXSTARTED"]
            #[inline(always)]
            pub fn set_rxstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "Enable or disable interrupt for event TXSTARTED"]
            #[inline(always)]
            pub const fn txstarted(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event TXSTARTED"]
            #[inline(always)]
            pub fn set_txstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "Enable or disable interrupt for event WRITE"]
            #[inline(always)]
            pub const fn write(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event WRITE"]
            #[inline(always)]
            pub fn set_write(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
            }
            #[doc = "Enable or disable interrupt for event READ"]
            #[inline(always)]
            pub const fn read(&self) -> bool {
                let val = (self.0 >> 26usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event READ"]
            #[inline(always)]
            pub fn set_read(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("stopped", &self.stopped())
                    .field("error", &self.error())
                    .field("rxstarted", &self.rxstarted())
                    .field("txstarted", &self.txstarted())
                    .field("write", &self.write())
                    .field("read", &self.read())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    stopped: bool,
                    error: bool,
                    rxstarted: bool,
                    txstarted: bool,
                    write: bool,
                    read: bool,
                }
                let proxy = Int {
                    stopped: self.stopped(),
                    error: self.error(),
                    rxstarted: self.rxstarted(),
                    txstarted: self.txstarted(),
                    write: self.write(),
                    read: self.read(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Status register indicating which address had a match"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Match(pub u32);
        impl Match {
            #[doc = "Indication of which address in {ADDRESS} that matched the incoming address"]
            #[inline(always)]
            pub const fn match_(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Indication of which address in {ADDRESS} that matched the incoming address"]
            #[inline(always)]
            pub fn set_match_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Match {
            #[inline(always)]
            fn default() -> Match {
                Match(0)
            }
        }
        impl core::fmt::Debug for Match {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Match")
                    .field("match_", &self.match_())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Match {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Match {
                    match_: bool,
                }
                let proxy = Match {
                    match_: self.match_(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Over-read character. Character sent out in case of an over-read of the transmit buffer."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Orc(pub u32);
        impl Orc {
            #[doc = "Over-read character. Character sent out in case of an over-read of the transmit buffer."]
            #[inline(always)]
            pub const fn orc(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Over-read character. Character sent out in case of an over-read of the transmit buffer."]
            #[inline(always)]
            pub fn set_orc(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Orc {
            #[inline(always)]
            fn default() -> Orc {
                Orc(0)
            }
        }
        impl core::fmt::Debug for Orc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Orc").field("orc", &self.orc()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Orc {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Orc {
                    orc: u8,
                }
                let proxy = Orc { orc: self.orc() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Number of bytes transferred in the last RXD transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdAmount(pub u32);
        impl RxdAmount {
            #[doc = "Number of bytes transferred in the last RXD transaction"]
            #[inline(always)]
            pub const fn amount(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "Number of bytes transferred in the last RXD transaction"]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for RxdAmount {
            #[inline(always)]
            fn default() -> RxdAmount {
                RxdAmount(0)
            }
        }
        impl core::fmt::Debug for RxdAmount {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RxdAmount")
                    .field("amount", &self.amount())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RxdAmount {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct RxdAmount {
                    amount: u16,
                }
                let proxy = RxdAmount {
                    amount: self.amount(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "EasyDMA list type"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdList(pub u32);
        impl RxdList {
            #[doc = "List type"]
            #[inline(always)]
            pub const fn list(&self) -> super::vals::RxdListList {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::RxdListList::from_bits(val as u8)
            }
            #[doc = "List type"]
            #[inline(always)]
            pub fn set_list(&mut self, val: super::vals::RxdListList) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for RxdList {
            #[inline(always)]
            fn default() -> RxdList {
                RxdList(0)
            }
        }
        impl core::fmt::Debug for RxdList {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RxdList")
                    .field("list", &self.list())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RxdList {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct RxdList {
                    list: super::vals::RxdListList,
                }
                let proxy = RxdList { list: self.list() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Maximum number of bytes in RXD buffer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdMaxcnt(pub u32);
        impl RxdMaxcnt {
            #[doc = "Maximum number of bytes in RXD buffer"]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "Maximum number of bytes in RXD buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for RxdMaxcnt {
            #[inline(always)]
            fn default() -> RxdMaxcnt {
                RxdMaxcnt(0)
            }
        }
        impl core::fmt::Debug for RxdMaxcnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RxdMaxcnt")
                    .field("maxcnt", &self.maxcnt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RxdMaxcnt {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct RxdMaxcnt {
                    maxcnt: u16,
                }
                let proxy = RxdMaxcnt {
                    maxcnt: self.maxcnt(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event WRITE and task SUSPEND"]
            #[inline(always)]
            pub const fn write_suspend(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event WRITE and task SUSPEND"]
            #[inline(always)]
            pub fn set_write_suspend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "Shortcut between event READ and task SUSPEND"]
            #[inline(always)]
            pub const fn read_suspend(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event READ and task SUSPEND"]
            #[inline(always)]
            pub fn set_read_suspend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
        impl core::fmt::Debug for Shorts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Shorts")
                    .field("write_suspend", &self.write_suspend())
                    .field("read_suspend", &self.read_suspend())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Shorts {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Shorts {
                    write_suspend: bool,
                    read_suspend: bool,
                }
                let proxy = Shorts {
                    write_suspend: self.write_suspend(),
                    read_suspend: self.read_suspend(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Number of bytes transferred in the last TXD transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdAmount(pub u32);
        impl TxdAmount {
            #[doc = "Number of bytes transferred in the last TXD transaction"]
            #[inline(always)]
            pub const fn amount(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "Number of bytes transferred in the last TXD transaction"]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for TxdAmount {
            #[inline(always)]
            fn default() -> TxdAmount {
                TxdAmount(0)
            }
        }
        impl core::fmt::Debug for TxdAmount {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("TxdAmount")
                    .field("amount", &self.amount())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for TxdAmount {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct TxdAmount {
                    amount: u16,
                }
                let proxy = TxdAmount {
                    amount: self.amount(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "EasyDMA list type"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdList(pub u32);
        impl TxdList {
            #[doc = "List type"]
            #[inline(always)]
            pub const fn list(&self) -> super::vals::TxdListList {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::TxdListList::from_bits(val as u8)
            }
            #[doc = "List type"]
            #[inline(always)]
            pub fn set_list(&mut self, val: super::vals::TxdListList) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
        }
        impl Default for TxdList {
            #[inline(always)]
            fn default() -> TxdList {
                TxdList(0)
            }
        }
        impl core::fmt::Debug for TxdList {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("TxdList")
                    .field("list", &self.list())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for TxdList {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct TxdList {
                    list: super::vals::TxdListList,
                }
                let proxy = TxdList { list: self.list() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Maximum number of bytes in TXD buffer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdMaxcnt(pub u32);
        impl TxdMaxcnt {
            #[doc = "Maximum number of bytes in TXD buffer"]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x3fff;
                val as u16
            }
            #[doc = "Maximum number of bytes in TXD buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
            }
        }
        impl Default for TxdMaxcnt {
            #[inline(always)]
            fn default() -> TxdMaxcnt {
                TxdMaxcnt(0)
            }
        }
        impl core::fmt::Debug for TxdMaxcnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("TxdMaxcnt")
                    .field("maxcnt", &self.maxcnt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for TxdMaxcnt {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct TxdMaxcnt {
                    maxcnt: u16,
                }
                let proxy = TxdMaxcnt {
                    maxcnt: self.maxcnt(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Enable {
            #[doc = "Disable TWIS"]
            DISABLED = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
            _RESERVED_8 = 0x08,
            #[doc = "Enable TWIS"]
            ENABLED = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl Enable {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Enable {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Enable {
            #[inline(always)]
            fn from(val: u8) -> Enable {
                Enable::from_bits(val)
            }
        }
        impl From<Enable> for u8 {
            #[inline(always)]
            fn from(val: Enable) -> u8 {
                Enable::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum RxdListList {
            #[doc = "Disable EasyDMA list"]
            DISABLED = 0x0,
            #[doc = "Use array list"]
            ARRAY_LIST = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl RxdListList {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> RxdListList {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for RxdListList {
            #[inline(always)]
            fn from(val: u8) -> RxdListList {
                RxdListList::from_bits(val)
            }
        }
        impl From<RxdListList> for u8 {
            #[inline(always)]
            fn from(val: RxdListList) -> u8 {
                RxdListList::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum TxdListList {
            #[doc = "Disable EasyDMA list"]
            DISABLED = 0x0,
            #[doc = "Use array list"]
            ARRAY_LIST = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl TxdListList {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> TxdListList {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for TxdListList {
            #[inline(always)]
            fn from(val: u8) -> TxdListList {
                TxdListList::from_bits(val)
            }
        }
        impl From<TxdListList> for u8 {
            #[inline(always)]
            fn from(val: TxdListList) -> u8 {
                TxdListList::to_bits(val)
            }
        }
    }
}
pub mod uart {
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psel {
        ptr: *mut u8,
    }
    unsafe impl Send for Psel {}
    unsafe impl Sync for Psel {}
    impl Psel {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Pin select for RTS"]
        #[inline(always)]
        pub const fn rts(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Pin select for TXD"]
        #[inline(always)]
        pub const fn txd(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Pin select for CTS"]
        #[inline(always)]
        pub const fn cts(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Pin select for RXD"]
        #[inline(always)]
        pub const fn rxd(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uart {
        ptr: *mut u8,
    }
    unsafe impl Send for Uart {}
    unsafe impl Sync for Uart {}
    impl Uart {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Start UART receiver"]
        #[inline(always)]
        pub const fn tasks_startrx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Stop UART receiver"]
        #[inline(always)]
        pub const fn tasks_stoprx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Start UART transmitter"]
        #[inline(always)]
        pub const fn tasks_starttx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Stop UART transmitter"]
        #[inline(always)]
        pub const fn tasks_stoptx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Suspend UART"]
        #[inline(always)]
        pub const fn tasks_suspend(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
        #[doc = "CTS is activated (set low). Clear To Send."]
        #[inline(always)]
        pub const fn events_cts(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "CTS is deactivated (set high). Not Clear To Send."]
        #[inline(always)]
        pub const fn events_ncts(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "Data received in RXD"]
        #[inline(always)]
        pub const fn events_rxdrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "Data sent from TXD"]
        #[inline(always)]
        pub const fn events_txdrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
        }
        #[doc = "Error detected"]
        #[inline(always)]
        pub const fn events_error(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
        }
        #[doc = "Receiver timeout"]
        #[inline(always)]
        pub const fn events_rxto(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Error source"]
        #[inline(always)]
        pub const fn errorsrc(self) -> crate::common::Reg<regs::Errorsrc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0480usize) as _) }
        }
        #[doc = "Enable UART"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn psel(self) -> Psel {
            unsafe { Psel::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "RXD register"]
        #[inline(always)]
        pub const fn rxd(self) -> crate::common::Reg<regs::Rxd, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
        }
        #[doc = "TXD register"]
        #[inline(always)]
        pub const fn txd(self) -> crate::common::Reg<regs::Txd, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
        }
        #[doc = "Baud rate. Accuracy depends on the HFCLK source selected."]
        #[inline(always)]
        pub const fn baudrate(self) -> crate::common::Reg<regs::Baudrate, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
        }
        #[doc = "Configuration of parity and hardware flow control"]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x056cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Baud rate. Accuracy depends on the HFCLK source selected."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Baudrate(pub u32);
        impl Baudrate {
            #[doc = "Baud rate"]
            #[inline(always)]
            pub const fn baudrate(&self) -> super::vals::Baudrate {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Baudrate::from_bits(val as u32)
            }
            #[doc = "Baud rate"]
            #[inline(always)]
            pub fn set_baudrate(&mut self, val: super::vals::Baudrate) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize))
                    | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Baudrate {
            #[inline(always)]
            fn default() -> Baudrate {
                Baudrate(0)
            }
        }
        impl core::fmt::Debug for Baudrate {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Baudrate")
                    .field("baudrate", &self.baudrate())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Baudrate {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Baudrate {
                    baudrate: super::vals::Baudrate,
                }
                let proxy = Baudrate {
                    baudrate: self.baudrate(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Configuration of parity and hardware flow control"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Hardware flow control"]
            #[inline(always)]
            pub const fn hwfc(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Hardware flow control"]
            #[inline(always)]
            pub fn set_hwfc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Parity"]
            #[inline(always)]
            pub const fn parity(&self) -> super::vals::ConfigParity {
                let val = (self.0 >> 1usize) & 0x07;
                super::vals::ConfigParity::from_bits(val as u8)
            }
            #[doc = "Parity"]
            #[inline(always)]
            pub fn set_parity(&mut self, val: super::vals::ConfigParity) {
                self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
            }
            #[doc = "Stop bits"]
            #[inline(always)]
            pub const fn stop(&self) -> super::vals::Stop {
                let val = (self.0 >> 4usize) & 0x01;
                super::vals::Stop::from_bits(val as u8)
            }
            #[doc = "Stop bits"]
            #[inline(always)]
            pub fn set_stop(&mut self, val: super::vals::Stop) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
            }
        }
        impl Default for Config {
            #[inline(always)]
            fn default() -> Config {
                Config(0)
            }
        }
        impl core::fmt::Debug for Config {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Config")
                    .field("hwfc", &self.hwfc())
                    .field("parity", &self.parity())
                    .field("stop", &self.stop())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Config {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Config {
                    hwfc: bool,
                    parity: super::vals::ConfigParity,
                    stop: super::vals::Stop,
                }
                let proxy = Config {
                    hwfc: self.hwfc(),
                    parity: self.parity(),
                    stop: self.stop(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Enable UART"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable UART"]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable UART"]
            #[inline(always)]
            pub fn set_enable(&mut self, val: super::vals::Enable) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        impl core::fmt::Debug for Enable {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Enable")
                    .field("enable", &self.enable())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Enable {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Enable {
                    enable: super::vals::Enable,
                }
                let proxy = Enable {
                    enable: self.enable(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Error source"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Errorsrc(pub u32);
        impl Errorsrc {
            #[doc = "Overrun error"]
            #[inline(always)]
            pub const fn overrun(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Overrun error"]
            #[inline(always)]
            pub fn set_overrun(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Parity error"]
            #[inline(always)]
            pub const fn parity(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Parity error"]
            #[inline(always)]
            pub fn set_parity(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Framing error occurred"]
            #[inline(always)]
            pub const fn framing(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Framing error occurred"]
            #[inline(always)]
            pub fn set_framing(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Break condition"]
            #[inline(always)]
            pub const fn break_(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Break condition"]
            #[inline(always)]
            pub fn set_break_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Errorsrc {
            #[inline(always)]
            fn default() -> Errorsrc {
                Errorsrc(0)
            }
        }
        impl core::fmt::Debug for Errorsrc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Errorsrc")
                    .field("overrun", &self.overrun())
                    .field("parity", &self.parity())
                    .field("framing", &self.framing())
                    .field("break_", &self.break_())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Errorsrc {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Errorsrc {
                    overrun: bool,
                    parity: bool,
                    framing: bool,
                    break_: bool,
                }
                let proxy = Errorsrc {
                    overrun: self.overrun(),
                    parity: self.parity(),
                    framing: self.framing(),
                    break_: self.break_(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Write '1' to disable interrupt for event CTS"]
            #[inline(always)]
            pub const fn cts(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event CTS"]
            #[inline(always)]
            pub fn set_cts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Write '1' to disable interrupt for event NCTS"]
            #[inline(always)]
            pub const fn ncts(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event NCTS"]
            #[inline(always)]
            pub fn set_ncts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Write '1' to disable interrupt for event RXDRDY"]
            #[inline(always)]
            pub const fn rxdrdy(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event RXDRDY"]
            #[inline(always)]
            pub fn set_rxdrdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Write '1' to disable interrupt for event TXDRDY"]
            #[inline(always)]
            pub const fn txdrdy(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event TXDRDY"]
            #[inline(always)]
            pub fn set_txdrdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Write '1' to disable interrupt for event ERROR"]
            #[inline(always)]
            pub const fn error(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event ERROR"]
            #[inline(always)]
            pub fn set_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Write '1' to disable interrupt for event RXTO"]
            #[inline(always)]
            pub const fn rxto(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event RXTO"]
            #[inline(always)]
            pub fn set_rxto(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("cts", &self.cts())
                    .field("ncts", &self.ncts())
                    .field("rxdrdy", &self.rxdrdy())
                    .field("txdrdy", &self.txdrdy())
                    .field("error", &self.error())
                    .field("rxto", &self.rxto())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    cts: bool,
                    ncts: bool,
                    rxdrdy: bool,
                    txdrdy: bool,
                    error: bool,
                    rxto: bool,
                }
                let proxy = Int {
                    cts: self.cts(),
                    ncts: self.ncts(),
                    rxdrdy: self.rxdrdy(),
                    txdrdy: self.txdrdy(),
                    error: self.error(),
                    rxto: self.rxto(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "RXD register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxd(pub u32);
        impl Rxd {
            #[doc = "RX data received in previous transfers, double buffered"]
            #[inline(always)]
            pub const fn rxd(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "RX data received in previous transfers, double buffered"]
            #[inline(always)]
            pub fn set_rxd(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Rxd {
            #[inline(always)]
            fn default() -> Rxd {
                Rxd(0)
            }
        }
        impl core::fmt::Debug for Rxd {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Rxd").field("rxd", &self.rxd()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rxd {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Rxd {
                    rxd: u8,
                }
                let proxy = Rxd { rxd: self.rxd() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event CTS and task STARTRX"]
            #[inline(always)]
            pub const fn cts_startrx(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event CTS and task STARTRX"]
            #[inline(always)]
            pub fn set_cts_startrx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Shortcut between event NCTS and task STOPRX"]
            #[inline(always)]
            pub const fn ncts_stoprx(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event NCTS and task STOPRX"]
            #[inline(always)]
            pub fn set_ncts_stoprx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
        impl core::fmt::Debug for Shorts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Shorts")
                    .field("cts_startrx", &self.cts_startrx())
                    .field("ncts_stoprx", &self.ncts_stoprx())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Shorts {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Shorts {
                    cts_startrx: bool,
                    ncts_stoprx: bool,
                }
                let proxy = Shorts {
                    cts_startrx: self.cts_startrx(),
                    ncts_stoprx: self.ncts_stoprx(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "TXD register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Txd(pub u32);
        impl Txd {
            #[doc = "TX data to be transferred"]
            #[inline(always)]
            pub const fn txd(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "TX data to be transferred"]
            #[inline(always)]
            pub fn set_txd(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Txd {
            #[inline(always)]
            fn default() -> Txd {
                Txd(0)
            }
        }
        impl core::fmt::Debug for Txd {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Txd").field("txd", &self.txd()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Txd {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Txd {
                    txd: u8,
                }
                let proxy = Txd { txd: self.txd() };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Baudrate(pub u32);
        impl Baudrate {
            #[doc = "1200 baud (actual rate: 1205)"]
            pub const BAUD1200: Self = Self(0x0004_f000);
            #[doc = "2400 baud (actual rate: 2396)"]
            pub const BAUD2400: Self = Self(0x0009_d000);
            #[doc = "4800 baud (actual rate: 4808)"]
            pub const BAUD4800: Self = Self(0x0013_b000);
            #[doc = "9600 baud (actual rate: 9598)"]
            pub const BAUD9600: Self = Self(0x0027_5000);
            #[doc = "14400 baud (actual rate: 14414)"]
            pub const BAUD14400: Self = Self(0x003b_0000);
            #[doc = "19200 baud (actual rate: 19208)"]
            pub const BAUD19200: Self = Self(0x004e_a000);
            #[doc = "28800 baud (actual rate: 28829)"]
            pub const BAUD28800: Self = Self(0x0075_f000);
            #[doc = "31250 baud"]
            pub const BAUD31250: Self = Self(0x0080_0000);
            #[doc = "38400 baud (actual rate: 38462)"]
            pub const BAUD38400: Self = Self(0x009d_5000);
            #[doc = "56000 baud (actual rate: 55944)"]
            pub const BAUD56000: Self = Self(0x00e5_0000);
            #[doc = "57600 baud (actual rate: 57762)"]
            pub const BAUD57600: Self = Self(0x00eb_f000);
            #[doc = "76800 baud (actual rate: 76923)"]
            pub const BAUD76800: Self = Self(0x013a_9000);
            #[doc = "115200 baud (actual rate: 115942)"]
            pub const BAUD115200: Self = Self(0x01d7_e000);
            #[doc = "230400 baud (actual rate: 231884)"]
            pub const BAUD230400: Self = Self(0x03af_b000);
            #[doc = "250000 baud"]
            pub const BAUD250000: Self = Self(0x0400_0000);
            #[doc = "460800 baud (actual rate: 470588)"]
            pub const BAUD460800: Self = Self(0x075f_7000);
            #[doc = "921600 baud (actual rate: 941176)"]
            pub const BAUD921600: Self = Self(0x0ebe_d000);
            #[doc = "1Mega baud"]
            pub const BAUD1M: Self = Self(0x1000_0000);
        }
        impl Baudrate {
            pub const fn from_bits(val: u32) -> Baudrate {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl core::fmt::Debug for Baudrate {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0004_f000 => f.write_str("BAUD1200"),
                    0x0009_d000 => f.write_str("BAUD2400"),
                    0x0013_b000 => f.write_str("BAUD4800"),
                    0x0027_5000 => f.write_str("BAUD9600"),
                    0x003b_0000 => f.write_str("BAUD14400"),
                    0x004e_a000 => f.write_str("BAUD19200"),
                    0x0075_f000 => f.write_str("BAUD28800"),
                    0x0080_0000 => f.write_str("BAUD31250"),
                    0x009d_5000 => f.write_str("BAUD38400"),
                    0x00e5_0000 => f.write_str("BAUD56000"),
                    0x00eb_f000 => f.write_str("BAUD57600"),
                    0x013a_9000 => f.write_str("BAUD76800"),
                    0x01d7_e000 => f.write_str("BAUD115200"),
                    0x03af_b000 => f.write_str("BAUD230400"),
                    0x0400_0000 => f.write_str("BAUD250000"),
                    0x075f_7000 => f.write_str("BAUD460800"),
                    0x0ebe_d000 => f.write_str("BAUD921600"),
                    0x1000_0000 => f.write_str("BAUD1M"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Baudrate {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0004_f000 => defmt::write!(f, "BAUD1200"),
                    0x0009_d000 => defmt::write!(f, "BAUD2400"),
                    0x0013_b000 => defmt::write!(f, "BAUD4800"),
                    0x0027_5000 => defmt::write!(f, "BAUD9600"),
                    0x003b_0000 => defmt::write!(f, "BAUD14400"),
                    0x004e_a000 => defmt::write!(f, "BAUD19200"),
                    0x0075_f000 => defmt::write!(f, "BAUD28800"),
                    0x0080_0000 => defmt::write!(f, "BAUD31250"),
                    0x009d_5000 => defmt::write!(f, "BAUD38400"),
                    0x00e5_0000 => defmt::write!(f, "BAUD56000"),
                    0x00eb_f000 => defmt::write!(f, "BAUD57600"),
                    0x013a_9000 => defmt::write!(f, "BAUD76800"),
                    0x01d7_e000 => defmt::write!(f, "BAUD115200"),
                    0x03af_b000 => defmt::write!(f, "BAUD230400"),
                    0x0400_0000 => defmt::write!(f, "BAUD250000"),
                    0x075f_7000 => defmt::write!(f, "BAUD460800"),
                    0x0ebe_d000 => defmt::write!(f, "BAUD921600"),
                    0x1000_0000 => defmt::write!(f, "BAUD1M"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u32> for Baudrate {
            #[inline(always)]
            fn from(val: u32) -> Baudrate {
                Baudrate::from_bits(val)
            }
        }
        impl From<Baudrate> for u32 {
            #[inline(always)]
            fn from(val: Baudrate) -> u32 {
                Baudrate::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum ConfigParity {
            #[doc = "Exclude parity bit"]
            EXCLUDED = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            #[doc = "Include parity bit"]
            INCLUDED = 0x07,
        }
        impl ConfigParity {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> ConfigParity {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for ConfigParity {
            #[inline(always)]
            fn from(val: u8) -> ConfigParity {
                ConfigParity::from_bits(val)
            }
        }
        impl From<ConfigParity> for u8 {
            #[inline(always)]
            fn from(val: ConfigParity) -> u8 {
                ConfigParity::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Enable {
            #[doc = "Disable UART"]
            DISABLED = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            #[doc = "Enable UART"]
            ENABLED = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
            _RESERVED_8 = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl Enable {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Enable {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Enable {
            #[inline(always)]
            fn from(val: u8) -> Enable {
                Enable::from_bits(val)
            }
        }
        impl From<Enable> for u8 {
            #[inline(always)]
            fn from(val: Enable) -> u8 {
                Enable::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Stop {
            #[doc = "One stop bit"]
            ONE = 0x0,
            #[doc = "Two stop bits"]
            TWO = 0x01,
        }
        impl Stop {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Stop {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Stop {
            #[inline(always)]
            fn from(val: u8) -> Stop {
                Stop::from_bits(val)
            }
        }
        impl From<Stop> for u8 {
            #[inline(always)]
            fn from(val: Stop) -> u8 {
                Stop::to_bits(val)
            }
        }
    }
}
pub mod uarte {
    #[doc = "Unspecified"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psel {
        ptr: *mut u8,
    }
    unsafe impl Send for Psel {}
    unsafe impl Sync for Psel {}
    impl Psel {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Pin select for RTS signal"]
        #[inline(always)]
        pub const fn rts(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Pin select for TXD signal"]
        #[inline(always)]
        pub const fn txd(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Pin select for CTS signal"]
        #[inline(always)]
        pub const fn cts(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Pin select for RXD signal"]
        #[inline(always)]
        pub const fn rxd(self) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
    }
    #[doc = "RXD EasyDMA channel"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxd {
        ptr: *mut u8,
    }
    unsafe impl Send for Rxd {}
    unsafe impl Sync for Rxd {}
    impl Rxd {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Data pointer"]
        #[inline(always)]
        pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Maximum number of bytes in receive buffer"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::RxdMaxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::RxdAmount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
    }
    #[doc = "TXD EasyDMA channel"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txd {
        ptr: *mut u8,
    }
    unsafe impl Send for Txd {}
    unsafe impl Sync for Txd {}
    impl Txd {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Data pointer"]
        #[inline(always)]
        pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Maximum number of bytes in transmit buffer"]
        #[inline(always)]
        pub const fn maxcnt(self) -> crate::common::Reg<regs::TxdMaxcnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[inline(always)]
        pub const fn amount(self) -> crate::common::Reg<regs::TxdAmount, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
    }
    #[doc = "UART with EasyDMA"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uarte {
        ptr: *mut u8,
    }
    unsafe impl Send for Uarte {}
    unsafe impl Sync for Uarte {}
    impl Uarte {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Start UART receiver"]
        #[inline(always)]
        pub const fn tasks_startrx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Stop UART receiver"]
        #[inline(always)]
        pub const fn tasks_stoprx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Start UART transmitter"]
        #[inline(always)]
        pub const fn tasks_starttx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Stop UART transmitter"]
        #[inline(always)]
        pub const fn tasks_stoptx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Flush RX FIFO into RX buffer"]
        #[inline(always)]
        pub const fn tasks_flushrx(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
        }
        #[doc = "CTS is activated (set low). Clear To Send."]
        #[inline(always)]
        pub const fn events_cts(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "CTS is deactivated (set high). Not Clear To Send."]
        #[inline(always)]
        pub const fn events_ncts(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "Data received in RXD (but potentially not yet transferred to Data RAM)"]
        #[inline(always)]
        pub const fn events_rxdrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "Receive buffer is filled up"]
        #[inline(always)]
        pub const fn events_endrx(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
        }
        #[doc = "Data sent from TXD"]
        #[inline(always)]
        pub const fn events_txdrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
        }
        #[doc = "Last TX byte transmitted"]
        #[inline(always)]
        pub const fn events_endtx(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
        }
        #[doc = "Error detected"]
        #[inline(always)]
        pub const fn events_error(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
        }
        #[doc = "Receiver timeout"]
        #[inline(always)]
        pub const fn events_rxto(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
        }
        #[doc = "UART receiver has started"]
        #[inline(always)]
        pub const fn events_rxstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
        }
        #[doc = "UART transmitter has started"]
        #[inline(always)]
        pub const fn events_txstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
        }
        #[doc = "Transmitter stopped"]
        #[inline(always)]
        pub const fn events_txstopped(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0158usize) as _) }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[inline(always)]
        pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Enable or disable interrupt"]
        #[inline(always)]
        pub const fn inten(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Error source This register is read/write one to clear."]
        #[inline(always)]
        pub const fn errorsrc(self) -> crate::common::Reg<regs::Errorsrc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0480usize) as _) }
        }
        #[doc = "Enable UART"]
        #[inline(always)]
        pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
        }
        #[doc = "Unspecified"]
        #[inline(always)]
        pub const fn psel(self) -> Psel {
            unsafe { Psel::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Baud rate. Accuracy depends on the HFCLK source selected."]
        #[inline(always)]
        pub const fn baudrate(self) -> crate::common::Reg<regs::Baudrate, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
        }
        #[doc = "RXD EasyDMA channel"]
        #[inline(always)]
        pub const fn rxd(self) -> Rxd {
            unsafe { Rxd::from_ptr(self.ptr.add(0x0534usize) as _) }
        }
        #[doc = "TXD EasyDMA channel"]
        #[inline(always)]
        pub const fn txd(self) -> Txd {
            unsafe { Txd::from_ptr(self.ptr.add(0x0544usize) as _) }
        }
        #[doc = "Configuration of parity and hardware flow control"]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x056cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Baud rate. Accuracy depends on the HFCLK source selected."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Baudrate(pub u32);
        impl Baudrate {
            #[doc = "Baud rate"]
            #[inline(always)]
            pub const fn baudrate(&self) -> super::vals::Baudrate {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Baudrate::from_bits(val as u32)
            }
            #[doc = "Baud rate"]
            #[inline(always)]
            pub fn set_baudrate(&mut self, val: super::vals::Baudrate) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize))
                    | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Baudrate {
            #[inline(always)]
            fn default() -> Baudrate {
                Baudrate(0)
            }
        }
        impl core::fmt::Debug for Baudrate {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Baudrate")
                    .field("baudrate", &self.baudrate())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Baudrate {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Baudrate {
                    baudrate: super::vals::Baudrate,
                }
                let proxy = Baudrate {
                    baudrate: self.baudrate(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Configuration of parity and hardware flow control"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Hardware flow control"]
            #[inline(always)]
            pub const fn hwfc(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Hardware flow control"]
            #[inline(always)]
            pub fn set_hwfc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Parity"]
            #[inline(always)]
            pub const fn parity(&self) -> super::vals::ConfigParity {
                let val = (self.0 >> 1usize) & 0x07;
                super::vals::ConfigParity::from_bits(val as u8)
            }
            #[doc = "Parity"]
            #[inline(always)]
            pub fn set_parity(&mut self, val: super::vals::ConfigParity) {
                self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
            }
            #[doc = "Stop bits"]
            #[inline(always)]
            pub const fn stop(&self) -> super::vals::Stop {
                let val = (self.0 >> 4usize) & 0x01;
                super::vals::Stop::from_bits(val as u8)
            }
            #[doc = "Stop bits"]
            #[inline(always)]
            pub fn set_stop(&mut self, val: super::vals::Stop) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
            }
        }
        impl Default for Config {
            #[inline(always)]
            fn default() -> Config {
                Config(0)
            }
        }
        impl core::fmt::Debug for Config {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Config")
                    .field("hwfc", &self.hwfc())
                    .field("parity", &self.parity())
                    .field("stop", &self.stop())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Config {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Config {
                    hwfc: bool,
                    parity: super::vals::ConfigParity,
                    stop: super::vals::Stop,
                }
                let proxy = Config {
                    hwfc: self.hwfc(),
                    parity: self.parity(),
                    stop: self.stop(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Enable UART"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Enable(pub u32);
        impl Enable {
            #[doc = "Enable or disable UARTE"]
            #[inline(always)]
            pub const fn enable(&self) -> super::vals::Enable {
                let val = (self.0 >> 0usize) & 0x0f;
                super::vals::Enable::from_bits(val as u8)
            }
            #[doc = "Enable or disable UARTE"]
            #[inline(always)]
            pub fn set_enable(&mut self, val: super::vals::Enable) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Enable {
            #[inline(always)]
            fn default() -> Enable {
                Enable(0)
            }
        }
        impl core::fmt::Debug for Enable {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Enable")
                    .field("enable", &self.enable())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Enable {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Enable {
                    enable: super::vals::Enable,
                }
                let proxy = Enable {
                    enable: self.enable(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Error source This register is read/write one to clear."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Errorsrc(pub u32);
        impl Errorsrc {
            #[doc = "Overrun error"]
            #[inline(always)]
            pub const fn overrun(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Overrun error"]
            #[inline(always)]
            pub fn set_overrun(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Parity error"]
            #[inline(always)]
            pub const fn parity(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Parity error"]
            #[inline(always)]
            pub fn set_parity(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Framing error occurred"]
            #[inline(always)]
            pub const fn framing(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Framing error occurred"]
            #[inline(always)]
            pub fn set_framing(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Break condition"]
            #[inline(always)]
            pub const fn break_(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Break condition"]
            #[inline(always)]
            pub fn set_break_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Errorsrc {
            #[inline(always)]
            fn default() -> Errorsrc {
                Errorsrc(0)
            }
        }
        impl core::fmt::Debug for Errorsrc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Errorsrc")
                    .field("overrun", &self.overrun())
                    .field("parity", &self.parity())
                    .field("framing", &self.framing())
                    .field("break_", &self.break_())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Errorsrc {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Errorsrc {
                    overrun: bool,
                    parity: bool,
                    framing: bool,
                    break_: bool,
                }
                let proxy = Errorsrc {
                    overrun: self.overrun(),
                    parity: self.parity(),
                    framing: self.framing(),
                    break_: self.break_(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Enable or disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Enable or disable interrupt for event CTS"]
            #[inline(always)]
            pub const fn cts(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event CTS"]
            #[inline(always)]
            pub fn set_cts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Enable or disable interrupt for event NCTS"]
            #[inline(always)]
            pub const fn ncts(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event NCTS"]
            #[inline(always)]
            pub fn set_ncts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable or disable interrupt for event RXDRDY"]
            #[inline(always)]
            pub const fn rxdrdy(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RXDRDY"]
            #[inline(always)]
            pub fn set_rxdrdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable or disable interrupt for event ENDRX"]
            #[inline(always)]
            pub const fn endrx(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event ENDRX"]
            #[inline(always)]
            pub fn set_endrx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Enable or disable interrupt for event TXDRDY"]
            #[inline(always)]
            pub const fn txdrdy(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event TXDRDY"]
            #[inline(always)]
            pub fn set_txdrdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Enable or disable interrupt for event ENDTX"]
            #[inline(always)]
            pub const fn endtx(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event ENDTX"]
            #[inline(always)]
            pub fn set_endtx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Enable or disable interrupt for event ERROR"]
            #[inline(always)]
            pub const fn error(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event ERROR"]
            #[inline(always)]
            pub fn set_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Enable or disable interrupt for event RXTO"]
            #[inline(always)]
            pub const fn rxto(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RXTO"]
            #[inline(always)]
            pub fn set_rxto(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "Enable or disable interrupt for event RXSTARTED"]
            #[inline(always)]
            pub const fn rxstarted(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event RXSTARTED"]
            #[inline(always)]
            pub fn set_rxstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "Enable or disable interrupt for event TXSTARTED"]
            #[inline(always)]
            pub const fn txstarted(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event TXSTARTED"]
            #[inline(always)]
            pub fn set_txstarted(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "Enable or disable interrupt for event TXSTOPPED"]
            #[inline(always)]
            pub const fn txstopped(&self) -> bool {
                let val = (self.0 >> 22usize) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable interrupt for event TXSTOPPED"]
            #[inline(always)]
            pub fn set_txstopped(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("cts", &self.cts())
                    .field("ncts", &self.ncts())
                    .field("rxdrdy", &self.rxdrdy())
                    .field("endrx", &self.endrx())
                    .field("txdrdy", &self.txdrdy())
                    .field("endtx", &self.endtx())
                    .field("error", &self.error())
                    .field("rxto", &self.rxto())
                    .field("rxstarted", &self.rxstarted())
                    .field("txstarted", &self.txstarted())
                    .field("txstopped", &self.txstopped())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    cts: bool,
                    ncts: bool,
                    rxdrdy: bool,
                    endrx: bool,
                    txdrdy: bool,
                    endtx: bool,
                    error: bool,
                    rxto: bool,
                    rxstarted: bool,
                    txstarted: bool,
                    txstopped: bool,
                }
                let proxy = Int {
                    cts: self.cts(),
                    ncts: self.ncts(),
                    rxdrdy: self.rxdrdy(),
                    endrx: self.endrx(),
                    txdrdy: self.txdrdy(),
                    endtx: self.endtx(),
                    error: self.error(),
                    rxto: self.rxto(),
                    rxstarted: self.rxstarted(),
                    txstarted: self.txstarted(),
                    txstopped: self.txstopped(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdAmount(pub u32);
        impl RxdAmount {
            #[doc = "Number of bytes transferred in the last transaction"]
            #[inline(always)]
            pub const fn amount(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x03ff;
                val as u16
            }
            #[doc = "Number of bytes transferred in the last transaction"]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
            }
        }
        impl Default for RxdAmount {
            #[inline(always)]
            fn default() -> RxdAmount {
                RxdAmount(0)
            }
        }
        impl core::fmt::Debug for RxdAmount {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RxdAmount")
                    .field("amount", &self.amount())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RxdAmount {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct RxdAmount {
                    amount: u16,
                }
                let proxy = RxdAmount {
                    amount: self.amount(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Maximum number of bytes in receive buffer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxdMaxcnt(pub u32);
        impl RxdMaxcnt {
            #[doc = "Maximum number of bytes in receive buffer"]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x03ff;
                val as u16
            }
            #[doc = "Maximum number of bytes in receive buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
            }
        }
        impl Default for RxdMaxcnt {
            #[inline(always)]
            fn default() -> RxdMaxcnt {
                RxdMaxcnt(0)
            }
        }
        impl core::fmt::Debug for RxdMaxcnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RxdMaxcnt")
                    .field("maxcnt", &self.maxcnt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RxdMaxcnt {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct RxdMaxcnt {
                    maxcnt: u16,
                }
                let proxy = RxdMaxcnt {
                    maxcnt: self.maxcnt(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Shortcuts between local events and tasks"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Shorts(pub u32);
        impl Shorts {
            #[doc = "Shortcut between event ENDRX and task STARTRX"]
            #[inline(always)]
            pub const fn endrx_startrx(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event ENDRX and task STARTRX"]
            #[inline(always)]
            pub fn set_endrx_startrx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Shortcut between event ENDRX and task STOPRX"]
            #[inline(always)]
            pub const fn endrx_stoprx(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Shortcut between event ENDRX and task STOPRX"]
            #[inline(always)]
            pub fn set_endrx_stoprx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
        }
        impl Default for Shorts {
            #[inline(always)]
            fn default() -> Shorts {
                Shorts(0)
            }
        }
        impl core::fmt::Debug for Shorts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Shorts")
                    .field("endrx_startrx", &self.endrx_startrx())
                    .field("endrx_stoprx", &self.endrx_stoprx())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Shorts {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Shorts {
                    endrx_startrx: bool,
                    endrx_stoprx: bool,
                }
                let proxy = Shorts {
                    endrx_startrx: self.endrx_startrx(),
                    endrx_stoprx: self.endrx_stoprx(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Number of bytes transferred in the last transaction"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdAmount(pub u32);
        impl TxdAmount {
            #[doc = "Number of bytes transferred in the last transaction"]
            #[inline(always)]
            pub const fn amount(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x03ff;
                val as u16
            }
            #[doc = "Number of bytes transferred in the last transaction"]
            #[inline(always)]
            pub fn set_amount(&mut self, val: u16) {
                self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
            }
        }
        impl Default for TxdAmount {
            #[inline(always)]
            fn default() -> TxdAmount {
                TxdAmount(0)
            }
        }
        impl core::fmt::Debug for TxdAmount {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("TxdAmount")
                    .field("amount", &self.amount())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for TxdAmount {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct TxdAmount {
                    amount: u16,
                }
                let proxy = TxdAmount {
                    amount: self.amount(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Maximum number of bytes in transmit buffer"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxdMaxcnt(pub u32);
        impl TxdMaxcnt {
            #[doc = "Maximum number of bytes in transmit buffer"]
            #[inline(always)]
            pub const fn maxcnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x03ff;
                val as u16
            }
            #[doc = "Maximum number of bytes in transmit buffer"]
            #[inline(always)]
            pub fn set_maxcnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
            }
        }
        impl Default for TxdMaxcnt {
            #[inline(always)]
            fn default() -> TxdMaxcnt {
                TxdMaxcnt(0)
            }
        }
        impl core::fmt::Debug for TxdMaxcnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("TxdMaxcnt")
                    .field("maxcnt", &self.maxcnt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for TxdMaxcnt {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct TxdMaxcnt {
                    maxcnt: u16,
                }
                let proxy = TxdMaxcnt {
                    maxcnt: self.maxcnt(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Baudrate(pub u32);
        impl Baudrate {
            #[doc = "1200 baud (actual rate: 1205)"]
            pub const BAUD1200: Self = Self(0x0004_f000);
            #[doc = "2400 baud (actual rate: 2396)"]
            pub const BAUD2400: Self = Self(0x0009_d000);
            #[doc = "4800 baud (actual rate: 4808)"]
            pub const BAUD4800: Self = Self(0x0013_b000);
            #[doc = "9600 baud (actual rate: 9598)"]
            pub const BAUD9600: Self = Self(0x0027_5000);
            #[doc = "14400 baud (actual rate: 14401)"]
            pub const BAUD14400: Self = Self(0x003a_f000);
            #[doc = "19200 baud (actual rate: 19208)"]
            pub const BAUD19200: Self = Self(0x004e_a000);
            #[doc = "28800 baud (actual rate: 28777)"]
            pub const BAUD28800: Self = Self(0x0075_c000);
            #[doc = "31250 baud"]
            pub const BAUD31250: Self = Self(0x0080_0000);
            #[doc = "38400 baud (actual rate: 38369)"]
            pub const BAUD38400: Self = Self(0x009d_0000);
            #[doc = "56000 baud (actual rate: 55944)"]
            pub const BAUD56000: Self = Self(0x00e5_0000);
            #[doc = "57600 baud (actual rate: 57554)"]
            pub const BAUD57600: Self = Self(0x00eb_0000);
            #[doc = "76800 baud (actual rate: 76923)"]
            pub const BAUD76800: Self = Self(0x013a_9000);
            #[doc = "115200 baud (actual rate: 115108)"]
            pub const BAUD115200: Self = Self(0x01d6_0000);
            #[doc = "230400 baud (actual rate: 231884)"]
            pub const BAUD230400: Self = Self(0x03b0_0000);
            #[doc = "250000 baud"]
            pub const BAUD250000: Self = Self(0x0400_0000);
            #[doc = "460800 baud (actual rate: 457143)"]
            pub const BAUD460800: Self = Self(0x0740_0000);
            #[doc = "921600 baud (actual rate: 941176)"]
            pub const BAUD921600: Self = Self(0x0f00_0000);
            #[doc = "1 megabaud"]
            pub const BAUD1M: Self = Self(0x1000_0000);
        }
        impl Baudrate {
            pub const fn from_bits(val: u32) -> Baudrate {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl core::fmt::Debug for Baudrate {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0004_f000 => f.write_str("BAUD1200"),
                    0x0009_d000 => f.write_str("BAUD2400"),
                    0x0013_b000 => f.write_str("BAUD4800"),
                    0x0027_5000 => f.write_str("BAUD9600"),
                    0x003a_f000 => f.write_str("BAUD14400"),
                    0x004e_a000 => f.write_str("BAUD19200"),
                    0x0075_c000 => f.write_str("BAUD28800"),
                    0x0080_0000 => f.write_str("BAUD31250"),
                    0x009d_0000 => f.write_str("BAUD38400"),
                    0x00e5_0000 => f.write_str("BAUD56000"),
                    0x00eb_0000 => f.write_str("BAUD57600"),
                    0x013a_9000 => f.write_str("BAUD76800"),
                    0x01d6_0000 => f.write_str("BAUD115200"),
                    0x03b0_0000 => f.write_str("BAUD230400"),
                    0x0400_0000 => f.write_str("BAUD250000"),
                    0x0740_0000 => f.write_str("BAUD460800"),
                    0x0f00_0000 => f.write_str("BAUD921600"),
                    0x1000_0000 => f.write_str("BAUD1M"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Baudrate {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0004_f000 => defmt::write!(f, "BAUD1200"),
                    0x0009_d000 => defmt::write!(f, "BAUD2400"),
                    0x0013_b000 => defmt::write!(f, "BAUD4800"),
                    0x0027_5000 => defmt::write!(f, "BAUD9600"),
                    0x003a_f000 => defmt::write!(f, "BAUD14400"),
                    0x004e_a000 => defmt::write!(f, "BAUD19200"),
                    0x0075_c000 => defmt::write!(f, "BAUD28800"),
                    0x0080_0000 => defmt::write!(f, "BAUD31250"),
                    0x009d_0000 => defmt::write!(f, "BAUD38400"),
                    0x00e5_0000 => defmt::write!(f, "BAUD56000"),
                    0x00eb_0000 => defmt::write!(f, "BAUD57600"),
                    0x013a_9000 => defmt::write!(f, "BAUD76800"),
                    0x01d6_0000 => defmt::write!(f, "BAUD115200"),
                    0x03b0_0000 => defmt::write!(f, "BAUD230400"),
                    0x0400_0000 => defmt::write!(f, "BAUD250000"),
                    0x0740_0000 => defmt::write!(f, "BAUD460800"),
                    0x0f00_0000 => defmt::write!(f, "BAUD921600"),
                    0x1000_0000 => defmt::write!(f, "BAUD1M"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u32> for Baudrate {
            #[inline(always)]
            fn from(val: u32) -> Baudrate {
                Baudrate::from_bits(val)
            }
        }
        impl From<Baudrate> for u32 {
            #[inline(always)]
            fn from(val: Baudrate) -> u32 {
                Baudrate::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum ConfigParity {
            #[doc = "Exclude parity bit"]
            EXCLUDED = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            #[doc = "Include even parity bit"]
            INCLUDED = 0x07,
        }
        impl ConfigParity {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> ConfigParity {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for ConfigParity {
            #[inline(always)]
            fn from(val: u8) -> ConfigParity {
                ConfigParity::from_bits(val)
            }
        }
        impl From<ConfigParity> for u8 {
            #[inline(always)]
            fn from(val: ConfigParity) -> u8 {
                ConfigParity::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Enable {
            #[doc = "Disable UARTE"]
            DISABLED = 0x0,
            _RESERVED_1 = 0x01,
            _RESERVED_2 = 0x02,
            _RESERVED_3 = 0x03,
            _RESERVED_4 = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
            #[doc = "Enable UARTE"]
            ENABLED = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
        }
        impl Enable {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Enable {
                unsafe { core::mem::transmute(val & 0x0f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Enable {
            #[inline(always)]
            fn from(val: u8) -> Enable {
                Enable::from_bits(val)
            }
        }
        impl From<Enable> for u8 {
            #[inline(always)]
            fn from(val: Enable) -> u8 {
                Enable::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Stop {
            #[doc = "One stop bit"]
            ONE = 0x0,
            #[doc = "Two stop bits"]
            TWO = 0x01,
        }
        impl Stop {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Stop {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Stop {
            #[inline(always)]
            fn from(val: u8) -> Stop {
                Stop::from_bits(val)
            }
        }
        impl From<Stop> for u8 {
            #[inline(always)]
            fn from(val: Stop) -> u8 {
                Stop::to_bits(val)
            }
        }
    }
}
pub mod uicr {
    #[doc = "User information configuration registers"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uicr {
        ptr: *mut u8,
    }
    unsafe impl Send for Uicr {}
    unsafe impl Sync for Uicr {}
    impl Uicr {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Description collection: Reserved for Nordic firmware design"]
        #[inline(always)]
        pub const fn nrffw(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 13usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Reserved for Nordic hardware design"]
        #[inline(always)]
        pub const fn nrfhw(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 12usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Reserved for customer"]
        #[inline(always)]
        pub const fn customer(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 32usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Reserved for Nordic MDK"]
        #[inline(always)]
        pub const fn nrfmdk(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
        }
        #[doc = "Description collection: Mapping of the nRESET function (see POWER chapter for details)"]
        #[inline(always)]
        pub const fn pselreset(
            self,
            n: usize,
        ) -> crate::common::Reg<super::shared::regs::Psel, crate::common::RW> {
            assert!(n < 2usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize + n * 4usize) as _) }
        }
        #[doc = "Access port protection"]
        #[inline(always)]
        pub const fn approtect(self) -> crate::common::Reg<regs::Approtect, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0208usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Access port protection"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Approtect(pub u32);
        impl Approtect {
            #[doc = "Enable or disable access port protection."]
            #[inline(always)]
            pub const fn pall(&self) -> super::vals::Pall {
                let val = (self.0 >> 0usize) & 0xff;
                super::vals::Pall::from_bits(val as u8)
            }
            #[doc = "Enable or disable access port protection."]
            #[inline(always)]
            pub fn set_pall(&mut self, val: super::vals::Pall) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Approtect {
            #[inline(always)]
            fn default() -> Approtect {
                Approtect(0)
            }
        }
        impl core::fmt::Debug for Approtect {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Approtect")
                    .field("pall", &self.pall())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Approtect {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Approtect {
                    pall: super::vals::Pall,
                }
                let proxy = Approtect { pall: self.pall() };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Pall(pub u8);
        impl Pall {
            #[doc = "Enable"]
            pub const ENABLED: Self = Self(0x0);
            #[doc = "Hardware disable of access port protection for devices where access port protection is controlled by hardware and software"]
            pub const HW_DISABLED: Self = Self(0x5a);
            #[doc = "Hardware disable of access port protection for devices where access port protection is controlled by hardware"]
            pub const DISABLED: Self = Self(0xff);
        }
        impl Pall {
            pub const fn from_bits(val: u8) -> Pall {
                Self(val & 0xff)
            }
            pub const fn to_bits(self) -> u8 {
                self.0
            }
        }
        impl core::fmt::Debug for Pall {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x0 => f.write_str("ENABLED"),
                    0x5a => f.write_str("HW_DISABLED"),
                    0xff => f.write_str("DISABLED"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Pall {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x0 => defmt::write!(f, "ENABLED"),
                    0x5a => defmt::write!(f, "HW_DISABLED"),
                    0xff => defmt::write!(f, "DISABLED"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u8> for Pall {
            #[inline(always)]
            fn from(val: u8) -> Pall {
                Pall::from_bits(val)
            }
        }
        impl From<Pall> for u8 {
            #[inline(always)]
            fn from(val: Pall) -> u8 {
                Pall::to_bits(val)
            }
        }
    }
}
pub mod wdt {
    #[doc = "Watchdog Timer"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wdt {
        ptr: *mut u8,
    }
    unsafe impl Send for Wdt {}
    unsafe impl Sync for Wdt {}
    impl Wdt {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Start the watchdog"]
        #[inline(always)]
        pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Watchdog timeout"]
        #[inline(always)]
        pub const fn events_timeout(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[doc = "Enable interrupt"]
        #[inline(always)]
        pub const fn intenset(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
        }
        #[doc = "Disable interrupt"]
        #[inline(always)]
        pub const fn intenclr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
        }
        #[doc = "Run status"]
        #[inline(always)]
        pub const fn runstatus(self) -> crate::common::Reg<regs::Runstatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
        }
        #[doc = "Request status"]
        #[inline(always)]
        pub const fn reqstatus(self) -> crate::common::Reg<regs::Reqstatus, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0404usize) as _) }
        }
        #[doc = "Counter reload value"]
        #[inline(always)]
        pub const fn crv(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
        }
        #[doc = "Enable register for reload request registers"]
        #[inline(always)]
        pub const fn rren(self) -> crate::common::Reg<regs::Rren, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
        }
        #[doc = "Configuration register"]
        #[inline(always)]
        pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
        }
        #[doc = "Description collection: Reload request n"]
        #[inline(always)]
        pub const fn rr(self, n: usize) -> crate::common::Reg<regs::Rr, crate::common::W> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0600usize + n * 4usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Configuration register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Config(pub u32);
        impl Config {
            #[doc = "Configure the watchdog to either be paused, or kept running, while the CPU is sleeping"]
            #[inline(always)]
            pub const fn sleep(&self) -> super::vals::Sleep {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Sleep::from_bits(val as u8)
            }
            #[doc = "Configure the watchdog to either be paused, or kept running, while the CPU is sleeping"]
            #[inline(always)]
            pub fn set_sleep(&mut self, val: super::vals::Sleep) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Configure the watchdog to either be paused, or kept running, while the CPU is halted by the debugger"]
            #[inline(always)]
            pub const fn halt(&self) -> super::vals::Halt {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Halt::from_bits(val as u8)
            }
            #[doc = "Configure the watchdog to either be paused, or kept running, while the CPU is halted by the debugger"]
            #[inline(always)]
            pub fn set_halt(&mut self, val: super::vals::Halt) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Config {
            #[inline(always)]
            fn default() -> Config {
                Config(0)
            }
        }
        impl core::fmt::Debug for Config {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Config")
                    .field("sleep", &self.sleep())
                    .field("halt", &self.halt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Config {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Config {
                    sleep: super::vals::Sleep,
                    halt: super::vals::Halt,
                }
                let proxy = Config {
                    sleep: self.sleep(),
                    halt: self.halt(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Disable interrupt"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Int(pub u32);
        impl Int {
            #[doc = "Write '1' to disable interrupt for event TIMEOUT"]
            #[inline(always)]
            pub const fn timeout(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Write '1' to disable interrupt for event TIMEOUT"]
            #[inline(always)]
            pub fn set_timeout(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Int {
            #[inline(always)]
            fn default() -> Int {
                Int(0)
            }
        }
        impl core::fmt::Debug for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Int")
                    .field("timeout", &self.timeout())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Int {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Int {
                    timeout: bool,
                }
                let proxy = Int {
                    timeout: self.timeout(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Request status"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Reqstatus(pub u32);
        impl Reqstatus {
            #[doc = "Request status for RR\\[0\\] register"]
            #[inline(always)]
            pub const fn rr(&self, n: usize) -> bool {
                assert!(n < 8usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Request status for RR\\[0\\] register"]
            #[inline(always)]
            pub fn set_rr(&mut self, n: usize, val: bool) {
                assert!(n < 8usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Reqstatus {
            #[inline(always)]
            fn default() -> Reqstatus {
                Reqstatus(0)
            }
        }
        impl core::fmt::Debug for Reqstatus {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Reqstatus")
                    .field(
                        "rr",
                        &[
                            self.rr(0usize),
                            self.rr(1usize),
                            self.rr(2usize),
                            self.rr(3usize),
                            self.rr(4usize),
                            self.rr(5usize),
                            self.rr(6usize),
                            self.rr(7usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Reqstatus {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Reqstatus {
                    rr: [bool; 8usize],
                }
                let proxy = Reqstatus {
                    rr: [
                        self.rr(0usize),
                        self.rr(1usize),
                        self.rr(2usize),
                        self.rr(3usize),
                        self.rr(4usize),
                        self.rr(5usize),
                        self.rr(6usize),
                        self.rr(7usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Description collection: Reload request n"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rr(pub u32);
        impl Rr {
            #[doc = "Reload request register"]
            #[inline(always)]
            pub const fn rr(&self) -> super::vals::Rr {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                super::vals::Rr::from_bits(val as u32)
            }
            #[doc = "Reload request register"]
            #[inline(always)]
            pub fn set_rr(&mut self, val: super::vals::Rr) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize))
                    | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Rr {
            #[inline(always)]
            fn default() -> Rr {
                Rr(0)
            }
        }
        impl core::fmt::Debug for Rr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Rr").field("rr", &self.rr()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rr {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Rr {
                    rr: super::vals::Rr,
                }
                let proxy = Rr { rr: self.rr() };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Enable register for reload request registers"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rren(pub u32);
        impl Rren {
            #[doc = "Enable or disable RR\\[0\\] register"]
            #[inline(always)]
            pub const fn rr(&self, n: usize) -> bool {
                assert!(n < 8usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Enable or disable RR\\[0\\] register"]
            #[inline(always)]
            pub fn set_rr(&mut self, n: usize, val: bool) {
                assert!(n < 8usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Rren {
            #[inline(always)]
            fn default() -> Rren {
                Rren(0)
            }
        }
        impl core::fmt::Debug for Rren {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Rren")
                    .field(
                        "rr",
                        &[
                            self.rr(0usize),
                            self.rr(1usize),
                            self.rr(2usize),
                            self.rr(3usize),
                            self.rr(4usize),
                            self.rr(5usize),
                            self.rr(6usize),
                            self.rr(7usize),
                        ],
                    )
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rren {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Rren {
                    rr: [bool; 8usize],
                }
                let proxy = Rren {
                    rr: [
                        self.rr(0usize),
                        self.rr(1usize),
                        self.rr(2usize),
                        self.rr(3usize),
                        self.rr(4usize),
                        self.rr(5usize),
                        self.rr(6usize),
                        self.rr(7usize),
                    ],
                };
                defmt::write!(f, "{}", proxy)
            }
        }
        #[doc = "Run status"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Runstatus(pub u32);
        impl Runstatus {
            #[doc = "Indicates whether or not the watchdog is running"]
            #[inline(always)]
            pub const fn runstatus(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Indicates whether or not the watchdog is running"]
            #[inline(always)]
            pub fn set_runstatus(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Runstatus {
            #[inline(always)]
            fn default() -> Runstatus {
                Runstatus(0)
            }
        }
        impl core::fmt::Debug for Runstatus {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Runstatus")
                    .field("runstatus", &self.runstatus())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Runstatus {
            fn format(&self, f: defmt::Formatter) {
                #[derive(defmt :: Format)]
                struct Runstatus {
                    runstatus: bool,
                }
                let proxy = Runstatus {
                    runstatus: self.runstatus(),
                };
                defmt::write!(f, "{}", proxy)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Halt {
            #[doc = "Pause watchdog while the CPU is halted by the debugger"]
            PAUSE = 0x0,
            #[doc = "Keep the watchdog running while the CPU is halted by the debugger"]
            RUN = 0x01,
        }
        impl Halt {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Halt {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Halt {
            #[inline(always)]
            fn from(val: u8) -> Halt {
                Halt::from_bits(val)
            }
        }
        impl From<Halt> for u8 {
            #[inline(always)]
            fn from(val: Halt) -> u8 {
                Halt::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Rr(pub u32);
        impl Rr {
            #[doc = "Value to request a reload of the watchdog timer"]
            pub const RELOAD: Self = Self(0x6e52_4635);
        }
        impl Rr {
            pub const fn from_bits(val: u32) -> Rr {
                Self(val & 0xffff_ffff)
            }
            pub const fn to_bits(self) -> u32 {
                self.0
            }
        }
        impl core::fmt::Debug for Rr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x6e52_4635 => f.write_str("RELOAD"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rr {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x6e52_4635 => defmt::write!(f, "RELOAD"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u32> for Rr {
            #[inline(always)]
            fn from(val: u32) -> Rr {
                Rr::from_bits(val)
            }
        }
        impl From<Rr> for u32 {
            #[inline(always)]
            fn from(val: Rr) -> u32 {
                Rr::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Sleep {
            #[doc = "Pause watchdog while the CPU is sleeping"]
            PAUSE = 0x0,
            #[doc = "Keep the watchdog running while the CPU is sleeping"]
            RUN = 0x01,
        }
        impl Sleep {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Sleep {
                unsafe { core::mem::transmute(val & 0x01) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Sleep {
            #[inline(always)]
            fn from(val: u8) -> Sleep {
                Sleep::from_bits(val)
            }
        }
        impl From<Sleep> for u8 {
            #[inline(always)]
            fn from(val: Sleep) -> u8 {
                Sleep::to_bits(val)
            }
        }
    }
}
