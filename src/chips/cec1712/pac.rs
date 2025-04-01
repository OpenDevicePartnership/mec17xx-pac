#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d771c7b 2025-03-27))"]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - GIRQ08"]
    GIRQ08 = 0,
    #[doc = "1 - GIRQ09"]
    GIRQ09 = 1,
    #[doc = "2 - GIRQ10"]
    GIRQ10 = 2,
    #[doc = "3 - GIRQ11"]
    GIRQ11 = 3,
    #[doc = "4 - GIRQ12"]
    GIRQ12 = 4,
    #[doc = "5 - GIRQ13"]
    GIRQ13 = 5,
    #[doc = "6 - GIRQ14"]
    GIRQ14 = 6,
    #[doc = "7 - GIRQ15"]
    GIRQ15 = 7,
    #[doc = "10 - GIRQ18"]
    GIRQ18 = 10,
    #[doc = "13 - GIRQ21"]
    GIRQ21 = 13,
    #[doc = "14 - GIRQ23"]
    GIRQ23 = 14,
    #[doc = "17 - GIRQ26"]
    GIRQ26 = 17,
    #[doc = "20 - I2CSMB0"]
    I2CSMB0 = 20,
    #[doc = "21 - I2CSMB1"]
    I2CSMB1 = 21,
    #[doc = "22 - I2CSMB2"]
    I2CSMB2 = 22,
    #[doc = "23 - I2CSMB3"]
    I2CSMB3 = 23,
    #[doc = "24 - DMA_CH00"]
    DMA_CH00 = 24,
    #[doc = "25 - DMA_CH01"]
    DMA_CH01 = 25,
    #[doc = "26 - DMA_CH02"]
    DMA_CH02 = 26,
    #[doc = "27 - DMA_CH03"]
    DMA_CH03 = 27,
    #[doc = "28 - DMA_CH04"]
    DMA_CH04 = 28,
    #[doc = "29 - DMA_CH05"]
    DMA_CH05 = 29,
    #[doc = "30 - DMA_CH06"]
    DMA_CH06 = 30,
    #[doc = "31 - DMA_CH07"]
    DMA_CH07 = 31,
    #[doc = "32 - DMA_CH08"]
    DMA_CH08 = 32,
    #[doc = "33 - DMA_CH09"]
    DMA_CH09 = 33,
    #[doc = "34 - DMA_CH10"]
    DMA_CH10 = 34,
    #[doc = "35 - DMA_CH11"]
    DMA_CH11 = 35,
    #[doc = "40 - UART0"]
    UART0 = 40,
    #[doc = "41 - UART1"]
    UART1 = 41,
    #[doc = "44 - UART2"]
    UART2 = 44,
    #[doc = "71 - TACH0"]
    TACH0 = 71,
    #[doc = "72 - TACH1"]
    TACH1 = 72,
    #[doc = "78 - ADC_SNGL"]
    ADC_SNGL = 78,
    #[doc = "79 - ADC_RPT"]
    ADC_RPT = 79,
    #[doc = "83 - LED0"]
    LED0 = 83,
    #[doc = "84 - LED1"]
    LED1 = 84,
    #[doc = "91 - QMSPI"]
    QMSPI = 91,
    #[doc = "111 - TMR"]
    TMR = 111,
    #[doc = "112 - HTMR0"]
    HTMR0 = 112,
    #[doc = "113 - HTMR1"]
    HTMR1 = 113,
    #[doc = "114 - WK"]
    WK = 114,
    #[doc = "115 - WKSUB"]
    WKSUB = 115,
    #[doc = "116 - WKSEC"]
    WKSEC = 116,
    #[doc = "117 - WKSUBSEC"]
    WKSUBSEC = 117,
    #[doc = "118 - SYSPWR"]
    SYSPWR = 118,
    #[doc = "119 - RTC"]
    RTC = 119,
    #[doc = "120 - RTC_ALARM"]
    RTC_ALARM = 120,
    #[doc = "122 - VCI_IN0"]
    VCI_IN0 = 122,
    #[doc = "123 - VCI_IN1"]
    VCI_IN1 = 123,
    #[doc = "136 - TIMER16_0"]
    TIMER16_0 = 136,
    #[doc = "137 - TIMER16_1"]
    TIMER16_1 = 137,
    #[doc = "140 - TIMER32_0"]
    TIMER32_0 = 140,
    #[doc = "141 - TIMER32_1"]
    TIMER32_1 = 141,
    #[doc = "146 - CCT"]
    CCT = 146,
    #[doc = "147 - CCT_CAP0"]
    CCT_CAP0 = 147,
    #[doc = "148 - CCT_CAP1"]
    CCT_CAP1 = 148,
    #[doc = "149 - CCT_CAP2"]
    CCT_CAP2 = 149,
    #[doc = "150 - CCT_CAP3"]
    CCT_CAP3 = 150,
    #[doc = "151 - CCT_CAP4"]
    CCT_CAP4 = 151,
    #[doc = "152 - CCT_CAP5"]
    CCT_CAP5 = 152,
    #[doc = "153 - CCT_CMP0"]
    CCT_CMP0 = 153,
    #[doc = "154 - CCT_CMP1"]
    CCT_CMP1 = 154,
    #[doc = "158 - I2CSMB4"]
    I2CSMB4 = 158,
    #[doc = "168 - I2C0"]
    I2C0 = 168,
    #[doc = "169 - I2C1"]
    I2C1 = 169,
    #[doc = "170 - I2C2"]
    I2C2 = 170,
    #[doc = "171 - WDT"]
    WDT = 171,
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
        fn GIRQ08();
        fn GIRQ09();
        fn GIRQ10();
        fn GIRQ11();
        fn GIRQ12();
        fn GIRQ13();
        fn GIRQ14();
        fn GIRQ15();
        fn GIRQ18();
        fn GIRQ21();
        fn GIRQ23();
        fn GIRQ26();
        fn I2CSMB0();
        fn I2CSMB1();
        fn I2CSMB2();
        fn I2CSMB3();
        fn DMA_CH00();
        fn DMA_CH01();
        fn DMA_CH02();
        fn DMA_CH03();
        fn DMA_CH04();
        fn DMA_CH05();
        fn DMA_CH06();
        fn DMA_CH07();
        fn DMA_CH08();
        fn DMA_CH09();
        fn DMA_CH10();
        fn DMA_CH11();
        fn UART0();
        fn UART1();
        fn UART2();
        fn TACH0();
        fn TACH1();
        fn ADC_SNGL();
        fn ADC_RPT();
        fn LED0();
        fn LED1();
        fn QMSPI();
        fn TMR();
        fn HTMR0();
        fn HTMR1();
        fn WK();
        fn WKSUB();
        fn WKSEC();
        fn WKSUBSEC();
        fn SYSPWR();
        fn RTC();
        fn RTC_ALARM();
        fn VCI_IN0();
        fn VCI_IN1();
        fn TIMER16_0();
        fn TIMER16_1();
        fn TIMER32_0();
        fn TIMER32_1();
        fn CCT();
        fn CCT_CAP0();
        fn CCT_CAP1();
        fn CCT_CAP2();
        fn CCT_CAP3();
        fn CCT_CAP4();
        fn CCT_CAP5();
        fn CCT_CMP0();
        fn CCT_CMP1();
        fn I2CSMB4();
        fn I2C0();
        fn I2C1();
        fn I2C2();
        fn WDT();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[link_section = ".vector_table.interrupts"]
    #[no_mangle]
    pub static __INTERRUPTS: [Vector; 172] = [
        Vector { _handler: GIRQ08 },
        Vector { _handler: GIRQ09 },
        Vector { _handler: GIRQ10 },
        Vector { _handler: GIRQ11 },
        Vector { _handler: GIRQ12 },
        Vector { _handler: GIRQ13 },
        Vector { _handler: GIRQ14 },
        Vector { _handler: GIRQ15 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: GIRQ18 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: GIRQ21 },
        Vector { _handler: GIRQ23 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: GIRQ26 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: I2CSMB0 },
        Vector { _handler: I2CSMB1 },
        Vector { _handler: I2CSMB2 },
        Vector { _handler: I2CSMB3 },
        Vector { _handler: DMA_CH00 },
        Vector { _handler: DMA_CH01 },
        Vector { _handler: DMA_CH02 },
        Vector { _handler: DMA_CH03 },
        Vector { _handler: DMA_CH04 },
        Vector { _handler: DMA_CH05 },
        Vector { _handler: DMA_CH06 },
        Vector { _handler: DMA_CH07 },
        Vector { _handler: DMA_CH08 },
        Vector { _handler: DMA_CH09 },
        Vector { _handler: DMA_CH10 },
        Vector { _handler: DMA_CH11 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: UART0 },
        Vector { _handler: UART1 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: UART2 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: TACH0 },
        Vector { _handler: TACH1 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: ADC_SNGL },
        Vector { _handler: ADC_RPT },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: LED0 },
        Vector { _handler: LED1 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: QMSPI },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: TMR },
        Vector { _handler: HTMR0 },
        Vector { _handler: HTMR1 },
        Vector { _handler: WK },
        Vector { _handler: WKSUB },
        Vector { _handler: WKSEC },
        Vector { _handler: WKSUBSEC },
        Vector { _handler: SYSPWR },
        Vector { _handler: RTC },
        Vector { _handler: RTC_ALARM },
        Vector { _reserved: 0 },
        Vector { _handler: VCI_IN0 },
        Vector { _handler: VCI_IN1 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: TIMER16_0 },
        Vector { _handler: TIMER16_1 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: TIMER32_0 },
        Vector { _handler: TIMER32_1 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: CCT },
        Vector { _handler: CCT_CAP0 },
        Vector { _handler: CCT_CAP1 },
        Vector { _handler: CCT_CAP2 },
        Vector { _handler: CCT_CAP3 },
        Vector { _handler: CCT_CAP4 },
        Vector { _handler: CCT_CAP5 },
        Vector { _handler: CCT_CMP0 },
        Vector { _handler: CCT_CMP1 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: I2CSMB4 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: I2C0 },
        Vector { _handler: I2C1 },
        Vector { _handler: I2C2 },
        Vector { _handler: WDT },
    ];
}
#[doc = "The function of the Watchdog Timer is to provide a mechanism to detect if the internal embedded controller has failed."]
pub const WDT: wdt::Wdt = unsafe { wdt::Wdt::from_ptr(0x4000_0400usize as _) };
#[doc = "This 16-bit timer block offers a simple mechanism for firmware to maintain a time base"]
pub const TIMER16_0: timer16_0::Timer160 = unsafe { timer16_0::Timer160::from_ptr(0x4000_0c00usize as _) };
pub const TIMER16_1: timer16_0::Timer160 = unsafe { timer16_0::Timer160::from_ptr(0x4000_0c20usize as _) };
#[doc = "This 32-bit timer block offers a simple mechanism for firmware to maintain a time base"]
pub const TIMER32_0: timer32_0::Timer320 = unsafe { timer32_0::Timer320::from_ptr(0x4000_0c80usize as _) };
pub const TIMER32_1: timer32_0::Timer320 = unsafe { timer32_0::Timer320::from_ptr(0x4000_0ca0usize as _) };
#[doc = "This is a 16-bit auto-reloading timer/counter."]
pub const CCT: cct::Cct = unsafe { cct::Cct::from_ptr(0x4000_1000usize as _) };
#[doc = "DMA Main Registers"]
pub const DMA_MAIN: dma_main::DmaMain = unsafe { dma_main::DmaMain::from_ptr(0x4000_2400usize as _) };
#[doc = "DMA Channel 00 Registers"]
pub const DMA_CHAN00: dma_chan00::DmaChan00 = unsafe { dma_chan00::DmaChan00::from_ptr(0x4000_2440usize as _) };
#[doc = "DMA Channel 01 Registers"]
pub const DMA_CHAN01: dma_chan01::DmaChan01 = unsafe { dma_chan01::DmaChan01::from_ptr(0x4000_2480usize as _) };
#[doc = "DMA Channel 02 Registers"]
pub const DMA_CHAN02: dma_chan02::DmaChan02 = unsafe { dma_chan02::DmaChan02::from_ptr(0x4000_24c0usize as _) };
pub const DMA_CHAN03: dma_chan02::DmaChan02 = unsafe { dma_chan02::DmaChan02::from_ptr(0x4000_2500usize as _) };
pub const DMA_CHAN04: dma_chan02::DmaChan02 = unsafe { dma_chan02::DmaChan02::from_ptr(0x4000_2540usize as _) };
pub const DMA_CHAN05: dma_chan02::DmaChan02 = unsafe { dma_chan02::DmaChan02::from_ptr(0x4000_2580usize as _) };
pub const DMA_CHAN06: dma_chan02::DmaChan02 = unsafe { dma_chan02::DmaChan02::from_ptr(0x4000_25c0usize as _) };
pub const DMA_CHAN07: dma_chan02::DmaChan02 = unsafe { dma_chan02::DmaChan02::from_ptr(0x4000_2600usize as _) };
pub const DMA_CHAN08: dma_chan02::DmaChan02 = unsafe { dma_chan02::DmaChan02::from_ptr(0x4000_2640usize as _) };
pub const DMA_CHAN09: dma_chan02::DmaChan02 = unsafe { dma_chan02::DmaChan02::from_ptr(0x4000_2680usize as _) };
pub const DMA_CHAN10: dma_chan02::DmaChan02 = unsafe { dma_chan02::DmaChan02::from_ptr(0x4000_26c0usize as _) };
pub const DMA_CHAN11: dma_chan02::DmaChan02 = unsafe { dma_chan02::DmaChan02::from_ptr(0x4000_2700usize as _) };
#[doc = "The SMBus interface can handle standard SMBus 2.0 protocols as well as I2C interface."]
pub const SMB0: smb0::Smb0 = unsafe { smb0::Smb0::from_ptr(0x4000_4000usize as _) };
pub const SMB1: smb0::Smb0 = unsafe { smb0::Smb0::from_ptr(0x4000_4400usize as _) };
pub const SMB2: smb0::Smb0 = unsafe { smb0::Smb0::from_ptr(0x4000_4800usize as _) };
pub const SMB3: smb0::Smb0 = unsafe { smb0::Smb0::from_ptr(0x4000_4c00usize as _) };
pub const SMB4: smb0::Smb0 = unsafe { smb0::Smb0::from_ptr(0x4000_5000usize as _) };
#[doc = "The I2C interface can handle standard I2C interface."]
pub const I2C0: i2c0::I2c0 = unsafe { i2c0::I2c0::from_ptr(0x4000_5100usize as _) };
pub const I2C1: i2c0::I2c0 = unsafe { i2c0::I2c0::from_ptr(0x4000_5200usize as _) };
pub const I2C2: i2c0::I2c0 = unsafe { i2c0::I2c0::from_ptr(0x4000_5300usize as _) };
#[doc = "The PWM block generates an arbitrary duty cycle output at frequencies from less than 0.1 Hz to 24 MHz"]
pub const PWM0: pwm0::Pwm0 = unsafe { pwm0::Pwm0::from_ptr(0x4000_5800usize as _) };
pub const PWM2: pwm0::Pwm0 = unsafe { pwm0::Pwm0::from_ptr(0x4000_5820usize as _) };
pub const PWM3: pwm0::Pwm0 = unsafe { pwm0::Pwm0::from_ptr(0x4000_5830usize as _) };
pub const PWM5: pwm0::Pwm0 = unsafe { pwm0::Pwm0::from_ptr(0x4000_5850usize as _) };
pub const PWM6: pwm0::Pwm0 = unsafe { pwm0::Pwm0::from_ptr(0x4000_5860usize as _) };
pub const PWM7: pwm0::Pwm0 = unsafe { pwm0::Pwm0::from_ptr(0x4000_5870usize as _) };
#[doc = "This block monitors TACH output signals from various types of fans, and determines their speed."]
pub const TACH0: tach0::Tach0 = unsafe { tach0::Tach0::from_ptr(0x4000_6000usize as _) };
pub const TACH1: tach0::Tach0 = unsafe { tach0::Tach0::from_ptr(0x4000_6010usize as _) };
#[doc = "RTOS is a 32-bit timer designed to operate on the 32kHz oscillator which is available during all chip sleep states."]
pub const RTOS: rtos::Rtos = unsafe { rtos::Rtos::from_ptr(0x4000_7400usize as _) };
#[doc = "This block is designed to convert external analog voltage readings into digital values."]
pub const ADC: adc::Adc = unsafe { adc::Adc::from_ptr(0x4000_7c00usize as _) };
#[doc = "The TFDP serially transmits EC-originated diagnostic vectors to an external debug trace system."]
pub const TFDP: tfdp::Tfdp = unsafe { tfdp::Tfdp::from_ptr(0x4000_8c00usize as _) };
#[doc = "The Hibernation Timer can generate a wake event to the Embedded Controller (EC) when it is in a hibernation mode"]
pub const HTM0: htm0::Htm0 = unsafe { htm0::Htm0::from_ptr(0x4000_9800usize as _) };
pub const HTM1: htm0::Htm0 = unsafe { htm0::Htm0::from_ptr(0x4000_9820usize as _) };
#[doc = "The VBAT Register Bank block is a block implemented for miscellaneous battery-backed registers"]
pub const VBAT: vbat::Vbat = unsafe { vbat::Vbat::from_ptr(0x4000_a400usize as _) };
#[doc = "The VBAT RAM is operational while the main power rail is operational, and will retain its values powered by battery power while the main rail is unpowered."]
pub const VBAT_RAM: vbat_ram::VbatRam = unsafe { vbat_ram::VbatRam::from_ptr(0x4000_a800usize as _) };
#[doc = "The Week Timer and the Sub-Week Timer assert the Power-Up Event Output which automatically powers-up the system from the G3 state"]
pub const WEEK: week::Week = unsafe { week::Week::from_ptr(0x4000_ac80usize as _) };
#[doc = "The VBAT-Powered Control Interfaces with the RTC With Date and DST Adjustment as well as the Week Alarm."]
pub const VCI: vci::Vci = unsafe { vci::Vci::from_ptr(0x4000_ae00usize as _) };
#[doc = "The LED is implemented using a PWM that can be driven either by the 48 MHz clock or by a 32.768 KHz clock input."]
pub const LED0: led0::Led0 = unsafe { led0::Led0::from_ptr(0x4000_b800usize as _) };
pub const LED1: led0::Led0 = unsafe { led0::Led0::from_ptr(0x4000_b900usize as _) };
#[doc = "The ECIA works in conjunction with the processor interrupt interface to handle hardware interrupts andd exceptions."]
pub const ECIA: ecia::Ecia = unsafe { ecia::Ecia::from_ptr(0x4000_e000usize as _) };
#[doc = "This block is designed to be accessed internally by the EC via the register interface."]
pub const EC_REG_BANK: ec_reg_bank::EcRegBank = unsafe { ec_reg_bank::EcRegBank::from_ptr(0x4000_fc00usize as _) };
#[doc = "The QMSPI may be used to communicate with various peripheral devices that use a Serial Peripheral Interface"]
pub const QMSPI: qmspi::Qmspi = unsafe { qmspi::Qmspi::from_ptr(0x4007_0000usize as _) };
#[doc = "The Power, Clocks, and Resets (PCR) Section identifies clock sources, and reset inputs to the chip"]
pub const PCR: pcr::Pcr = unsafe { pcr::Pcr::from_ptr(0x4008_0100usize as _) };
#[doc = "GPIO Pin Control Registers"]
pub const GPIO: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4008_1000usize as _) };
#[doc = "The 16550 UART is a full-function Two Pin Serial Port that supports the standard RS-232 Interface."]
pub const UART0: uart0::Uart0 = unsafe { uart0::Uart0::from_ptr(0x400f_2400usize as _) };
pub const UART1: uart0::Uart0 = unsafe { uart0::Uart0::from_ptr(0x400f_2800usize as _) };
pub const UART2: uart0::Uart0 = unsafe { uart0::Uart0::from_ptr(0x400f_2c00usize as _) };
#[doc = "This is the set of registers that are automatically counted by hardware every 1 second while the block is enabled"]
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x400f_5000usize as _) };
#[doc = "The Logical Device Configuration registers support motherboard designs in which the resources required by their components are known and assigned by the BIOS at POST."]
pub const GCR: gcr::Gcr = unsafe { gcr::Gcr::from_ptr(0x400f_ff00usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[doc = "Pin buffer drive type."]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BufferType {
    #[doc = "Output buffer type is Open-drain."]
    PUSH_PULL = 0x0,
    #[doc = "Output buffer type is Push-pull."]
    OPEN_DRAIN = 0x01,
}
impl BufferType {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BufferType {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BufferType {
    #[inline(always)]
    fn from(val: u8) -> BufferType {
        BufferType::from_bits(val)
    }
}
impl From<BufferType> for u8 {
    #[inline(always)]
    fn from(val: BufferType) -> u8 {
        BufferType::to_bits(val)
    }
}
#[doc = "Pin direction."]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dir {
    #[doc = "Input GPIO."]
    INPUT = 0x0,
    #[doc = "Output GPIO."]
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
pub enum Function {
    #[doc = "GPIO function selected."]
    GPIO = 0x0,
    #[doc = "Function 1 selected."]
    F1 = 0x01,
    #[doc = "Function 2 selected."]
    F2 = 0x02,
    #[doc = "Function 3 selected."]
    F3 = 0x03,
    #[doc = "Function 4 selected."]
    F4 = 0x04,
    #[doc = "Function 5 selected."]
    F5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Function {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Function {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Function {
    #[inline(always)]
    fn from(val: u8) -> Function {
        Function::from_bits(val)
    }
}
impl From<Function> for u8 {
    #[inline(always)]
    fn from(val: Function) -> u8 {
        Function::to_bits(val)
    }
}
#[doc = "Power Gating Signals provide the chip Power Emulation options."]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pgs {
    #[doc = "The output buffer is tristated when VTR_PWRGD=0."]
    VTR = 0x0,
    #[doc = "The output buffer is tristated when VCC_PWRGD=0."]
    VCC = 0x01,
    #[doc = "The always unpowered setting on a GPIO will force the pin to tristate. The input and output are disabled, and the pad is in the lowest power state."]
    UNPOWERED = 0x02,
    _RESERVED_3 = 0x03,
}
impl Pgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pgs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pgs {
    #[inline(always)]
    fn from(val: u8) -> Pgs {
        Pgs::from_bits(val)
    }
}
impl From<Pgs> for u8 {
    #[inline(always)]
    fn from(val: Pgs) -> u8 {
        Pgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pol {
    #[doc = "Non-inverted polarity."]
    NON_INVERTED = 0x0,
    #[doc = "Inverted polarity."]
    INVERTED = 0x01,
}
impl Pol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pol {
    #[inline(always)]
    fn from(val: u8) -> Pol {
        Pol::from_bits(val)
    }
}
impl From<Pol> for u8 {
    #[inline(always)]
    fn from(val: Pol) -> u8 {
        Pol::to_bits(val)
    }
}
#[doc = "Configure internal pull-up and pull-down resistors."]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pull {
    #[doc = "Pin tristates when no active driver is present on the pin."]
    NONE = 0x0,
    #[doc = "Pull up enabled."]
    UP = 0x01,
    #[doc = "Pull down enabled."]
    DOWN = 0x02,
    #[doc = "Pin is kept at previous voltage level when no active driver is present on the pin."]
    REPEATER = 0x03,
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
pub enum Sel {
    #[doc = "Single GPIO output data bit is enabled."]
    PIN = 0x0,
    #[doc = "Grouped Output GPIO is enabled."]
    GROUP = 0x01,
}
impl Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sel {
    #[inline(always)]
    fn from(val: u8) -> Sel {
        Sel::from_bits(val)
    }
}
impl From<Sel> for u8 {
    #[inline(always)]
    fn from(val: Sel) -> u8 {
        Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SlewCtrl {
    SLOW = 0x0,
    FAST = 0x01,
}
impl SlewCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SlewCtrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SlewCtrl {
    #[inline(always)]
    fn from(val: u8) -> SlewCtrl {
        SlewCtrl::from_bits(val)
    }
}
impl From<SlewCtrl> for u8 {
    #[inline(always)]
    fn from(val: SlewCtrl) -> u8 {
        SlewCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Strength {
    #[doc = "2mA for PIO-12 pins, 4mA for PIO-24 pins."]
    LOWEST = 0x0,
    #[doc = "4mA for PIO-12 pins, 8mA for PIO-24 pins."]
    LOW = 0x01,
    #[doc = "8mA for PIO-12 pins, 16mA for PIO-24 pins."]
    MEDIUM = 0x02,
    #[doc = "12mA for PIO-12 pins, 24mA for PIO-24 pins."]
    FULL = 0x03,
}
impl Strength {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Strength {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Strength {
    #[inline(always)]
    fn from(val: u8) -> Strength {
        Strength::from_bits(val)
    }
}
impl From<Strength> for u8 {
    #[inline(always)]
    fn from(val: Strength) -> u8 {
        Strength::to_bits(val)
    }
}
pub mod adc {
    #[doc = "This block is designed to convert external analog voltage readings into digital values."]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Adc {
        ptr: *mut u8,
    }
    unsafe impl Send for Adc {}
    unsafe impl Sync for Adc {}
    impl Adc {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "The ADC Control Register is used to control the behavior of the Analog to Digital Converter."]
        #[inline(always)]
        pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "The ADC Delay register determines the delay from setting Start_Repeat in the ADC Control Register and the start of a conversion cycle. This register also controls the interval between conversion cycles in repeat mode."]
        #[inline(always)]
        pub const fn delay(self) -> crate::common::Reg<regs::Delay, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "The ADC Status Register indicates whether the ADC has completed a conversion cycle. All bits are cleared by being written with a 1. 0: conversion of the corresponding ADC channel is not complete 1: conversion of the corresponding ADC channel is complete"]
        #[inline(always)]
        pub const fn chan_sts(self) -> crate::common::Reg<regs::ChanSts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "The ADC Single Register is used to control which ADC channel is captured during a Single-Sample conversion cycle initiated by the Start_Single bit in the ADC Control Register. APPLICATION NOTE: Do not change the bits in this register in the middle of a conversion cycle to insure proper operation. 0: single cycle conversions for this channel are disabled 1: single cycle conversions for this channel are enabled"]
        #[inline(always)]
        pub const fn sng_en(self) -> crate::common::Reg<regs::SngEn, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "The ADC Repeat Register is used to control which ADC channels are captured during a repeat conversion cycle initiated by the Start_Repeat bit in the ADC Control Register."]
        #[inline(always)]
        pub const fn rept_en(self) -> crate::common::Reg<regs::ReptEn, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "All 16 ADC channels return their results into a 32-bit reading register. In each case the low 10 bits of the reading register return the result of the Analog to Digital conversion and the upper 22 bits return 0."]
        #[inline(always)]
        pub const fn chan_rd(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 16usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize + n * 4usize) as _) }
        }
        #[doc = "The ADC Configuration Register is used to configure the ADC clock timing."]
        #[inline(always)]
        pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
        }
        #[doc = "The ADC Channel Register is used to configure the reference voltage to the clock timing."]
        #[inline(always)]
        pub const fn vref_chan(self) -> crate::common::Reg<regs::VrefChan, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
        }
        #[doc = "This is the VREF Control Register"]
        #[inline(always)]
        pub const fn vref_ctrl(self) -> crate::common::Reg<regs::VrefCtrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
        }
        #[doc = "This is the SAR ADC Control Register."]
        #[inline(always)]
        pub const fn sar_ctrl(self) -> crate::common::Reg<regs::SarCtrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
        }
        #[doc = "This is the SAR ADC Configuration Register."]
        #[inline(always)]
        pub const fn sar_cfg(self) -> crate::common::Reg<regs::SarCfg, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "The ADC Configuration Register is used to configure the ADC clock timing."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cfg(pub u32);
        impl Cfg {
            #[doc = "These bits define the low time count of the ADC clock. 0= not used. 1= 1 System Clock. 2= 2 System Clock."]
            #[inline(always)]
            pub const fn clklw_tim(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "These bits define the low time count of the ADC clock. 0= not used. 1= 1 System Clock. 2= 2 System Clock."]
            #[inline(always)]
            pub fn set_clklw_tim(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "These bits define the high time count of the ADC clock. 0= not used. 1= 1 System Clock. 2= 2 System Clock."]
            #[inline(always)]
            pub const fn clkhigh_tim(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "These bits define the high time count of the ADC clock. 0= not used. 1= 1 System Clock. 2= 2 System Clock."]
            #[inline(always)]
            pub fn set_clkhigh_tim(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
            #[doc = "These bits define the dummy cycles of the ADC clock. Valid Values are from 0x0 to 0xF."]
            #[inline(always)]
            pub const fn clkdumy_tim(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0x0f;
                val as u8
            }
            #[doc = "These bits define the dummy cycles of the ADC clock. Valid Values are from 0x0 to 0xF."]
            #[inline(always)]
            pub fn set_clkdumy_tim(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
            }
            #[doc = "These bits define the power up delay in number of micro-seconds. Valid Values are from 0x0 to 0xF."]
            #[inline(always)]
            pub const fn pwrup_dly(&self) -> u8 {
                let val = (self.0 >> 20usize) & 0x0f;
                val as u8
            }
            #[doc = "These bits define the power up delay in number of micro-seconds. Valid Values are from 0x0 to 0xF."]
            #[inline(always)]
            pub fn set_pwrup_dly(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
            }
            #[doc = "These bits define the number of micro-seconds between consective Starts."]
            #[inline(always)]
            pub const fn dumycyc_gap(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0xff;
                val as u8
            }
            #[doc = "These bits define the number of micro-seconds between consective Starts."]
            #[inline(always)]
            pub fn set_dumycyc_gap(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
            }
        }
        impl Default for Cfg {
            #[inline(always)]
            fn default() -> Cfg {
                Cfg(0)
            }
        }
        impl core::fmt::Debug for Cfg {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Cfg")
                    .field("clklw_tim", &self.clklw_tim())
                    .field("clkhigh_tim", &self.clkhigh_tim())
                    .field("clkdumy_tim", &self.clkdumy_tim())
                    .field("pwrup_dly", &self.pwrup_dly())
                    .field("dumycyc_gap", &self.dumycyc_gap())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Cfg {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Cfg {{ clklw_tim: {=u8:?}, clkhigh_tim: {=u8:?}, clkdumy_tim: {=u8:?}, pwrup_dly: {=u8:?}, dumycyc_gap: {=u8:?} }}" , self . clklw_tim () , self . clkhigh_tim () , self . clkdumy_tim () , self . pwrup_dly () , self . dumycyc_gap ())
            }
        }
        #[doc = "The ADC Status Register indicates whether the ADC has completed a conversion cycle. All bits are cleared by being written with a 1. 0: conversion of the corresponding ADC channel is not complete 1: conversion of the corresponding ADC channel is complete"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct ChanSts(pub u32);
        impl ChanSts {
            #[doc = "All bits are cleared by being written with a '1'. 1=conversion of the corresponding ADC channel is complete; 0=conversion of the corresponding ADC channel is not complete. For enabled single cycles, the SINGLE_DONE_STATUS bit in the ADC Control Register is also set after all enabled channel conversion are done; for enabled repeat cycles, the REPEAT_DONE_STATUS in the ADC Control Register is also set after all enabled channel conversion are done."]
            #[inline(always)]
            pub const fn sts(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "All bits are cleared by being written with a '1'. 1=conversion of the corresponding ADC channel is complete; 0=conversion of the corresponding ADC channel is not complete. For enabled single cycles, the SINGLE_DONE_STATUS bit in the ADC Control Register is also set after all enabled channel conversion are done; for enabled repeat cycles, the REPEAT_DONE_STATUS in the ADC Control Register is also set after all enabled channel conversion are done."]
            #[inline(always)]
            pub fn set_sts(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for ChanSts {
            #[inline(always)]
            fn default() -> ChanSts {
                ChanSts(0)
            }
        }
        impl core::fmt::Debug for ChanSts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("ChanSts").field("sts", &self.sts()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for ChanSts {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "ChanSts {{ sts: {=u16:?} }}", self.sts())
            }
        }
        #[doc = "The ADC Control Register is used to control the behavior of the Analog to Digital Converter."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctrl(pub u32);
        impl Ctrl {
            #[doc = "0: The ADC is disabled and placed in its lowest power state. 1: ADC block is enabled for operation."]
            #[inline(always)]
            pub const fn act(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "0: The ADC is disabled and placed in its lowest power state. 1: ADC block is enabled for operation."]
            #[inline(always)]
            pub fn set_act(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "(START_SINGLE) 0: The ADC Single Mode is disabled. 1: The ADC Single Mode is enabled. Note: This bit is self-clearing"]
            #[inline(always)]
            pub const fn strt_sin(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "(START_SINGLE) 0: The ADC Single Mode is disabled. 1: The ADC Single Mode is enabled. Note: This bit is self-clearing"]
            #[inline(always)]
            pub fn set_strt_sin(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "0: The ADC Repeat Mode is disabled. 1: The ADC Repeat Mode is enabled."]
            #[inline(always)]
            pub const fn strt_rpt(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "0: The ADC Repeat Mode is disabled. 1: The ADC Repeat Mode is enabled."]
            #[inline(always)]
            pub fn set_strt_rpt(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "0: Power saving feature is enabled. 1: Power saving feature is disabled."]
            #[inline(always)]
            pub const fn pwr_sav_dis(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "0: Power saving feature is enabled. 1: Power saving feature is disabled."]
            #[inline(always)]
            pub fn set_pwr_sav_dis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "(SOFT_RESET) 1: writing one causes a reset of the ADC block hardware (not the registers) 0: writing zero takes the ADC block out of reset"]
            #[inline(always)]
            pub const fn sft_rst(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "(SOFT_RESET) 1: writing one causes a reset of the ADC block hardware (not the registers) 0: writing zero takes the ADC block out of reset"]
            #[inline(always)]
            pub fn set_sft_rst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "0: ADC repeat-sample conversion is not complete. 1: ADC repeat-sample conversion is completed. (R/WC)"]
            #[inline(always)]
            pub const fn rpt_done_sts(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "0: ADC repeat-sample conversion is not complete. 1: ADC repeat-sample conversion is completed. (R/WC)"]
            #[inline(always)]
            pub fn set_rpt_done_sts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "0: ADC single-sample conversion is not complete. 1: ADC single-sample conversion is completed. (R/WC)"]
            #[inline(always)]
            pub const fn sin_done_sts(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "0: ADC single-sample conversion is not complete. 1: ADC single-sample conversion is completed. (R/WC)"]
            #[inline(always)]
            pub fn set_sin_done_sts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
        }
        impl Default for Ctrl {
            #[inline(always)]
            fn default() -> Ctrl {
                Ctrl(0)
            }
        }
        impl core::fmt::Debug for Ctrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ctrl")
                    .field("act", &self.act())
                    .field("strt_sin", &self.strt_sin())
                    .field("strt_rpt", &self.strt_rpt())
                    .field("pwr_sav_dis", &self.pwr_sav_dis())
                    .field("sft_rst", &self.sft_rst())
                    .field("rpt_done_sts", &self.rpt_done_sts())
                    .field("sin_done_sts", &self.sin_done_sts())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ctrl {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Ctrl {{ act: {=bool:?}, strt_sin: {=bool:?}, strt_rpt: {=bool:?}, pwr_sav_dis: {=bool:?}, sft_rst: {=bool:?}, rpt_done_sts: {=bool:?}, sin_done_sts: {=bool:?} }}" , self . act () , self . strt_sin () , self . strt_rpt () , self . pwr_sav_dis () , self . sft_rst () , self . rpt_done_sts () , self . sin_done_sts ())
            }
        }
        #[doc = "The ADC Delay register determines the delay from setting Start_Repeat in the ADC Control Register and the start of a conversion cycle. This register also controls the interval between conversion cycles in repeat mode."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Delay(pub u32);
        impl Delay {
            #[doc = "This field determines the starting delay before a conversion cycle is begun when Start_Repeat is written with a 1."]
            #[inline(always)]
            pub const fn strt_dly(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "This field determines the starting delay before a conversion cycle is begun when Start_Repeat is written with a 1."]
            #[inline(always)]
            pub fn set_strt_dly(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
            #[doc = "This field determines the interval between conversion cycles when Start_Repeat is 1."]
            #[inline(always)]
            pub const fn rpt_dly(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0xffff;
                val as u16
            }
            #[doc = "This field determines the interval between conversion cycles when Start_Repeat is 1."]
            #[inline(always)]
            pub fn set_rpt_dly(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
            }
        }
        impl Default for Delay {
            #[inline(always)]
            fn default() -> Delay {
                Delay(0)
            }
        }
        impl core::fmt::Debug for Delay {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Delay")
                    .field("strt_dly", &self.strt_dly())
                    .field("rpt_dly", &self.rpt_dly())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Delay {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Delay {{ strt_dly: {=u16:?}, rpt_dly: {=u16:?} }}",
                    self.strt_dly(),
                    self.rpt_dly()
                )
            }
        }
        #[doc = "The ADC Repeat Register is used to control which ADC channels are captured during a repeat conversion cycle initiated by the Start_Repeat bit in the ADC Control Register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct ReptEn(pub u32);
        impl ReptEn {
            #[doc = "Each bit in this field enables the corresponding ADC channel for each pass of the Repeated ADC Conversion that is controlled by bit START_REPEAT in the ADC Control Register. 1=repeat conversions for this channel are enabled; 0=repeat conversions for this channel are disabled"]
            #[inline(always)]
            pub const fn r_en(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Each bit in this field enables the corresponding ADC channel for each pass of the Repeated ADC Conversion that is controlled by bit START_REPEAT in the ADC Control Register. 1=repeat conversions for this channel are enabled; 0=repeat conversions for this channel are disabled"]
            #[inline(always)]
            pub fn set_r_en(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for ReptEn {
            #[inline(always)]
            fn default() -> ReptEn {
                ReptEn(0)
            }
        }
        impl core::fmt::Debug for ReptEn {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("ReptEn").field("r_en", &self.r_en()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for ReptEn {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "ReptEn {{ r_en: {=u16:?} }}", self.r_en())
            }
        }
        #[doc = "This is the SAR ADC Configuration Register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SarCfg(pub u32);
        impl SarCfg {
            #[doc = "Enable Common Mode Buffer Amplifier. 0= Common Mode Buffer Amplifier is high all the time. 1= Controls Common Mode Buffer Amplifier during power cycling."]
            #[inline(always)]
            pub const fn en_cmbf(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable Common Mode Buffer Amplifier. 0= Common Mode Buffer Amplifier is high all the time. 1= Controls Common Mode Buffer Amplifier during power cycling."]
            #[inline(always)]
            pub fn set_en_cmbf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Disable Parallel Output. 0= Enable Parallel Output. 1= Disable Parallel Output."]
            #[inline(always)]
            pub const fn dis_dout(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Disable Parallel Output. 0= Enable Parallel Output. 1= Disable Parallel Output."]
            #[inline(always)]
            pub fn set_dis_dout(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Enable Dithering. 0= Disable Dither. 1= Enable Dither."]
            #[inline(always)]
            pub const fn en_dither(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Enable Dithering. 0= Disable Dither. 1= Enable Dither."]
            #[inline(always)]
            pub fn set_en_dither(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable F_AZ AUTOZEROING. 1= Disable f_az autozeroing. 0= Enable f_az autozeroing."]
            #[inline(always)]
            pub const fn faz_au_zero(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable F_AZ AUTOZEROING. 1= Disable f_az autozeroing. 0= Enable f_az autozeroing."]
            #[inline(always)]
            pub fn set_faz_au_zero(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Enable S_AZ AUTOZEROING. 1= Disable S_AZ autozeroing. 0= Enable S_AZ autozeroing."]
            #[inline(always)]
            pub const fn saz_au_zero(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Enable S_AZ AUTOZEROING. 1= Disable S_AZ autozeroing. 0= Enable S_AZ autozeroing."]
            #[inline(always)]
            pub fn set_saz_au_zero(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Enable L_AZ AUTOZEROING. 1= Disable L_AZ autozeroing. 0= Enable L_AZ autozeroing."]
            #[inline(always)]
            pub const fn laz_au_zero(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Enable L_AZ AUTOZEROING. 1= Disable L_AZ autozeroing. 0= Enable L_AZ autozeroing."]
            #[inline(always)]
            pub fn set_laz_au_zero(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Enable RADC. 1 = RDAC remains high during power cycling. 0 = Controls RDAC during power cycling."]
            #[inline(always)]
            pub const fn en_radc(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Enable RADC. 1 = RDAC remains high during power cycling. 0 = Controls RDAC during power cycling."]
            #[inline(always)]
            pub fn set_en_radc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "This register defines the delay between regen and latch."]
            #[inline(always)]
            pub const fn regen_dly(&self) -> u8 {
                let val = (self.0 >> 9usize) & 0x03;
                val as u8
            }
            #[doc = "This register defines the delay between regen and latch."]
            #[inline(always)]
            pub fn set_regen_dly(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
            }
            #[doc = "This register defines the programmable ADC Clock divider value. Divider ratios of 256,128,64,32,16 are supported."]
            #[inline(always)]
            pub const fn clk_div(&self) -> u8 {
                let val = (self.0 >> 11usize) & 0x1f;
                val as u8
            }
            #[doc = "This register defines the programmable ADC Clock divider value. Divider ratios of 256,128,64,32,16 are supported."]
            #[inline(always)]
            pub fn set_clk_div(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
            }
            #[doc = "This register controls the current consumption for the whole ADC."]
            #[inline(always)]
            pub const fn iadc_range2(&self) -> u8 {
                let val = (self.0 >> 20usize) & 0x03;
                val as u8
            }
            #[doc = "This register controls the current consumption for the whole ADC."]
            #[inline(always)]
            pub fn set_iadc_range2(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
            }
            #[doc = "This register controls the current consumption for the whole ADC."]
            #[inline(always)]
            pub const fn iadc_range1(&self) -> u8 {
                let val = (self.0 >> 22usize) & 0x03;
                val as u8
            }
            #[doc = "This register controls the current consumption for the whole ADC."]
            #[inline(always)]
            pub fn set_iadc_range1(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
            }
            #[doc = "This register controls the bias current for the 1st stage of the comparator."]
            #[inline(always)]
            pub const fn icmbf_stg1(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0x03;
                val as u8
            }
            #[doc = "This register controls the bias current for the 1st stage of the comparator."]
            #[inline(always)]
            pub fn set_icmbf_stg1(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
            }
            #[doc = "This register controls the bias current for the 2nd stage of the comparator."]
            #[inline(always)]
            pub const fn icmbf_stg2(&self) -> u8 {
                let val = (self.0 >> 26usize) & 0x03;
                val as u8
            }
            #[doc = "This register controls the bias current for the 2nd stage of the comparator."]
            #[inline(always)]
            pub fn set_icmbf_stg2(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
            }
            #[doc = "This register controls the bias current for common mode buffer amplifier."]
            #[inline(always)]
            pub const fn icmbf(&self) -> u8 {
                let val = (self.0 >> 28usize) & 0x03;
                val as u8
            }
            #[doc = "This register controls the bias current for common mode buffer amplifier."]
            #[inline(always)]
            pub fn set_icmbf(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
            }
            #[doc = "EN external bias. 1 = Disables internal switched cap bias circuit. 0 = Enables internal switched cap bias circuit."]
            #[inline(always)]
            pub const fn en_ext_bias(&self) -> bool {
                let val = (self.0 >> 31usize) & 0x01;
                val != 0
            }
            #[doc = "EN external bias. 1 = Disables internal switched cap bias circuit. 0 = Enables internal switched cap bias circuit."]
            #[inline(always)]
            pub fn set_en_ext_bias(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
            }
        }
        impl Default for SarCfg {
            #[inline(always)]
            fn default() -> SarCfg {
                SarCfg(0)
            }
        }
        impl core::fmt::Debug for SarCfg {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("SarCfg")
                    .field("en_cmbf", &self.en_cmbf())
                    .field("dis_dout", &self.dis_dout())
                    .field("en_dither", &self.en_dither())
                    .field("faz_au_zero", &self.faz_au_zero())
                    .field("saz_au_zero", &self.saz_au_zero())
                    .field("laz_au_zero", &self.laz_au_zero())
                    .field("en_radc", &self.en_radc())
                    .field("regen_dly", &self.regen_dly())
                    .field("clk_div", &self.clk_div())
                    .field("iadc_range2", &self.iadc_range2())
                    .field("iadc_range1", &self.iadc_range1())
                    .field("icmbf_stg1", &self.icmbf_stg1())
                    .field("icmbf_stg2", &self.icmbf_stg2())
                    .field("icmbf", &self.icmbf())
                    .field("en_ext_bias", &self.en_ext_bias())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for SarCfg {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "SarCfg {{ en_cmbf: {=bool:?}, dis_dout: {=bool:?}, en_dither: {=bool:?}, faz_au_zero: {=bool:?}, saz_au_zero: {=bool:?}, laz_au_zero: {=bool:?}, en_radc: {=bool:?}, regen_dly: {=u8:?}, clk_div: {=u8:?}, iadc_range2: {=u8:?}, iadc_range1: {=u8:?}, icmbf_stg1: {=u8:?}, icmbf_stg2: {=u8:?}, icmbf: {=u8:?}, en_ext_bias: {=bool:?} }}" , self . en_cmbf () , self . dis_dout () , self . en_dither () , self . faz_au_zero () , self . saz_au_zero () , self . laz_au_zero () , self . en_radc () , self . regen_dly () , self . clk_div () , self . iadc_range2 () , self . iadc_range1 () , self . icmbf_stg1 () , self . icmbf_stg2 () , self . icmbf () , self . en_ext_bias ())
            }
        }
        #[doc = "This is the SAR ADC Control Register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SarCtrl(pub u32);
        impl SarCtrl {
            #[doc = "This field select between Single ended / Differential input. 0= ADC core is enabled for single ended input operation. 1= ADC core is enabled for differential input operation."]
            #[inline(always)]
            pub const fn sel_diff(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This field select between Single ended / Differential input. 0= ADC core is enabled for single ended input operation. 1= ADC core is enabled for differential input operation."]
            #[inline(always)]
            pub fn set_sel_diff(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "This field select the ADC Resolution (10/12 bits). 0x0= Reserved. 0x1= Reserved. 0x2= 10 bit ADC resolution. 0x3= 12 bit ADC resolution."]
            #[inline(always)]
            pub const fn sel_res(&self) -> u8 {
                let val = (self.0 >> 1usize) & 0x03;
                val as u8
            }
            #[doc = "This field select the ADC Resolution (10/12 bits). 0x0= Reserved. 0x1= Reserved. 0x2= 10 bit ADC resolution. 0x3= 12 bit ADC resolution."]
            #[inline(always)]
            pub fn set_sel_res(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
            }
            #[doc = "This field defined if the ADC output is Right or Left Justified. 1= adc_dout is not shifted and lower bits are set to 0. 0= adc_dout is shifted right following resolution selected."]
            #[inline(always)]
            pub const fn shift_dat(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "This field defined if the ADC output is Right or Left Justified. 1= adc_dout is not shifted and lower bits are set to 0. 0= adc_dout is shifted right following resolution selected."]
            #[inline(always)]
            pub fn set_shift_dat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "This field enables asynchronous sampling. 0= Async Sampling Disabled. 1= Async Sampling Enabled."]
            #[inline(always)]
            pub const fn en_asyn_smpl(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "This field enables asynchronous sampling. 0= Async Sampling Disabled. 1= Async Sampling Enabled."]
            #[inline(always)]
            pub fn set_en_asyn_smpl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "This field enables serial output (dout) from ADC. 0= Parallel dout. 1= Serial dout."]
            #[inline(always)]
            pub const fn en_serial(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "This field enables serial output (dout) from ADC. 0= Parallel dout. 1= Serial dout."]
            #[inline(always)]
            pub fn set_en_serial(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "This field represents the warmup delay number in microseconds."]
            #[inline(always)]
            pub const fn warm_up_dly(&self) -> u16 {
                let val = (self.0 >> 7usize) & 0x01ff;
                val as u16
            }
            #[doc = "This field represents the warmup delay number in microseconds."]
            #[inline(always)]
            pub fn set_warm_up_dly(&mut self, val: u16) {
                self.0 = (self.0 & !(0x01ff << 7usize)) | (((val as u32) & 0x01ff) << 7usize);
            }
        }
        impl Default for SarCtrl {
            #[inline(always)]
            fn default() -> SarCtrl {
                SarCtrl(0)
            }
        }
        impl core::fmt::Debug for SarCtrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("SarCtrl")
                    .field("sel_diff", &self.sel_diff())
                    .field("sel_res", &self.sel_res())
                    .field("shift_dat", &self.shift_dat())
                    .field("en_asyn_smpl", &self.en_asyn_smpl())
                    .field("en_serial", &self.en_serial())
                    .field("warm_up_dly", &self.warm_up_dly())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for SarCtrl {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "SarCtrl {{ sel_diff: {=bool:?}, sel_res: {=u8:?}, shift_dat: {=bool:?}, en_asyn_smpl: {=bool:?}, en_serial: {=bool:?}, warm_up_dly: {=u16:?} }}" , self . sel_diff () , self . sel_res () , self . shift_dat () , self . en_asyn_smpl () , self . en_serial () , self . warm_up_dly ())
            }
        }
        #[doc = "The ADC Single Register is used to control which ADC channel is captured during a Single-Sample conversion cycle initiated by the Start_Single bit in the ADC Control Register. APPLICATION NOTE: Do not change the bits in this register in the middle of a conversion cycle to insure proper operation. 0: single cycle conversions for this channel are disabled 1: single cycle conversions for this channel are enabled"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SngEn(pub u32);
        impl SngEn {
            #[doc = "Each bit in this field enables the corresponding ADC channel when a single cycle of conversions is started when the START_SINGLE bit in the ADC Control Register is written with a 1. 1=single cycle conversions for this channel are enabled 0=single cycle conversions for this channel are disabled. Note: If this register is changed while a conversion."]
            #[inline(always)]
            pub const fn s_en(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Each bit in this field enables the corresponding ADC channel when a single cycle of conversions is started when the START_SINGLE bit in the ADC Control Register is written with a 1. 1=single cycle conversions for this channel are enabled 0=single cycle conversions for this channel are disabled. Note: If this register is changed while a conversion."]
            #[inline(always)]
            pub fn set_s_en(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for SngEn {
            #[inline(always)]
            fn default() -> SngEn {
                SngEn(0)
            }
        }
        impl core::fmt::Debug for SngEn {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("SngEn").field("s_en", &self.s_en()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for SngEn {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "SngEn {{ s_en: {=u16:?} }}", self.s_en())
            }
        }
        #[doc = "The ADC Channel Register is used to configure the reference voltage to the clock timing."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct VrefChan(pub u32);
        impl VrefChan {
            #[doc = "These bits define the reference voltage for Channel 0. 0h= VREF0 1h= VREF1 2h= Reserved 3h= Reserved"]
            #[inline(always)]
            pub const fn sel0(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x03;
                val as u8
            }
            #[doc = "These bits define the reference voltage for Channel 0. 0h= VREF0 1h= VREF1 2h= Reserved 3h= Reserved"]
            #[inline(always)]
            pub fn set_sel0(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
            }
            #[doc = "These bits define the reference voltage for Channel 1. 0h= VREF0 1h= VREF1 2h= Reserved 3h= Reserved"]
            #[inline(always)]
            pub const fn sel1(&self) -> u8 {
                let val = (self.0 >> 2usize) & 0x03;
                val as u8
            }
            #[doc = "These bits define the reference voltage for Channel 1. 0h= VREF0 1h= VREF1 2h= Reserved 3h= Reserved"]
            #[inline(always)]
            pub fn set_sel1(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
            }
            #[doc = "These bits define the reference voltage for Channel 2. 0h= VREF0 1h= VREF1 2h= Reserved 3h= Reserved"]
            #[inline(always)]
            pub const fn sel2(&self) -> u8 {
                let val = (self.0 >> 4usize) & 0x03;
                val as u8
            }
            #[doc = "These bits define the reference voltage for Channel 2. 0h= VREF0 1h= VREF1 2h= Reserved 3h= Reserved"]
            #[inline(always)]
            pub fn set_sel2(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
            }
            #[doc = "These bits define the reference voltage for Channel 3. 0h= VREF0 1h= VREF1 2h= Reserved 3h= Reserved"]
            #[inline(always)]
            pub const fn sel3(&self) -> u8 {
                let val = (self.0 >> 6usize) & 0x03;
                val as u8
            }
            #[doc = "These bits define the reference voltage for Channel 3. 0h= VREF0 1h= VREF1 2h= Reserved 3h= Reserved"]
            #[inline(always)]
            pub fn set_sel3(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
            }
            #[doc = "These bits define the reference voltage for Channel 4. 0h= VREF0 1h= VREF1 2h= Reserved 3h= Reserved"]
            #[inline(always)]
            pub const fn sel4(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0x03;
                val as u8
            }
            #[doc = "These bits define the reference voltage for Channel 4. 0h= VREF0 1h= VREF1 2h= Reserved 3h= Reserved"]
            #[inline(always)]
            pub fn set_sel4(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
            }
            #[doc = "These bits define the reference voltage for Channel 5. 0h= VREF0 1h= VREF1 2h= Reserved 3h= Reserved"]
            #[inline(always)]
            pub const fn sel5(&self) -> u8 {
                let val = (self.0 >> 10usize) & 0x03;
                val as u8
            }
            #[doc = "These bits define the reference voltage for Channel 5. 0h= VREF0 1h= VREF1 2h= Reserved 3h= Reserved"]
            #[inline(always)]
            pub fn set_sel5(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
            }
            #[doc = "These bits define the reference voltage for Channel 6. 0h= VREF0 1h= VREF1 2h= Reserved 3h= Reserved"]
            #[inline(always)]
            pub const fn sel6(&self) -> u8 {
                let val = (self.0 >> 12usize) & 0x03;
                val as u8
            }
            #[doc = "These bits define the reference voltage for Channel 6. 0h= VREF0 1h= VREF1 2h= Reserved 3h= Reserved"]
            #[inline(always)]
            pub fn set_sel6(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
            }
            #[doc = "These bits define the reference voltage for Channel 7. 0h= VREF0 1h= VREF1 2h= Reserved 3h= Reserved"]
            #[inline(always)]
            pub const fn sel7(&self) -> u8 {
                let val = (self.0 >> 14usize) & 0x03;
                val as u8
            }
            #[doc = "These bits define the reference voltage for Channel 7. 0h= VREF0 1h= VREF1 2h= Reserved 3h= Reserved"]
            #[inline(always)]
            pub fn set_sel7(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
            }
            #[doc = "These bits define the reference voltage for Channel 8. 0h= VREF0 1h= VREF1 2h= Reserved 3h= Reserved"]
            #[inline(always)]
            pub const fn sel8(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0x03;
                val as u8
            }
            #[doc = "These bits define the reference voltage for Channel 8. 0h= VREF0 1h= VREF1 2h= Reserved 3h= Reserved"]
            #[inline(always)]
            pub fn set_sel8(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
            }
            #[doc = "These bits define the reference voltage for Channel 9. 0h= VREF0 1h= VREF1 2h= Reserved 3h= Reserved"]
            #[inline(always)]
            pub const fn sel9(&self) -> u8 {
                let val = (self.0 >> 18usize) & 0x03;
                val as u8
            }
            #[doc = "These bits define the reference voltage for Channel 9. 0h= VREF0 1h= VREF1 2h= Reserved 3h= Reserved"]
            #[inline(always)]
            pub fn set_sel9(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
            }
            #[doc = "These bits define the reference voltage for Channel 10. 0h= VREF0 1h= VREF1 2h= Reserved 3h= Reserved"]
            #[inline(always)]
            pub const fn sel10(&self) -> u8 {
                let val = (self.0 >> 20usize) & 0x03;
                val as u8
            }
            #[doc = "These bits define the reference voltage for Channel 10. 0h= VREF0 1h= VREF1 2h= Reserved 3h= Reserved"]
            #[inline(always)]
            pub fn set_sel10(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
            }
            #[doc = "These bits define the reference voltage for Channel 11. 0h= VREF0 1h= VREF1 2h= Reserved 3h= Reserved"]
            #[inline(always)]
            pub const fn sel11(&self) -> u8 {
                let val = (self.0 >> 22usize) & 0x03;
                val as u8
            }
            #[doc = "These bits define the reference voltage for Channel 11. 0h= VREF0 1h= VREF1 2h= Reserved 3h= Reserved"]
            #[inline(always)]
            pub fn set_sel11(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
            }
            #[doc = "These bits define the reference voltage for Channel 12. 0h= VREF0 1h= VREF1 2h= Reserved 3h= Reserved"]
            #[inline(always)]
            pub const fn sel12(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0x03;
                val as u8
            }
            #[doc = "These bits define the reference voltage for Channel 12. 0h= VREF0 1h= VREF1 2h= Reserved 3h= Reserved"]
            #[inline(always)]
            pub fn set_sel12(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
            }
            #[doc = "These bits define the reference voltage for Channel 13. 0h= VREF0 1h= VREF1 2h= Reserved 3h= Reserved"]
            #[inline(always)]
            pub const fn sel13(&self) -> u8 {
                let val = (self.0 >> 26usize) & 0x03;
                val as u8
            }
            #[doc = "These bits define the reference voltage for Channel 13. 0h= VREF0 1h= VREF1 2h= Reserved 3h= Reserved"]
            #[inline(always)]
            pub fn set_sel13(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
            }
            #[doc = "These bits define the reference voltage for Channel 14. 0h= VREF0 1h= VREF1 2h= Reserved 3h= Reserved"]
            #[inline(always)]
            pub const fn sel14(&self) -> u8 {
                let val = (self.0 >> 28usize) & 0x03;
                val as u8
            }
            #[doc = "These bits define the reference voltage for Channel 14. 0h= VREF0 1h= VREF1 2h= Reserved 3h= Reserved"]
            #[inline(always)]
            pub fn set_sel14(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
            }
            #[doc = "These bits define the reference voltage for Channel 15. 0h= VREF0 1h= VREF1 2h= Reserved 3h= Reserved"]
            #[inline(always)]
            pub const fn sel15(&self) -> u8 {
                let val = (self.0 >> 30usize) & 0x03;
                val as u8
            }
            #[doc = "These bits define the reference voltage for Channel 15. 0h= VREF0 1h= VREF1 2h= Reserved 3h= Reserved"]
            #[inline(always)]
            pub fn set_sel15(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
            }
        }
        impl Default for VrefChan {
            #[inline(always)]
            fn default() -> VrefChan {
                VrefChan(0)
            }
        }
        impl core::fmt::Debug for VrefChan {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("VrefChan")
                    .field("sel0", &self.sel0())
                    .field("sel1", &self.sel1())
                    .field("sel2", &self.sel2())
                    .field("sel3", &self.sel3())
                    .field("sel4", &self.sel4())
                    .field("sel5", &self.sel5())
                    .field("sel6", &self.sel6())
                    .field("sel7", &self.sel7())
                    .field("sel8", &self.sel8())
                    .field("sel9", &self.sel9())
                    .field("sel10", &self.sel10())
                    .field("sel11", &self.sel11())
                    .field("sel12", &self.sel12())
                    .field("sel13", &self.sel13())
                    .field("sel14", &self.sel14())
                    .field("sel15", &self.sel15())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for VrefChan {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "VrefChan {{ sel0: {=u8:?}, sel1: {=u8:?}, sel2: {=u8:?}, sel3: {=u8:?}, sel4: {=u8:?}, sel5: {=u8:?}, sel6: {=u8:?}, sel7: {=u8:?}, sel8: {=u8:?}, sel9: {=u8:?}, sel10: {=u8:?}, sel11: {=u8:?}, sel12: {=u8:?}, sel13: {=u8:?}, sel14: {=u8:?}, sel15: {=u8:?} }}" , self . sel0 () , self . sel1 () , self . sel2 () , self . sel3 () , self . sel4 () , self . sel5 () , self . sel6 () , self . sel7 () , self . sel8 () , self . sel9 () , self . sel10 () , self . sel11 () , self . sel12 () , self . sel13 () , self . sel14 () , self . sel15 ())
            }
        }
        #[doc = "This is the VREF Control Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct VrefCtrl(pub u32);
        impl VrefCtrl {
            #[doc = "This field represnts the delay time to charge up the external VREF capacitor."]
            #[inline(always)]
            pub const fn chrg_dly(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "This field represnts the delay time to charge up the external VREF capacitor."]
            #[inline(always)]
            pub fn set_chrg_dly(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
            #[doc = "This field represnts the delay time interval between switching VREF Selects."]
            #[inline(always)]
            pub const fn switch_dly(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0x1fff;
                val as u16
            }
            #[doc = "This field represnts the delay time interval between switching VREF Selects."]
            #[inline(always)]
            pub fn set_switch_dly(&mut self, val: u16) {
                self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
            }
            #[doc = "This fields give the choice to the application whether to float the unused PAD's or to Drive them to 0. 1= Drive unused PAD's Low 0b 0= Leave unused PAD's Floating."]
            #[inline(always)]
            pub const fn padctrl(&self) -> bool {
                let val = (self.0 >> 29usize) & 0x01;
                val != 0
            }
            #[doc = "This fields give the choice to the application whether to float the unused PAD's or to Drive them to 0. 1= Drive unused PAD's Low 0b 0= Leave unused PAD's Floating."]
            #[inline(always)]
            pub fn set_padctrl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
            }
            #[doc = "This fields gives information about the current VREF selected. 0x0= VREF0 0x1= VREF1 0x2= Reserved 0x3= Reserved"]
            #[inline(always)]
            pub const fn selstat(&self) -> u8 {
                let val = (self.0 >> 30usize) & 0x03;
                val as u8
            }
            #[doc = "This fields gives information about the current VREF selected. 0x0= VREF0 0x1= VREF1 0x2= Reserved 0x3= Reserved"]
            #[inline(always)]
            pub fn set_selstat(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
            }
        }
        impl Default for VrefCtrl {
            #[inline(always)]
            fn default() -> VrefCtrl {
                VrefCtrl(0)
            }
        }
        impl core::fmt::Debug for VrefCtrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("VrefCtrl")
                    .field("chrg_dly", &self.chrg_dly())
                    .field("switch_dly", &self.switch_dly())
                    .field("padctrl", &self.padctrl())
                    .field("selstat", &self.selstat())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for VrefCtrl {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "VrefCtrl {{ chrg_dly: {=u16:?}, switch_dly: {=u16:?}, padctrl: {=bool:?}, selstat: {=u8:?} }}",
                    self.chrg_dly(),
                    self.switch_dly(),
                    self.padctrl(),
                    self.selstat()
                )
            }
        }
    }
}
pub mod cct {
    #[doc = "This is a 16-bit auto-reloading timer/counter."]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cct {
        ptr: *mut u8,
    }
    unsafe impl Send for Cct {}
    unsafe impl Sync for Cct {}
    impl Cct {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "This register controls the capture and compare timer."]
        #[inline(always)]
        pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "This register is used to configure capture and compare timers 0-3."]
        #[inline(always)]
        pub const fn cap0_ctrl(self) -> crate::common::Reg<regs::Cap0ctrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "This register is used to configure capture and compare timers 4-5."]
        #[inline(always)]
        pub const fn cap1_ctrl(self) -> crate::common::Reg<regs::Cap1ctrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "This register contains the current value of the Free Running Timer."]
        #[inline(always)]
        pub const fn free_run(self) -> crate::common::Reg<regs::FreeRun, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT0."]
        #[inline(always)]
        pub const fn cap0(self) -> crate::common::Reg<regs::Cap0, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT1."]
        #[inline(always)]
        pub const fn cap1(self) -> crate::common::Reg<regs::Cap1, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT0."]
        #[inline(always)]
        pub const fn cap2(self) -> crate::common::Reg<regs::Cap2, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
        }
        #[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT0."]
        #[inline(always)]
        pub const fn cap3(self) -> crate::common::Reg<regs::Cap3, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
        #[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT4."]
        #[inline(always)]
        pub const fn cap4(self) -> crate::common::Reg<regs::Cap4, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }
        #[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT5."]
        #[inline(always)]
        pub const fn cap5(self) -> crate::common::Reg<regs::Cap5, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
        }
        #[doc = "A COMPARE 0 interrupt is generated when this register matches the value in the Free Running Timer."]
        #[inline(always)]
        pub const fn comp0(self) -> crate::common::Reg<regs::Comp0, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
        }
        #[doc = "A COMPARE 1 interrupt is generated when this register matches the value in the Free Running Timer."]
        #[inline(always)]
        pub const fn comp1(self) -> crate::common::Reg<regs::Comp1, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
        }
        #[doc = "This register selects the pin mapping to the capture register."]
        #[inline(always)]
        pub const fn mux_sel(self) -> crate::common::Reg<regs::MuxSel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT0."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cap0(pub u32);
        impl Cap0 {
            #[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT0."]
            #[inline(always)]
            pub const fn cap_0(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT0."]
            #[inline(always)]
            pub fn set_cap_0(&mut self, val: u32) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Cap0 {
            #[inline(always)]
            fn default() -> Cap0 {
                Cap0(0)
            }
        }
        impl core::fmt::Debug for Cap0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Cap0").field("cap_0", &self.cap_0()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Cap0 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Cap0 {{ cap_0: {=u32:?} }}", self.cap_0())
            }
        }
        #[doc = "This register is used to configure capture and compare timers 0-3."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cap0ctrl(pub u32);
        impl Cap0ctrl {
            #[doc = "This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 0."]
            #[inline(always)]
            pub const fn cap_edge0(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x03;
                val as u8
            }
            #[doc = "This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 0."]
            #[inline(always)]
            pub fn set_cap_edge0(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
            }
            #[doc = "This bit enables bypassing the input noise filter for Capture Register 0, so that the input signal goes directly into the timer."]
            #[inline(always)]
            pub const fn filter_byp0(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "This bit enables bypassing the input noise filter for Capture Register 0, so that the input signal goes directly into the timer."]
            #[inline(always)]
            pub fn set_filter_byp0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "This 3-bit field sets the clock source for the input filter for Capture Register 0."]
            #[inline(always)]
            pub const fn fclk_sel0(&self) -> u8 {
                let val = (self.0 >> 5usize) & 0x07;
                val as u8
            }
            #[doc = "This 3-bit field sets the clock source for the input filter for Capture Register 0."]
            #[inline(always)]
            pub fn set_fclk_sel0(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
            }
            #[doc = "This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 1."]
            #[inline(always)]
            pub const fn cap_edge1(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0x03;
                val as u8
            }
            #[doc = "This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 1."]
            #[inline(always)]
            pub fn set_cap_edge1(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
            }
            #[doc = "This bit enables bypassing the input noise filter for Capture Register 1, so that the input signal goes directly into the timer."]
            #[inline(always)]
            pub const fn filter_byp1(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "This bit enables bypassing the input noise filter for Capture Register 1, so that the input signal goes directly into the timer."]
            #[inline(always)]
            pub fn set_filter_byp1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "This 3-bit field sets the clock source for the input filter for Capture Register 1."]
            #[inline(always)]
            pub const fn fclk_sel1(&self) -> u8 {
                let val = (self.0 >> 13usize) & 0x07;
                val as u8
            }
            #[doc = "This 3-bit field sets the clock source for the input filter for Capture Register 1."]
            #[inline(always)]
            pub fn set_fclk_sel1(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
            }
            #[doc = "This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 2."]
            #[inline(always)]
            pub const fn cap_edge2(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0x03;
                val as u8
            }
            #[doc = "This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 2."]
            #[inline(always)]
            pub fn set_cap_edge2(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
            }
            #[doc = "This bit enables bypassing the input noise filter for Capture Register 2, so that the input signal goes directly into the timer."]
            #[inline(always)]
            pub const fn filter_byp2(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "This bit enables bypassing the input noise filter for Capture Register 2, so that the input signal goes directly into the timer."]
            #[inline(always)]
            pub fn set_filter_byp2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "This 3-bit field sets the clock source for the input filter for Capture Register 2."]
            #[inline(always)]
            pub const fn fclk_sel2(&self) -> u8 {
                let val = (self.0 >> 21usize) & 0x07;
                val as u8
            }
            #[doc = "This 3-bit field sets the clock source for the input filter for Capture Register 2."]
            #[inline(always)]
            pub fn set_fclk_sel2(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
            }
            #[doc = "This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 3."]
            #[inline(always)]
            pub const fn cap_edge3(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0x03;
                val as u8
            }
            #[doc = "This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 3."]
            #[inline(always)]
            pub fn set_cap_edge3(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
            }
            #[doc = "This bit enables bypassing the input noise filter for Capture Register 3, so that the input signal goes directly into the timer."]
            #[inline(always)]
            pub const fn filter_byp3(&self) -> bool {
                let val = (self.0 >> 26usize) & 0x01;
                val != 0
            }
            #[doc = "This bit enables bypassing the input noise filter for Capture Register 3, so that the input signal goes directly into the timer."]
            #[inline(always)]
            pub fn set_filter_byp3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
            }
            #[doc = "This 3-bit field sets the clock source for the input filter for Capture Register 3."]
            #[inline(always)]
            pub const fn fclk_sel3(&self) -> u8 {
                let val = (self.0 >> 29usize) & 0x07;
                val as u8
            }
            #[doc = "This 3-bit field sets the clock source for the input filter for Capture Register 3."]
            #[inline(always)]
            pub fn set_fclk_sel3(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
            }
        }
        impl Default for Cap0ctrl {
            #[inline(always)]
            fn default() -> Cap0ctrl {
                Cap0ctrl(0)
            }
        }
        impl core::fmt::Debug for Cap0ctrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Cap0ctrl")
                    .field("cap_edge0", &self.cap_edge0())
                    .field("filter_byp0", &self.filter_byp0())
                    .field("fclk_sel0", &self.fclk_sel0())
                    .field("cap_edge1", &self.cap_edge1())
                    .field("filter_byp1", &self.filter_byp1())
                    .field("fclk_sel1", &self.fclk_sel1())
                    .field("cap_edge2", &self.cap_edge2())
                    .field("filter_byp2", &self.filter_byp2())
                    .field("fclk_sel2", &self.fclk_sel2())
                    .field("cap_edge3", &self.cap_edge3())
                    .field("filter_byp3", &self.filter_byp3())
                    .field("fclk_sel3", &self.fclk_sel3())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Cap0ctrl {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Cap0ctrl {{ cap_edge0: {=u8:?}, filter_byp0: {=bool:?}, fclk_sel0: {=u8:?}, cap_edge1: {=u8:?}, filter_byp1: {=bool:?}, fclk_sel1: {=u8:?}, cap_edge2: {=u8:?}, filter_byp2: {=bool:?}, fclk_sel2: {=u8:?}, cap_edge3: {=u8:?}, filter_byp3: {=bool:?}, fclk_sel3: {=u8:?} }}" , self . cap_edge0 () , self . filter_byp0 () , self . fclk_sel0 () , self . cap_edge1 () , self . filter_byp1 () , self . fclk_sel1 () , self . cap_edge2 () , self . filter_byp2 () , self . fclk_sel2 () , self . cap_edge3 () , self . filter_byp3 () , self . fclk_sel3 ())
            }
        }
        #[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT1."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cap1(pub u32);
        impl Cap1 {
            #[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT1."]
            #[inline(always)]
            pub const fn cap_1(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT1."]
            #[inline(always)]
            pub fn set_cap_1(&mut self, val: u32) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Cap1 {
            #[inline(always)]
            fn default() -> Cap1 {
                Cap1(0)
            }
        }
        impl core::fmt::Debug for Cap1 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Cap1").field("cap_1", &self.cap_1()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Cap1 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Cap1 {{ cap_1: {=u32:?} }}", self.cap_1())
            }
        }
        #[doc = "This register is used to configure capture and compare timers 4-5."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cap1ctrl(pub u32);
        impl Cap1ctrl {
            #[doc = "This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 4."]
            #[inline(always)]
            pub const fn cap_edge4(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x03;
                val as u8
            }
            #[doc = "This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 4."]
            #[inline(always)]
            pub fn set_cap_edge4(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
            }
            #[doc = "This bit enables bypassing the input noise filter for Capture Register 4, so that the input signal goes directly into the timer."]
            #[inline(always)]
            pub const fn filter_byp4(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "This bit enables bypassing the input noise filter for Capture Register 4, so that the input signal goes directly into the timer."]
            #[inline(always)]
            pub fn set_filter_byp4(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "This 3-bit field sets the clock source for the input filter for Capture Register 4."]
            #[inline(always)]
            pub const fn fclk_sel4(&self) -> u8 {
                let val = (self.0 >> 5usize) & 0x07;
                val as u8
            }
            #[doc = "This 3-bit field sets the clock source for the input filter for Capture Register 4."]
            #[inline(always)]
            pub fn set_fclk_sel4(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
            }
            #[doc = "This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 5."]
            #[inline(always)]
            pub const fn cap_edge5(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0x03;
                val as u8
            }
            #[doc = "This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 5."]
            #[inline(always)]
            pub fn set_cap_edge5(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
            }
            #[doc = "This bit enables bypassing the input noise filter for Capture Register 5, so that the input signal goes directly into the timer."]
            #[inline(always)]
            pub const fn filter_byp5(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "This bit enables bypassing the input noise filter for Capture Register 5, so that the input signal goes directly into the timer."]
            #[inline(always)]
            pub fn set_filter_byp5(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "This 3-bit field sets the clock source for the input filter for Capture Register 5."]
            #[inline(always)]
            pub const fn fclk_sel5(&self) -> u8 {
                let val = (self.0 >> 13usize) & 0x07;
                val as u8
            }
            #[doc = "This 3-bit field sets the clock source for the input filter for Capture Register 5."]
            #[inline(always)]
            pub fn set_fclk_sel5(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
            }
        }
        impl Default for Cap1ctrl {
            #[inline(always)]
            fn default() -> Cap1ctrl {
                Cap1ctrl(0)
            }
        }
        impl core::fmt::Debug for Cap1ctrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Cap1ctrl")
                    .field("cap_edge4", &self.cap_edge4())
                    .field("filter_byp4", &self.filter_byp4())
                    .field("fclk_sel4", &self.fclk_sel4())
                    .field("cap_edge5", &self.cap_edge5())
                    .field("filter_byp5", &self.filter_byp5())
                    .field("fclk_sel5", &self.fclk_sel5())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Cap1ctrl {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Cap1ctrl {{ cap_edge4: {=u8:?}, filter_byp4: {=bool:?}, fclk_sel4: {=u8:?}, cap_edge5: {=u8:?}, filter_byp5: {=bool:?}, fclk_sel5: {=u8:?} }}" , self . cap_edge4 () , self . filter_byp4 () , self . fclk_sel4 () , self . cap_edge5 () , self . filter_byp5 () , self . fclk_sel5 ())
            }
        }
        #[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT0."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cap2(pub u32);
        impl Cap2 {
            #[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT2."]
            #[inline(always)]
            pub const fn cap_2(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT2."]
            #[inline(always)]
            pub fn set_cap_2(&mut self, val: u32) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Cap2 {
            #[inline(always)]
            fn default() -> Cap2 {
                Cap2(0)
            }
        }
        impl core::fmt::Debug for Cap2 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Cap2").field("cap_2", &self.cap_2()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Cap2 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Cap2 {{ cap_2: {=u32:?} }}", self.cap_2())
            }
        }
        #[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT0."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cap3(pub u32);
        impl Cap3 {
            #[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT3."]
            #[inline(always)]
            pub const fn cap_3(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT3."]
            #[inline(always)]
            pub fn set_cap_3(&mut self, val: u32) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Cap3 {
            #[inline(always)]
            fn default() -> Cap3 {
                Cap3(0)
            }
        }
        impl core::fmt::Debug for Cap3 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Cap3").field("cap_3", &self.cap_3()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Cap3 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Cap3 {{ cap_3: {=u32:?} }}", self.cap_3())
            }
        }
        #[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT4."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cap4(pub u32);
        impl Cap4 {
            #[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT4."]
            #[inline(always)]
            pub const fn cap_4(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT4."]
            #[inline(always)]
            pub fn set_cap_4(&mut self, val: u32) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Cap4 {
            #[inline(always)]
            fn default() -> Cap4 {
                Cap4(0)
            }
        }
        impl core::fmt::Debug for Cap4 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Cap4").field("cap_4", &self.cap_4()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Cap4 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Cap4 {{ cap_4: {=u32:?} }}", self.cap_4())
            }
        }
        #[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT5."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cap5(pub u32);
        impl Cap5 {
            #[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT5."]
            #[inline(always)]
            pub const fn cap_5(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT5."]
            #[inline(always)]
            pub fn set_cap_5(&mut self, val: u32) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Cap5 {
            #[inline(always)]
            fn default() -> Cap5 {
                Cap5(0)
            }
        }
        impl core::fmt::Debug for Cap5 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Cap5").field("cap_5", &self.cap_5()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Cap5 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Cap5 {{ cap_5: {=u32:?} }}", self.cap_5())
            }
        }
        #[doc = "A COMPARE 0 interrupt is generated when this register matches the value in the Free Running Timer."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Comp0(pub u32);
        impl Comp0 {
            #[doc = "A COMPARE 0 interrupt is generated when this register matches the value in the Free Running Timer."]
            #[inline(always)]
            pub const fn comp_0(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "A COMPARE 0 interrupt is generated when this register matches the value in the Free Running Timer."]
            #[inline(always)]
            pub fn set_comp_0(&mut self, val: u32) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Comp0 {
            #[inline(always)]
            fn default() -> Comp0 {
                Comp0(0)
            }
        }
        impl core::fmt::Debug for Comp0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Comp0").field("comp_0", &self.comp_0()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Comp0 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Comp0 {{ comp_0: {=u32:?} }}", self.comp_0())
            }
        }
        #[doc = "A COMPARE 1 interrupt is generated when this register matches the value in the Free Running Timer."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Comp1(pub u32);
        impl Comp1 {
            #[doc = "A COMPARE 1 interrupt is generated when this register matches the value in the Free Running Timer."]
            #[inline(always)]
            pub const fn comp_1(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "A COMPARE 1 interrupt is generated when this register matches the value in the Free Running Timer."]
            #[inline(always)]
            pub fn set_comp_1(&mut self, val: u32) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Comp1 {
            #[inline(always)]
            fn default() -> Comp1 {
                Comp1(0)
            }
        }
        impl core::fmt::Debug for Comp1 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Comp1").field("comp_1", &self.comp_1()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Comp1 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Comp1 {{ comp_1: {=u32:?} }}", self.comp_1())
            }
        }
        #[doc = "This register controls the capture and compare timer."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctrl(pub u32);
        impl Ctrl {
            #[doc = "This bit is used to start the capture and compare timer running and power it down."]
            #[inline(always)]
            pub const fn act(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This bit is used to start the capture and compare timer running and power it down."]
            #[inline(always)]
            pub fn set_act(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Free-Running Timer Enable. This bit is used to start and stop the free running timer."]
            #[inline(always)]
            pub const fn free_en(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Free-Running Timer Enable. This bit is used to start and stop the free running timer."]
            #[inline(always)]
            pub fn set_free_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Free Running Timer Reset. This bit stops the timer and resets the internal counter to 0000_0000h."]
            #[inline(always)]
            pub const fn free_rst(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Free Running Timer Reset. This bit stops the timer and resets the internal counter to 0000_0000h."]
            #[inline(always)]
            pub fn set_free_rst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "This 3-bit field sets the clock source for the Free-Running Counter."]
            #[inline(always)]
            pub const fn tclk(&self) -> u8 {
                let val = (self.0 >> 4usize) & 0x07;
                val as u8
            }
            #[doc = "This 3-bit field sets the clock source for the Free-Running Counter."]
            #[inline(always)]
            pub fn set_tclk(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
            }
            #[doc = "Compare Enable for Compare 0 Register."]
            #[inline(always)]
            pub const fn cmp_en0(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Compare Enable for Compare 0 Register."]
            #[inline(always)]
            pub fn set_cmp_en0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Compare Enable for Compare 1 Register."]
            #[inline(always)]
            pub const fn cmp_en1(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Compare Enable for Compare 1 Register."]
            #[inline(always)]
            pub fn set_cmp_en1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "When read, returns the current value off the Compare Timer Output 1 state."]
            #[inline(always)]
            pub const fn cmp_set1(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "When read, returns the current value off the Compare Timer Output 1 state."]
            #[inline(always)]
            pub fn set_cmp_set1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "When read, returns the current value off the Compare Timer Output 0 state."]
            #[inline(always)]
            pub const fn cmp_set0(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "When read, returns the current value off the Compare Timer Output 0 state."]
            #[inline(always)]
            pub fn set_cmp_set0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "When read, returns the current value off the Compare Timer Output 1 state."]
            #[inline(always)]
            pub const fn cmp_clr1(&self) -> bool {
                let val = (self.0 >> 24usize) & 0x01;
                val != 0
            }
            #[doc = "When read, returns the current value off the Compare Timer Output 1 state."]
            #[inline(always)]
            pub fn set_cmp_clr1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
            }
            #[doc = "When read, returns the current value off the Compare Timer Output 0 state."]
            #[inline(always)]
            pub const fn cmp_clr0(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[doc = "When read, returns the current value off the Compare Timer Output 0 state."]
            #[inline(always)]
            pub fn set_cmp_clr0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
            }
        }
        impl Default for Ctrl {
            #[inline(always)]
            fn default() -> Ctrl {
                Ctrl(0)
            }
        }
        impl core::fmt::Debug for Ctrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ctrl")
                    .field("act", &self.act())
                    .field("free_en", &self.free_en())
                    .field("free_rst", &self.free_rst())
                    .field("tclk", &self.tclk())
                    .field("cmp_en0", &self.cmp_en0())
                    .field("cmp_en1", &self.cmp_en1())
                    .field("cmp_set1", &self.cmp_set1())
                    .field("cmp_set0", &self.cmp_set0())
                    .field("cmp_clr1", &self.cmp_clr1())
                    .field("cmp_clr0", &self.cmp_clr0())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ctrl {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Ctrl {{ act: {=bool:?}, free_en: {=bool:?}, free_rst: {=bool:?}, tclk: {=u8:?}, cmp_en0: {=bool:?}, cmp_en1: {=bool:?}, cmp_set1: {=bool:?}, cmp_set0: {=bool:?}, cmp_clr1: {=bool:?}, cmp_clr0: {=bool:?} }}" , self . act () , self . free_en () , self . free_rst () , self . tclk () , self . cmp_en0 () , self . cmp_en1 () , self . cmp_set1 () , self . cmp_set0 () , self . cmp_clr1 () , self . cmp_clr0 ())
            }
        }
        #[doc = "This register contains the current value of the Free Running Timer."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct FreeRun(pub u32);
        impl FreeRun {
            #[doc = "This register contains the current value of the Free Running Timer."]
            #[inline(always)]
            pub const fn tmr(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "This register contains the current value of the Free Running Timer."]
            #[inline(always)]
            pub fn set_tmr(&mut self, val: u32) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for FreeRun {
            #[inline(always)]
            fn default() -> FreeRun {
                FreeRun(0)
            }
        }
        impl core::fmt::Debug for FreeRun {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("FreeRun").field("tmr", &self.tmr()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for FreeRun {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "FreeRun {{ tmr: {=u32:?} }}", self.tmr())
            }
        }
        #[doc = "This register selects the pin mapping to the capture register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct MuxSel(pub u32);
        impl MuxSel {
            #[doc = "Mux Select for Capture 0 register."]
            #[inline(always)]
            pub const fn cap0(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "Mux Select for Capture 0 register."]
            #[inline(always)]
            pub fn set_cap0(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
            #[doc = "Mux Select for Capture 1 register."]
            #[inline(always)]
            pub const fn cap1(&self) -> u8 {
                let val = (self.0 >> 4usize) & 0x0f;
                val as u8
            }
            #[doc = "Mux Select for Capture 1 register."]
            #[inline(always)]
            pub fn set_cap1(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
            }
            #[doc = "Mux Select for Capture 2 register."]
            #[inline(always)]
            pub const fn cap2(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0x0f;
                val as u8
            }
            #[doc = "Mux Select for Capture 2 register."]
            #[inline(always)]
            pub fn set_cap2(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
            }
            #[doc = "Mux Select for Capture 3 register."]
            #[inline(always)]
            pub const fn cap3(&self) -> u8 {
                let val = (self.0 >> 12usize) & 0x0f;
                val as u8
            }
            #[doc = "Mux Select for Capture 3 register."]
            #[inline(always)]
            pub fn set_cap3(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
            }
            #[doc = "Mux Select for Capture 4 register."]
            #[inline(always)]
            pub const fn cap4(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0x0f;
                val as u8
            }
            #[doc = "Mux Select for Capture 4 register."]
            #[inline(always)]
            pub fn set_cap4(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
            }
            #[doc = "Mux Select for Capture 5 register."]
            #[inline(always)]
            pub const fn cap5(&self) -> u8 {
                let val = (self.0 >> 20usize) & 0x0f;
                val as u8
            }
            #[doc = "Mux Select for Capture 5 register."]
            #[inline(always)]
            pub fn set_cap5(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
            }
        }
        impl Default for MuxSel {
            #[inline(always)]
            fn default() -> MuxSel {
                MuxSel(0)
            }
        }
        impl core::fmt::Debug for MuxSel {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("MuxSel")
                    .field("cap0", &self.cap0())
                    .field("cap1", &self.cap1())
                    .field("cap2", &self.cap2())
                    .field("cap3", &self.cap3())
                    .field("cap4", &self.cap4())
                    .field("cap5", &self.cap5())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for MuxSel {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "MuxSel {{ cap0: {=u8:?}, cap1: {=u8:?}, cap2: {=u8:?}, cap3: {=u8:?}, cap4: {=u8:?}, cap5: {=u8:?} }}" , self . cap0 () , self . cap1 () , self . cap2 () , self . cap3 () , self . cap4 () , self . cap5 ())
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
pub mod dma_chan00 {
    #[doc = "DMA Channel 00 Registers"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaChan00 {
        ptr: *mut u8,
    }
    unsafe impl Send for DmaChan00 {}
    unsafe impl Sync for DmaChan00 {}
    impl DmaChan00 {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Enable this channel for operation. The DMA Main Control: Activate must also be enabled for this channel to be operational."]
        #[inline(always)]
        pub const fn activate(self) -> crate::common::Reg<regs::Activate, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "This is the starting address for the Memory device."]
        #[inline(always)]
        pub const fn mstart(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "This is the ending address for the Memory device."]
        #[inline(always)]
        pub const fn mend(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "This is the Master Device address."]
        #[inline(always)]
        pub const fn dstart(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "DMA Channel N Control"]
        #[inline(always)]
        pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "DMA Channel N Interrupt Status"]
        #[inline(always)]
        pub const fn ists(self) -> crate::common::Reg<regs::Ists, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "DMA CHANNEL N INTERRUPT ENABLE"]
        #[inline(always)]
        pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
        }
        #[doc = "DMA CHANNEL N CRC ENABLE"]
        #[inline(always)]
        pub const fn crc_en(self) -> crate::common::Reg<regs::CrcEn, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }
        #[doc = "DMA CHANNEL N CRC DATA"]
        #[inline(always)]
        pub const fn crc_data(self) -> crate::common::Reg<regs::CrcData, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
        }
        #[doc = "DMA CHANNEL N CRC POST STATUS"]
        #[inline(always)]
        pub const fn crc_post_sts(self) -> crate::common::Reg<regs::CrcPostSts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Enable this channel for operation. The DMA Main Control: Activate must also be enabled for this channel to be operational."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Activate(pub u8);
        impl Activate {
            #[doc = "Enable this channel for operation. The DMA Main Control:Activate must also be enabled for this channel to be operational. 1=Enable channel(block). Each individual channel must be enabled separately. 0=Disable channel(block)."]
            #[inline(always)]
            pub const fn chn(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable this channel for operation. The DMA Main Control:Activate must also be enabled for this channel to be operational. 1=Enable channel(block). Each individual channel must be enabled separately. 0=Disable channel(block)."]
            #[inline(always)]
            pub fn set_chn(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
        }
        impl Default for Activate {
            #[inline(always)]
            fn default() -> Activate {
                Activate(0)
            }
        }
        impl core::fmt::Debug for Activate {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Activate").field("chn", &self.chn()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Activate {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Activate {{ chn: {=bool:?} }}", self.chn())
            }
        }
        #[doc = "DMA CHANNEL N CRC DATA"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct CrcData(pub u32);
        impl CrcData {
            #[doc = "Writes to this register initialize the CRC generator. Reads from this register return the output of the CRC that is calculated from the data transferred by DMA Channel N. The output of the CRC generator is bit-reversed and inverted on reads, as required by the CRC-32-IEEE definition. A CRC can be accumulated across multiple DMA transactions on Channel N. If it is necessary to save the intermediate CRC value, the result of the read of this register must be bit-reversed and inverted before being written back to this register."]
            #[inline(always)]
            pub const fn crc(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "Writes to this register initialize the CRC generator. Reads from this register return the output of the CRC that is calculated from the data transferred by DMA Channel N. The output of the CRC generator is bit-reversed and inverted on reads, as required by the CRC-32-IEEE definition. A CRC can be accumulated across multiple DMA transactions on Channel N. If it is necessary to save the intermediate CRC value, the result of the read of this register must be bit-reversed and inverted before being written back to this register."]
            #[inline(always)]
            pub fn set_crc(&mut self, val: u32) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for CrcData {
            #[inline(always)]
            fn default() -> CrcData {
                CrcData(0)
            }
        }
        impl core::fmt::Debug for CrcData {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("CrcData").field("crc", &self.crc()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for CrcData {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "CrcData {{ crc: {=u32:?} }}", self.crc())
            }
        }
        #[doc = "DMA CHANNEL N CRC ENABLE"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct CrcEn(pub u32);
        impl CrcEn {
            #[doc = "1=Enable the calculation of CRC-32 for DMA Channel N 0=Disable the calculation of CRC-32 for DMA Channel N"]
            #[inline(always)]
            pub const fn mode(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "1=Enable the calculation of CRC-32 for DMA Channel N 0=Disable the calculation of CRC-32 for DMA Channel N"]
            #[inline(always)]
            pub fn set_mode(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "The bit enables the transfer of the calculated CRC-32 after the completion of the DMA transaction. If the DMA transaction is aborted by either firmware or an internal bus error, the transfer will not occur. If the target of the DMA transfer is a device and the device signaled the termination of the DMA transaction, the CRC post transfer will not occur. 1=Enable the transfer of CRC-32 for DMA Channel N after the DMA transaction completes 0=Disable the automatic transfer of the CRC"]
            #[inline(always)]
            pub const fn post_trans(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "The bit enables the transfer of the calculated CRC-32 after the completion of the DMA transaction. If the DMA transaction is aborted by either firmware or an internal bus error, the transfer will not occur. If the target of the DMA transfer is a device and the device signaled the termination of the DMA transaction, the CRC post transfer will not occur. 1=Enable the transfer of CRC-32 for DMA Channel N after the DMA transaction completes 0=Disable the automatic transfer of the CRC"]
            #[inline(always)]
            pub fn set_post_trans(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
        }
        impl Default for CrcEn {
            #[inline(always)]
            fn default() -> CrcEn {
                CrcEn(0)
            }
        }
        impl core::fmt::Debug for CrcEn {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("CrcEn")
                    .field("mode", &self.mode())
                    .field("post_trans", &self.post_trans())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for CrcEn {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "CrcEn {{ mode: {=bool:?}, post_trans: {=bool:?} }}",
                    self.mode(),
                    self.post_trans()
                )
            }
        }
        #[doc = "DMA CHANNEL N CRC POST STATUS"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct CrcPostSts(pub u32);
        impl CrcPostSts {
            #[doc = "This bit is set to '1b' when the CRC calculation has completed from either normal or forced termination. It is cleared to '0b' when the DMA controller starts a new transfer on the channel."]
            #[inline(always)]
            pub const fn crc_done(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This bit is set to '1b' when the CRC calculation has completed from either normal or forced termination. It is cleared to '0b' when the DMA controller starts a new transfer on the channel."]
            #[inline(always)]
            pub fn set_crc_done(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "This bit is set to '1b' when the DMA controller starts the post-transfer transmission of the CRC. It is only set when the post-transfer is enabled by the CRC_POST_TRANSFER_ENABLE field. This bit is cleared to '0b' when the post-transfer completes."]
            #[inline(always)]
            pub const fn crc_running(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "This bit is set to '1b' when the DMA controller starts the post-transfer transmission of the CRC. It is only set when the post-transfer is enabled by the CRC_POST_TRANSFER_ENABLE field. This bit is cleared to '0b' when the post-transfer completes."]
            #[inline(always)]
            pub fn set_crc_running(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "This bit is set to '1b' when the DMA controller has completed the post-transfer of the CRC data. This bit is cleared to '0b' when the a new DMA transfer starts."]
            #[inline(always)]
            pub const fn crc_data_done(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "This bit is set to '1b' when the DMA controller has completed the post-transfer of the CRC data. This bit is cleared to '0b' when the a new DMA transfer starts."]
            #[inline(always)]
            pub fn set_crc_data_done(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "This bit is set to '1b' when the DMA controller is processing the post-transfer of the CRC data. This bit is cleared to '0b' when the post-transfer completes."]
            #[inline(always)]
            pub const fn crc_data_ready(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "This bit is set to '1b' when the DMA controller is processing the post-transfer of the CRC data. This bit is cleared to '0b' when the post-transfer completes."]
            #[inline(always)]
            pub fn set_crc_data_ready(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
        }
        impl Default for CrcPostSts {
            #[inline(always)]
            fn default() -> CrcPostSts {
                CrcPostSts(0)
            }
        }
        impl core::fmt::Debug for CrcPostSts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("CrcPostSts")
                    .field("crc_done", &self.crc_done())
                    .field("crc_running", &self.crc_running())
                    .field("crc_data_done", &self.crc_data_done())
                    .field("crc_data_ready", &self.crc_data_ready())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for CrcPostSts {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "CrcPostSts {{ crc_done: {=bool:?}, crc_running: {=bool:?}, crc_data_done: {=bool:?}, crc_data_ready: {=bool:?} }}" , self . crc_done () , self . crc_running () , self . crc_data_done () , self . crc_data_ready ())
            }
        }
        #[doc = "DMA Channel N Control"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctrl(pub u32);
        impl Ctrl {
            #[doc = "This is a control field. Note: This bit only applies to Hardware Flow Control mode. 1= This channel is enabled and will service transfer requests 0=This channel is disabled. All transfer requests are ignored."]
            #[inline(always)]
            pub const fn run(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This is a control field. Note: This bit only applies to Hardware Flow Control mode. 1= This channel is enabled and will service transfer requests 0=This channel is disabled. All transfer requests are ignored."]
            #[inline(always)]
            pub fn set_run(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "This is a status field. 1= There is a transfer request from the Master Device 0= There is no transfer request from the Master Device"]
            #[inline(always)]
            pub const fn req(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "This is a status field. 1= There is a transfer request from the Master Device 0= There is no transfer request from the Master Device"]
            #[inline(always)]
            pub fn set_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "This is a status signal. It is only valid while DMA Channel Control: Run is Enabled. This is the inverse of the DMA Channel Control: Busy field, except this is qualified with the DMA Channel Control:Run field. 1=Channel is done 0=Channel is not done or it is OFF"]
            #[inline(always)]
            pub const fn done(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "This is a status signal. It is only valid while DMA Channel Control: Run is Enabled. This is the inverse of the DMA Channel Control: Busy field, except this is qualified with the DMA Channel Control:Run field. 1=Channel is done 0=Channel is not done or it is OFF"]
            #[inline(always)]
            pub fn set_done(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "This is a status signal. The status decode is listed in priority order with the highest priority first. 3: Error detected by the DMA 2: The DMA Channel is externally done, in that the Device has terminated the transfer over the Hardware Flow Control through the Port dma_term 1: The DMA Channel is locally done, in that Memory Start Address equals Memory End Address 0: DMA Channel Control:Run is Disabled (0x0)"]
            #[inline(always)]
            pub const fn sts(&self) -> super::vals::Sts {
                let val = (self.0 >> 3usize) & 0x03;
                super::vals::Sts::from_bits(val as u8)
            }
            #[doc = "This is a status signal. The status decode is listed in priority order with the highest priority first. 3: Error detected by the DMA 2: The DMA Channel is externally done, in that the Device has terminated the transfer over the Hardware Flow Control through the Port dma_term 1: The DMA Channel is locally done, in that Memory Start Address equals Memory End Address 0: DMA Channel Control:Run is Disabled (0x0)"]
            #[inline(always)]
            pub fn set_sts(&mut self, val: super::vals::Sts) {
                self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
            }
            #[doc = "This is a status signal. 1=The DMA Channel is busy (FSM is not IDLE) 0=The DMA Channel is not busy (FSM is IDLE)"]
            #[inline(always)]
            pub const fn busy(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "This is a status signal. 1=The DMA Channel is busy (FSM is not IDLE) 0=The DMA Channel is not busy (FSM is IDLE)"]
            #[inline(always)]
            pub fn set_busy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "This determines the direction of the DMA Transfer. 1=Data Packet Read from Memory Start Address followed by Data Packet Write to Device Address 0=Data Packet Read from Device Address followed by Data Packet Write to Memory Start Address"]
            #[inline(always)]
            pub const fn tx_dir(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "This determines the direction of the DMA Transfer. 1=Data Packet Read from Memory Start Address followed by Data Packet Write to Device Address 0=Data Packet Read from Device Address followed by Data Packet Write to Memory Start Address"]
            #[inline(always)]
            pub fn set_tx_dir(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "This is the device that is connected to this channel as its Hardware Flow Control master. The Flow Control Interface is a bus with each master concatenated onto it. This selects which bus index of the concatenated Flow Control Interface bus is targeted towards this channel. The Flow Control Interface Port list is dma_req, dma_term, and dma_done."]
            #[inline(always)]
            pub const fn hw_flow_ctrl_dev(&self) -> u8 {
                let val = (self.0 >> 9usize) & 0x7f;
                val as u8
            }
            #[doc = "This is the device that is connected to this channel as its Hardware Flow Control master. The Flow Control Interface is a bus with each master concatenated onto it. This selects which bus index of the concatenated Flow Control Interface bus is targeted towards this channel. The Flow Control Interface Port list is dma_req, dma_term, and dma_done."]
            #[inline(always)]
            pub fn set_hw_flow_ctrl_dev(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
            }
            #[doc = "This will enable an auto-increment to the DMA Channel Memory Address. 1=Increment the DMA Channel Memory Address by DMA Channel Control:Transfer Size after every Data Packet transfer 0=Do nothing"]
            #[inline(always)]
            pub const fn inc_mem_addr(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "This will enable an auto-increment to the DMA Channel Memory Address. 1=Increment the DMA Channel Memory Address by DMA Channel Control:Transfer Size after every Data Packet transfer 0=Do nothing"]
            #[inline(always)]
            pub fn set_inc_mem_addr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "This will enable an auto-increment to the DMA Channel Device Address. 1: Increment the DMA Channel Device Address by DMA Channel Control:Transfer Size after every Data Packet transfer 0: Do nothing"]
            #[inline(always)]
            pub const fn inc_dev_addr(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "This will enable an auto-increment to the DMA Channel Device Address. 1: Increment the DMA Channel Device Address by DMA Channel Control:Transfer Size after every Data Packet transfer 0: Do nothing"]
            #[inline(always)]
            pub fn set_inc_dev_addr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "This is used to lock the arbitration of the Channel Arbiter on this channel once this channel is granted. Once this is locked, it will remain on the arbiter until it has completed it transfer (either the Transfer Aborted, Transfer Done or Transfer Terminated conditions)."]
            #[inline(always)]
            pub const fn lock(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "This is used to lock the arbitration of the Channel Arbiter on this channel once this channel is granted. Once this is locked, it will remain on the arbiter until it has completed it transfer (either the Transfer Aborted, Transfer Done or Transfer Terminated conditions)."]
            #[inline(always)]
            pub fn set_lock(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "This will Disable the Hardware Flow Control. When disabled, any DMA Master device attempting to communicate to the DMA over the DMA Flow Control Interface (Ports: dma_req, dma_term, and dma_done) will be ignored. This should be set before using the DMA channel in Firmware Flow Control mode."]
            #[inline(always)]
            pub const fn dis_hw_flow_ctrl(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "This will Disable the Hardware Flow Control. When disabled, any DMA Master device attempting to communicate to the DMA over the DMA Flow Control Interface (Ports: dma_req, dma_term, and dma_done) will be ignored. This should be set before using the DMA channel in Firmware Flow Control mode."]
            #[inline(always)]
            pub fn set_dis_hw_flow_ctrl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "This is the transfer size in Bytes of each Data Packet transfer. Note: The transfer size must be a legal AMBA transfer size. Valid sizes are 1, 2 and 4 Bytes."]
            #[inline(always)]
            pub const fn trans_size(&self) -> u8 {
                let val = (self.0 >> 20usize) & 0x07;
                val as u8
            }
            #[doc = "This is the transfer size in Bytes of each Data Packet transfer. Note: The transfer size must be a legal AMBA transfer size. Valid sizes are 1, 2 and 4 Bytes."]
            #[inline(always)]
            pub fn set_trans_size(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
            }
            #[doc = "This is used for the Firmware Flow Control DMA transfer."]
            #[inline(always)]
            pub const fn trans_go(&self) -> bool {
                let val = (self.0 >> 24usize) & 0x01;
                val != 0
            }
            #[doc = "This is used for the Firmware Flow Control DMA transfer."]
            #[inline(always)]
            pub fn set_trans_go(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
            }
            #[doc = "This is used to abort the current transfer on this DMA Channel. The aborted transfer will be forced to terminate immediately."]
            #[inline(always)]
            pub const fn trans_abort(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[doc = "This is used to abort the current transfer on this DMA Channel. The aborted transfer will be forced to terminate immediately."]
            #[inline(always)]
            pub fn set_trans_abort(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
            }
        }
        impl Default for Ctrl {
            #[inline(always)]
            fn default() -> Ctrl {
                Ctrl(0)
            }
        }
        impl core::fmt::Debug for Ctrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ctrl")
                    .field("run", &self.run())
                    .field("req", &self.req())
                    .field("done", &self.done())
                    .field("sts", &self.sts())
                    .field("busy", &self.busy())
                    .field("tx_dir", &self.tx_dir())
                    .field("hw_flow_ctrl_dev", &self.hw_flow_ctrl_dev())
                    .field("inc_mem_addr", &self.inc_mem_addr())
                    .field("inc_dev_addr", &self.inc_dev_addr())
                    .field("lock", &self.lock())
                    .field("dis_hw_flow_ctrl", &self.dis_hw_flow_ctrl())
                    .field("trans_size", &self.trans_size())
                    .field("trans_go", &self.trans_go())
                    .field("trans_abort", &self.trans_abort())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ctrl {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Ctrl {{ run: {=bool:?}, req: {=bool:?}, done: {=bool:?}, sts: {:?}, busy: {=bool:?}, tx_dir: {=bool:?}, hw_flow_ctrl_dev: {=u8:?}, inc_mem_addr: {=bool:?}, inc_dev_addr: {=bool:?}, lock: {=bool:?}, dis_hw_flow_ctrl: {=bool:?}, trans_size: {=u8:?}, trans_go: {=bool:?}, trans_abort: {=bool:?} }}" , self . run () , self . req () , self . done () , self . sts () , self . busy () , self . tx_dir () , self . hw_flow_ctrl_dev () , self . inc_mem_addr () , self . inc_dev_addr () , self . lock () , self . dis_hw_flow_ctrl () , self . trans_size () , self . trans_go () , self . trans_abort ())
            }
        }
        #[doc = "DMA CHANNEL N INTERRUPT ENABLE"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ien(pub u8);
        impl Ien {
            #[doc = "This is an interrupt enable for DMA Channel Interrupt:Status Bus Error. 1=Enable Interrupt 0=Disable Interrupt"]
            #[inline(always)]
            pub const fn sts_en_bus_err(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This is an interrupt enable for DMA Channel Interrupt:Status Bus Error. 1=Enable Interrupt 0=Disable Interrupt"]
            #[inline(always)]
            pub fn set_sts_en_bus_err(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
            #[doc = "This is an interrupt enable for DMA Channel Interrupt:Status Flow Control Error. 1=Enable Interrupt 0=Disable Interrupt"]
            #[inline(always)]
            pub const fn sts_en_flow_ctrl(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "This is an interrupt enable for DMA Channel Interrupt:Status Flow Control Error. 1=Enable Interrupt 0=Disable Interrupt"]
            #[inline(always)]
            pub fn set_sts_en_flow_ctrl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
            }
            #[doc = "This is an interrupt enable for DMA Channel Interrupt:Status Done. 1=Enable Interrupt 0=Disable Interrupt"]
            #[inline(always)]
            pub const fn sts_en_done(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "This is an interrupt enable for DMA Channel Interrupt:Status Done. 1=Enable Interrupt 0=Disable Interrupt"]
            #[inline(always)]
            pub fn set_sts_en_done(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
            }
        }
        impl Default for Ien {
            #[inline(always)]
            fn default() -> Ien {
                Ien(0)
            }
        }
        impl core::fmt::Debug for Ien {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ien")
                    .field("sts_en_bus_err", &self.sts_en_bus_err())
                    .field("sts_en_flow_ctrl", &self.sts_en_flow_ctrl())
                    .field("sts_en_done", &self.sts_en_done())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ien {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Ien {{ sts_en_bus_err: {=bool:?}, sts_en_flow_ctrl: {=bool:?}, sts_en_done: {=bool:?} }}",
                    self.sts_en_bus_err(),
                    self.sts_en_flow_ctrl(),
                    self.sts_en_done()
                )
            }
        }
        #[doc = "DMA Channel N Interrupt Status"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ists(pub u8);
        impl Ists {
            #[doc = "This is an interrupt source register. This flags when there is an Error detected over the internal 32-bit Bus. 1: Error detected. (R/WC)"]
            #[inline(always)]
            pub const fn bus_err(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This is an interrupt source register. This flags when there is an Error detected over the internal 32-bit Bus. 1: Error detected. (R/WC)"]
            #[inline(always)]
            pub fn set_bus_err(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
            #[doc = "This is an interrupt source register. This flags when the DMA Channel has encountered a Hardware Flow Control Request after the DMA Channel has completed the transfer. This means the Master Device is attempting to overflow the DMA. 1=Hardware Flow Control is requesting after the transfer has completed 0=No Hardware Flow Control event"]
            #[inline(always)]
            pub const fn flow_ctrl(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "This is an interrupt source register. This flags when the DMA Channel has encountered a Hardware Flow Control Request after the DMA Channel has completed the transfer. This means the Master Device is attempting to overflow the DMA. 1=Hardware Flow Control is requesting after the transfer has completed 0=No Hardware Flow Control event"]
            #[inline(always)]
            pub fn set_flow_ctrl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
            }
            #[doc = "This is an interrupt source register. This flags when the DMA Channel has completed a transfer successfully on its side. A completed transfer is defined as when the DMA Channel reaches its limit; Memory Start Address equals Memory End Address. A completion due to a Hardware Flow Control Terminate will not flag this interrupt. 1=Memory Start Address equals Memory End Address 0=Memory Start Address does not equal Memory End Address"]
            #[inline(always)]
            pub const fn done(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "This is an interrupt source register. This flags when the DMA Channel has completed a transfer successfully on its side. A completed transfer is defined as when the DMA Channel reaches its limit; Memory Start Address equals Memory End Address. A completion due to a Hardware Flow Control Terminate will not flag this interrupt. 1=Memory Start Address equals Memory End Address 0=Memory Start Address does not equal Memory End Address"]
            #[inline(always)]
            pub fn set_done(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
            }
        }
        impl Default for Ists {
            #[inline(always)]
            fn default() -> Ists {
                Ists(0)
            }
        }
        impl core::fmt::Debug for Ists {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ists")
                    .field("bus_err", &self.bus_err())
                    .field("flow_ctrl", &self.flow_ctrl())
                    .field("done", &self.done())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ists {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Ists {{ bus_err: {=bool:?}, flow_ctrl: {=bool:?}, done: {=bool:?} }}",
                    self.bus_err(),
                    self.flow_ctrl(),
                    self.done()
                )
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Sts {
            #[doc = "0: DMA Channel Control:Run is Disabled (0x0)"]
            DIS = 0x0,
            #[doc = "1: The DMA Channel is locally done, in that Memory Start Address equals Memory End Address"]
            LOC_DONE = 0x01,
            #[doc = "2: The DMA Channel is externally done, in that the Device has terminated the transfer over the Hardware Flow Control through the Port dma_term"]
            EXT_DONE = 0x02,
            #[doc = "3: Error detected by the DMA"]
            ERROR = 0x03,
        }
        impl Sts {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Sts {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Sts {
            #[inline(always)]
            fn from(val: u8) -> Sts {
                Sts::from_bits(val)
            }
        }
        impl From<Sts> for u8 {
            #[inline(always)]
            fn from(val: Sts) -> u8 {
                Sts::to_bits(val)
            }
        }
    }
}
pub mod dma_chan01 {
    #[doc = "DMA Channel 01 Registers"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaChan01 {
        ptr: *mut u8,
    }
    unsafe impl Send for DmaChan01 {}
    unsafe impl Sync for DmaChan01 {}
    impl DmaChan01 {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Enable this channel for operation. The DMA Main Control: Activate must also be enabled for this channel to be operational."]
        #[inline(always)]
        pub const fn activate(self) -> crate::common::Reg<regs::Activate, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "This is the starting address for the Memory device."]
        #[inline(always)]
        pub const fn mstart(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "This is the ending address for the Memory device."]
        #[inline(always)]
        pub const fn mend(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "This is the Master Device address."]
        #[inline(always)]
        pub const fn dstart(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "DMA Channel N Control"]
        #[inline(always)]
        pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "DMA Channel N Interrupt Status"]
        #[inline(always)]
        pub const fn ists(self) -> crate::common::Reg<regs::Ists, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "DMA CHANNEL N INTERRUPT ENABLE"]
        #[inline(always)]
        pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
        }
        #[doc = "DMA CHANNEL N FILL ENABLE"]
        #[inline(always)]
        pub const fn fill_en(self) -> crate::common::Reg<regs::FillEn, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }
        #[doc = "DMA CHANNEL N FILL DATA"]
        #[inline(always)]
        pub const fn fill_data(self) -> crate::common::Reg<regs::FillData, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
        }
        #[doc = "DMA CHANNEL N FILL STATUS"]
        #[inline(always)]
        pub const fn fill_sts(self) -> crate::common::Reg<regs::FillSts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Enable this channel for operation. The DMA Main Control: Activate must also be enabled for this channel to be operational."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Activate(pub u8);
        impl Activate {
            #[doc = "Enable this channel for operation. The DMA Main Control:Activate must also be enabled for this channel to be operational. 1=Enable channel(block). Each individual channel must be enabled separately. 0=Disable channel(block)."]
            #[inline(always)]
            pub const fn chn(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable this channel for operation. The DMA Main Control:Activate must also be enabled for this channel to be operational. 1=Enable channel(block). Each individual channel must be enabled separately. 0=Disable channel(block)."]
            #[inline(always)]
            pub fn set_chn(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
        }
        impl Default for Activate {
            #[inline(always)]
            fn default() -> Activate {
                Activate(0)
            }
        }
        impl core::fmt::Debug for Activate {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Activate").field("chn", &self.chn()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Activate {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Activate {{ chn: {=bool:?} }}", self.chn())
            }
        }
        #[doc = "DMA Channel N Control"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctrl(pub u32);
        impl Ctrl {
            #[doc = "This is a control field. Note: This bit only applies to Hardware Flow Control mode. 1= This channel is enabled and will service transfer requests 0=This channel is disabled. All transfer requests are ignored."]
            #[inline(always)]
            pub const fn run(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This is a control field. Note: This bit only applies to Hardware Flow Control mode. 1= This channel is enabled and will service transfer requests 0=This channel is disabled. All transfer requests are ignored."]
            #[inline(always)]
            pub fn set_run(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "This is a status field. 1= There is a transfer request from the Master Device 0= There is no transfer request from the Master Device"]
            #[inline(always)]
            pub const fn req(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "This is a status field. 1= There is a transfer request from the Master Device 0= There is no transfer request from the Master Device"]
            #[inline(always)]
            pub fn set_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "This is a status signal. It is only valid while DMA Channel Control: Run is Enabled. This is the inverse of the DMA Channel Control: Busy field, except this is qualified with the DMA Channel Control:Run field. 1=Channel is done 0=Channel is not done or it is OFF"]
            #[inline(always)]
            pub const fn done(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "This is a status signal. It is only valid while DMA Channel Control: Run is Enabled. This is the inverse of the DMA Channel Control: Busy field, except this is qualified with the DMA Channel Control:Run field. 1=Channel is done 0=Channel is not done or it is OFF"]
            #[inline(always)]
            pub fn set_done(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "This is a status signal. The status decode is listed in priority order with the highest priority first. 3: Error detected by the DMA 2: The DMA Channel is externally done, in that the Device has terminated the transfer over the Hardware Flow Control through the Port dma_term 1: The DMA Channel is locally done, in that Memory Start Address equals Memory End Address 0: DMA Channel Control:Run is Disabled (0x0)"]
            #[inline(always)]
            pub const fn sts(&self) -> super::vals::Sts {
                let val = (self.0 >> 3usize) & 0x03;
                super::vals::Sts::from_bits(val as u8)
            }
            #[doc = "This is a status signal. The status decode is listed in priority order with the highest priority first. 3: Error detected by the DMA 2: The DMA Channel is externally done, in that the Device has terminated the transfer over the Hardware Flow Control through the Port dma_term 1: The DMA Channel is locally done, in that Memory Start Address equals Memory End Address 0: DMA Channel Control:Run is Disabled (0x0)"]
            #[inline(always)]
            pub fn set_sts(&mut self, val: super::vals::Sts) {
                self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
            }
            #[doc = "This is a status signal. 1=The DMA Channel is busy (FSM is not IDLE) 0=The DMA Channel is not busy (FSM is IDLE)"]
            #[inline(always)]
            pub const fn busy(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "This is a status signal. 1=The DMA Channel is busy (FSM is not IDLE) 0=The DMA Channel is not busy (FSM is IDLE)"]
            #[inline(always)]
            pub fn set_busy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "This determines the direction of the DMA Transfer. 1=Data Packet Read from Memory Start Address followed by Data Packet Write to Device Address 0=Data Packet Read from Device Address followed by Data Packet Write to Memory Start Address"]
            #[inline(always)]
            pub const fn tx_dir(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "This determines the direction of the DMA Transfer. 1=Data Packet Read from Memory Start Address followed by Data Packet Write to Device Address 0=Data Packet Read from Device Address followed by Data Packet Write to Memory Start Address"]
            #[inline(always)]
            pub fn set_tx_dir(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "This is the device that is connected to this channel as its Hardware Flow Control master. The Flow Control Interface is a bus with each master concatenated onto it. This selects which bus index of the concatenated Flow Control Interface bus is targeted towards this channel. The Flow Control Interface Port list is dma_req, dma_term, and dma_done."]
            #[inline(always)]
            pub const fn hw_flow_ctrl_dev(&self) -> u8 {
                let val = (self.0 >> 9usize) & 0x7f;
                val as u8
            }
            #[doc = "This is the device that is connected to this channel as its Hardware Flow Control master. The Flow Control Interface is a bus with each master concatenated onto it. This selects which bus index of the concatenated Flow Control Interface bus is targeted towards this channel. The Flow Control Interface Port list is dma_req, dma_term, and dma_done."]
            #[inline(always)]
            pub fn set_hw_flow_ctrl_dev(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
            }
            #[doc = "This will enable an auto-increment to the DMA Channel Memory Address. 1=Increment the DMA Channel Memory Address by DMA Channel Control:Transfer Size after every Data Packet transfer 0=Do nothing"]
            #[inline(always)]
            pub const fn inc_mem_addr(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "This will enable an auto-increment to the DMA Channel Memory Address. 1=Increment the DMA Channel Memory Address by DMA Channel Control:Transfer Size after every Data Packet transfer 0=Do nothing"]
            #[inline(always)]
            pub fn set_inc_mem_addr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "This will enable an auto-increment to the DMA Channel Device Address. 1: Increment the DMA Channel Device Address by DMA Channel Control:Transfer Size after every Data Packet transfer 0: Do nothing"]
            #[inline(always)]
            pub const fn inc_dev_addr(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "This will enable an auto-increment to the DMA Channel Device Address. 1: Increment the DMA Channel Device Address by DMA Channel Control:Transfer Size after every Data Packet transfer 0: Do nothing"]
            #[inline(always)]
            pub fn set_inc_dev_addr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "This is used to lock the arbitration of the Channel Arbiter on this channel once this channel is granted. Once this is locked, it will remain on the arbiter until it has completed it transfer (either the Transfer Aborted, Transfer Done or Transfer Terminated conditions)."]
            #[inline(always)]
            pub const fn lock(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "This is used to lock the arbitration of the Channel Arbiter on this channel once this channel is granted. Once this is locked, it will remain on the arbiter until it has completed it transfer (either the Transfer Aborted, Transfer Done or Transfer Terminated conditions)."]
            #[inline(always)]
            pub fn set_lock(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "This will Disable the Hardware Flow Control. When disabled, any DMA Master device attempting to communicate to the DMA over the DMA Flow Control Interface (Ports: dma_req, dma_term, and dma_done) will be ignored. This should be set before using the DMA channel in Firmware Flow Control mode."]
            #[inline(always)]
            pub const fn dis_hw_flow_ctrl(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "This will Disable the Hardware Flow Control. When disabled, any DMA Master device attempting to communicate to the DMA over the DMA Flow Control Interface (Ports: dma_req, dma_term, and dma_done) will be ignored. This should be set before using the DMA channel in Firmware Flow Control mode."]
            #[inline(always)]
            pub fn set_dis_hw_flow_ctrl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "This is the transfer size in Bytes of each Data Packet transfer. Note: The transfer size must be a legal AMBA transfer size. Valid sizes are 1, 2 and 4 Bytes."]
            #[inline(always)]
            pub const fn trans_size(&self) -> u8 {
                let val = (self.0 >> 20usize) & 0x07;
                val as u8
            }
            #[doc = "This is the transfer size in Bytes of each Data Packet transfer. Note: The transfer size must be a legal AMBA transfer size. Valid sizes are 1, 2 and 4 Bytes."]
            #[inline(always)]
            pub fn set_trans_size(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
            }
            #[doc = "This is used for the Firmware Flow Control DMA transfer."]
            #[inline(always)]
            pub const fn trans_go(&self) -> bool {
                let val = (self.0 >> 24usize) & 0x01;
                val != 0
            }
            #[doc = "This is used for the Firmware Flow Control DMA transfer."]
            #[inline(always)]
            pub fn set_trans_go(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
            }
            #[doc = "This is used to abort the current transfer on this DMA Channel. The aborted transfer will be forced to terminate immediately."]
            #[inline(always)]
            pub const fn trans_abort(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[doc = "This is used to abort the current transfer on this DMA Channel. The aborted transfer will be forced to terminate immediately."]
            #[inline(always)]
            pub fn set_trans_abort(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
            }
        }
        impl Default for Ctrl {
            #[inline(always)]
            fn default() -> Ctrl {
                Ctrl(0)
            }
        }
        impl core::fmt::Debug for Ctrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ctrl")
                    .field("run", &self.run())
                    .field("req", &self.req())
                    .field("done", &self.done())
                    .field("sts", &self.sts())
                    .field("busy", &self.busy())
                    .field("tx_dir", &self.tx_dir())
                    .field("hw_flow_ctrl_dev", &self.hw_flow_ctrl_dev())
                    .field("inc_mem_addr", &self.inc_mem_addr())
                    .field("inc_dev_addr", &self.inc_dev_addr())
                    .field("lock", &self.lock())
                    .field("dis_hw_flow_ctrl", &self.dis_hw_flow_ctrl())
                    .field("trans_size", &self.trans_size())
                    .field("trans_go", &self.trans_go())
                    .field("trans_abort", &self.trans_abort())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ctrl {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Ctrl {{ run: {=bool:?}, req: {=bool:?}, done: {=bool:?}, sts: {:?}, busy: {=bool:?}, tx_dir: {=bool:?}, hw_flow_ctrl_dev: {=u8:?}, inc_mem_addr: {=bool:?}, inc_dev_addr: {=bool:?}, lock: {=bool:?}, dis_hw_flow_ctrl: {=bool:?}, trans_size: {=u8:?}, trans_go: {=bool:?}, trans_abort: {=bool:?} }}" , self . run () , self . req () , self . done () , self . sts () , self . busy () , self . tx_dir () , self . hw_flow_ctrl_dev () , self . inc_mem_addr () , self . inc_dev_addr () , self . lock () , self . dis_hw_flow_ctrl () , self . trans_size () , self . trans_go () , self . trans_abort ())
            }
        }
        #[doc = "DMA CHANNEL N FILL DATA"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct FillData(pub u32);
        impl FillData {
            #[doc = "This is the data pattern used to fill memory."]
            #[inline(always)]
            pub const fn data(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "This is the data pattern used to fill memory."]
            #[inline(always)]
            pub fn set_data(&mut self, val: u32) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for FillData {
            #[inline(always)]
            fn default() -> FillData {
                FillData(0)
            }
        }
        impl core::fmt::Debug for FillData {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("FillData").field("data", &self.data()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for FillData {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "FillData {{ data: {=u32:?} }}", self.data())
            }
        }
        #[doc = "DMA CHANNEL N FILL ENABLE"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct FillEn(pub u32);
        impl FillEn {
            #[doc = "1=Enable the DMA Channel Fill Engine N 0=Disable the DMA Channel Fill Engine"]
            #[inline(always)]
            pub const fn mode(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "1=Enable the DMA Channel Fill Engine N 0=Disable the DMA Channel Fill Engine"]
            #[inline(always)]
            pub fn set_mode(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for FillEn {
            #[inline(always)]
            fn default() -> FillEn {
                FillEn(0)
            }
        }
        impl core::fmt::Debug for FillEn {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("FillEn").field("mode", &self.mode()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for FillEn {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "FillEn {{ mode: {=bool:?} }}", self.mode())
            }
        }
        #[doc = "DMA CHANNEL N FILL STATUS"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct FillSts(pub u32);
        impl FillSts {
            #[doc = "This bit is set to '1b' when the CRC calculation has completed from either normal or forced termination. It is cleared to '0b' when the DMA controller starts a new transfer on the channel."]
            #[inline(always)]
            pub const fn done(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This bit is set to '1b' when the CRC calculation has completed from either normal or forced termination. It is cleared to '0b' when the DMA controller starts a new transfer on the channel."]
            #[inline(always)]
            pub fn set_done(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "This bit is set to '1b' when the DMA controller starts the post-transfer transmission of the CRC. It is only set when the post-transfer is enabled by the CRC_POST_TRANSFER_ENABLE field. This bit is cleared to '0b' when the post-transfer completes."]
            #[inline(always)]
            pub const fn running(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "This bit is set to '1b' when the DMA controller starts the post-transfer transmission of the CRC. It is only set when the post-transfer is enabled by the CRC_POST_TRANSFER_ENABLE field. This bit is cleared to '0b' when the post-transfer completes."]
            #[inline(always)]
            pub fn set_running(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
        }
        impl Default for FillSts {
            #[inline(always)]
            fn default() -> FillSts {
                FillSts(0)
            }
        }
        impl core::fmt::Debug for FillSts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("FillSts")
                    .field("done", &self.done())
                    .field("running", &self.running())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for FillSts {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "FillSts {{ done: {=bool:?}, running: {=bool:?} }}",
                    self.done(),
                    self.running()
                )
            }
        }
        #[doc = "DMA CHANNEL N INTERRUPT ENABLE"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ien(pub u8);
        impl Ien {
            #[doc = "This is an interrupt enable for DMA Channel Interrupt:Status Bus Error. 1=Enable Interrupt 0=Disable Interrupt"]
            #[inline(always)]
            pub const fn sts_en_bus_err(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This is an interrupt enable for DMA Channel Interrupt:Status Bus Error. 1=Enable Interrupt 0=Disable Interrupt"]
            #[inline(always)]
            pub fn set_sts_en_bus_err(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
            #[doc = "This is an interrupt enable for DMA Channel Interrupt:Status Flow Control Error. 1=Enable Interrupt 0=Disable Interrupt"]
            #[inline(always)]
            pub const fn sts_en_flow_ctrl(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "This is an interrupt enable for DMA Channel Interrupt:Status Flow Control Error. 1=Enable Interrupt 0=Disable Interrupt"]
            #[inline(always)]
            pub fn set_sts_en_flow_ctrl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
            }
            #[doc = "This is an interrupt enable for DMA Channel Interrupt:Status Done. 1=Enable Interrupt 0=Disable Interrupt"]
            #[inline(always)]
            pub const fn sts_en_done(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "This is an interrupt enable for DMA Channel Interrupt:Status Done. 1=Enable Interrupt 0=Disable Interrupt"]
            #[inline(always)]
            pub fn set_sts_en_done(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
            }
        }
        impl Default for Ien {
            #[inline(always)]
            fn default() -> Ien {
                Ien(0)
            }
        }
        impl core::fmt::Debug for Ien {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ien")
                    .field("sts_en_bus_err", &self.sts_en_bus_err())
                    .field("sts_en_flow_ctrl", &self.sts_en_flow_ctrl())
                    .field("sts_en_done", &self.sts_en_done())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ien {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Ien {{ sts_en_bus_err: {=bool:?}, sts_en_flow_ctrl: {=bool:?}, sts_en_done: {=bool:?} }}",
                    self.sts_en_bus_err(),
                    self.sts_en_flow_ctrl(),
                    self.sts_en_done()
                )
            }
        }
        #[doc = "DMA Channel N Interrupt Status"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ists(pub u8);
        impl Ists {
            #[doc = "This is an interrupt source register. This flags when there is an Error detected over the internal 32-bit Bus. 1: Error detected. (R/WC)"]
            #[inline(always)]
            pub const fn bus_error(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This is an interrupt source register. This flags when there is an Error detected over the internal 32-bit Bus. 1: Error detected. (R/WC)"]
            #[inline(always)]
            pub fn set_bus_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
            #[doc = "This is an interrupt source register. This flags when the DMA Channel has encountered a Hardware Flow Control Request after the DMA Channel has completed the transfer. This means the Master Device is attempting to overflow the DMA. 1=Hardware Flow Control is requesting after the transfer has completed 0=No Hardware Flow Control event"]
            #[inline(always)]
            pub const fn flow_ctrl(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "This is an interrupt source register. This flags when the DMA Channel has encountered a Hardware Flow Control Request after the DMA Channel has completed the transfer. This means the Master Device is attempting to overflow the DMA. 1=Hardware Flow Control is requesting after the transfer has completed 0=No Hardware Flow Control event"]
            #[inline(always)]
            pub fn set_flow_ctrl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
            }
            #[doc = "This is an interrupt source register. This flags when the DMA Channel has completed a transfer successfully on its side. A completed transfer is defined as when the DMA Channel reaches its limit; Memory Start Address equals Memory End Address. A completion due to a Hardware Flow Control Terminate will not flag this interrupt. 1=Memory Start Address equals Memory End Address 0=Memory Start Address does not equal Memory End Address"]
            #[inline(always)]
            pub const fn done(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "This is an interrupt source register. This flags when the DMA Channel has completed a transfer successfully on its side. A completed transfer is defined as when the DMA Channel reaches its limit; Memory Start Address equals Memory End Address. A completion due to a Hardware Flow Control Terminate will not flag this interrupt. 1=Memory Start Address equals Memory End Address 0=Memory Start Address does not equal Memory End Address"]
            #[inline(always)]
            pub fn set_done(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
            }
        }
        impl Default for Ists {
            #[inline(always)]
            fn default() -> Ists {
                Ists(0)
            }
        }
        impl core::fmt::Debug for Ists {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ists")
                    .field("bus_error", &self.bus_error())
                    .field("flow_ctrl", &self.flow_ctrl())
                    .field("done", &self.done())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ists {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Ists {{ bus_error: {=bool:?}, flow_ctrl: {=bool:?}, done: {=bool:?} }}",
                    self.bus_error(),
                    self.flow_ctrl(),
                    self.done()
                )
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Sts {
            #[doc = "0: DMA Channel Control:Run is Disabled (0x0)"]
            DIS = 0x0,
            #[doc = "1: The DMA Channel is locally done, in that Memory Start Address equals Memory End Address"]
            LOC_DONE = 0x01,
            #[doc = "2: The DMA Channel is externally done, in that the Device has terminated the transfer over the Hardware Flow Control through the Port dma_term"]
            EXT_DONE = 0x02,
            #[doc = "3: Error detected by the DMA"]
            ERROR = 0x03,
        }
        impl Sts {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Sts {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Sts {
            #[inline(always)]
            fn from(val: u8) -> Sts {
                Sts::from_bits(val)
            }
        }
        impl From<Sts> for u8 {
            #[inline(always)]
            fn from(val: Sts) -> u8 {
                Sts::to_bits(val)
            }
        }
    }
}
pub mod dma_chan02 {
    #[doc = "DMA Channel 02 Registers"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaChan02 {
        ptr: *mut u8,
    }
    unsafe impl Send for DmaChan02 {}
    unsafe impl Sync for DmaChan02 {}
    impl DmaChan02 {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Enable this channel for operation. The DMA Main Control: Activate must also be enabled for this channel to be operational."]
        #[inline(always)]
        pub const fn activate(self) -> crate::common::Reg<regs::Activate, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "This is the starting address for the Memory device."]
        #[inline(always)]
        pub const fn mstart(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "This is the ending address for the Memory device."]
        #[inline(always)]
        pub const fn mend(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "This is the Master Device address."]
        #[inline(always)]
        pub const fn dstart(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "DMA Channel N Control"]
        #[inline(always)]
        pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "DMA Channel N Interrupt Status"]
        #[inline(always)]
        pub const fn ists(self) -> crate::common::Reg<regs::Ists, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "DMA CHANNEL N INTERRUPT ENABLE"]
        #[inline(always)]
        pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Enable this channel for operation. The DMA Main Control: Activate must also be enabled for this channel to be operational."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Activate(pub u8);
        impl Activate {
            #[doc = "Enable this channel for operation. The DMA Main Control:Activate must also be enabled for this channel to be operational. 1=Enable channel(block). Each individual channel must be enabled separately. 0=Disable channel(block)."]
            #[inline(always)]
            pub const fn chn(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable this channel for operation. The DMA Main Control:Activate must also be enabled for this channel to be operational. 1=Enable channel(block). Each individual channel must be enabled separately. 0=Disable channel(block)."]
            #[inline(always)]
            pub fn set_chn(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
        }
        impl Default for Activate {
            #[inline(always)]
            fn default() -> Activate {
                Activate(0)
            }
        }
        impl core::fmt::Debug for Activate {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Activate").field("chn", &self.chn()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Activate {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Activate {{ chn: {=bool:?} }}", self.chn())
            }
        }
        #[doc = "DMA Channel N Control"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctrl(pub u32);
        impl Ctrl {
            #[doc = "This is a control field. Note: This bit only applies to Hardware Flow Control mode. 1= This channel is enabled and will service transfer requests 0=This channel is disabled. All transfer requests are ignored."]
            #[inline(always)]
            pub const fn run(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This is a control field. Note: This bit only applies to Hardware Flow Control mode. 1= This channel is enabled and will service transfer requests 0=This channel is disabled. All transfer requests are ignored."]
            #[inline(always)]
            pub fn set_run(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "This is a status field. 1= There is a transfer request from the Master Device 0= There is no transfer request from the Master Device"]
            #[inline(always)]
            pub const fn req(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "This is a status field. 1= There is a transfer request from the Master Device 0= There is no transfer request from the Master Device"]
            #[inline(always)]
            pub fn set_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "This is a status signal. It is only valid while DMA Channel Control: Run is Enabled. This is the inverse of the DMA Channel Control: Busy field, except this is qualified with the DMA Channel Control:Run field. 1=Channel is done 0=Channel is not done or it is OFF"]
            #[inline(always)]
            pub const fn done(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "This is a status signal. It is only valid while DMA Channel Control: Run is Enabled. This is the inverse of the DMA Channel Control: Busy field, except this is qualified with the DMA Channel Control:Run field. 1=Channel is done 0=Channel is not done or it is OFF"]
            #[inline(always)]
            pub fn set_done(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "This is a status signal. The status decode is listed in priority order with the highest priority first. 3: Error detected by the DMA 2: The DMA Channel is externally done, in that the Device has terminated the transfer over the Hardware Flow Control through the Port dma_term 1: The DMA Channel is locally done, in that Memory Start Address equals Memory End Address 0: DMA Channel Control:Run is Disabled (0x0)"]
            #[inline(always)]
            pub const fn sts(&self) -> super::vals::Sts {
                let val = (self.0 >> 3usize) & 0x03;
                super::vals::Sts::from_bits(val as u8)
            }
            #[doc = "This is a status signal. The status decode is listed in priority order with the highest priority first. 3: Error detected by the DMA 2: The DMA Channel is externally done, in that the Device has terminated the transfer over the Hardware Flow Control through the Port dma_term 1: The DMA Channel is locally done, in that Memory Start Address equals Memory End Address 0: DMA Channel Control:Run is Disabled (0x0)"]
            #[inline(always)]
            pub fn set_sts(&mut self, val: super::vals::Sts) {
                self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
            }
            #[doc = "This is a status signal. 1=The DMA Channel is busy (FSM is not IDLE) 0=The DMA Channel is not busy (FSM is IDLE)"]
            #[inline(always)]
            pub const fn busy(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "This is a status signal. 1=The DMA Channel is busy (FSM is not IDLE) 0=The DMA Channel is not busy (FSM is IDLE)"]
            #[inline(always)]
            pub fn set_busy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "This determines the direction of the DMA Transfer. 1=Data Packet Read from Memory Start Address followed by Data Packet Write to Device Address 0=Data Packet Read from Device Address followed by Data Packet Write to Memory Start Address"]
            #[inline(always)]
            pub const fn tx_dir(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "This determines the direction of the DMA Transfer. 1=Data Packet Read from Memory Start Address followed by Data Packet Write to Device Address 0=Data Packet Read from Device Address followed by Data Packet Write to Memory Start Address"]
            #[inline(always)]
            pub fn set_tx_dir(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "This is the device that is connected to this channel as its Hardware Flow Control master. The Flow Control Interface is a bus with each master concatenated onto it. This selects which bus index of the concatenated Flow Control Interface bus is targeted towards this channel. The Flow Control Interface Port list is dma_req, dma_term, and dma_done."]
            #[inline(always)]
            pub const fn hw_flow_ctrl_dev(&self) -> u8 {
                let val = (self.0 >> 9usize) & 0x7f;
                val as u8
            }
            #[doc = "This is the device that is connected to this channel as its Hardware Flow Control master. The Flow Control Interface is a bus with each master concatenated onto it. This selects which bus index of the concatenated Flow Control Interface bus is targeted towards this channel. The Flow Control Interface Port list is dma_req, dma_term, and dma_done."]
            #[inline(always)]
            pub fn set_hw_flow_ctrl_dev(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
            }
            #[doc = "This will enable an auto-increment to the DMA Channel Memory Address. 1=Increment the DMA Channel Memory Address by DMA Channel Control:Transfer Size after every Data Packet transfer 0=Do nothing"]
            #[inline(always)]
            pub const fn inc_mem_addr(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "This will enable an auto-increment to the DMA Channel Memory Address. 1=Increment the DMA Channel Memory Address by DMA Channel Control:Transfer Size after every Data Packet transfer 0=Do nothing"]
            #[inline(always)]
            pub fn set_inc_mem_addr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "This will enable an auto-increment to the DMA Channel Device Address. 1: Increment the DMA Channel Device Address by DMA Channel Control:Transfer Size after every Data Packet transfer 0: Do nothing"]
            #[inline(always)]
            pub const fn inc_dev_addr(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "This will enable an auto-increment to the DMA Channel Device Address. 1: Increment the DMA Channel Device Address by DMA Channel Control:Transfer Size after every Data Packet transfer 0: Do nothing"]
            #[inline(always)]
            pub fn set_inc_dev_addr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "This is used to lock the arbitration of the Channel Arbiter on this channel once this channel is granted. Once this is locked, it will remain on the arbiter until it has completed it transfer (either the Transfer Aborted, Transfer Done or Transfer Terminated conditions)."]
            #[inline(always)]
            pub const fn lock(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "This is used to lock the arbitration of the Channel Arbiter on this channel once this channel is granted. Once this is locked, it will remain on the arbiter until it has completed it transfer (either the Transfer Aborted, Transfer Done or Transfer Terminated conditions)."]
            #[inline(always)]
            pub fn set_lock(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "This will Disable the Hardware Flow Control. When disabled, any DMA Master device attempting to communicate to the DMA over the DMA Flow Control Interface (Ports: dma_req, dma_term, and dma_done) will be ignored. This should be set before using the DMA channel in Firmware Flow Control mode."]
            #[inline(always)]
            pub const fn dis_hw_flow_ctrl(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "This will Disable the Hardware Flow Control. When disabled, any DMA Master device attempting to communicate to the DMA over the DMA Flow Control Interface (Ports: dma_req, dma_term, and dma_done) will be ignored. This should be set before using the DMA channel in Firmware Flow Control mode."]
            #[inline(always)]
            pub fn set_dis_hw_flow_ctrl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "This is the transfer size in Bytes of each Data Packet transfer. Note: The transfer size must be a legal AMBA transfer size. Valid sizes are 1, 2 and 4 Bytes."]
            #[inline(always)]
            pub const fn trans_size(&self) -> u8 {
                let val = (self.0 >> 20usize) & 0x07;
                val as u8
            }
            #[doc = "This is the transfer size in Bytes of each Data Packet transfer. Note: The transfer size must be a legal AMBA transfer size. Valid sizes are 1, 2 and 4 Bytes."]
            #[inline(always)]
            pub fn set_trans_size(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
            }
            #[doc = "This is used for the Firmware Flow Control DMA transfer."]
            #[inline(always)]
            pub const fn trans_go(&self) -> bool {
                let val = (self.0 >> 24usize) & 0x01;
                val != 0
            }
            #[doc = "This is used for the Firmware Flow Control DMA transfer."]
            #[inline(always)]
            pub fn set_trans_go(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
            }
            #[doc = "This is used to abort the current transfer on this DMA Channel. The aborted transfer will be forced to terminate immediately."]
            #[inline(always)]
            pub const fn trans_abort(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[doc = "This is used to abort the current transfer on this DMA Channel. The aborted transfer will be forced to terminate immediately."]
            #[inline(always)]
            pub fn set_trans_abort(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
            }
        }
        impl Default for Ctrl {
            #[inline(always)]
            fn default() -> Ctrl {
                Ctrl(0)
            }
        }
        impl core::fmt::Debug for Ctrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ctrl")
                    .field("run", &self.run())
                    .field("req", &self.req())
                    .field("done", &self.done())
                    .field("sts", &self.sts())
                    .field("busy", &self.busy())
                    .field("tx_dir", &self.tx_dir())
                    .field("hw_flow_ctrl_dev", &self.hw_flow_ctrl_dev())
                    .field("inc_mem_addr", &self.inc_mem_addr())
                    .field("inc_dev_addr", &self.inc_dev_addr())
                    .field("lock", &self.lock())
                    .field("dis_hw_flow_ctrl", &self.dis_hw_flow_ctrl())
                    .field("trans_size", &self.trans_size())
                    .field("trans_go", &self.trans_go())
                    .field("trans_abort", &self.trans_abort())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ctrl {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Ctrl {{ run: {=bool:?}, req: {=bool:?}, done: {=bool:?}, sts: {:?}, busy: {=bool:?}, tx_dir: {=bool:?}, hw_flow_ctrl_dev: {=u8:?}, inc_mem_addr: {=bool:?}, inc_dev_addr: {=bool:?}, lock: {=bool:?}, dis_hw_flow_ctrl: {=bool:?}, trans_size: {=u8:?}, trans_go: {=bool:?}, trans_abort: {=bool:?} }}" , self . run () , self . req () , self . done () , self . sts () , self . busy () , self . tx_dir () , self . hw_flow_ctrl_dev () , self . inc_mem_addr () , self . inc_dev_addr () , self . lock () , self . dis_hw_flow_ctrl () , self . trans_size () , self . trans_go () , self . trans_abort ())
            }
        }
        #[doc = "DMA CHANNEL N INTERRUPT ENABLE"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ien(pub u8);
        impl Ien {
            #[doc = "This is an interrupt enable for DMA Channel Interrupt:Status Bus Error. 1=Enable Interrupt 0=Disable Interrupt"]
            #[inline(always)]
            pub const fn sts_en_bus_err(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This is an interrupt enable for DMA Channel Interrupt:Status Bus Error. 1=Enable Interrupt 0=Disable Interrupt"]
            #[inline(always)]
            pub fn set_sts_en_bus_err(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
            #[doc = "This is an interrupt enable for DMA Channel Interrupt:Status Flow Control Error. 1=Enable Interrupt 0=Disable Interrupt"]
            #[inline(always)]
            pub const fn sts_en_flow_ctrl(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "This is an interrupt enable for DMA Channel Interrupt:Status Flow Control Error. 1=Enable Interrupt 0=Disable Interrupt"]
            #[inline(always)]
            pub fn set_sts_en_flow_ctrl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
            }
            #[doc = "This is an interrupt enable for DMA Channel Interrupt:Status Done. 1=Enable Interrupt 0=Disable Interrupt"]
            #[inline(always)]
            pub const fn sts_en_done(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "This is an interrupt enable for DMA Channel Interrupt:Status Done. 1=Enable Interrupt 0=Disable Interrupt"]
            #[inline(always)]
            pub fn set_sts_en_done(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
            }
        }
        impl Default for Ien {
            #[inline(always)]
            fn default() -> Ien {
                Ien(0)
            }
        }
        impl core::fmt::Debug for Ien {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ien")
                    .field("sts_en_bus_err", &self.sts_en_bus_err())
                    .field("sts_en_flow_ctrl", &self.sts_en_flow_ctrl())
                    .field("sts_en_done", &self.sts_en_done())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ien {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Ien {{ sts_en_bus_err: {=bool:?}, sts_en_flow_ctrl: {=bool:?}, sts_en_done: {=bool:?} }}",
                    self.sts_en_bus_err(),
                    self.sts_en_flow_ctrl(),
                    self.sts_en_done()
                )
            }
        }
        #[doc = "DMA Channel N Interrupt Status"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ists(pub u8);
        impl Ists {
            #[doc = "This is an interrupt source register. This flags when there is an Error detected over the internal 32-bit Bus. 1: Error detected. (R/WC)"]
            #[inline(always)]
            pub const fn bus_err(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This is an interrupt source register. This flags when there is an Error detected over the internal 32-bit Bus. 1: Error detected. (R/WC)"]
            #[inline(always)]
            pub fn set_bus_err(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
            #[doc = "This is an interrupt source register. This flags when the DMA Channel has encountered a Hardware Flow Control Request after the DMA Channel has completed the transfer. This means the Master Device is attempting to overflow the DMA. 1=Hardware Flow Control is requesting after the transfer has completed 0=No Hardware Flow Control event"]
            #[inline(always)]
            pub const fn flow_ctrl(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "This is an interrupt source register. This flags when the DMA Channel has encountered a Hardware Flow Control Request after the DMA Channel has completed the transfer. This means the Master Device is attempting to overflow the DMA. 1=Hardware Flow Control is requesting after the transfer has completed 0=No Hardware Flow Control event"]
            #[inline(always)]
            pub fn set_flow_ctrl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
            }
            #[doc = "This is an interrupt source register. This flags when the DMA Channel has completed a transfer successfully on its side. A completed transfer is defined as when the DMA Channel reaches its limit; Memory Start Address equals Memory End Address. A completion due to a Hardware Flow Control Terminate will not flag this interrupt. 1=Memory Start Address equals Memory End Address 0=Memory Start Address does not equal Memory End Address"]
            #[inline(always)]
            pub const fn done(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "This is an interrupt source register. This flags when the DMA Channel has completed a transfer successfully on its side. A completed transfer is defined as when the DMA Channel reaches its limit; Memory Start Address equals Memory End Address. A completion due to a Hardware Flow Control Terminate will not flag this interrupt. 1=Memory Start Address equals Memory End Address 0=Memory Start Address does not equal Memory End Address"]
            #[inline(always)]
            pub fn set_done(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
            }
        }
        impl Default for Ists {
            #[inline(always)]
            fn default() -> Ists {
                Ists(0)
            }
        }
        impl core::fmt::Debug for Ists {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ists")
                    .field("bus_err", &self.bus_err())
                    .field("flow_ctrl", &self.flow_ctrl())
                    .field("done", &self.done())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ists {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Ists {{ bus_err: {=bool:?}, flow_ctrl: {=bool:?}, done: {=bool:?} }}",
                    self.bus_err(),
                    self.flow_ctrl(),
                    self.done()
                )
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Sts {
            #[doc = "0: DMA Channel Control:Run is Disabled (0x0)"]
            DIS = 0x0,
            #[doc = "1: The DMA Channel is locally done, in that Memory Start Address equals Memory End Address"]
            LOC_DONE = 0x01,
            #[doc = "2: The DMA Channel is externally done, in that the Device has terminated the transfer over the Hardware Flow Control through the Port dma_term"]
            EXT_DONE = 0x02,
            #[doc = "3: Error detected by the DMA"]
            ERROR = 0x03,
        }
        impl Sts {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Sts {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Sts {
            #[inline(always)]
            fn from(val: u8) -> Sts {
                Sts::from_bits(val)
            }
        }
        impl From<Sts> for u8 {
            #[inline(always)]
            fn from(val: Sts) -> u8 {
                Sts::to_bits(val)
            }
        }
    }
}
pub mod dma_main {
    #[doc = "DMA Main Registers"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaMain {
        ptr: *mut u8,
    }
    unsafe impl Send for DmaMain {}
    unsafe impl Sync for DmaMain {}
    impl DmaMain {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Soft reset the entire module. Enable the blocks operation."]
        #[inline(always)]
        pub const fn actrst(self) -> crate::common::Reg<regs::Actrst, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Debug register that has the data that is stored in the Data Packet. This data is read data from the currently active transfer source."]
        #[inline(always)]
        pub const fn data_pkt(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Soft reset the entire module. Enable the blocks operation."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Actrst(pub u8);
        impl Actrst {
            #[doc = "Enable the blocks operation. (R/WS) 1=Enable block. Each individual channel must be enabled separately. 0=Disable all channels."]
            #[inline(always)]
            pub const fn act(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable the blocks operation. (R/WS) 1=Enable block. Each individual channel must be enabled separately. 0=Disable all channels."]
            #[inline(always)]
            pub fn set_act(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
            #[doc = "Soft reset the entire module. This bit is self-clearing."]
            #[inline(always)]
            pub const fn soft_rst(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Soft reset the entire module. This bit is self-clearing."]
            #[inline(always)]
            pub fn set_soft_rst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
            }
        }
        impl Default for Actrst {
            #[inline(always)]
            fn default() -> Actrst {
                Actrst(0)
            }
        }
        impl core::fmt::Debug for Actrst {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Actrst")
                    .field("act", &self.act())
                    .field("soft_rst", &self.soft_rst())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Actrst {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Actrst {{ act: {=bool:?}, soft_rst: {=bool:?} }}",
                    self.act(),
                    self.soft_rst()
                )
            }
        }
    }
}
pub mod ec_reg_bank {
    #[doc = "This block is designed to be accessed internally by the EC via the register interface."]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EcRegBank {
        ptr: *mut u8,
    }
    unsafe impl Send for EcRegBank {}
    unsafe impl Sync for EcRegBank {}
    impl EcRegBank {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "AHB Error Address \\[0:0\\] AHB_ERR_ADDR, In priority order: 1. AHB address is registered when an AHB error occurs on the processor's AHB master port and the register value was already 0. This way only the first address to generate an exception is captured. 2. The processor can clear this register by writing any 32-bit value to this register."]
        #[inline(always)]
        pub const fn ahb_err_addr(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "AHB Error Control \\[0:0\\] AHB_ERROR_DISABLE, 0: EC memory exceptions are enabled. 1: EC memory exceptions are disabled."]
        #[inline(always)]
        pub const fn ahb_err_ctrl(self) -> crate::common::Reg<u8, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "Interrupt Control \\[0:0\\] NVIC_EN (NVIC_EN) This bit enables Alternate NVIC IRQ's Vectors. The Alternate NVIC Vectors provides each interrupt event with a dedicated (direct) NVIC vector. 0 = Alternate NVIC vectors disabled, 1= Alternate NVIC vectors enabled"]
        #[inline(always)]
        pub const fn intr_ctrl(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
        }
        #[doc = "ETM TRACE Enable \\[0:0\\] TRACE_EN (TRACE_EN) This bit enables the ARM TRACE debug port (ETM/ITM). The Trace Debug Interface pins are forced to the TRACE functions. 0 = ARM TRACE port disabled, 1= ARM TRACE port enabled"]
        #[inline(always)]
        pub const fn etm_ctrl(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
        #[doc = "Debug Enable Register"]
        #[inline(always)]
        pub const fn debug_ctrl(self) -> crate::common::Reg<regs::DebugCtrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }
        #[doc = "Lock Register"]
        #[inline(always)]
        pub const fn otp_lock(self) -> crate::common::Reg<regs::OtpLock, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
        }
        #[doc = "WDT Event Count \\[3:0\\] WDT_COUNT (WDT_COUNT) These EC R/W bits are cleared to 0 on VCC1 POR, but not on a WDT. Note: This field is written by Boot ROM firmware to indicate the number of times a WDT fired before loading a good EC code image."]
        #[inline(always)]
        pub const fn wdt_cnt(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
        }
        #[doc = "AES HASH Byte Swap Control Register."]
        #[inline(always)]
        pub const fn aesh_bswap_ctrl(self) -> crate::common::Reg<regs::AeshBswapCtrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
        }
        #[doc = "PECI Disable"]
        #[inline(always)]
        pub const fn peci_dis(self) -> crate::common::Reg<regs::PeciDis, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
        }
        #[doc = "This register is a mirror of the Boot Control Register."]
        #[inline(always)]
        pub const fn stap_tmir(self) -> crate::common::Reg<regs::StapTmir, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
        }
        #[doc = "This register contains the VTR Reset Status for BOOT ROM."]
        #[inline(always)]
        pub const fn brom_sts(self) -> crate::common::Reg<regs::BromSts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
        }
        #[doc = "System Shutdown Reset"]
        #[inline(always)]
        pub const fn crypto_srst(self) -> crate::common::Reg<regs::CryptoSrst, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
        }
        #[doc = "GPIO Bank Power Register"]
        #[inline(always)]
        pub const fn gpio_bank_pwr(self) -> crate::common::Reg<regs::GpioBankPwr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
        }
        #[doc = "JTAG Master Configuration Register"]
        #[inline(always)]
        pub const fn jtag_mcfg(self) -> crate::common::Reg<regs::JtagMcfg, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
        }
        #[doc = "JTAG Master Status Register"]
        #[inline(always)]
        pub const fn jtag_msts(self) -> crate::common::Reg<regs::JtagMsts, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
        }
        #[doc = "JTAG Master TDO Register"]
        #[inline(always)]
        pub const fn jtag_mtdo(self) -> crate::common::Reg<regs::JtagMtdo, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
        }
        #[doc = "JTAG Master TDI Register"]
        #[inline(always)]
        pub const fn jtag_mtdi(self) -> crate::common::Reg<regs::JtagMtdi, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
        }
        #[doc = "JTAG Master TMS Register"]
        #[inline(always)]
        pub const fn jtag_mtms(self) -> crate::common::Reg<regs::JtagMtms, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
        }
        #[doc = "JTAG Master Command Register"]
        #[inline(always)]
        pub const fn jtag_mcmd(self) -> crate::common::Reg<regs::JtagMcmd, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
        }
        #[doc = "BOOT ROM Scratch 0 Register"]
        #[inline(always)]
        pub const fn fw_scr0(self) -> crate::common::Reg<regs::FwScr0, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
        }
        #[doc = "BOOT ROM Scratch 1 Register"]
        #[inline(always)]
        pub const fn fw_scr1(self) -> crate::common::Reg<regs::FwScr1, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
        }
        #[doc = "BOOT ROM Scratch 2 Register"]
        #[inline(always)]
        pub const fn fw_scr2(self) -> crate::common::Reg<regs::FwScr2, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
        }
        #[doc = "BOOT ROM Scratch 3 Register"]
        #[inline(always)]
        pub const fn fw_scr3(self) -> crate::common::Reg<regs::FwScr3, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x018cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "AES HASH Byte Swap Control Register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct AeshBswapCtrl(pub u32);
        impl AeshBswapCtrl {
            #[doc = "Used to enable byte swap on a DWORD during AHB read from AES / HASH block: 1=Enable; 0=Disable."]
            #[inline(always)]
            pub const fn ip_byte_swap_en(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Used to enable byte swap on a DWORD during AHB read from AES / HASH block: 1=Enable; 0=Disable."]
            #[inline(always)]
            pub fn set_ip_byte_swap_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Used to enable byte swap on a DWORD during AHB write from AES / HASH block: 1=Enable; 0=Disable."]
            #[inline(always)]
            pub const fn op_byte_swap_en(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Used to enable byte swap on a DWORD during AHB write from AES / HASH block: 1=Enable; 0=Disable."]
            #[inline(always)]
            pub fn set_op_byte_swap_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Used to enable word swap on a DWORD during AHB read from AES / HASH block 4=Swap 32-bit doublewords in 128-byte blocks 3=Swap doublewords in 64-byte blocks. Useful for SHA-256. Bus references issued in the order 0x3C, 0x38, 0x34, 0x30, 0x2C, 0x28, 0x24, 0x20, 0x1C, 0x18, 0x14, 0x10, 0xC, 0x8, 0x4, 0x0,... 2=Swap doublewords in 16-byte blocks. Useful for AES. Bus references issued in the order 0xC, 0x8, 0x4, 0x0, 0x1C, 0x18,... 1=Swap doublewords in 8-byte blocks. Useful for SHA-512, which works on 64-bit words. Bus references issued in the order 0x4, 0x0, 0xC, 0x8, ... 0=Disable."]
            #[inline(always)]
            pub const fn ip_blk_swap_en(&self) -> super::vals::IpBlkSwapEn {
                let val = (self.0 >> 2usize) & 0x07;
                super::vals::IpBlkSwapEn::from_bits(val as u8)
            }
            #[doc = "Used to enable word swap on a DWORD during AHB read from AES / HASH block 4=Swap 32-bit doublewords in 128-byte blocks 3=Swap doublewords in 64-byte blocks. Useful for SHA-256. Bus references issued in the order 0x3C, 0x38, 0x34, 0x30, 0x2C, 0x28, 0x24, 0x20, 0x1C, 0x18, 0x14, 0x10, 0xC, 0x8, 0x4, 0x0,... 2=Swap doublewords in 16-byte blocks. Useful for AES. Bus references issued in the order 0xC, 0x8, 0x4, 0x0, 0x1C, 0x18,... 1=Swap doublewords in 8-byte blocks. Useful for SHA-512, which works on 64-bit words. Bus references issued in the order 0x4, 0x0, 0xC, 0x8, ... 0=Disable."]
            #[inline(always)]
            pub fn set_ip_blk_swap_en(&mut self, val: super::vals::IpBlkSwapEn) {
                self.0 = (self.0 & !(0x07 << 2usize)) | (((val.to_bits() as u32) & 0x07) << 2usize);
            }
            #[doc = "Used to enable word swap on a DWORD during AHB write from AES / HASH block 4=Swap 32-bit doublewords in 128-byte blocks 3=Swap doublewords in 64-byte blocks. Useful for SHA-256. Bus references issued in the order 0x3C, 0x38, 0x34, 0x30, 0x2C, 0x28, 0x24, 0x20, 0x1C, 0x18, 0x14, 0x10, 0xC, 0x8, 0x4, 0x0,... 2=Swap doublewords in 16-byte blocks. Useful for AES. Bus references issued in the order 0xC, 0x8, 0x4, 0x0, 0x1C, 0x18,... 1=Swap doublewords in 8-byte blocks. Useful for SHA-512, which works on 64-bit words. Bus references issued in the order 0x4, 0x0, 0xC, 0x8, ... 0=Disable."]
            #[inline(always)]
            pub const fn op_blk_swap_en(&self) -> super::vals::OpBlkSwapEn {
                let val = (self.0 >> 5usize) & 0x07;
                super::vals::OpBlkSwapEn::from_bits(val as u8)
            }
            #[doc = "Used to enable word swap on a DWORD during AHB write from AES / HASH block 4=Swap 32-bit doublewords in 128-byte blocks 3=Swap doublewords in 64-byte blocks. Useful for SHA-256. Bus references issued in the order 0x3C, 0x38, 0x34, 0x30, 0x2C, 0x28, 0x24, 0x20, 0x1C, 0x18, 0x14, 0x10, 0xC, 0x8, 0x4, 0x0,... 2=Swap doublewords in 16-byte blocks. Useful for AES. Bus references issued in the order 0xC, 0x8, 0x4, 0x0, 0x1C, 0x18,... 1=Swap doublewords in 8-byte blocks. Useful for SHA-512, which works on 64-bit words. Bus references issued in the order 0x4, 0x0, 0xC, 0x8, ... 0=Disable."]
            #[inline(always)]
            pub fn set_op_blk_swap_en(&mut self, val: super::vals::OpBlkSwapEn) {
                self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
            }
        }
        impl Default for AeshBswapCtrl {
            #[inline(always)]
            fn default() -> AeshBswapCtrl {
                AeshBswapCtrl(0)
            }
        }
        impl core::fmt::Debug for AeshBswapCtrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("AeshBswapCtrl")
                    .field("ip_byte_swap_en", &self.ip_byte_swap_en())
                    .field("op_byte_swap_en", &self.op_byte_swap_en())
                    .field("ip_blk_swap_en", &self.ip_blk_swap_en())
                    .field("op_blk_swap_en", &self.op_blk_swap_en())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for AeshBswapCtrl {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "AeshBswapCtrl {{ ip_byte_swap_en: {=bool:?}, op_byte_swap_en: {=bool:?}, ip_blk_swap_en: {:?}, op_blk_swap_en: {:?} }}" , self . ip_byte_swap_en () , self . op_byte_swap_en () , self . ip_blk_swap_en () , self . op_blk_swap_en ())
            }
        }
        #[doc = "This register contains the VTR Reset Status for BOOT ROM."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct BromSts(pub u8);
        impl BromSts {
            #[doc = "This bit contains the RESET_SYS Status for BOOT ROM usage. 1= RESET_SYS event occured. 0= RESET_SYS event did not occured. This registe is R/W1C (read / write 1 to clear)."]
            #[inline(always)]
            pub const fn vtr_rst_sts(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This bit contains the RESET_SYS Status for BOOT ROM usage. 1= RESET_SYS event occured. 0= RESET_SYS event did not occured. This registe is R/W1C (read / write 1 to clear)."]
            #[inline(always)]
            pub fn set_vtr_rst_sts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
            #[doc = "This bit contains the WDT Event Status for BOOT ROM usage. 1= WDT event occured. 0= WDT event did not occured. This registe is R/W1C (read / write 1 to clear)."]
            #[inline(always)]
            pub const fn wdt_evt(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "This bit contains the WDT Event Status for BOOT ROM usage. 1= WDT event occured. 0= WDT event did not occured. This registe is R/W1C (read / write 1 to clear)."]
            #[inline(always)]
            pub fn set_wdt_evt(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
            }
        }
        impl Default for BromSts {
            #[inline(always)]
            fn default() -> BromSts {
                BromSts(0)
            }
        }
        impl core::fmt::Debug for BromSts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("BromSts")
                    .field("vtr_rst_sts", &self.vtr_rst_sts())
                    .field("wdt_evt", &self.wdt_evt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for BromSts {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "BromSts {{ vtr_rst_sts: {=bool:?}, wdt_evt: {=bool:?} }}",
                    self.vtr_rst_sts(),
                    self.wdt_evt()
                )
            }
        }
        #[doc = "System Shutdown Reset"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct CryptoSrst(pub u32);
        impl CryptoSrst {
            #[doc = "When this bit is asserted ('1'), the Random Number Generator block is reset."]
            #[inline(always)]
            pub const fn rng(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "When this bit is asserted ('1'), the Random Number Generator block is reset."]
            #[inline(always)]
            pub fn set_rng(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "When this bit is asserted ('1'), the Public Key block is reset."]
            #[inline(always)]
            pub const fn pub_key(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "When this bit is asserted ('1'), the Public Key block is reset."]
            #[inline(always)]
            pub fn set_pub_key(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "When this bit is asserted ('1'), the AES and Hash blocks are reset."]
            #[inline(always)]
            pub const fn aes_hash(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "When this bit is asserted ('1'), the AES and Hash blocks are reset."]
            #[inline(always)]
            pub fn set_aes_hash(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
        }
        impl Default for CryptoSrst {
            #[inline(always)]
            fn default() -> CryptoSrst {
                CryptoSrst(0)
            }
        }
        impl core::fmt::Debug for CryptoSrst {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("CryptoSrst")
                    .field("rng", &self.rng())
                    .field("pub_key", &self.pub_key())
                    .field("aes_hash", &self.aes_hash())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for CryptoSrst {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "CryptoSrst {{ rng: {=bool:?}, pub_key: {=bool:?}, aes_hash: {=bool:?} }}",
                    self.rng(),
                    self.pub_key(),
                    self.aes_hash()
                )
            }
        }
        #[doc = "Debug Enable Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DebugCtrl(pub u32);
        impl DebugCtrl {
            #[doc = "DEBUG_EN (JTAG_EN) This bit enables the JTAG/SWD debug port. 0= JTAG/SWD port disabled. JTAG/SWD cannot be enabled (i.e., the TRST# pin is ignored and the JTAG signals remain in their non-JTAG state) 1= JTAG/SWD port enabled. A high on TRST# enables JTAG or SWD, as determined by SWD_EN."]
            #[inline(always)]
            pub const fn en(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "DEBUG_EN (JTAG_EN) This bit enables the JTAG/SWD debug port. 0= JTAG/SWD port disabled. JTAG/SWD cannot be enabled (i.e., the TRST# pin is ignored and the JTAG signals remain in their non-JTAG state) 1= JTAG/SWD port enabled. A high on TRST# enables JTAG or SWD, as determined by SWD_EN."]
            #[inline(always)]
            pub fn set_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "This field determines which pins are affected by the TRST# debug enable pin. 3=Reserved 2=The pins associated with the JTAG TCK and TMS switch to the debug interface when TRST# is de-asserted high. The pins associated with TDI and TDO remain controlled by the associated GPIO. This setting should be used when the ARM Serial Wire Debug (SWD) is required for debugging and the Serial Wire Viewer is not required 1=The pins associated with the JTAG TCK, TMS and TDO switch to the debug interface when TRST# is de-asserted high. The pin associated with TDI remains controlled by the associated GPIO. This setting should be used when the ARM Serial Wire Debug (SWD) and Serial Wire Viewer (SWV) are both required for debugging 0=All four pins associated with JTAG (TCK, TMS, TDI and TDO) switch to the debug interface when TRST# is de-asserted high. This setting should be used when the JTAG TAP controller is required for debugging."]
            #[inline(always)]
            pub const fn pin_cfg(&self) -> super::vals::PinCfg {
                let val = (self.0 >> 1usize) & 0x03;
                super::vals::PinCfg::from_bits(val as u8)
            }
            #[doc = "This field determines which pins are affected by the TRST# debug enable pin. 3=Reserved 2=The pins associated with the JTAG TCK and TMS switch to the debug interface when TRST# is de-asserted high. The pins associated with TDI and TDO remain controlled by the associated GPIO. This setting should be used when the ARM Serial Wire Debug (SWD) is required for debugging and the Serial Wire Viewer is not required 1=The pins associated with the JTAG TCK, TMS and TDO switch to the debug interface when TRST# is de-asserted high. The pin associated with TDI remains controlled by the associated GPIO. This setting should be used when the ARM Serial Wire Debug (SWD) and Serial Wire Viewer (SWV) are both required for debugging 0=All four pins associated with JTAG (TCK, TMS, TDI and TDO) switch to the debug interface when TRST# is de-asserted high. This setting should be used when the JTAG TAP controller is required for debugging."]
            #[inline(always)]
            pub fn set_pin_cfg(&mut self, val: super::vals::PinCfg) {
                self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
            }
            #[doc = "If this bit is set to '1b' internal pull-up resistors are automatically enabled on the appropriate debugging port wires whenever the debug port is enabled (the DEBUG_EN bit in this register is '1b' and the JTAG_RST# pin is high). The setting of DEBUG_PIN_CFG determines which pins have pull-ups enabled when the debug port is enabled."]
            #[inline(always)]
            pub const fn pu_en(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "If this bit is set to '1b' internal pull-up resistors are automatically enabled on the appropriate debugging port wires whenever the debug port is enabled (the DEBUG_EN bit in this register is '1b' and the JTAG_RST# pin is high). The setting of DEBUG_PIN_CFG determines which pins have pull-ups enabled when the debug port is enabled."]
            #[inline(always)]
            pub fn set_pu_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "This bit sets the boundary scan tap controller accessibility from JTAG port. 1= Boundary scan tap controller accessibile through JTAG Port. 0= Boundary scan tap controller not accessibile through JTAG Port."]
            #[inline(always)]
            pub const fn bsp_en(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "This bit sets the boundary scan tap controller accessibility from JTAG port. 1= Boundary scan tap controller accessibile through JTAG Port. 0= Boundary scan tap controller not accessibile through JTAG Port."]
            #[inline(always)]
            pub fn set_bsp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
        }
        impl Default for DebugCtrl {
            #[inline(always)]
            fn default() -> DebugCtrl {
                DebugCtrl(0)
            }
        }
        impl core::fmt::Debug for DebugCtrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DebugCtrl")
                    .field("en", &self.en())
                    .field("pin_cfg", &self.pin_cfg())
                    .field("pu_en", &self.pu_en())
                    .field("bsp_en", &self.bsp_en())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DebugCtrl {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "DebugCtrl {{ en: {=bool:?}, pin_cfg: {:?}, pu_en: {=bool:?}, bsp_en: {=bool:?} }}",
                    self.en(),
                    self.pin_cfg(),
                    self.pu_en(),
                    self.bsp_en()
                )
            }
        }
        #[doc = "BOOT ROM Scratch 0 Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct FwScr0(pub u32);
        impl FwScr0 {
            #[doc = "This field has no functionality other than storage. This register is aliased to ESPI Config Scratch Register."]
            #[inline(always)]
            pub const fn scr0(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "This field has no functionality other than storage. This register is aliased to ESPI Config Scratch Register."]
            #[inline(always)]
            pub fn set_scr0(&mut self, val: u32) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for FwScr0 {
            #[inline(always)]
            fn default() -> FwScr0 {
                FwScr0(0)
            }
        }
        impl core::fmt::Debug for FwScr0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("FwScr0").field("scr0", &self.scr0()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for FwScr0 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "FwScr0 {{ scr0: {=u32:?} }}", self.scr0())
            }
        }
        #[doc = "BOOT ROM Scratch 1 Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct FwScr1(pub u32);
        impl FwScr1 {
            #[doc = "This field has no functionality other than storage. This register is aliased to ESPI Config Scratch Register."]
            #[inline(always)]
            pub const fn scr1(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "This field has no functionality other than storage. This register is aliased to ESPI Config Scratch Register."]
            #[inline(always)]
            pub fn set_scr1(&mut self, val: u32) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for FwScr1 {
            #[inline(always)]
            fn default() -> FwScr1 {
                FwScr1(0)
            }
        }
        impl core::fmt::Debug for FwScr1 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("FwScr1").field("scr1", &self.scr1()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for FwScr1 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "FwScr1 {{ scr1: {=u32:?} }}", self.scr1())
            }
        }
        #[doc = "BOOT ROM Scratch 2 Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct FwScr2(pub u32);
        impl FwScr2 {
            #[doc = "This field has no functionality other than storage. This register is aliased to ESPI Config Scratch Register."]
            #[inline(always)]
            pub const fn scr2(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "This field has no functionality other than storage. This register is aliased to ESPI Config Scratch Register."]
            #[inline(always)]
            pub fn set_scr2(&mut self, val: u32) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for FwScr2 {
            #[inline(always)]
            fn default() -> FwScr2 {
                FwScr2(0)
            }
        }
        impl core::fmt::Debug for FwScr2 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("FwScr2").field("scr2", &self.scr2()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for FwScr2 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "FwScr2 {{ scr2: {=u32:?} }}", self.scr2())
            }
        }
        #[doc = "BOOT ROM Scratch 3 Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct FwScr3(pub u32);
        impl FwScr3 {
            #[doc = "This field has no functionality other than storage. This register is aliased to ESPI Config Scratch Register."]
            #[inline(always)]
            pub const fn scr3(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "This field has no functionality other than storage. This register is aliased to ESPI Config Scratch Register."]
            #[inline(always)]
            pub fn set_scr3(&mut self, val: u32) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for FwScr3 {
            #[inline(always)]
            fn default() -> FwScr3 {
                FwScr3(0)
            }
        }
        impl core::fmt::Debug for FwScr3 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("FwScr3").field("scr3", &self.scr3()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for FwScr3 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "FwScr3 {{ scr3: {=u32:?} }}", self.scr3())
            }
        }
        #[doc = "GPIO Bank Power Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct GpioBankPwr(pub u32);
        impl GpioBankPwr {
            #[doc = "This bit must be programmed to 0h for proper operation of the device."]
            #[inline(always)]
            pub const fn test(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This bit must be programmed to 0h for proper operation of the device."]
            #[inline(always)]
            pub fn set_test(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Voltage value on VTR2. This bit is set by Firmware after a VTR Power On Reset. It must be set by software if the VTR power rail is not active when RESET_SYS is de-asserted. Write access is determined by bit 7. 1=VTR2 is powered by 1.8V 0=VTR2 is powered by 3.3V."]
            #[inline(always)]
            pub const fn vtr_lvl2(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Voltage value on VTR2. This bit is set by Firmware after a VTR Power On Reset. It must be set by software if the VTR power rail is not active when RESET_SYS is de-asserted. Write access is determined by bit 7. 1=VTR2 is powered by 1.8V 0=VTR2 is powered by 3.3V."]
            #[inline(always)]
            pub fn set_vtr_lvl2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Voltage value on VTR3. This bit is set by hardware after a VTR Power On Reset, but may be overridden by software. It must be set by software if the VTR power rail is not active when RESET_SYS is de-asserted. Write access is determined by bit 7. 1=VTR3 is powered by 1.8V 0=VTR3 is powered by 3.3V."]
            #[inline(always)]
            pub const fn vtr_lvl3(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Voltage value on VTR3. This bit is set by hardware after a VTR Power On Reset, but may be overridden by software. It must be set by software if the VTR power rail is not active when RESET_SYS is de-asserted. Write access is determined by bit 7. 1=VTR3 is powered by 1.8V 0=VTR3 is powered by 3.3V."]
            #[inline(always)]
            pub fn set_vtr_lvl3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "GPIO Bank Power Lock. 0: VTR_LEVEL bits\\[2:0\\] and GPIO Bank Power Lock bit are R/W 1 = VTR_LEVEL bits\\[2:0\\] and GPIO Bank Power Lock bit are Read Only."]
            #[inline(always)]
            pub const fn gpio_bank_pwr_lock(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "GPIO Bank Power Lock. 0: VTR_LEVEL bits\\[2:0\\] and GPIO Bank Power Lock bit are R/W 1 = VTR_LEVEL bits\\[2:0\\] and GPIO Bank Power Lock bit are Read Only."]
            #[inline(always)]
            pub fn set_gpio_bank_pwr_lock(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
        }
        impl Default for GpioBankPwr {
            #[inline(always)]
            fn default() -> GpioBankPwr {
                GpioBankPwr(0)
            }
        }
        impl core::fmt::Debug for GpioBankPwr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("GpioBankPwr")
                    .field("test", &self.test())
                    .field("vtr_lvl2", &self.vtr_lvl2())
                    .field("vtr_lvl3", &self.vtr_lvl3())
                    .field("gpio_bank_pwr_lock", &self.gpio_bank_pwr_lock())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for GpioBankPwr {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "GpioBankPwr {{ test: {=bool:?}, vtr_lvl2: {=bool:?}, vtr_lvl3: {=bool:?}, gpio_bank_pwr_lock: {=bool:?} }}" , self . test () , self . vtr_lvl2 () , self . vtr_lvl3 () , self . gpio_bank_pwr_lock ())
            }
        }
        #[doc = "JTAG Master Configuration Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct JtagMcfg(pub u32);
        impl JtagMcfg {
            #[doc = "This field determines the JTAG Master clock rate, derived from the 48MHz master clock. 7=375KHz; 6=750KHz; 5=1.5Mhz; 4=3Mhz; 3=6Mhz; 2=12Mhz; 1=24MHz; 0=Reserved."]
            #[inline(always)]
            pub const fn jtm_clk(&self) -> super::vals::JtmClk {
                let val = (self.0 >> 0usize) & 0x07;
                super::vals::JtmClk::from_bits(val as u8)
            }
            #[doc = "This field determines the JTAG Master clock rate, derived from the 48MHz master clock. 7=375KHz; 6=750KHz; 5=1.5Mhz; 4=3Mhz; 3=6Mhz; 2=12Mhz; 1=24MHz; 0=Reserved."]
            #[inline(always)]
            pub fn set_jtm_clk(&mut self, val: super::vals::JtmClk) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
            }
            #[doc = "This bit controls the direction of the JTAG port. 1=The JTAG Port is configured as a Master 0=The JTAG Port is configures as a Slave."]
            #[inline(always)]
            pub const fn mas_slv(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "This bit controls the direction of the JTAG port. 1=The JTAG Port is configured as a Master 0=The JTAG Port is configures as a Slave."]
            #[inline(always)]
            pub fn set_mas_slv(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
        }
        impl Default for JtagMcfg {
            #[inline(always)]
            fn default() -> JtagMcfg {
                JtagMcfg(0)
            }
        }
        impl core::fmt::Debug for JtagMcfg {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("JtagMcfg")
                    .field("jtm_clk", &self.jtm_clk())
                    .field("mas_slv", &self.mas_slv())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for JtagMcfg {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "JtagMcfg {{ jtm_clk: {:?}, mas_slv: {=bool:?} }}",
                    self.jtm_clk(),
                    self.mas_slv()
                )
            }
        }
        #[doc = "JTAG Master Command Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct JtagMcmd(pub u32);
        impl JtagMcmd {
            #[doc = "If the JTAG Port is configured as a Master, writing this register starts clocking and shifting on the JTAG port. The JTAG Master port will shift JTM_COUNT+1 times, so writing a '0h' will shift 1 bit, and writing '31h' will shift 32 bits. The signal JTAG_CLK will cycle JTM_COUNT+1 times. The contents of the JTAG Master TMS Register and the JTAG Master TDI Register will be shifted out on the falling edge of JTAG_CLK and the.JTAG Master TDO Register will get shifted in on the rising edge of JTAG_CLK. If the JTAG Port is configured as a Slave, writing this register has no effect."]
            #[inline(always)]
            pub const fn jtm_count(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x1f;
                val as u8
            }
            #[doc = "If the JTAG Port is configured as a Master, writing this register starts clocking and shifting on the JTAG port. The JTAG Master port will shift JTM_COUNT+1 times, so writing a '0h' will shift 1 bit, and writing '31h' will shift 32 bits. The signal JTAG_CLK will cycle JTM_COUNT+1 times. The contents of the JTAG Master TMS Register and the JTAG Master TDI Register will be shifted out on the falling edge of JTAG_CLK and the.JTAG Master TDO Register will get shifted in on the rising edge of JTAG_CLK. If the JTAG Port is configured as a Slave, writing this register has no effect."]
            #[inline(always)]
            pub fn set_jtm_count(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
            }
        }
        impl Default for JtagMcmd {
            #[inline(always)]
            fn default() -> JtagMcmd {
                JtagMcmd(0)
            }
        }
        impl core::fmt::Debug for JtagMcmd {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("JtagMcmd")
                    .field("jtm_count", &self.jtm_count())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for JtagMcmd {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "JtagMcmd {{ jtm_count: {=u8:?} }}", self.jtm_count())
            }
        }
        #[doc = "JTAG Master Status Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct JtagMsts(pub u32);
        impl JtagMsts {
            #[doc = "This bit is set to '1b' when the JTAG Master Command Register is written. It becomes '0b' when shifting has completed. Software can poll this bit to determine when a command has completed and it is therefore safe to remove the data in the JTAG Master TDO Register and load new data into the JTAG Master TMS Register and the JTAG Master TDI Register."]
            #[inline(always)]
            pub const fn jtm_done(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This bit is set to '1b' when the JTAG Master Command Register is written. It becomes '0b' when shifting has completed. Software can poll this bit to determine when a command has completed and it is therefore safe to remove the data in the JTAG Master TDO Register and load new data into the JTAG Master TMS Register and the JTAG Master TDI Register."]
            #[inline(always)]
            pub fn set_jtm_done(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for JtagMsts {
            #[inline(always)]
            fn default() -> JtagMsts {
                JtagMsts(0)
            }
        }
        impl core::fmt::Debug for JtagMsts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("JtagMsts").field("jtm_done", &self.jtm_done()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for JtagMsts {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "JtagMsts {{ jtm_done: {=bool:?} }}", self.jtm_done())
            }
        }
        #[doc = "JTAG Master TDI Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct JtagMtdi(pub u32);
        impl JtagMtdi {
            #[doc = "When the JTAG Master Command Register is written, from 1 to 32 bits are shifted out of this register, starting with bit 0, onto the JTAG_TDI pin. Shifting is at the rate determined by the JTM_CLK field in the JTAG Master Configuration Register."]
            #[inline(always)]
            pub const fn jtm_tdi(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "When the JTAG Master Command Register is written, from 1 to 32 bits are shifted out of this register, starting with bit 0, onto the JTAG_TDI pin. Shifting is at the rate determined by the JTM_CLK field in the JTAG Master Configuration Register."]
            #[inline(always)]
            pub fn set_jtm_tdi(&mut self, val: u32) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for JtagMtdi {
            #[inline(always)]
            fn default() -> JtagMtdi {
                JtagMtdi(0)
            }
        }
        impl core::fmt::Debug for JtagMtdi {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("JtagMtdi").field("jtm_tdi", &self.jtm_tdi()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for JtagMtdi {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "JtagMtdi {{ jtm_tdi: {=u32:?} }}", self.jtm_tdi())
            }
        }
        #[doc = "JTAG Master TDO Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct JtagMtdo(pub u32);
        impl JtagMtdo {
            #[doc = "When the JTAG Master Command Register is written, from 1 to 32 bits are shifted into this register, starting with bit 0, from the JTAG_TDO pin. Shifting is at the rate determined by the JTM_CLK field in the JTAG Master Configuration Register."]
            #[inline(always)]
            pub const fn jtm_tdo(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "When the JTAG Master Command Register is written, from 1 to 32 bits are shifted into this register, starting with bit 0, from the JTAG_TDO pin. Shifting is at the rate determined by the JTM_CLK field in the JTAG Master Configuration Register."]
            #[inline(always)]
            pub fn set_jtm_tdo(&mut self, val: u32) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for JtagMtdo {
            #[inline(always)]
            fn default() -> JtagMtdo {
                JtagMtdo(0)
            }
        }
        impl core::fmt::Debug for JtagMtdo {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("JtagMtdo").field("jtm_tdo", &self.jtm_tdo()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for JtagMtdo {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "JtagMtdo {{ jtm_tdo: {=u32:?} }}", self.jtm_tdo())
            }
        }
        #[doc = "JTAG Master TMS Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct JtagMtms(pub u32);
        impl JtagMtms {
            #[doc = "When the JTAG Master Command Register is written, from 1 to 32 bits are shifted out of this register, starting with bit 0, onto the JTAG_TMS pin. Shifting is at the rate determined by the JTM_CLK field in the JTAG Master Configuration Register."]
            #[inline(always)]
            pub const fn jtm_tms(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "When the JTAG Master Command Register is written, from 1 to 32 bits are shifted out of this register, starting with bit 0, onto the JTAG_TMS pin. Shifting is at the rate determined by the JTM_CLK field in the JTAG Master Configuration Register."]
            #[inline(always)]
            pub fn set_jtm_tms(&mut self, val: u32) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for JtagMtms {
            #[inline(always)]
            fn default() -> JtagMtms {
                JtagMtms(0)
            }
        }
        impl core::fmt::Debug for JtagMtms {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("JtagMtms").field("jtm_tms", &self.jtm_tms()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for JtagMtms {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "JtagMtms {{ jtm_tms: {=u32:?} }}", self.jtm_tms())
            }
        }
        #[doc = "Lock Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct OtpLock(pub u32);
        impl OtpLock {
            #[doc = "Test"]
            #[inline(always)]
            pub const fn test(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Test"]
            #[inline(always)]
            pub fn set_test(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "VBAT RAM LOCK bit. 0 = Not Locked. 1 = Locked."]
            #[inline(always)]
            pub const fn vbat_ram_lock(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "VBAT RAM LOCK bit. 0 = Not Locked. 1 = Locked."]
            #[inline(always)]
            pub fn set_vbat_ram_lock(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "VBAT REG LOCK. 0 = Not Locked. 1 = Locked."]
            #[inline(always)]
            pub const fn vbat_reg_lock(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "VBAT REG LOCK. 0 = Not Locked. 1 = Locked."]
            #[inline(always)]
            pub fn set_vbat_reg_lock(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
        }
        impl Default for OtpLock {
            #[inline(always)]
            fn default() -> OtpLock {
                OtpLock(0)
            }
        }
        impl core::fmt::Debug for OtpLock {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("OtpLock")
                    .field("test", &self.test())
                    .field("vbat_ram_lock", &self.vbat_ram_lock())
                    .field("vbat_reg_lock", &self.vbat_reg_lock())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for OtpLock {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "OtpLock {{ test: {=bool:?}, vbat_ram_lock: {=bool:?}, vbat_reg_lock: {=bool:?} }}",
                    self.test(),
                    self.vbat_ram_lock(),
                    self.vbat_reg_lock()
                )
            }
        }
        #[doc = "PECI Disable"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct PeciDis(pub u32);
        impl PeciDis {
            #[doc = "When this bit is asserted ('1'), it disables the PECI pads to reduce leakage."]
            #[inline(always)]
            pub const fn p_dis(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "When this bit is asserted ('1'), it disables the PECI pads to reduce leakage."]
            #[inline(always)]
            pub fn set_p_dis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for PeciDis {
            #[inline(always)]
            fn default() -> PeciDis {
                PeciDis(0)
            }
        }
        impl core::fmt::Debug for PeciDis {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("PeciDis").field("p_dis", &self.p_dis()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for PeciDis {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "PeciDis {{ p_dis: {=bool:?} }}", self.p_dis())
            }
        }
        #[doc = "This register is a mirror of the Boot Control Register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct StapTmir(pub u32);
        impl StapTmir {
            #[doc = "This is the mirror of the QA_MODE (bit 0) of Boot Control Register. This register bit tells BOOT ROM to enter the QA mode."]
            #[inline(always)]
            pub const fn qa_mode(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This is the mirror of the QA_MODE (bit 0) of Boot Control Register. This register bit tells BOOT ROM to enter the QA mode."]
            #[inline(always)]
            pub fn set_qa_mode(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "This is the mirror of the Validation MODE (bit 1) of Boot Control Register. This register bit tells BOOT ROM to enter the Validation mode."]
            #[inline(always)]
            pub const fn vld_mode(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "This is the mirror of the Validation MODE (bit 1) of Boot Control Register. This register bit tells BOOT ROM to enter the Validation mode."]
            #[inline(always)]
            pub fn set_vld_mode(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "This register bit tells BOOT ROM about the Boundary Scan Status."]
            #[inline(always)]
            pub const fn bs_status(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "This register bit tells BOOT ROM about the Boundary Scan Status."]
            #[inline(always)]
            pub fn set_bs_status(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "This register bit tells BOOT ROM that SPI FLASH Recovery Mode is entered."]
            #[inline(always)]
            pub const fn int_spi_recov(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "This register bit tells BOOT ROM that SPI FLASH Recovery Mode is entered."]
            #[inline(always)]
            pub fn set_int_spi_recov(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
        }
        impl Default for StapTmir {
            #[inline(always)]
            fn default() -> StapTmir {
                StapTmir(0)
            }
        }
        impl core::fmt::Debug for StapTmir {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("StapTmir")
                    .field("qa_mode", &self.qa_mode())
                    .field("vld_mode", &self.vld_mode())
                    .field("bs_status", &self.bs_status())
                    .field("int_spi_recov", &self.int_spi_recov())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for StapTmir {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "StapTmir {{ qa_mode: {=bool:?}, vld_mode: {=bool:?}, bs_status: {=bool:?}, int_spi_recov: {=bool:?} }}" , self . qa_mode () , self . vld_mode () , self . bs_status () , self . int_spi_recov ())
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum IpBlkSwapEn {
            #[doc = "0=Disable."]
            DIS = 0x0,
            #[doc = "1=Swap doublewords in 8-byte blocks. Useful for SHA-512, which works on 64-bit words. Bus references issued in the order 0x4, 0x0, 0xC, 0x8, ..."]
            SWAP_8_BYTE_BLOCK = 0x01,
            #[doc = "2=Swap doublewords in 16-byte blocks. Useful for AES. Bus references issued in the order 0xC, 0x8, 0x4, 0x0, 0x1C, 0x18,..."]
            SWAP_16_BYTE_BLOCK = 0x02,
            #[doc = "3=Swap doublewords in 64-byte blocks. Useful for SHA-256. Bus references issued in the order 0x3C, 0x38, 0x34, 0x30, 0x2C, 0x28, 0x24, 0x20, 0x1C, 0x18, 0x14, 0x10, 0xC, 0x8, 0x4, 0x0,..."]
            SWAP_64_BYTE_BLOCK = 0x03,
            #[doc = "4=Swap 32-bit doublewords in 128-byte blocks"]
            SWAP_128_BYTE_BLOCK = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
        }
        impl IpBlkSwapEn {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> IpBlkSwapEn {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for IpBlkSwapEn {
            #[inline(always)]
            fn from(val: u8) -> IpBlkSwapEn {
                IpBlkSwapEn::from_bits(val)
            }
        }
        impl From<IpBlkSwapEn> for u8 {
            #[inline(always)]
            fn from(val: IpBlkSwapEn) -> u8 {
                IpBlkSwapEn::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum JtmClk {
            _RESERVED_0 = 0x0,
            #[doc = "1=24MHz"]
            JTM_CLK_24MHZ = 0x01,
            #[doc = "2=12Mhz"]
            JTM_CLK_12MHZ = 0x02,
            #[doc = "3=6Mhz"]
            JTM_CLK_6MHZ = 0x03,
            #[doc = "4=3Mhz"]
            JTM_CLK_3MHZ = 0x04,
            #[doc = "5=1.5Mhz"]
            JTM_CLK_1MHZ = 0x05,
            #[doc = "6=750KHz"]
            JTM_CLK_750KHZ = 0x06,
            #[doc = "7=375KHz"]
            JTM_CLK_375KHZ = 0x07,
        }
        impl JtmClk {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> JtmClk {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for JtmClk {
            #[inline(always)]
            fn from(val: u8) -> JtmClk {
                JtmClk::from_bits(val)
            }
        }
        impl From<JtmClk> for u8 {
            #[inline(always)]
            fn from(val: JtmClk) -> u8 {
                JtmClk::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum OpBlkSwapEn {
            #[doc = "0=Disable."]
            DIS = 0x0,
            #[doc = "1=Swap doublewords in 8-byte blocks. Useful for SHA-512, which works on 64-bit words. Bus references issued in the order 0x4, 0x0, 0xC, 0x8, ..."]
            SWAP_8_BYTE_BLOCK = 0x01,
            #[doc = "2=Swap doublewords in 16-byte blocks. Useful for AES. Bus references issued in the order 0xC, 0x8, 0x4, 0x0, 0x1C, 0x18,..."]
            SWAP_16_BYTE_BLOCK = 0x02,
            #[doc = "3=Swap doublewords in 64-byte blocks. Useful for SHA-256. Bus references issued in the order 0x3C, 0x38, 0x34, 0x30, 0x2C, 0x28, 0x24, 0x20, 0x1C, 0x18, 0x14, 0x10, 0xC, 0x8, 0x4, 0x0,..."]
            SWAP_64_BYTE_BLOCK = 0x03,
            #[doc = "4=Swap 32-bit doublewords in 128-byte blocks"]
            SWAP_128_BYTE_BLOCK = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
        }
        impl OpBlkSwapEn {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> OpBlkSwapEn {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for OpBlkSwapEn {
            #[inline(always)]
            fn from(val: u8) -> OpBlkSwapEn {
                OpBlkSwapEn::from_bits(val)
            }
        }
        impl From<OpBlkSwapEn> for u8 {
            #[inline(always)]
            fn from(val: OpBlkSwapEn) -> u8 {
                OpBlkSwapEn::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum PinCfg {
            #[doc = "0=All four pins associated with JTAG (TCK, TMS, TDI and TDO) switch to the debug interface when TRST# is de-asserted high. This setting should be used when the JTAG TAP controller is required for debugging."]
            JTAG_TCK_TMS_TDO_TDI = 0x0,
            #[doc = "1=The pins associated with the JTAG TCK, TMS and TDO switch to the debug interface when TRST# is de-asserted high. The pin associated with TDI remains controlled by the associated GPIO. This setting should be used when the ARM Serial Wire Debug (SWD) and Serial Wire Viewer (SWV) are both required for debugging"]
            JTAG_TCK_TMS_TDO = 0x01,
            #[doc = "2=The pins associated with the JTAG TCK and TMS switch to the debug interface when TRST# is de-asserted high. The pins associated with TDI and TDO remain controlled by the associated GPIO. This setting should be used when the ARM Serial Wire Debug (SWD) is required for debugging and the Serial Wire Viewer is not required"]
            JTAG_TCK_TMS = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl PinCfg {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> PinCfg {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for PinCfg {
            #[inline(always)]
            fn from(val: u8) -> PinCfg {
                PinCfg::from_bits(val)
            }
        }
        impl From<PinCfg> for u8 {
            #[inline(always)]
            fn from(val: PinCfg) -> u8 {
                PinCfg::to_bits(val)
            }
        }
    }
}
pub mod ecia {
    #[doc = "The ECIA works in conjunction with the processor interrupt interface to handle hardware interrupts andd exceptions."]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ecia {
        ptr: *mut u8,
    }
    unsafe impl Send for Ecia {}
    unsafe impl Sync for Ecia {}
    impl Ecia {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "GIRQ8 Source Register"]
        #[inline(always)]
        pub const fn src8(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "GIRQ8 Enable Set Register"]
        #[inline(always)]
        pub const fn en_set8(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "GIRQ8 Result Register"]
        #[inline(always)]
        pub const fn result8(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "GIRQ8 Enable Clear Register"]
        #[inline(always)]
        pub const fn en_clr8(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "GIRQ9 Source Register"]
        #[inline(always)]
        pub const fn src9(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "GIRQ9 Enable Set Register"]
        #[inline(always)]
        pub const fn en_set9(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
        }
        #[doc = "GIRQ9 Result Register"]
        #[inline(always)]
        pub const fn result9(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
        #[doc = "GIRQ9 Enable Clear Register"]
        #[inline(always)]
        pub const fn en_clr9(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }
        #[doc = "GIRQ10 Source Register"]
        #[inline(always)]
        pub const fn src10(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
        }
        #[doc = "GIRQ10 Enable Set Register"]
        #[inline(always)]
        pub const fn en_set10(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
        }
        #[doc = "GIRQ10 Result Register"]
        #[inline(always)]
        pub const fn result10(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
        }
        #[doc = "GIRQ10 Enable Clear Register"]
        #[inline(always)]
        pub const fn en_clr10(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
        }
        #[doc = "GIRQ11 Source Register"]
        #[inline(always)]
        pub const fn src11(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
        }
        #[doc = "GIRQ11 Enable Set Register"]
        #[inline(always)]
        pub const fn en_set11(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
        }
        #[doc = "GIRQ11 Result Register"]
        #[inline(always)]
        pub const fn result11(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
        }
        #[doc = "GIRQ11 Enable Clear Register"]
        #[inline(always)]
        pub const fn en_clr11(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
        }
        #[doc = "GIRQ12 Source Register"]
        #[inline(always)]
        pub const fn src12(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
        }
        #[doc = "GIRQ12 Enable Set Register"]
        #[inline(always)]
        pub const fn en_set12(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
        }
        #[doc = "GIRQ12 Result Register"]
        #[inline(always)]
        pub const fn result12(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
        }
        #[doc = "GIRQ12 Enable Clear Register"]
        #[inline(always)]
        pub const fn en_clr12(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
        }
        #[doc = "GIRQ13 Source Register"]
        #[inline(always)]
        pub const fn src13(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
        }
        #[doc = "GIRQ13 Enable Set Register"]
        #[inline(always)]
        pub const fn en_set13(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
        }
        #[doc = "GIRQ13 Result Register"]
        #[inline(always)]
        pub const fn result13(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
        }
        #[doc = "GIRQ13 Enable Clear Register"]
        #[inline(always)]
        pub const fn en_clr13(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
        }
        #[doc = "GIRQ14 Source Register"]
        #[inline(always)]
        pub const fn src14(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
        }
        #[doc = "GIRQ14 Enable Set Register"]
        #[inline(always)]
        pub const fn en_set14(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
        }
        #[doc = "GIRQ14 Result Register"]
        #[inline(always)]
        pub const fn result14(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
        }
        #[doc = "GIRQ14 Enable Clear Register"]
        #[inline(always)]
        pub const fn en_clr14(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
        }
        #[doc = "GIRQ15 Source Register"]
        #[inline(always)]
        pub const fn src15(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
        }
        #[doc = "GIRQ15 Enable Set Register"]
        #[inline(always)]
        pub const fn en_set15(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
        }
        #[doc = "GIRQ15 Result Register"]
        #[inline(always)]
        pub const fn result15(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
        }
        #[doc = "GIRQ15 Enable Clear Register"]
        #[inline(always)]
        pub const fn en_clr15(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
        }
        #[doc = "GIRQ16 Source Register"]
        #[inline(always)]
        pub const fn src16(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
        }
        #[doc = "GIRQ16 Enable Set Register"]
        #[inline(always)]
        pub const fn en_set16(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
        }
        #[doc = "GIRQ16 Result Register"]
        #[inline(always)]
        pub const fn result16(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
        }
        #[doc = "GIRQ16 Enable Clear Register"]
        #[inline(always)]
        pub const fn en_clr16(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
        }
        #[doc = "GIRQ17 Source Register"]
        #[inline(always)]
        pub const fn src17(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
        }
        #[doc = "GIRQ17 Enable Set Register"]
        #[inline(always)]
        pub const fn en_set17(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
        }
        #[doc = "GIRQ17 Result Register"]
        #[inline(always)]
        pub const fn result17(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
        }
        #[doc = "GIRQ17 Enable Clear Register"]
        #[inline(always)]
        pub const fn en_clr17(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
        }
        #[doc = "GIRQ18 Source Register"]
        #[inline(always)]
        pub const fn src18(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
        }
        #[doc = "GIRQ18 Enable Set Register"]
        #[inline(always)]
        pub const fn en_set18(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
        }
        #[doc = "GIRQ18 Result Register"]
        #[inline(always)]
        pub const fn result18(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
        }
        #[doc = "GIRQ18 Enable Clear Register"]
        #[inline(always)]
        pub const fn en_clr18(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
        }
        #[doc = "GIRQ19 Source Register"]
        #[inline(always)]
        pub const fn src19(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
        }
        #[doc = "GIRQ19 Enable Set Register"]
        #[inline(always)]
        pub const fn en_set19(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
        }
        #[doc = "GIRQ19 Result Register"]
        #[inline(always)]
        pub const fn result19(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
        }
        #[doc = "GIRQ19 Enable Clear Register"]
        #[inline(always)]
        pub const fn en_clr19(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
        }
        #[doc = "GIRQ20 Source Register"]
        #[inline(always)]
        pub const fn src20(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
        }
        #[doc = "GIRQ20 Enable Set Register"]
        #[inline(always)]
        pub const fn en_set20(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
        }
        #[doc = "GIRQ20 Result Register"]
        #[inline(always)]
        pub const fn result20(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
        }
        #[doc = "GIRQ20 Enable Clear Register"]
        #[inline(always)]
        pub const fn en_clr20(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
        }
        #[doc = "GIRQ21 Source Register"]
        #[inline(always)]
        pub const fn src21(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
        }
        #[doc = "GIRQ21 Enable Set Register"]
        #[inline(always)]
        pub const fn en_set21(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
        }
        #[doc = "GIRQ21 Result Register"]
        #[inline(always)]
        pub const fn result21(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
        }
        #[doc = "GIRQ21 Enable Clear Register"]
        #[inline(always)]
        pub const fn en_clr21(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
        }
        #[doc = "GIRQ22 Source Register"]
        #[inline(always)]
        pub const fn src22(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
        }
        #[doc = "GIRQ22 Enable Set Register"]
        #[inline(always)]
        pub const fn en_set22(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
        }
        #[doc = "GIRQ22 Result Register"]
        #[inline(always)]
        pub const fn result22(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
        }
        #[doc = "GIRQ22 Enable Clear Register"]
        #[inline(always)]
        pub const fn en_clr22(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
        }
        #[doc = "GIRQ23 Source Register"]
        #[inline(always)]
        pub const fn src23(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x012cusize) as _) }
        }
        #[doc = "GIRQ23 Enable Set Register"]
        #[inline(always)]
        pub const fn en_set23(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
        }
        #[doc = "GIRQ23 Result Register"]
        #[inline(always)]
        pub const fn result23(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
        }
        #[doc = "GIRQ23 Enable Clear Register"]
        #[inline(always)]
        pub const fn en_clr23(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
        }
        #[doc = "GIRQ24 Source Register"]
        #[inline(always)]
        pub const fn src24(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
        }
        #[doc = "GIRQ24 Enable Set Register"]
        #[inline(always)]
        pub const fn en_set24(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
        }
        #[doc = "GIRQ24 Result Register"]
        #[inline(always)]
        pub const fn result24(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
        }
        #[doc = "GIRQ24 Enable Clear Register"]
        #[inline(always)]
        pub const fn en_clr24(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
        }
        #[doc = "GIRQ25 Source Register"]
        #[inline(always)]
        pub const fn src25(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0154usize) as _) }
        }
        #[doc = "GIRQ25 Enable Set Register"]
        #[inline(always)]
        pub const fn en_set25(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0158usize) as _) }
        }
        #[doc = "GIRQ25 Result Register"]
        #[inline(always)]
        pub const fn result25(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x015cusize) as _) }
        }
        #[doc = "GIRQ25 Enable Clear Register"]
        #[inline(always)]
        pub const fn en_clr25(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0160usize) as _) }
        }
        #[doc = "GIRQ26 Source Register"]
        #[inline(always)]
        pub const fn src26(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0168usize) as _) }
        }
        #[doc = "GIRQ26 Enable Set Register"]
        #[inline(always)]
        pub const fn en_set26(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x016cusize) as _) }
        }
        #[doc = "GIRQ26 Result Register"]
        #[inline(always)]
        pub const fn result26(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0170usize) as _) }
        }
        #[doc = "GIRQ26 Enable Clear Register"]
        #[inline(always)]
        pub const fn en_clr26(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0174usize) as _) }
        }
        #[doc = "Block Enable Set Register"]
        #[inline(always)]
        pub const fn blk_en_set(self) -> crate::common::Reg<regs::BlkEnSet, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[doc = "Block Enable Clear Register."]
        #[inline(always)]
        pub const fn blk_en_clr(self) -> crate::common::Reg<regs::BlkEnClr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0204usize) as _) }
        }
        #[doc = "Block IRQ Vector Register"]
        #[inline(always)]
        pub const fn blk_irq_vtor(self) -> crate::common::Reg<regs::BlkIrqVtor, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0208usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Block Enable Clear Register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct BlkEnClr(pub u32);
        impl BlkEnClr {
            #[doc = "Each GIRQx bit can be individually disabled to inhibit an interrupt event. Reads always return the current value of the internal GIRQX_ENABLE bit. The state of the GIRQX_ENABLE bit is determined by the corresponding GIRQX_ENABLE_SET bit and the GIRQX_ENABLE_CLEAR bit. (0=disabled, 1=enabled) (R/WC) 1=All interrupts in the GIRQx Source Register are disabled 0=No effect."]
            #[inline(always)]
            pub const fn vtor_en_clr(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x7fff_ffff;
                val as u32
            }
            #[doc = "Each GIRQx bit can be individually disabled to inhibit an interrupt event. Reads always return the current value of the internal GIRQX_ENABLE bit. The state of the GIRQX_ENABLE bit is determined by the corresponding GIRQX_ENABLE_SET bit and the GIRQX_ENABLE_CLEAR bit. (0=disabled, 1=enabled) (R/WC) 1=All interrupts in the GIRQx Source Register are disabled 0=No effect."]
            #[inline(always)]
            pub fn set_vtor_en_clr(&mut self, val: u32) {
                self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
            }
        }
        impl Default for BlkEnClr {
            #[inline(always)]
            fn default() -> BlkEnClr {
                BlkEnClr(0)
            }
        }
        impl core::fmt::Debug for BlkEnClr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("BlkEnClr")
                    .field("vtor_en_clr", &self.vtor_en_clr())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for BlkEnClr {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "BlkEnClr {{ vtor_en_clr: {=u32:?} }}", self.vtor_en_clr())
            }
        }
        #[doc = "Block Enable Set Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct BlkEnSet(pub u32);
        impl BlkEnSet {
            #[doc = "Each GIRQx bit can be individually enabled to assert an interrupt event. Reads always return the current value of the internal GIRQX_ENABLE bit. The state of the GIRQX_ENABLE bit is determined by the corresponding GIRQX_ENABLE_SET bit and the GIRQX_ENABLE_CLEAR bit. (0=disabled, 1=enabled) (R/WS) 1=Interrupts in the GIRQx Source Register may be enabled 0=No effect."]
            #[inline(always)]
            pub const fn vtor_en_set(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x7fff_ffff;
                val as u32
            }
            #[doc = "Each GIRQx bit can be individually enabled to assert an interrupt event. Reads always return the current value of the internal GIRQX_ENABLE bit. The state of the GIRQX_ENABLE bit is determined by the corresponding GIRQX_ENABLE_SET bit and the GIRQX_ENABLE_CLEAR bit. (0=disabled, 1=enabled) (R/WS) 1=Interrupts in the GIRQx Source Register may be enabled 0=No effect."]
            #[inline(always)]
            pub fn set_vtor_en_set(&mut self, val: u32) {
                self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
            }
        }
        impl Default for BlkEnSet {
            #[inline(always)]
            fn default() -> BlkEnSet {
                BlkEnSet(0)
            }
        }
        impl core::fmt::Debug for BlkEnSet {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("BlkEnSet")
                    .field("vtor_en_set", &self.vtor_en_set())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for BlkEnSet {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "BlkEnSet {{ vtor_en_set: {=u32:?} }}", self.vtor_en_set())
            }
        }
        #[doc = "Block IRQ Vector Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct BlkIrqVtor(pub u32);
        impl BlkIrqVtor {
            #[doc = "Each bit in this field reports the status of the group GIRQ interrupt assertion to the NVIC. If the GIRQx interrupt is disabled as a group, by the Block Enable Clear Register, then the corresponding bit will be '0'b and no interrupt will be asserted."]
            #[inline(always)]
            pub const fn vtor(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x01ff_ffff;
                val as u32
            }
            #[doc = "Each bit in this field reports the status of the group GIRQ interrupt assertion to the NVIC. If the GIRQx interrupt is disabled as a group, by the Block Enable Clear Register, then the corresponding bit will be '0'b and no interrupt will be asserted."]
            #[inline(always)]
            pub fn set_vtor(&mut self, val: u32) {
                self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
            }
        }
        impl Default for BlkIrqVtor {
            #[inline(always)]
            fn default() -> BlkIrqVtor {
                BlkIrqVtor(0)
            }
        }
        impl core::fmt::Debug for BlkIrqVtor {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("BlkIrqVtor").field("vtor", &self.vtor()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for BlkIrqVtor {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "BlkIrqVtor {{ vtor: {=u32:?} }}", self.vtor())
            }
        }
    }
}
pub mod gcr {
    #[doc = "The Logical Device Configuration registers support motherboard designs in which the resources required by their components are known and assigned by the BIOS at POST."]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gcr {
        ptr: *mut u8,
    }
    unsafe impl Send for Gcr {}
    unsafe impl Sync for Gcr {}
    impl Gcr {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "A write to this register selects the current logical device. This allows access to the control and configuration registers for each logical device. Note: The Activate command operates only on the selected logical device."]
        #[inline(always)]
        pub const fn ldn(self) -> crate::common::Reg<u8, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07usize) as _) }
        }
        #[doc = "A read-only register which provides device revision information."]
        #[inline(always)]
        pub const fn dev_rev(self) -> crate::common::Reg<u8, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
        #[doc = "A read-only register which provides device sub ID information."]
        #[inline(always)]
        pub const fn dev_subid(self) -> crate::common::Reg<u8, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1dusize) as _) }
        }
        #[doc = "A read-only register which provides device identification."]
        #[inline(always)]
        pub const fn dev_id(self) -> crate::common::Reg<u16, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1eusize) as _) }
        }
        #[doc = "A read-only register which provides legacy device identification."]
        #[inline(always)]
        pub const fn leg_dev_id(self) -> crate::common::Reg<u8, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }
        #[doc = "A read-only register which provides legacy device revision information."]
        #[inline(always)]
        pub const fn leg_dev_rev(self) -> crate::common::Reg<u8, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x21usize) as _) }
        }
    }
}
pub mod gpio {
    #[doc = "GPIO Pin Control Registers"]
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
        #[inline(always)]
        pub const fn port0(self) -> Port {
            unsafe { Port::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[inline(always)]
        pub const fn port1(self) -> Port {
            unsafe { Port::from_ptr(self.ptr.add(0x20usize) as _) }
        }
        #[inline(always)]
        pub const fn port2(self) -> Port {
            unsafe { Port::from_ptr(self.ptr.add(0x40usize) as _) }
        }
        #[inline(always)]
        pub const fn port3(self) -> Port {
            unsafe { Port::from_ptr(self.ptr.add(0x60usize) as _) }
        }
        #[inline(always)]
        pub const fn port4(self) -> Port {
            unsafe { Port::from_ptr(self.ptr.add(0x80usize) as _) }
        }
        #[inline(always)]
        pub const fn port5(self) -> Port {
            unsafe { Port::from_ptr(self.ptr.add(0xa0usize) as _) }
        }
        #[inline(always)]
        pub const fn port6(self) -> Port {
            unsafe { Port::from_ptr(self.ptr.add(0xc0usize) as _) }
        }
        #[inline(always)]
        pub const fn port7(self) -> Port {
            unsafe { Port::from_ptr(self.ptr.add(0xe0usize) as _) }
        }
        #[inline(always)]
        pub const fn port10(self) -> Port {
            unsafe { Port::from_ptr(self.ptr.add(0x0100usize) as _) }
        }
        #[inline(always)]
        pub const fn port11(self) -> Port {
            unsafe { Port::from_ptr(self.ptr.add(0x0120usize) as _) }
        }
        #[inline(always)]
        pub const fn port12(self) -> Port {
            unsafe { Port::from_ptr(self.ptr.add(0x0140usize) as _) }
        }
        #[inline(always)]
        pub const fn port13(self) -> Port {
            unsafe { Port::from_ptr(self.ptr.add(0x0160usize) as _) }
        }
        #[inline(always)]
        pub const fn port14(self) -> Port {
            unsafe { Port::from_ptr(self.ptr.add(0x0180usize) as _) }
        }
        #[inline(always)]
        pub const fn port15(self) -> Port {
            unsafe { Port::from_ptr(self.ptr.add(0x01a0usize) as _) }
        }
        #[inline(always)]
        pub const fn port16(self) -> Port {
            unsafe { Port::from_ptr(self.ptr.add(0x01c0usize) as _) }
        }
        #[inline(always)]
        pub const fn port17(self) -> Port {
            unsafe { Port::from_ptr(self.ptr.add(0x01e0usize) as _) }
        }
        #[inline(always)]
        pub const fn port20(self) -> Port {
            unsafe { Port::from_ptr(self.ptr.add(0x0200usize) as _) }
        }
        #[inline(always)]
        pub const fn port21(self) -> Port {
            unsafe { Port::from_ptr(self.ptr.add(0x0220usize) as _) }
        }
        #[inline(always)]
        pub const fn port22(self) -> Port {
            unsafe { Port::from_ptr(self.ptr.add(0x0240usize) as _) }
        }
        #[inline(always)]
        pub const fn port23(self) -> Port {
            unsafe { Port::from_ptr(self.ptr.add(0x0260usize) as _) }
        }
        #[inline(always)]
        pub const fn port24(self) -> Port {
            unsafe { Port::from_ptr(self.ptr.add(0x0280usize) as _) }
        }
        #[inline(always)]
        pub const fn port25(self) -> Port {
            unsafe { Port::from_ptr(self.ptr.add(0x02a0usize) as _) }
        }
        #[inline(always)]
        pub const fn port26(self) -> Port {
            unsafe { Port::from_ptr(self.ptr.add(0x02c0usize) as _) }
        }
        #[doc = "The GPIO Input Registers."]
        #[inline(always)]
        pub const fn parin(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 6usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize + n * 4usize) as _) }
        }
        #[doc = "The GPIO Output Registers."]
        #[inline(always)]
        pub const fn parout(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 6usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0380usize + n * 4usize) as _) }
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Port {
        ptr: *mut u8,
    }
    unsafe impl Send for Port {}
    unsafe impl Sync for Port {}
    impl Port {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "GPIO Pin Control Register"]
        #[inline(always)]
        pub const fn ctrl1(self, n: usize) -> crate::common::Reg<regs::Ctrl1, crate::common::RW> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
        }
        #[doc = "The GPIO PIN_CTRL2 Registers"]
        #[inline(always)]
        pub const fn ctrl2(self, n: usize) -> crate::common::Reg<regs::Ctrl2, crate::common::RW> {
            assert!(n < 8usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize + n * 4usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "GPIO Pin Control Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctrl1(pub u32);
        impl Ctrl1 {
            #[doc = "These bits are used to enable an internal pull-up or pull-down resistor."]
            #[inline(always)]
            pub const fn pu_pd(&self) -> super::super::Pull {
                let val = (self.0 >> 0usize) & 0x03;
                super::super::Pull::from_bits(val as u8)
            }
            #[doc = "These bits are used to enable an internal pull-up or pull-down resistor."]
            #[inline(always)]
            pub fn set_pu_pd(&mut self, val: super::super::Pull) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
            }
            #[doc = "The GPIO pin will be tristated when the selected power well is off."]
            #[inline(always)]
            pub const fn pwr_gating(&self) -> super::super::Pgs {
                let val = (self.0 >> 2usize) & 0x03;
                super::super::Pgs::from_bits(val as u8)
            }
            #[doc = "The GPIO pin will be tristated when the selected power well is off."]
            #[inline(always)]
            pub fn set_pwr_gating(&mut self, val: super::super::Pgs) {
                self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
            }
            #[doc = "Determines the interrupt capability of the GPIO input."]
            #[inline(always)]
            pub const fn intr_det(&self) -> u8 {
                let val = (self.0 >> 4usize) & 0x07;
                val as u8
            }
            #[doc = "Determines the interrupt capability of the GPIO input."]
            #[inline(always)]
            pub fn set_intr_det(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
            }
            #[doc = "Determines the interrupt capability of the GPIO input."]
            #[inline(always)]
            pub const fn edge_en(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Determines the interrupt capability of the GPIO input."]
            #[inline(always)]
            pub fn set_edge_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "0 = Push-Pull, 1 = Open Drain"]
            #[inline(always)]
            pub const fn out_buff_type(&self) -> super::super::BufferType {
                let val = (self.0 >> 8usize) & 0x01;
                super::super::BufferType::from_bits(val as u8)
            }
            #[doc = "0 = Push-Pull, 1 = Open Drain"]
            #[inline(always)]
            pub fn set_out_buff_type(&mut self, val: super::super::BufferType) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
            }
            #[doc = "Buffer direction when GPIO selected by pin mux 0 = Input, 1 = Output"]
            #[inline(always)]
            pub const fn dir(&self) -> super::super::Dir {
                let val = (self.0 >> 9usize) & 0x01;
                super::super::Dir::from_bits(val as u8)
            }
            #[doc = "Buffer direction when GPIO selected by pin mux 0 = Input, 1 = Output"]
            #[inline(always)]
            pub fn set_dir(&mut self, val: super::super::Dir) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
            }
            #[doc = "GPIO outputs registe select.0=GPIO ALTERNATE_GPIO_DATA 1=GPIO Output Register."]
            #[inline(always)]
            pub const fn out_sel(&self) -> super::super::Sel {
                let val = (self.0 >> 10usize) & 0x01;
                super::super::Sel::from_bits(val as u8)
            }
            #[doc = "GPIO outputs registe select.0=GPIO ALTERNATE_GPIO_DATA 1=GPIO Output Register."]
            #[inline(always)]
            pub fn set_out_sel(&mut self, val: super::super::Sel) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
            }
            #[doc = "1=Inverted; 0=Non-inverted"]
            #[inline(always)]
            pub const fn pol(&self) -> super::super::Pol {
                let val = (self.0 >> 11usize) & 0x01;
                super::super::Pol::from_bits(val as u8)
            }
            #[doc = "1=Inverted; 0=Non-inverted"]
            #[inline(always)]
            pub fn set_pol(&mut self, val: super::super::Pol) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
            }
            #[doc = "00 = GPIO Function, 01 = Function 1, 10 = Function 2, 11 = Function 3."]
            #[inline(always)]
            pub const fn mux_ctrl(&self) -> super::super::Function {
                let val = (self.0 >> 12usize) & 0x07;
                super::super::Function::from_bits(val as u8)
            }
            #[doc = "00 = GPIO Function, 01 = Function 1, 10 = Function 2, 11 = Function 3."]
            #[inline(always)]
            pub fn set_mux_ctrl(&mut self, val: super::super::Function) {
                self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
            }
            #[doc = "GPIO input disable"]
            #[inline(always)]
            pub const fn inp_dis(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "GPIO input disable"]
            #[inline(always)]
            pub fn set_inp_dis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
            #[doc = "GPIO Alternate Data Register."]
            #[inline(always)]
            pub const fn alt_data(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "GPIO Alternate Data Register."]
            #[inline(always)]
            pub fn set_alt_data(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "GPIO input from pin, independent of the Mux selection for the pin or the Direction."]
            #[inline(always)]
            pub const fn inp(&self) -> bool {
                let val = (self.0 >> 24usize) & 0x01;
                val != 0
            }
            #[doc = "GPIO input from pin, independent of the Mux selection for the pin or the Direction."]
            #[inline(always)]
            pub fn set_inp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
            }
        }
        impl Default for Ctrl1 {
            #[inline(always)]
            fn default() -> Ctrl1 {
                Ctrl1(0)
            }
        }
        impl core::fmt::Debug for Ctrl1 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ctrl1")
                    .field("pu_pd", &self.pu_pd())
                    .field("pwr_gating", &self.pwr_gating())
                    .field("intr_det", &self.intr_det())
                    .field("edge_en", &self.edge_en())
                    .field("out_buff_type", &self.out_buff_type())
                    .field("dir", &self.dir())
                    .field("out_sel", &self.out_sel())
                    .field("pol", &self.pol())
                    .field("mux_ctrl", &self.mux_ctrl())
                    .field("inp_dis", &self.inp_dis())
                    .field("alt_data", &self.alt_data())
                    .field("inp", &self.inp())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ctrl1 {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Ctrl1 {{ pu_pd: {:?}, pwr_gating: {:?}, intr_det: {=u8:?}, edge_en: {=bool:?}, out_buff_type: {:?}, dir: {:?}, out_sel: {:?}, pol: {:?}, mux_ctrl: {:?}, inp_dis: {=bool:?}, alt_data: {=bool:?}, inp: {=bool:?} }}" , self . pu_pd () , self . pwr_gating () , self . intr_det () , self . edge_en () , self . out_buff_type () , self . dir () , self . out_sel () , self . pol () , self . mux_ctrl () , self . inp_dis () , self . alt_data () , self . inp ())
            }
        }
        #[doc = "The GPIO PIN_CTRL2 Registers"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctrl2(pub u32);
        impl Ctrl2 {
            #[doc = "Selects slew rate on the pin. 1=fast 0=slow"]
            #[inline(always)]
            pub const fn slew_ctrl(&self) -> super::super::SlewCtrl {
                let val = (self.0 >> 0usize) & 0x01;
                super::super::SlewCtrl::from_bits(val as u8)
            }
            #[doc = "Selects slew rate on the pin. 1=fast 0=slow"]
            #[inline(always)]
            pub fn set_slew_ctrl(&mut self, val: super::super::SlewCtrl) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
            }
            #[doc = "Selects the drive strength on the pin. 00 = 2mA, 01 = 4mA, 10 = 8mA, 11 = 12mA"]
            #[inline(always)]
            pub const fn driv_stren(&self) -> super::super::Strength {
                let val = (self.0 >> 4usize) & 0x03;
                super::super::Strength::from_bits(val as u8)
            }
            #[doc = "Selects the drive strength on the pin. 00 = 2mA, 01 = 4mA, 10 = 8mA, 11 = 12mA"]
            #[inline(always)]
            pub fn set_driv_stren(&mut self, val: super::super::Strength) {
                self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
            }
        }
        impl Default for Ctrl2 {
            #[inline(always)]
            fn default() -> Ctrl2 {
                Ctrl2(0)
            }
        }
        impl core::fmt::Debug for Ctrl2 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ctrl2")
                    .field("slew_ctrl", &self.slew_ctrl())
                    .field("driv_stren", &self.driv_stren())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ctrl2 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Ctrl2 {{ slew_ctrl: {:?}, driv_stren: {:?} }}",
                    self.slew_ctrl(),
                    self.driv_stren()
                )
            }
        }
    }
}
pub mod htm0 {
    #[doc = "The Hibernation Timer can generate a wake event to the Embedded Controller (EC) when it is in a hibernation mode"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Htm0 {
        ptr: *mut u8,
    }
    unsafe impl Send for Htm0 {}
    unsafe impl Sync for Htm0 {}
    impl Htm0 {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "15:0\\] This register is used to set the Hibernation Timer Preload value."]
        #[inline(always)]
        pub const fn prld(self) -> crate::common::Reg<u16, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "HTimer Control Register"]
        #[inline(always)]
        pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "The current state of the Hibernation Timer."]
        #[inline(always)]
        pub const fn cnt(self) -> crate::common::Reg<u16, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "HTimer Control Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctrl(pub u16);
        impl Ctrl {
            #[doc = "1= The Hibernation Timer has a resolution of 0.125s per LSB, which yields a maximum time in excess of 2 hours. 0= The Hibernation Timer has a resolution of 30.5us per LSB, which yields a maximum time of ~2seconds."]
            #[inline(always)]
            pub const fn ctrl(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "1= The Hibernation Timer has a resolution of 0.125s per LSB, which yields a maximum time in excess of 2 hours. 0= The Hibernation Timer has a resolution of 30.5us per LSB, which yields a maximum time of ~2seconds."]
            #[inline(always)]
            pub fn set_ctrl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
            }
        }
        impl Default for Ctrl {
            #[inline(always)]
            fn default() -> Ctrl {
                Ctrl(0)
            }
        }
        impl core::fmt::Debug for Ctrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ctrl").field("ctrl", &self.ctrl()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ctrl {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Ctrl {{ ctrl: {=bool:?} }}", self.ctrl())
            }
        }
    }
}
pub mod i2c0 {
    #[doc = "The I2C interface can handle standard I2C interface."]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I2c0 {
        ptr: *mut u8,
    }
    unsafe impl Send for I2c0 {}
    unsafe impl Sync for I2c0 {}
    impl I2c0 {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Status Register"]
        #[inline(always)]
        pub const fn rsts(self) -> crate::common::Reg<regs::Rsts, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Control Register"]
        #[inline(always)]
        pub const fn wctrl(self) -> crate::common::Reg<regs::Wctrl, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Own Address Register Note that the Data Register and Own Address fields are offset by one bit, so that programming Own Address 1 with a value of 55h will result in the value AAh being recognized as the SMB Controller Core slave address."]
        #[inline(always)]
        pub const fn own_addr(self) -> crate::common::Reg<regs::OwnAddr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "This register holds the data that are either shifted out to or shifted in from the I2C port."]
        #[inline(always)]
        pub const fn i2cdata(self) -> crate::common::Reg<u8, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Repeated Start Hold Time Register"]
        #[inline(always)]
        pub const fn rshtm(self) -> crate::common::Reg<regs::Rshtm, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
        }
        #[doc = "Reserved"]
        #[inline(always)]
        pub const fn rsvd1(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
        #[doc = "Completion Register"]
        #[inline(always)]
        pub const fn compl(self) -> crate::common::Reg<regs::Compl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }
        #[doc = "Reserved"]
        #[inline(always)]
        pub const fn rsvd2(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
        }
        #[doc = "Configuration Register"]
        #[inline(always)]
        pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
        }
        #[doc = "Bus Clock Register"]
        #[inline(always)]
        pub const fn busclk(self) -> crate::common::Reg<regs::Busclk, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
        }
        #[doc = "Block ID Register"]
        #[inline(always)]
        pub const fn blkid(self) -> crate::common::Reg<regs::Blkid, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
        }
        #[doc = "Revision Register"]
        #[inline(always)]
        pub const fn blkrev(self) -> crate::common::Reg<regs::Blkrev, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
        }
        #[doc = "Bit-Bang Control Register"]
        #[inline(always)]
        pub const fn bb_ctrl(self) -> crate::common::Reg<regs::BbCtrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
        }
        #[doc = "This is Clock Sync Register. This register must not be written, or undesirable results may occur."]
        #[inline(always)]
        pub const fn clksync(self) -> crate::common::Reg<regs::Clksync, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
        }
        #[doc = "Data Timing Register"]
        #[inline(always)]
        pub const fn datatm(self) -> crate::common::Reg<regs::Datatm, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
        }
        #[doc = "Time-Out Scaling Register"]
        #[inline(always)]
        pub const fn tmoutsc(self) -> crate::common::Reg<regs::Tmoutsc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
        }
        #[doc = "WAKE STATUS Register"]
        #[inline(always)]
        pub const fn wake_sts(self) -> crate::common::Reg<regs::WakeSts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
        }
        #[doc = "WAKE ENABLE Register"]
        #[inline(always)]
        pub const fn wake_en(self) -> crate::common::Reg<regs::WakeEn, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
        }
        #[doc = "This is the Slave Address Register."]
        #[inline(always)]
        pub const fn slv_addr(self) -> crate::common::Reg<regs::SlvAddr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
        }
        #[doc = "This is the Promiscuous Interrupt Register. This register bit will be functional only in Promiscuous mode."]
        #[inline(always)]
        pub const fn prm_sts(self) -> crate::common::Reg<regs::PrmSts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
        }
        #[doc = "This is the Promiscuous Interrupt Enable Register."]
        #[inline(always)]
        pub const fn prm_ien(self) -> crate::common::Reg<regs::PrmIen, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
        }
        #[doc = "This is the Promiscuous Control Register. This register is functional only in Promiscuous mode."]
        #[inline(always)]
        pub const fn prm_ctrl(self) -> crate::common::Reg<regs::PrmCtrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Bit-Bang Control Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct BbCtrl(pub u32);
        impl BbCtrl {
            #[doc = "Bit-Bang Mode Enable. 0 - Bit Bang Mode Disabled. 1 - Bit Bang Mode Enabled"]
            #[inline(always)]
            pub const fn bben(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Bit-Bang Mode Enable. 0 - Bit Bang Mode Disabled. 1 - Bit Bang Mode Enabled"]
            #[inline(always)]
            pub fn set_bben(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Bit-Bang Clock Direction. The CLDIR bit controls the direction of SCLK. 0 - Input, 1 - Output"]
            #[inline(always)]
            pub const fn cldir(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Bit-Bang Clock Direction. The CLDIR bit controls the direction of SCLK. 0 - Input, 1 - Output"]
            #[inline(always)]
            pub fn set_cldir(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Bit-Bang Data Direction. The DADIR bit controls the direction of SDAT. 0 - Input. 1 - Output"]
            #[inline(always)]
            pub const fn dadir(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Bit-Bang Data Direction. The DADIR bit controls the direction of SDAT. 0 - Input. 1 - Output"]
            #[inline(always)]
            pub fn set_dadir(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Bit-Bang Clock. The BBCLK bit controls the state of SCLK when BBEN = and CLDIR = '1'"]
            #[inline(always)]
            pub const fn bbclk(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Bit-Bang Clock. The BBCLK bit controls the state of SCLK when BBEN = and CLDIR = '1'"]
            #[inline(always)]
            pub fn set_bbclk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Bit-Bang Data. The BBDAT bit controls the state of SDAT when BBEN = and DADIR = '1'"]
            #[inline(always)]
            pub const fn bbdat(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Bit-Bang Data. The BBDAT bit controls the state of SDAT when BBEN = and DADIR = '1'"]
            #[inline(always)]
            pub fn set_bbdat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Bit-Bang Clock In. The BBCLKI bit always returns the state of SCLK."]
            #[inline(always)]
            pub const fn bbclki(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Bit-Bang Clock In. The BBCLKI bit always returns the state of SCLK."]
            #[inline(always)]
            pub fn set_bbclki(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Bit-Bang Data In. The BBDATI bit always returns the state of SDAT"]
            #[inline(always)]
            pub const fn bbdati(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Bit-Bang Data In. The BBDATI bit always returns the state of SDAT"]
            #[inline(always)]
            pub fn set_bbdati(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
        }
        impl Default for BbCtrl {
            #[inline(always)]
            fn default() -> BbCtrl {
                BbCtrl(0)
            }
        }
        impl core::fmt::Debug for BbCtrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("BbCtrl")
                    .field("bben", &self.bben())
                    .field("cldir", &self.cldir())
                    .field("dadir", &self.dadir())
                    .field("bbclk", &self.bbclk())
                    .field("bbdat", &self.bbdat())
                    .field("bbclki", &self.bbclki())
                    .field("bbdati", &self.bbdati())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for BbCtrl {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "BbCtrl {{ bben: {=bool:?}, cldir: {=bool:?}, dadir: {=bool:?}, bbclk: {=bool:?}, bbdat: {=bool:?}, bbclki: {=bool:?}, bbdati: {=bool:?} }}" , self . bben () , self . cldir () , self . dadir () , self . bbclk () , self . bbdat () , self . bbclki () , self . bbdati ())
            }
        }
        #[doc = "Block ID Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Blkid(pub u8);
        impl Blkid {
            #[doc = "Block ID."]
            #[inline(always)]
            pub const fn id(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Block ID."]
            #[inline(always)]
            pub fn set_id(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
            }
        }
        impl Default for Blkid {
            #[inline(always)]
            fn default() -> Blkid {
                Blkid(0)
            }
        }
        impl core::fmt::Debug for Blkid {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Blkid").field("id", &self.id()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Blkid {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Blkid {{ id: {=u8:?} }}", self.id())
            }
        }
        #[doc = "Revision Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Blkrev(pub u8);
        impl Blkrev {
            #[doc = "Block Revision Number"]
            #[inline(always)]
            pub const fn rev(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Block Revision Number"]
            #[inline(always)]
            pub fn set_rev(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
            }
        }
        impl Default for Blkrev {
            #[inline(always)]
            fn default() -> Blkrev {
                Blkrev(0)
            }
        }
        impl core::fmt::Debug for Blkrev {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Blkrev").field("rev", &self.rev()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Blkrev {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Blkrev {{ rev: {=u8:?} }}", self.rev())
            }
        }
        #[doc = "Bus Clock Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Busclk(pub u32);
        impl Busclk {
            #[doc = "This field defines the number of I2C Baud Clock periods that make up the low phase of the I2C/SMBus bus clock."]
            #[inline(always)]
            pub const fn low_per(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "This field defines the number of I2C Baud Clock periods that make up the low phase of the I2C/SMBus bus clock."]
            #[inline(always)]
            pub fn set_low_per(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "This field defines the number of I2C Baud Clock periods that make up the high phase of the I2C/SMBus bus clock."]
            #[inline(always)]
            pub const fn high_per(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "This field defines the number of I2C Baud Clock periods that make up the high phase of the I2C/SMBus bus clock."]
            #[inline(always)]
            pub fn set_high_per(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
        }
        impl Default for Busclk {
            #[inline(always)]
            fn default() -> Busclk {
                Busclk(0)
            }
        }
        impl core::fmt::Debug for Busclk {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Busclk")
                    .field("low_per", &self.low_per())
                    .field("high_per", &self.high_per())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Busclk {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Busclk {{ low_per: {=u8:?}, high_per: {=u8:?} }}",
                    self.low_per(),
                    self.high_per()
                )
            }
        }
        #[doc = "Configuration Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cfg(pub u32);
        impl Cfg {
            #[doc = "The PORT SEL \\[3:0\\] bits determine which one of 16 possible bus ports apply to the active 2-wire SDAT and SCLK bus pair."]
            #[inline(always)]
            pub const fn port_sel(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "The PORT SEL \\[3:0\\] bits determine which one of 16 possible bus ports apply to the active 2-wire SDAT and SCLK bus pair."]
            #[inline(always)]
            pub fn set_port_sel(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
            #[doc = "When the Timing Check Enable bit (TCEN) is asserted ('1'), Bus Time-Outs are enabled"]
            #[inline(always)]
            pub const fn tcen(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "When the Timing Check Enable bit (TCEN) is asserted ('1'), Bus Time-Outs are enabled"]
            #[inline(always)]
            pub fn set_tcen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "When this bit is 1, the base period for the Bus Clock Register is multiplied by 4, and thus the frequency is divided by 4."]
            #[inline(always)]
            pub const fn slow_clk(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "When this bit is 1, the base period for the Bus Clock Register is multiplied by 4, and thus the frequency is divided by 4."]
            #[inline(always)]
            pub fn set_slow_clk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Must be always written with 0."]
            #[inline(always)]
            pub const fn test(&self) -> u8 {
                let val = (self.0 >> 6usize) & 0x03;
                val as u8
            }
            #[doc = "Must be always written with 0."]
            #[inline(always)]
            pub fn set_test(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
            }
            #[doc = "Input filtering enable. Input filtering is required by the I2C specification if external filtering is not available. 1=Input filtering is enabled; 0=Input filtering is disabled."]
            #[inline(always)]
            pub const fn fen(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Input filtering enable. Input filtering is required by the I2C specification if external filtering is not available. 1=Input filtering is enabled; 0=Input filtering is disabled."]
            #[inline(always)]
            pub fn set_fen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "When RESET is asserted ('1'), all logic and registers except for the RESET bit itself are initialized to the power-on default state."]
            #[inline(always)]
            pub const fn rst(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "When RESET is asserted ('1'), all logic and registers except for the RESET bit itself are initialized to the power-on default state."]
            #[inline(always)]
            pub fn set_rst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "When ENAB (Enable) is not asserted ('0') (default), the SMB Controller Core is disabled and in the lowest power consumption state (Disabled State). The ENAB bit must be asserted ('1') for normal operation."]
            #[inline(always)]
            pub const fn en(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "When ENAB (Enable) is not asserted ('0') (default), the SMB Controller Core is disabled and in the lowest power consumption state (Disabled State). The ENAB bit must be asserted ('1') for normal operation."]
            #[inline(always)]
            pub fn set_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "Must be always written with 0."]
            #[inline(always)]
            pub const fn test0(&self) -> u8 {
                let val = (self.0 >> 11usize) & 0x07;
                val as u8
            }
            #[doc = "Must be always written with 0."]
            #[inline(always)]
            pub fn set_test0(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
            }
            #[doc = "This is the General Call Disable bit. 0: the response to the General Call address as a slave is enabled 1: the response to the General Call address as a slave is disabled."]
            #[inline(always)]
            pub const fn gc_dis(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "This is the General Call Disable bit. 0: the response to the General Call address as a slave is enabled 1: the response to the General Call address as a slave is disabled."]
            #[inline(always)]
            pub fn set_gc_dis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "This is the configur Promiscuous bit. 0: Normal operation is enabled. 1: Promiscuous Mode enabled , General Call Address disabled, Promiscuous Address Interrupt function enabled. Stall 9th clock of address byte enabled. Address byte ACK/NAK done by Promiscuous ACK setting."]
            #[inline(always)]
            pub const fn cnfg_promis(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "This is the configur Promiscuous bit. 0: Normal operation is enabled. 1: Promiscuous Mode enabled , General Call Address disabled, Promiscuous Address Interrupt function enabled. Stall 9th clock of address byte enabled. Address byte ACK/NAK done by Promiscuous ACK setting."]
            #[inline(always)]
            pub fn set_cnfg_promis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
        }
        impl Default for Cfg {
            #[inline(always)]
            fn default() -> Cfg {
                Cfg(0)
            }
        }
        impl core::fmt::Debug for Cfg {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Cfg")
                    .field("port_sel", &self.port_sel())
                    .field("tcen", &self.tcen())
                    .field("slow_clk", &self.slow_clk())
                    .field("test", &self.test())
                    .field("fen", &self.fen())
                    .field("rst", &self.rst())
                    .field("en", &self.en())
                    .field("test0", &self.test0())
                    .field("gc_dis", &self.gc_dis())
                    .field("cnfg_promis", &self.cnfg_promis())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Cfg {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Cfg {{ port_sel: {=u8:?}, tcen: {=bool:?}, slow_clk: {=bool:?}, test: {=u8:?}, fen: {=bool:?}, rst: {=bool:?}, en: {=bool:?}, test0: {=u8:?}, gc_dis: {=bool:?}, cnfg_promis: {=bool:?} }}" , self . port_sel () , self . tcen () , self . slow_clk () , self . test () , self . fen () , self . rst () , self . en () , self . test0 () , self . gc_dis () , self . cnfg_promis ())
            }
        }
        #[doc = "This is Clock Sync Register. This register must not be written, or undesirable results may occur."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Clksync(pub u8);
        impl Clksync {
            #[doc = "This register must not be written, or undesirable results may occur."]
            #[inline(always)]
            pub const fn clk_sync(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "This register must not be written, or undesirable results may occur."]
            #[inline(always)]
            pub fn set_clk_sync(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
            }
        }
        impl Default for Clksync {
            #[inline(always)]
            fn default() -> Clksync {
                Clksync(0)
            }
        }
        impl core::fmt::Debug for Clksync {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Clksync").field("clk_sync", &self.clk_sync()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Clksync {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Clksync {{ clk_sync: {=u8:?} }}", self.clk_sync())
            }
        }
        #[doc = "Completion Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Compl(pub u32);
        impl Compl {
            #[doc = "When DTEN is asserted ('1'), Device Time-out checking is enabled. When DTEN is not asserted ('0'), Device Time-out checking is disabled."]
            #[inline(always)]
            pub const fn dten(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "When DTEN is asserted ('1'), Device Time-out checking is enabled. When DTEN is not asserted ('0'), Device Time-out checking is disabled."]
            #[inline(always)]
            pub fn set_dten(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "When MCEN is asserted ('1'), Master Cumulative Time-Out checking is enabled. When MCEN is not asserted ('0'), Master Cumulative Time-Out checking is disabled."]
            #[inline(always)]
            pub const fn mcen(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "When MCEN is asserted ('1'), Master Cumulative Time-Out checking is enabled. When MCEN is not asserted ('0'), Master Cumulative Time-Out checking is disabled."]
            #[inline(always)]
            pub fn set_mcen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "When SCEN is asserted ('1'), Slave Cumulative Time-Out checking is enabled. When SCEN is not asserted ('0'), Slave Cumulative Time-Out checking is disabled."]
            #[inline(always)]
            pub const fn scen(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "When SCEN is asserted ('1'), Slave Cumulative Time-Out checking is enabled. When SCEN is not asserted ('0'), Slave Cumulative Time-Out checking is disabled."]
            #[inline(always)]
            pub fn set_scen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "When BIDEN is asserted ('1'), Bus Idle Detect Time-Out checking is enabled. When BIDEN is not asserted ('0'), Bus Idle Detect Time-Out checking is disabled."]
            #[inline(always)]
            pub const fn biden(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "When BIDEN is asserted ('1'), Bus Idle Detect Time-Out checking is enabled. When BIDEN is not asserted ('0'), Bus Idle Detect Time-Out checking is disabled."]
            #[inline(always)]
            pub fn set_biden(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "The Time-out Error Detected bit (TIMERR) is asserted ('1') whenever any of the enabled time-out error detect status bits (CHDH, CHDL, SCTO, MCTO and DTO) are asserted."]
            #[inline(always)]
            pub const fn timerr(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "The Time-out Error Detected bit (TIMERR) is asserted ('1') whenever any of the enabled time-out error detect status bits (CHDH, CHDL, SCTO, MCTO and DTO) are asserted."]
            #[inline(always)]
            pub fn set_timerr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "DTO is the Device Time-out bit. (R/WC)"]
            #[inline(always)]
            pub const fn dto(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "DTO is the Device Time-out bit. (R/WC)"]
            #[inline(always)]
            pub fn set_dto(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "MCTO is the Master Cumulative Time-out bit. (R/WC)"]
            #[inline(always)]
            pub const fn mcto(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "MCTO is the Master Cumulative Time-out bit. (R/WC)"]
            #[inline(always)]
            pub fn set_mcto(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "SCTO is the Slave Cumulative Time-out bit(R/WC)"]
            #[inline(always)]
            pub const fn scto(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "SCTO is the Slave Cumulative Time-out bit(R/WC)"]
            #[inline(always)]
            pub fn set_scto(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "CHDL is the clock high time-out detect bit(R/WC)"]
            #[inline(always)]
            pub const fn chdl(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "CHDL is the clock high time-out detect bit(R/WC)"]
            #[inline(always)]
            pub fn set_chdl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "CHDH is the bus idle time-out detect bit(R/WC)"]
            #[inline(always)]
            pub const fn chdh(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "CHDH is the bus idle time-out detect bit(R/WC)"]
            #[inline(always)]
            pub fn set_chdh(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "If this bit is 1, the BER bit in the Status register was set while either the Slave state machine or the Master state machine was not in the Idle state.(R/WC)"]
            #[inline(always)]
            pub const fn ber(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "If this bit is 1, the BER bit in the Status register was set while either the Slave state machine or the Master state machine was not in the Idle state.(R/WC)"]
            #[inline(always)]
            pub fn set_ber(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "If this bit is 1, the LAB bit in the Status register was set while either the Slave state machine or the Master state machine was not in the Idle state.(R/WC)"]
            #[inline(always)]
            pub const fn lab(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "If this bit is 1, the LAB bit in the Status register was set while either the Slave state machine or the Master state machine was not in the Idle state.(R/WC)"]
            #[inline(always)]
            pub fn set_lab(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "If this bit is 1, the Slave state machine sent a NACK to the transmitting Master while the Slave was receiving data from the SMBus interface."]
            #[inline(always)]
            pub const fn snakr(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "If this bit is 1, the Slave state machine sent a NACK to the transmitting Master while the Slave was receiving data from the SMBus interface."]
            #[inline(always)]
            pub fn set_snakr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "0: Slave has just finished the receive phase of a transaction. 1: Slave has just finished the transmit phase of a transaction."]
            #[inline(always)]
            pub const fn str(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "0: Slave has just finished the receive phase of a transaction. 1: Slave has just finished the transmit phase of a transaction."]
            #[inline(always)]
            pub fn set_str(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "If this bit is 1, the Slave State Machine stopped because it detected a Repeat Start bit, with bit\\[0\\] of the byte containing the slave address equal to 1, indicating that the Master requested a Read operation."]
            #[inline(always)]
            pub const fn rep_rd(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "If this bit is 1, the Slave State Machine stopped because it detected a Repeat Start bit, with bit\\[0\\] of the byte containing the slave address equal to 1, indicating that the Master requested a Read operation."]
            #[inline(always)]
            pub fn set_rep_rd(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "If this bit is 1, the Slave State Machine stopped because it detected a Repeat Start bit, with bit\\[0\\] of the byte containing the slave address equal to 0, indicating that the Master requested a Write operation."]
            #[inline(always)]
            pub const fn rep_wr(&self) -> bool {
                let val = (self.0 >> 21usize) & 0x01;
                val != 0
            }
            #[doc = "If this bit is 1, the Slave State Machine stopped because it detected a Repeat Start bit, with bit\\[0\\] of the byte containing the slave address equal to 0, indicating that the Master requested a Write operation."]
            #[inline(always)]
            pub fn set_rep_wr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
            }
            #[doc = "If this bit is 1, the Master state machine received a NACK from the receiving Slave while the Master was transmitting data over the SMBus interface. (R/WC)"]
            #[inline(always)]
            pub const fn mnakx(&self) -> bool {
                let val = (self.0 >> 24usize) & 0x01;
                val != 0
            }
            #[doc = "If this bit is 1, the Master state machine received a NACK from the receiving Slave while the Master was transmitting data over the SMBus interface. (R/WC)"]
            #[inline(always)]
            pub fn set_mnakx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
            }
            #[doc = "0: Master has just finished the receive phase of a transaction. 1: Master has just finished the transmit phase of a transaction."]
            #[inline(always)]
            pub const fn mtr(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[doc = "0: Master has just finished the receive phase of a transaction. 1: Master has just finished the transmit phase of a transaction."]
            #[inline(always)]
            pub fn set_mtr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
            }
            #[doc = "This bit is set when the I2C bus becomes idle (on the rising edge of nBB). (R/WC)"]
            #[inline(always)]
            pub const fn idle(&self) -> bool {
                let val = (self.0 >> 29usize) & 0x01;
                val != 0
            }
            #[doc = "This bit is set when the I2C bus becomes idle (on the rising edge of nBB). (R/WC)"]
            #[inline(always)]
            pub fn set_idle(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
            }
            #[doc = "If this bit is 1, Master State machine completed operation and returned to the Idle state. It is cleared when written with a 1. Writes of a 0 have no effect. (R/WC)"]
            #[inline(always)]
            pub const fn mdone(&self) -> bool {
                let val = (self.0 >> 30usize) & 0x01;
                val != 0
            }
            #[doc = "If this bit is 1, Master State machine completed operation and returned to the Idle state. It is cleared when written with a 1. Writes of a 0 have no effect. (R/WC)"]
            #[inline(always)]
            pub fn set_mdone(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
            }
            #[doc = "If this bit is 1, Slave State machine completed operation and returned to the Idle state. It is cleared when written with a 1. Writes of a 0 have no effect.(R/WC)"]
            #[inline(always)]
            pub const fn sdone(&self) -> bool {
                let val = (self.0 >> 31usize) & 0x01;
                val != 0
            }
            #[doc = "If this bit is 1, Slave State machine completed operation and returned to the Idle state. It is cleared when written with a 1. Writes of a 0 have no effect.(R/WC)"]
            #[inline(always)]
            pub fn set_sdone(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
            }
        }
        impl Default for Compl {
            #[inline(always)]
            fn default() -> Compl {
                Compl(0)
            }
        }
        impl core::fmt::Debug for Compl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Compl")
                    .field("dten", &self.dten())
                    .field("mcen", &self.mcen())
                    .field("scen", &self.scen())
                    .field("biden", &self.biden())
                    .field("timerr", &self.timerr())
                    .field("dto", &self.dto())
                    .field("mcto", &self.mcto())
                    .field("scto", &self.scto())
                    .field("chdl", &self.chdl())
                    .field("chdh", &self.chdh())
                    .field("ber", &self.ber())
                    .field("lab", &self.lab())
                    .field("snakr", &self.snakr())
                    .field("str", &self.str())
                    .field("rep_rd", &self.rep_rd())
                    .field("rep_wr", &self.rep_wr())
                    .field("mnakx", &self.mnakx())
                    .field("mtr", &self.mtr())
                    .field("idle", &self.idle())
                    .field("mdone", &self.mdone())
                    .field("sdone", &self.sdone())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Compl {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Compl {{ dten: {=bool:?}, mcen: {=bool:?}, scen: {=bool:?}, biden: {=bool:?}, timerr: {=bool:?}, dto: {=bool:?}, mcto: {=bool:?}, scto: {=bool:?}, chdl: {=bool:?}, chdh: {=bool:?}, ber: {=bool:?}, lab: {=bool:?}, snakr: {=bool:?}, str: {=bool:?}, rep_rd: {=bool:?}, rep_wr: {=bool:?}, mnakx: {=bool:?}, mtr: {=bool:?}, idle: {=bool:?}, mdone: {=bool:?}, sdone: {=bool:?} }}" , self . dten () , self . mcen () , self . scen () , self . biden () , self . timerr () , self . dto () , self . mcto () , self . scto () , self . chdl () , self . chdh () , self . ber () , self . lab () , self . snakr () , self . str () , self . rep_rd () , self . rep_wr () , self . mnakx () , self . mtr () , self . idle () , self . mdone () , self . sdone ())
            }
        }
        #[doc = "Data Timing Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Datatm(pub u32);
        impl Datatm {
            #[doc = "The Data Hold \\[7:0\\] timer determines the SDAT hold time following SCLK driven low."]
            #[inline(always)]
            pub const fn data_hold(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "The Data Hold \\[7:0\\] timer determines the SDAT hold time following SCLK driven low."]
            #[inline(always)]
            pub fn set_data_hold(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "The Restart Setup \\[7:0\\] timer determines the SDAT setup time from the rising edge of SCLK for a repeated START condition."]
            #[inline(always)]
            pub const fn restart_setup(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "The Restart Setup \\[7:0\\] timer determines the SDAT setup time from the rising edge of SCLK for a repeated START condition."]
            #[inline(always)]
            pub fn set_restart_setup(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
            #[doc = "The Stop Setup \\[7:0\\] timer determines the SDAT setup time from the rising edge of SCLK for a STOP condition."]
            #[inline(always)]
            pub const fn stop_setup(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0xff;
                val as u8
            }
            #[doc = "The Stop Setup \\[7:0\\] timer determines the SDAT setup time from the rising edge of SCLK for a STOP condition."]
            #[inline(always)]
            pub fn set_stop_setup(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
            }
            #[doc = "This field determines the SCL hold time following SDA driven low during the first START bit in a transfer. It is the parameter THD:STA in the I2C Specification for an initial START bit. Repeated START hold time is determined by the Repeated START Hold Time Register."]
            #[inline(always)]
            pub const fn first_start_hold(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0xff;
                val as u8
            }
            #[doc = "This field determines the SCL hold time following SDA driven low during the first START bit in a transfer. It is the parameter THD:STA in the I2C Specification for an initial START bit. Repeated START hold time is determined by the Repeated START Hold Time Register."]
            #[inline(always)]
            pub fn set_first_start_hold(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
            }
        }
        impl Default for Datatm {
            #[inline(always)]
            fn default() -> Datatm {
                Datatm(0)
            }
        }
        impl core::fmt::Debug for Datatm {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Datatm")
                    .field("data_hold", &self.data_hold())
                    .field("restart_setup", &self.restart_setup())
                    .field("stop_setup", &self.stop_setup())
                    .field("first_start_hold", &self.first_start_hold())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Datatm {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Datatm {{ data_hold: {=u8:?}, restart_setup: {=u8:?}, stop_setup: {=u8:?}, first_start_hold: {=u8:?} }}" , self . data_hold () , self . restart_setup () , self . stop_setup () , self . first_start_hold ())
            }
        }
        #[doc = "Own Address Register Note that the Data Register and Own Address fields are offset by one bit, so that programming Own Address 1 with a value of 55h will result in the value AAh being recognized as the SMB Controller Core slave address."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct OwnAddr(pub u32);
        impl OwnAddr {
            #[doc = "The Own Address 1 bits configure one of the two addresses to which the SMB Controller Core will respond when addressed as a slave."]
            #[inline(always)]
            pub const fn addr1(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x7f;
                val as u8
            }
            #[doc = "The Own Address 1 bits configure one of the two addresses to which the SMB Controller Core will respond when addressed as a slave."]
            #[inline(always)]
            pub fn set_addr1(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
            }
            #[doc = "The Own Address 2 bits configure one of the two addresses to which the SMB Controller Core will respond when addressed as a slave."]
            #[inline(always)]
            pub const fn addr2(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0x7f;
                val as u8
            }
            #[doc = "The Own Address 2 bits configure one of the two addresses to which the SMB Controller Core will respond when addressed as a slave."]
            #[inline(always)]
            pub fn set_addr2(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
            }
        }
        impl Default for OwnAddr {
            #[inline(always)]
            fn default() -> OwnAddr {
                OwnAddr(0)
            }
        }
        impl core::fmt::Debug for OwnAddr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("OwnAddr")
                    .field("addr1", &self.addr1())
                    .field("addr2", &self.addr2())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for OwnAddr {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "OwnAddr {{ addr1: {=u8:?}, addr2: {=u8:?} }}",
                    self.addr1(),
                    self.addr2()
                )
            }
        }
        #[doc = "This is the Promiscuous Control Register. This register is functional only in Promiscuous mode."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct PrmCtrl(pub u8);
        impl PrmCtrl {
            #[doc = "This is the Promiscuous ACK / NAK response register. This bit has no effect in Normal mode of operation. 1= ACK the address byte. 0= NAK the address byte."]
            #[inline(always)]
            pub const fn ack_nak(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This is the Promiscuous ACK / NAK response register. This bit has no effect in Normal mode of operation. 1= ACK the address byte. 0= NAK the address byte."]
            #[inline(always)]
            pub fn set_ack_nak(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
        }
        impl Default for PrmCtrl {
            #[inline(always)]
            fn default() -> PrmCtrl {
                PrmCtrl(0)
            }
        }
        impl core::fmt::Debug for PrmCtrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("PrmCtrl").field("ack_nak", &self.ack_nak()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for PrmCtrl {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "PrmCtrl {{ ack_nak: {=bool:?} }}", self.ack_nak())
            }
        }
        #[doc = "This is the Promiscuous Interrupt Enable Register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct PrmIen(pub u8);
        impl PrmIen {
            #[doc = "This is the Promiscuous interrupt enable register. 1= Interrupt Enable. 0= Interrupt Disabled."]
            #[inline(always)]
            pub const fn addr(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This is the Promiscuous interrupt enable register. 1= Interrupt Enable. 0= Interrupt Disabled."]
            #[inline(always)]
            pub fn set_addr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
        }
        impl Default for PrmIen {
            #[inline(always)]
            fn default() -> PrmIen {
                PrmIen(0)
            }
        }
        impl core::fmt::Debug for PrmIen {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("PrmIen").field("addr", &self.addr()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for PrmIen {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "PrmIen {{ addr: {=bool:?} }}", self.addr())
            }
        }
        #[doc = "This is the Promiscuous Interrupt Register. This register bit will be functional only in Promiscuous mode."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct PrmSts(pub u8);
        impl PrmSts {
            #[doc = "This is the Promiscuous Address Status interrupt. This bit is functional only in Promiscuous mode. This bit is set on the 8th clock of the I2C Address. This bit will hold the Clock line low till this register bit is cleared. In other words, Software should read the address program ACK/NAK and clear this bit for the ACK/NAK clock to appear on the I2C clock (SCL) line. This is Write 1 to clear. R/W1C."]
            #[inline(always)]
            pub const fn addr_intr(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This is the Promiscuous Address Status interrupt. This bit is functional only in Promiscuous mode. This bit is set on the 8th clock of the I2C Address. This bit will hold the Clock line low till this register bit is cleared. In other words, Software should read the address program ACK/NAK and clear this bit for the ACK/NAK clock to appear on the I2C clock (SCL) line. This is Write 1 to clear. R/W1C."]
            #[inline(always)]
            pub fn set_addr_intr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
        }
        impl Default for PrmSts {
            #[inline(always)]
            fn default() -> PrmSts {
                PrmSts(0)
            }
        }
        impl core::fmt::Debug for PrmSts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("PrmSts").field("addr_intr", &self.addr_intr()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for PrmSts {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "PrmSts {{ addr_intr: {=bool:?} }}", self.addr_intr())
            }
        }
        #[doc = "Repeated Start Hold Time Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rshtm(pub u32);
        impl Rshtm {
            #[doc = "This is the value of the timing requirement tHd:Sta in the I2C specification for a repeated START bit. This is used to hold the clock until the Hold Time for the repeated Start Bit has been satisfied."]
            #[inline(always)]
            pub const fn rshtm(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "This is the value of the timing requirement tHd:Sta in the I2C specification for a repeated START bit. This is used to hold the clock until the Hold Time for the repeated Start Bit has been satisfied."]
            #[inline(always)]
            pub fn set_rshtm(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Rshtm {
            #[inline(always)]
            fn default() -> Rshtm {
                Rshtm(0)
            }
        }
        impl core::fmt::Debug for Rshtm {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Rshtm").field("rshtm", &self.rshtm()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rshtm {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Rshtm {{ rshtm: {=u8:?} }}", self.rshtm())
            }
        }
        #[doc = "Status Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rsts(pub u32);
        impl Rsts {
            #[doc = "The Bus Busy bit (NBB) is a read-only flag indicating when the bus is in use. A zero indicates that the bus is busy and access is not possible."]
            #[inline(always)]
            pub const fn nbb(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "The Bus Busy bit (NBB) is a read-only flag indicating when the bus is in use. A zero indicates that the bus is busy and access is not possible."]
            #[inline(always)]
            pub fn set_nbb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "The Lost Arbitration Bit (LAB) is set when, in multi-master operation, arbitration is lost to another master on the bus"]
            #[inline(always)]
            pub const fn lab(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "The Lost Arbitration Bit (LAB) is set when, in multi-master operation, arbitration is lost to another master on the bus"]
            #[inline(always)]
            pub fn set_lab(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "The Addressed As Slave bit (AAS) is valid only when PIN is asserted ('0'). When acting as slave, AAS is set when an incoming address over the bus matches the value in the Own Address Register or if the 'general call' address (00h) has been received"]
            #[inline(always)]
            pub const fn aas(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "The Addressed As Slave bit (AAS) is valid only when PIN is asserted ('0'). When acting as slave, AAS is set when an incoming address over the bus matches the value in the Own Address Register or if the 'general call' address (00h) has been received"]
            #[inline(always)]
            pub fn set_aas(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "The (Last Received Bit) or (Address 0) (general call) bit (LRB/AD0) serves a dual function and is valid only while the PIN bit is asserted ('0'). When the AAS bit is not asserted ('0') (i.e., not addressed as a slave), the LRB/AD0 holds the value of the last received bit over the bus. When the AAS bit is asserted ('1') (i.e., addressed as slave), the SMB Controller Core has been addressed as a slave."]
            #[inline(always)]
            pub const fn lrb_ad0(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "The (Last Received Bit) or (Address 0) (general call) bit (LRB/AD0) serves a dual function and is valid only while the PIN bit is asserted ('0'). When the AAS bit is not asserted ('0') (i.e., not addressed as a slave), the LRB/AD0 holds the value of the last received bit over the bus. When the AAS bit is asserted ('1') (i.e., addressed as slave), the SMB Controller Core has been addressed as a slave."]
            #[inline(always)]
            pub fn set_lrb_ad0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "When Bus Error (BER) is asserted, a misplaced START or STOP condition or Bus Time-Outs have been detected."]
            #[inline(always)]
            pub const fn ber(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "When Bus Error (BER) is asserted, a misplaced START or STOP condition or Bus Time-Outs have been detected."]
            #[inline(always)]
            pub fn set_ber(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "When in slave receiver mode, STS is asserted ('1') when an externally generated STOP condition is detected. Note that STS is used only in slave receiver mode."]
            #[inline(always)]
            pub const fn sts(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "When in slave receiver mode, STS is asserted ('1') when an externally generated STOP condition is detected. Note that STS is used only in slave receiver mode."]
            #[inline(always)]
            pub fn set_sts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "SMBus Address Decoded (SAD)"]
            #[inline(always)]
            pub const fn sad(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "SMBus Address Decoded (SAD)"]
            #[inline(always)]
            pub fn set_sad(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Pending Interrupt bit"]
            #[inline(always)]
            pub const fn pin(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Pending Interrupt bit"]
            #[inline(always)]
            pub fn set_pin(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
        }
        impl Default for Rsts {
            #[inline(always)]
            fn default() -> Rsts {
                Rsts(0)
            }
        }
        impl core::fmt::Debug for Rsts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Rsts")
                    .field("nbb", &self.nbb())
                    .field("lab", &self.lab())
                    .field("aas", &self.aas())
                    .field("lrb_ad0", &self.lrb_ad0())
                    .field("ber", &self.ber())
                    .field("sts", &self.sts())
                    .field("sad", &self.sad())
                    .field("pin", &self.pin())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rsts {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Rsts {{ nbb: {=bool:?}, lab: {=bool:?}, aas: {=bool:?}, lrb_ad0: {=bool:?}, ber: {=bool:?}, sts: {=bool:?}, sad: {=bool:?}, pin: {=bool:?} }}" , self . nbb () , self . lab () , self . aas () , self . lrb_ad0 () , self . ber () , self . sts () , self . sad () , self . pin ())
            }
        }
        #[doc = "This is the Slave Address Register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SlvAddr(pub u8);
        impl SlvAddr {
            #[doc = "This register stores the value of the Slave Address and Direction Bit from the 1st byte of a Slave transfer. This is loaded immediately upon receipt of the address byte, before the ACK/NAK 9th clock."]
            #[inline(always)]
            pub const fn saddr(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "This register stores the value of the Slave Address and Direction Bit from the 1st byte of a Slave transfer. This is loaded immediately upon receipt of the address byte, before the ACK/NAK 9th clock."]
            #[inline(always)]
            pub fn set_saddr(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
            }
        }
        impl Default for SlvAddr {
            #[inline(always)]
            fn default() -> SlvAddr {
                SlvAddr(0)
            }
        }
        impl core::fmt::Debug for SlvAddr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("SlvAddr").field("saddr", &self.saddr()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for SlvAddr {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "SlvAddr {{ saddr: {=u8:?} }}", self.saddr())
            }
        }
        #[doc = "Time-Out Scaling Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Tmoutsc(pub u32);
        impl Tmoutsc {
            #[doc = "Bus Idle Minimum time = Bus Idle Min \\[7:0\\] x Baud_Clock_Period"]
            #[inline(always)]
            pub const fn bus_idle_min(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0xff;
                val as u8
            }
            #[doc = "Bus Idle Minimum time = Bus Idle Min \\[7:0\\] x Baud_Clock_Period"]
            #[inline(always)]
            pub fn set_bus_idle_min(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
            }
        }
        impl Default for Tmoutsc {
            #[inline(always)]
            fn default() -> Tmoutsc {
                Tmoutsc(0)
            }
        }
        impl core::fmt::Debug for Tmoutsc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Tmoutsc")
                    .field("bus_idle_min", &self.bus_idle_min())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Tmoutsc {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Tmoutsc {{ bus_idle_min: {=u8:?} }}", self.bus_idle_min())
            }
        }
        #[doc = "WAKE ENABLE Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct WakeEn(pub u32);
        impl WakeEn {
            #[doc = "Enable Start Bit Detection Interrupt. The Start Bit Detection Interrupt is wake-capable. 1=Start Bit Detection Interrupt enabled. 0=Start Bit Detection Interrupt disabled."]
            #[inline(always)]
            pub const fn start_det_int_en(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable Start Bit Detection Interrupt. The Start Bit Detection Interrupt is wake-capable. 1=Start Bit Detection Interrupt enabled. 0=Start Bit Detection Interrupt disabled."]
            #[inline(always)]
            pub fn set_start_det_int_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for WakeEn {
            #[inline(always)]
            fn default() -> WakeEn {
                WakeEn(0)
            }
        }
        impl core::fmt::Debug for WakeEn {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("WakeEn")
                    .field("start_det_int_en", &self.start_det_int_en())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for WakeEn {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "WakeEn {{ start_det_int_en: {=bool:?} }}", self.start_det_int_en())
            }
        }
        #[doc = "WAKE STATUS Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct WakeSts(pub u32);
        impl WakeSts {
            #[doc = "This bit is set to '1' when a START bit is detected while the controller is enabled. This bit is cleared to '0' when written with a '1'. Writes of '0' have no effect. (R/WC)"]
            #[inline(always)]
            pub const fn start_bit_det(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This bit is set to '1' when a START bit is detected while the controller is enabled. This bit is cleared to '0' when written with a '1'. Writes of '0' have no effect. (R/WC)"]
            #[inline(always)]
            pub fn set_start_bit_det(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for WakeSts {
            #[inline(always)]
            fn default() -> WakeSts {
                WakeSts(0)
            }
        }
        impl core::fmt::Debug for WakeSts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("WakeSts")
                    .field("start_bit_det", &self.start_bit_det())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for WakeSts {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "WakeSts {{ start_bit_det: {=bool:?} }}", self.start_bit_det())
            }
        }
        #[doc = "Control Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Wctrl(pub u32);
        impl Wctrl {
            #[doc = "The Acknowledge bit (ACK) must normally be asserted ('1'). This causes the controller to send an acknowledge automatically after each byte (this occurs during the 9th clock pulse). The ACK bit must not be asserted ('0') when the controller is operating in master/receiver mode and requires no further data to be sent from the slave transmitter. This causes a negative acknowledge on the I2C bus, which halts further transmission from the slave device."]
            #[inline(always)]
            pub const fn ack(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "The Acknowledge bit (ACK) must normally be asserted ('1'). This causes the controller to send an acknowledge automatically after each byte (this occurs during the 9th clock pulse). The ACK bit must not be asserted ('0') when the controller is operating in master/receiver mode and requires no further data to be sent from the slave transmitter. This causes a negative acknowledge on the I2C bus, which halts further transmission from the slave device."]
            #[inline(always)]
            pub fn set_ack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "See STA description"]
            #[inline(always)]
            pub const fn sto(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "See STA description"]
            #[inline(always)]
            pub fn set_sto(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "The STA and STO bits control the generation of the I2C Start condition and the transmission of the Slave Address and R/nW bit (from the Data Register), generation of repeated Start condition, and generation of the Stop condition"]
            #[inline(always)]
            pub const fn sta(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "The STA and STO bits control the generation of the I2C Start condition and the transmission of the Slave Address and R/nW bit (from the Data Register), generation of repeated Start condition, and generation of the Stop condition"]
            #[inline(always)]
            pub fn set_sta(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable Interrupt bit (ENI) controls the Interrupt Interface"]
            #[inline(always)]
            pub const fn eni(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable Interrupt bit (ENI) controls the Interrupt Interface"]
            #[inline(always)]
            pub fn set_eni(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "The Enable Serial Output bit (ESO) enables and disables the SMB Controller Core serial data output (SDAT)"]
            #[inline(always)]
            pub const fn eso(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "The Enable Serial Output bit (ESO) enables and disables the SMB Controller Core serial data output (SDAT)"]
            #[inline(always)]
            pub fn set_eso(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "The Pending Interrupt Not (PIN) bit serves as a software reset function. Writing the PIN bit to a logic '1' de-asserts all status bits except for the nBB bit which is not affected by the PIN bit. The PIN bit is a self-clearing bit. Writing this bit to a logic '0' has no effect."]
            #[inline(always)]
            pub const fn pin(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "The Pending Interrupt Not (PIN) bit serves as a software reset function. Writing the PIN bit to a logic '1' de-asserts all status bits except for the nBB bit which is not affected by the PIN bit. The PIN bit is a self-clearing bit. Writing this bit to a logic '0' has no effect."]
            #[inline(always)]
            pub fn set_pin(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
        }
        impl Default for Wctrl {
            #[inline(always)]
            fn default() -> Wctrl {
                Wctrl(0)
            }
        }
        impl core::fmt::Debug for Wctrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Wctrl")
                    .field("ack", &self.ack())
                    .field("sto", &self.sto())
                    .field("sta", &self.sta())
                    .field("eni", &self.eni())
                    .field("eso", &self.eso())
                    .field("pin", &self.pin())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Wctrl {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Wctrl {{ ack: {=bool:?}, sto: {=bool:?}, sta: {=bool:?}, eni: {=bool:?}, eso: {=bool:?}, pin: {=bool:?} }}" , self . ack () , self . sto () , self . sta () , self . eni () , self . eso () , self . pin ())
            }
        }
    }
}
pub mod led0 {
    #[doc = "The LED is implemented using a PWM that can be driven either by the 48 MHz clock or by a 32.768 KHz clock input."]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Led0 {
        ptr: *mut u8,
    }
    unsafe impl Send for Led0 {}
    unsafe impl Sync for Led0 {}
    impl Led0 {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "LED Configuration"]
        #[inline(always)]
        pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "LED Limits This register may be written at any time. Values written into the register are held in an holding register, which is transferred into the actual register at the end of a PWM period. The two byte fields may be written independently. Reads of this register return the current contents and not the value of the holding register."]
        #[inline(always)]
        pub const fn limit(self) -> crate::common::Reg<regs::Limit, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "LED Delay"]
        #[inline(always)]
        pub const fn dly(self) -> crate::common::Reg<regs::Dly, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "This register has eight segment fields which provide the amount the current duty cycle is adjusted at the end of every PWM period. Segment field selection is decoded based on the segment index. The segment index equation utilized depends on the SYMMETRY bit in the LED Configuration Register Register) . In Symmetric Mode the Segment_Index\\[2:0\\] = Duty Cycle Bits\\[7:5\\] . In Asymmetric Mode the Segment_Index\\[2:0\\] is the bit concatenation of following: Segment_Index\\[2\\] = (FALLING RAMP TIME in Figure 30-3, Clipping Example) and Segment_Index\\[1:0\\] = Duty Cycle Bits\\[7:6\\]."]
        #[inline(always)]
        pub const fn step(self) -> crate::common::Reg<regs::Step, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "LED Update Interval"]
        #[inline(always)]
        pub const fn intrvl(self) -> crate::common::Reg<regs::Intrvl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "LED Output Delay"]
        #[inline(always)]
        pub const fn outdly(self) -> crate::common::Reg<regs::Outdly, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "LED Configuration"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cfg(pub u32);
        impl Cfg {
            #[doc = "CONTROL 3=PWM is always on 2=LED blinking (standard PWM) 1=LED breathing configuration 0=PWM is always off. All internal registers and counters are reset to 0. Clocks are gated"]
            #[inline(always)]
            pub const fn ctrl(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x03;
                val as u8
            }
            #[doc = "CONTROL 3=PWM is always on 2=LED blinking (standard PWM) 1=LED breathing configuration 0=PWM is always off. All internal registers and counters are reset to 0. Clocks are gated"]
            #[inline(always)]
            pub fn set_ctrl(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
            }
            #[doc = "1=Clock source is the 48 MHz clock, 0=Clock source is the 32.768 KHz clock"]
            #[inline(always)]
            pub const fn clk_src(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "1=Clock source is the 48 MHz clock, 0=Clock source is the 32.768 KHz clock"]
            #[inline(always)]
            pub fn set_clk_src(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "SYNCHRONIZE When this bit is '1', all counters for all LEDs are reset to their initial values. When this bit is '0' in the LED Configuration Register for all LEDs, then all counters for LEDs that are configured to blink or breathe will increment or decrement, as required."]
            #[inline(always)]
            pub const fn synch(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "SYNCHRONIZE When this bit is '1', all counters for all LEDs are reset to their initial values. When this bit is '0' in the LED Configuration Register for all LEDs, then all counters for LEDs that are configured to blink or breathe will increment or decrement, as required."]
            #[inline(always)]
            pub fn set_synch(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "PWM_SIZE This bit controls the behavior of PWM: 3=Reserved 2=PWM is configured as a 6-bit PWM 1=PWM is configured as a 7-bit PWM 0=PWM is configured as an 8-bit PWM"]
            #[inline(always)]
            pub const fn pwm_size(&self) -> u8 {
                let val = (self.0 >> 4usize) & 0x03;
                val as u8
            }
            #[doc = "PWM_SIZE This bit controls the behavior of PWM: 3=Reserved 2=PWM is configured as a 6-bit PWM 1=PWM is configured as a 7-bit PWM 0=PWM is configured as an 8-bit PWM"]
            #[inline(always)]
            pub fn set_pwm_size(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
            }
            #[doc = "ENABLE_UPDATE This bit is set to 1 when written with a '1'. Writes of '0' have no effect. Hardware clears this bit to 0 when the breathing configuration registers are updated at the end of a PWM period. The current state of the bit is readable any time."]
            #[inline(always)]
            pub const fn en_update(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "ENABLE_UPDATE This bit is set to 1 when written with a '1'. Writes of '0' have no effect. Hardware clears this bit to 0 when the breathing configuration registers are updated at the end of a PWM period. The current state of the bit is readable any time."]
            #[inline(always)]
            pub fn set_en_update(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "RESET Writes of '1' to this bit resets the PWM registers to their default values. This bit is self clearing. Writes of '0' to this bit have no effect."]
            #[inline(always)]
            pub const fn rst(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "RESET Writes of '1' to this bit resets the PWM registers to their default values. This bit is self clearing. Writes of '0' to this bit have no effect."]
            #[inline(always)]
            pub fn set_rst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "WDT_RELOAD The PWM Watchdog Timer counter reload value. On system reset, it defaults to 14h, which corresponds to a 4 second Watchdog timeout value."]
            #[inline(always)]
            pub const fn wdt_reload(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "WDT_RELOAD The PWM Watchdog Timer counter reload value. On system reset, it defaults to 14h, which corresponds to a 4 second Watchdog timeout value."]
            #[inline(always)]
            pub fn set_wdt_reload(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
            #[doc = "SYMMETRY 1=The rising and falling ramp times are in Asymmetric mode. 0=The rising and falling ramp times are in Symmetric mode."]
            #[inline(always)]
            pub const fn symmetry(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "SYMMETRY 1=The rising and falling ramp times are in Asymmetric mode. 0=The rising and falling ramp times are in Symmetric mode."]
            #[inline(always)]
            pub fn set_symmetry(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
        }
        impl Default for Cfg {
            #[inline(always)]
            fn default() -> Cfg {
                Cfg(0)
            }
        }
        impl core::fmt::Debug for Cfg {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Cfg")
                    .field("ctrl", &self.ctrl())
                    .field("clk_src", &self.clk_src())
                    .field("synch", &self.synch())
                    .field("pwm_size", &self.pwm_size())
                    .field("en_update", &self.en_update())
                    .field("rst", &self.rst())
                    .field("wdt_reload", &self.wdt_reload())
                    .field("symmetry", &self.symmetry())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Cfg {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Cfg {{ ctrl: {=u8:?}, clk_src: {=bool:?}, synch: {=bool:?}, pwm_size: {=u8:?}, en_update: {=bool:?}, rst: {=bool:?}, wdt_reload: {=u8:?}, symmetry: {=bool:?} }}" , self . ctrl () , self . clk_src () , self . synch () , self . pwm_size () , self . en_update () , self . rst () , self . wdt_reload () , self . symmetry ())
            }
        }
        #[doc = "LED Delay"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dly(pub u32);
        impl Dly {
            #[doc = "The number of PWM periods to wait before updating the current duty cycle when the current duty cycle is greater than or equal to the value MIN in register LED_LIMIT."]
            #[inline(always)]
            pub const fn low_pulse(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "The number of PWM periods to wait before updating the current duty cycle when the current duty cycle is greater than or equal to the value MIN in register LED_LIMIT."]
            #[inline(always)]
            pub fn set_low_pulse(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
            #[doc = "In breathing mode, the number of PWM periods to wait before updating the current duty cycle when the current duty cycle is greater than or equal to the value MAX in register LED_LIMIT."]
            #[inline(always)]
            pub const fn high_pulse(&self) -> u16 {
                let val = (self.0 >> 12usize) & 0x0fff;
                val as u16
            }
            #[doc = "In breathing mode, the number of PWM periods to wait before updating the current duty cycle when the current duty cycle is greater than or equal to the value MAX in register LED_LIMIT."]
            #[inline(always)]
            pub fn set_high_pulse(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
            }
        }
        impl Default for Dly {
            #[inline(always)]
            fn default() -> Dly {
                Dly(0)
            }
        }
        impl core::fmt::Debug for Dly {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Dly")
                    .field("low_pulse", &self.low_pulse())
                    .field("high_pulse", &self.high_pulse())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Dly {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Dly {{ low_pulse: {=u16:?}, high_pulse: {=u16:?} }}",
                    self.low_pulse(),
                    self.high_pulse()
                )
            }
        }
        #[doc = "LED Update Interval"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Intrvl(pub u32);
        impl Intrvl {
            #[doc = "The number of PWM periods between updates to current duty cycle when the segment index is equal to 000b."]
            #[inline(always)]
            pub const fn i0(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "The number of PWM periods between updates to current duty cycle when the segment index is equal to 000b."]
            #[inline(always)]
            pub fn set_i0(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
            #[doc = "The number of PWM periods between updates to current duty cycle when the segment index is equal to 001b."]
            #[inline(always)]
            pub const fn i1(&self) -> u8 {
                let val = (self.0 >> 4usize) & 0x0f;
                val as u8
            }
            #[doc = "The number of PWM periods between updates to current duty cycle when the segment index is equal to 001b."]
            #[inline(always)]
            pub fn set_i1(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
            }
            #[doc = "The number of PWM periods between updates to current duty cycle when the segment index is equal to 010b."]
            #[inline(always)]
            pub const fn i2(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0x0f;
                val as u8
            }
            #[doc = "The number of PWM periods between updates to current duty cycle when the segment index is equal to 010b."]
            #[inline(always)]
            pub fn set_i2(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
            }
            #[doc = "The number of PWM periods between updates to current duty cycle when the segment index is equal to 011b."]
            #[inline(always)]
            pub const fn i3(&self) -> u8 {
                let val = (self.0 >> 12usize) & 0x0f;
                val as u8
            }
            #[doc = "The number of PWM periods between updates to current duty cycle when the segment index is equal to 011b."]
            #[inline(always)]
            pub fn set_i3(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
            }
            #[doc = "The number of PWM periods between updates to current duty cycle when the segment index is equal to 100b."]
            #[inline(always)]
            pub const fn i4(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0x0f;
                val as u8
            }
            #[doc = "The number of PWM periods between updates to current duty cycle when the segment index is equal to 100b."]
            #[inline(always)]
            pub fn set_i4(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
            }
            #[doc = "The number of PWM periods between updates to current duty cycle when the segment index is equal to 101b."]
            #[inline(always)]
            pub const fn i5(&self) -> u8 {
                let val = (self.0 >> 20usize) & 0x0f;
                val as u8
            }
            #[doc = "The number of PWM periods between updates to current duty cycle when the segment index is equal to 101b."]
            #[inline(always)]
            pub fn set_i5(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
            }
            #[doc = "The number of PWM periods between updates to current duty cycle when the segment index is equal to 110b."]
            #[inline(always)]
            pub const fn i6(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0x0f;
                val as u8
            }
            #[doc = "The number of PWM periods between updates to current duty cycle when the segment index is equal to 110b."]
            #[inline(always)]
            pub fn set_i6(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
            }
            #[doc = "The number of PWM periods between updates to current duty cycle when the segment index is equal to 111b."]
            #[inline(always)]
            pub const fn i7(&self) -> u8 {
                let val = (self.0 >> 28usize) & 0x0f;
                val as u8
            }
            #[doc = "The number of PWM periods between updates to current duty cycle when the segment index is equal to 111b."]
            #[inline(always)]
            pub fn set_i7(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
            }
        }
        impl Default for Intrvl {
            #[inline(always)]
            fn default() -> Intrvl {
                Intrvl(0)
            }
        }
        impl core::fmt::Debug for Intrvl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Intrvl")
                    .field("i0", &self.i0())
                    .field("i1", &self.i1())
                    .field("i2", &self.i2())
                    .field("i3", &self.i3())
                    .field("i4", &self.i4())
                    .field("i5", &self.i5())
                    .field("i6", &self.i6())
                    .field("i7", &self.i7())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Intrvl {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Intrvl {{ i0: {=u8:?}, i1: {=u8:?}, i2: {=u8:?}, i3: {=u8:?}, i4: {=u8:?}, i5: {=u8:?}, i6: {=u8:?}, i7: {=u8:?} }}" , self . i0 () , self . i1 () , self . i2 () , self . i3 () , self . i4 () , self . i5 () , self . i6 () , self . i7 ())
            }
        }
        #[doc = "LED Limits This register may be written at any time. Values written into the register are held in an holding register, which is transferred into the actual register at the end of a PWM period. The two byte fields may be written independently. Reads of this register return the current contents and not the value of the holding register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Limit(pub u32);
        impl Limit {
            #[doc = "In breathing mode, when the current duty cycle is less than or equal to this value the breathing apparatus holds the current duty cycle for the period specified by the field LD in register LED_DELAY, then starts incrementing the current duty cycle In blinking mode, this field defines the duty cycle of the blink function."]
            #[inline(always)]
            pub const fn min(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "In breathing mode, when the current duty cycle is less than or equal to this value the breathing apparatus holds the current duty cycle for the period specified by the field LD in register LED_DELAY, then starts incrementing the current duty cycle In blinking mode, this field defines the duty cycle of the blink function."]
            #[inline(always)]
            pub fn set_min(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "In breathing mode, when the current duty cycle is greater than or equal to this value the breathing apparatus holds the current duty cycle for the period specified by the field HD in register LED_DELAY, then starts decrementing the current duty cycle"]
            #[inline(always)]
            pub const fn max(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "In breathing mode, when the current duty cycle is greater than or equal to this value the breathing apparatus holds the current duty cycle for the period specified by the field HD in register LED_DELAY, then starts decrementing the current duty cycle"]
            #[inline(always)]
            pub fn set_max(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
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
                    .field("min", &self.min())
                    .field("max", &self.max())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Limit {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Limit {{ min: {=u8:?}, max: {=u8:?} }}", self.min(), self.max())
            }
        }
        #[doc = "LED Output Delay"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Outdly(pub u32);
        impl Outdly {
            #[doc = "The delay, in counts of the clock defined in Clock Source (CLKSRC), in which output transitions are delayed. When this field is 0, there is no added transition delay. When the LED is programmed to be Always On or Always Off, the Output Delay field has no effect."]
            #[inline(always)]
            pub const fn delay(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "The delay, in counts of the clock defined in Clock Source (CLKSRC), in which output transitions are delayed. When this field is 0, there is no added transition delay. When the LED is programmed to be Always On or Always Off, the Output Delay field has no effect."]
            #[inline(always)]
            pub fn set_delay(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Outdly {
            #[inline(always)]
            fn default() -> Outdly {
                Outdly(0)
            }
        }
        impl core::fmt::Debug for Outdly {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Outdly").field("delay", &self.delay()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Outdly {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Outdly {{ delay: {=u8:?} }}", self.delay())
            }
        }
        #[doc = "This register has eight segment fields which provide the amount the current duty cycle is adjusted at the end of every PWM period. Segment field selection is decoded based on the segment index. The segment index equation utilized depends on the SYMMETRY bit in the LED Configuration Register Register) . In Symmetric Mode the Segment_Index\\[2:0\\] = Duty Cycle Bits\\[7:5\\] . In Asymmetric Mode the Segment_Index\\[2:0\\] is the bit concatenation of following: Segment_Index\\[2\\] = (FALLING RAMP TIME in Figure 30-3, Clipping Example) and Segment_Index\\[1:0\\] = Duty Cycle Bits\\[7:6\\]."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Step(pub u32);
        impl Step {
            #[doc = "Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 000."]
            #[inline(always)]
            pub const fn s0(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 000."]
            #[inline(always)]
            pub fn set_s0(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
            #[doc = "Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 001."]
            #[inline(always)]
            pub const fn s1(&self) -> u8 {
                let val = (self.0 >> 4usize) & 0x0f;
                val as u8
            }
            #[doc = "Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 001."]
            #[inline(always)]
            pub fn set_s1(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
            }
            #[doc = "Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 010."]
            #[inline(always)]
            pub const fn s2(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0x0f;
                val as u8
            }
            #[doc = "Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 010."]
            #[inline(always)]
            pub fn set_s2(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
            }
            #[doc = "Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 011."]
            #[inline(always)]
            pub const fn s3(&self) -> u8 {
                let val = (self.0 >> 12usize) & 0x0f;
                val as u8
            }
            #[doc = "Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 011."]
            #[inline(always)]
            pub fn set_s3(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
            }
            #[doc = "Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 100."]
            #[inline(always)]
            pub const fn s4(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0x0f;
                val as u8
            }
            #[doc = "Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 100."]
            #[inline(always)]
            pub fn set_s4(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
            }
            #[doc = "Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 101"]
            #[inline(always)]
            pub const fn s5(&self) -> u8 {
                let val = (self.0 >> 20usize) & 0x0f;
                val as u8
            }
            #[doc = "Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 101"]
            #[inline(always)]
            pub fn set_s5(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
            }
            #[doc = "Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 110."]
            #[inline(always)]
            pub const fn s6(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0x0f;
                val as u8
            }
            #[doc = "Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 110."]
            #[inline(always)]
            pub fn set_s6(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
            }
            #[doc = "Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 111."]
            #[inline(always)]
            pub const fn s7(&self) -> u8 {
                let val = (self.0 >> 28usize) & 0x0f;
                val as u8
            }
            #[doc = "Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 111."]
            #[inline(always)]
            pub fn set_s7(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
            }
        }
        impl Default for Step {
            #[inline(always)]
            fn default() -> Step {
                Step(0)
            }
        }
        impl core::fmt::Debug for Step {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Step")
                    .field("s0", &self.s0())
                    .field("s1", &self.s1())
                    .field("s2", &self.s2())
                    .field("s3", &self.s3())
                    .field("s4", &self.s4())
                    .field("s5", &self.s5())
                    .field("s6", &self.s6())
                    .field("s7", &self.s7())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Step {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Step {{ s0: {=u8:?}, s1: {=u8:?}, s2: {=u8:?}, s3: {=u8:?}, s4: {=u8:?}, s5: {=u8:?}, s6: {=u8:?}, s7: {=u8:?} }}" , self . s0 () , self . s1 () , self . s2 () , self . s3 () , self . s4 () , self . s5 () , self . s6 () , self . s7 ())
            }
        }
    }
}
pub mod pcr {
    #[doc = "The Power, Clocks, and Resets (PCR) Section identifies clock sources, and reset inputs to the chip"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcr {
        ptr: *mut u8,
    }
    unsafe impl Send for Pcr {}
    unsafe impl Sync for Pcr {}
    impl Pcr {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "System Sleep Control"]
        #[inline(always)]
        pub const fn sys_slp_ctrl(self) -> crate::common::Reg<regs::SysSlpCtrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Processor Clock Control Register \\[7:0\\] Processor Clock Divide Value (PROC_DIV)"]
        #[inline(always)]
        pub const fn proc_clk_ctrl(self) -> crate::common::Reg<regs::ProcClkCtrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Configures the EC_CLK clock domain"]
        #[inline(always)]
        pub const fn slow_clk_ctrl(self) -> crate::common::Reg<regs::SlowClkCtrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Oscillator ID Register"]
        #[inline(always)]
        pub const fn osc_id(self) -> crate::common::Reg<regs::OscId, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "PCR Power Reset Status Register"]
        #[inline(always)]
        pub const fn pwr_rst_sts(self) -> crate::common::Reg<regs::PwrRstSts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "Power Reset Control Register"]
        #[inline(always)]
        pub const fn pwr_rst_ctrl(self) -> crate::common::Reg<regs::PwrRstCtrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "System Reset Register"]
        #[inline(always)]
        pub const fn sys_rst(self) -> crate::common::Reg<regs::SysRst, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
        }
        #[doc = "Sleep Enable 0 Register"]
        #[inline(always)]
        pub const fn slp_en_0(self) -> crate::common::Reg<regs::SlpEn0, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
        }
        #[doc = "Sleep Enable 1 Register"]
        #[inline(always)]
        pub const fn slp_en_1(self) -> crate::common::Reg<regs::SlpEn1, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
        }
        #[doc = "Sleep Enable 2 Register"]
        #[inline(always)]
        pub const fn slp_en_2(self) -> crate::common::Reg<regs::SlpEn2, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
        }
        #[doc = "Sleep Enable 3 Register"]
        #[inline(always)]
        pub const fn slp_en_3(self) -> crate::common::Reg<regs::SlpEn3, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
        }
        #[doc = "Sleep Enable 4 Register"]
        #[inline(always)]
        pub const fn slp_en_4(self) -> crate::common::Reg<regs::SlpEn4, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
        }
        #[doc = "Clock Required 0 Register"]
        #[inline(always)]
        pub const fn clk_req_0(self) -> crate::common::Reg<regs::ClkReq0, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
        }
        #[doc = "Clock Required 1 Register"]
        #[inline(always)]
        pub const fn clk_req_1(self) -> crate::common::Reg<regs::ClkReq1, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
        }
        #[doc = "Clock Required 2 Register"]
        #[inline(always)]
        pub const fn clk_req_2(self) -> crate::common::Reg<regs::ClkReq2, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
        }
        #[doc = "Clock Required 3 Register"]
        #[inline(always)]
        pub const fn clk_req_3(self) -> crate::common::Reg<regs::ClkReq3, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
        }
        #[doc = "Clock Required 4 Register"]
        #[inline(always)]
        pub const fn clk_req_4(self) -> crate::common::Reg<regs::ClkReq4, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
        }
        #[doc = "Reset Enable 0 Register"]
        #[inline(always)]
        pub const fn rst_en_0(self) -> crate::common::Reg<regs::RstEn0, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
        }
        #[doc = "Reset Enable 1 Register"]
        #[inline(always)]
        pub const fn rst_en_1(self) -> crate::common::Reg<regs::RstEn1, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
        }
        #[doc = "Reset Enable 2 Register"]
        #[inline(always)]
        pub const fn rst_en_2(self) -> crate::common::Reg<regs::RstEn2, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
        }
        #[doc = "Reset Enable 3 Register"]
        #[inline(always)]
        pub const fn rst_en_3(self) -> crate::common::Reg<regs::RstEn3, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
        }
        #[doc = "Reset Enable 4 Register"]
        #[inline(always)]
        pub const fn rst_en_4(self) -> crate::common::Reg<regs::RstEn4, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
        }
        #[doc = "LOCK Register"]
        #[inline(always)]
        pub const fn lock_reg(self) -> crate::common::Reg<regs::LockReg, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Clock Required 0 Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct ClkReq0(pub u32);
        impl ClkReq0 {
            #[doc = "JTAG STAP Enable"]
            #[inline(always)]
            pub const fn jtag_stap_clk_req(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "JTAG STAP Enable"]
            #[inline(always)]
            pub fn set_jtag_stap_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "OTP Enable"]
            #[inline(always)]
            pub const fn otp_clk_req(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "OTP Enable"]
            #[inline(always)]
            pub fn set_otp_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
        }
        impl Default for ClkReq0 {
            #[inline(always)]
            fn default() -> ClkReq0 {
                ClkReq0(0)
            }
        }
        impl core::fmt::Debug for ClkReq0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("ClkReq0")
                    .field("jtag_stap_clk_req", &self.jtag_stap_clk_req())
                    .field("otp_clk_req", &self.otp_clk_req())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for ClkReq0 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "ClkReq0 {{ jtag_stap_clk_req: {=bool:?}, otp_clk_req: {=bool:?} }}",
                    self.jtag_stap_clk_req(),
                    self.otp_clk_req()
                )
            }
        }
        #[doc = "Clock Required 1 Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct ClkReq1(pub u32);
        impl ClkReq1 {
            #[doc = "Interrupt Clock Required"]
            #[inline(always)]
            pub const fn int_clk_req(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Interrupt Clock Required"]
            #[inline(always)]
            pub fn set_int_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "TACH0 Clock Required (TACH0_CLK_REQ)"]
            #[inline(always)]
            pub const fn tach0_clk_req(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "TACH0 Clock Required (TACH0_CLK_REQ)"]
            #[inline(always)]
            pub fn set_tach0_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "PWM0 Clock Required (PWM0_CLK_REQ)"]
            #[inline(always)]
            pub const fn pwm0_clk_req(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "PWM0 Clock Required (PWM0_CLK_REQ)"]
            #[inline(always)]
            pub fn set_pwm0_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "PMC Clock Required (PMC_CLK_REQ)"]
            #[inline(always)]
            pub const fn pmc_clk_req(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "PMC Clock Required (PMC_CLK_REQ)"]
            #[inline(always)]
            pub fn set_pmc_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "DMA Clock Required (DMA_CLK_REQ)"]
            #[inline(always)]
            pub const fn dma_clk_req(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "DMA Clock Required (DMA_CLK_REQ)"]
            #[inline(always)]
            pub fn set_dma_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "TFDP Clock Required (TFDP_CLK_REQ)"]
            #[inline(always)]
            pub const fn tfdp_clk_req(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "TFDP Clock Required (TFDP_CLK_REQ)"]
            #[inline(always)]
            pub fn set_tfdp_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "PROCESSOR Clock Required (PROCESSOR_CLK_REQ)"]
            #[inline(always)]
            pub const fn proc_clk_req(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "PROCESSOR Clock Required (PROCESSOR_CLK_REQ)"]
            #[inline(always)]
            pub fn set_proc_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "WDT Clock Required (WDT_CLK_REQ)"]
            #[inline(always)]
            pub const fn wdt_clk_req(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "WDT Clock Required (WDT_CLK_REQ)"]
            #[inline(always)]
            pub fn set_wdt_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "SMB0 Clock Required (SMB0_CLK_REQ)"]
            #[inline(always)]
            pub const fn smb0_clk_req(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "SMB0 Clock Required (SMB0_CLK_REQ)"]
            #[inline(always)]
            pub fn set_smb0_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "TACH2 Clock Required (TACH2_CLK_REQ)"]
            #[inline(always)]
            pub const fn tach2_clk_req(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "TACH2 Clock Required (TACH2_CLK_REQ)"]
            #[inline(always)]
            pub fn set_tach2_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "PWM1 Clock Required (PWM1_CLK_REQ)"]
            #[inline(always)]
            pub const fn pwm1_clk_req(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "PWM1 Clock Required (PWM1_CLK_REQ)"]
            #[inline(always)]
            pub fn set_pwm1_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "PWM4 Clock Required (PWM4_CLK_REQ)"]
            #[inline(always)]
            pub const fn pwm4_clk_req(&self) -> bool {
                let val = (self.0 >> 23usize) & 0x01;
                val != 0
            }
            #[doc = "PWM4 Clock Required (PWM4_CLK_REQ)"]
            #[inline(always)]
            pub fn set_pwm4_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
            }
            #[doc = "PWM6 Clock Required (PWM6_CLK_REQ)"]
            #[inline(always)]
            pub const fn pwm6_clk_req(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[doc = "PWM6 Clock Required (PWM6_CLK_REQ)"]
            #[inline(always)]
            pub fn set_pwm6_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
            }
            #[doc = "PWM7 Clock Required (PWM7_CLK_REQ)"]
            #[inline(always)]
            pub const fn pwm7_clk_req(&self) -> bool {
                let val = (self.0 >> 26usize) & 0x01;
                val != 0
            }
            #[doc = "PWM7 Clock Required (PWM7_CLK_REQ)"]
            #[inline(always)]
            pub fn set_pwm7_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
            }
            #[doc = "EC_REG_BANK Clock Required (EC_REG_BANK_CLK_REQ)"]
            #[inline(always)]
            pub const fn ec_reg_bank_clk_req(&self) -> bool {
                let val = (self.0 >> 29usize) & 0x01;
                val != 0
            }
            #[doc = "EC_REG_BANK Clock Required (EC_REG_BANK_CLK_REQ)"]
            #[inline(always)]
            pub fn set_ec_reg_bank_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
            }
            #[doc = "TIMER16_0 Clock Required (TIMER16_0_CLK_REQ)"]
            #[inline(always)]
            pub const fn tmr16_0_clk_req(&self) -> bool {
                let val = (self.0 >> 30usize) & 0x01;
                val != 0
            }
            #[doc = "TIMER16_0 Clock Required (TIMER16_0_CLK_REQ)"]
            #[inline(always)]
            pub fn set_tmr16_0_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
            }
            #[doc = "TIMER16_1 Clock Required (TIMER16_1_CLK_REQ)"]
            #[inline(always)]
            pub const fn tmr16_1_clk_req(&self) -> bool {
                let val = (self.0 >> 31usize) & 0x01;
                val != 0
            }
            #[doc = "TIMER16_1 Clock Required (TIMER16_1_CLK_REQ)"]
            #[inline(always)]
            pub fn set_tmr16_1_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
            }
        }
        impl Default for ClkReq1 {
            #[inline(always)]
            fn default() -> ClkReq1 {
                ClkReq1(0)
            }
        }
        impl core::fmt::Debug for ClkReq1 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("ClkReq1")
                    .field("int_clk_req", &self.int_clk_req())
                    .field("tach0_clk_req", &self.tach0_clk_req())
                    .field("pwm0_clk_req", &self.pwm0_clk_req())
                    .field("pmc_clk_req", &self.pmc_clk_req())
                    .field("dma_clk_req", &self.dma_clk_req())
                    .field("tfdp_clk_req", &self.tfdp_clk_req())
                    .field("proc_clk_req", &self.proc_clk_req())
                    .field("wdt_clk_req", &self.wdt_clk_req())
                    .field("smb0_clk_req", &self.smb0_clk_req())
                    .field("tach2_clk_req", &self.tach2_clk_req())
                    .field("pwm1_clk_req", &self.pwm1_clk_req())
                    .field("pwm4_clk_req", &self.pwm4_clk_req())
                    .field("pwm6_clk_req", &self.pwm6_clk_req())
                    .field("pwm7_clk_req", &self.pwm7_clk_req())
                    .field("ec_reg_bank_clk_req", &self.ec_reg_bank_clk_req())
                    .field("tmr16_0_clk_req", &self.tmr16_0_clk_req())
                    .field("tmr16_1_clk_req", &self.tmr16_1_clk_req())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for ClkReq1 {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "ClkReq1 {{ int_clk_req: {=bool:?}, tach0_clk_req: {=bool:?}, pwm0_clk_req: {=bool:?}, pmc_clk_req: {=bool:?}, dma_clk_req: {=bool:?}, tfdp_clk_req: {=bool:?}, proc_clk_req: {=bool:?}, wdt_clk_req: {=bool:?}, smb0_clk_req: {=bool:?}, tach2_clk_req: {=bool:?}, pwm1_clk_req: {=bool:?}, pwm4_clk_req: {=bool:?}, pwm6_clk_req: {=bool:?}, pwm7_clk_req: {=bool:?}, ec_reg_bank_clk_req: {=bool:?}, tmr16_0_clk_req: {=bool:?}, tmr16_1_clk_req: {=bool:?} }}" , self . int_clk_req () , self . tach0_clk_req () , self . pwm0_clk_req () , self . pmc_clk_req () , self . dma_clk_req () , self . tfdp_clk_req () , self . proc_clk_req () , self . wdt_clk_req () , self . smb0_clk_req () , self . tach2_clk_req () , self . pwm1_clk_req () , self . pwm4_clk_req () , self . pwm6_clk_req () , self . pwm7_clk_req () , self . ec_reg_bank_clk_req () , self . tmr16_0_clk_req () , self . tmr16_1_clk_req ())
            }
        }
        #[doc = "Clock Required 2 Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct ClkReq2(pub u32);
        impl ClkReq2 {
            #[doc = "IMAP Clock Required (IMAP_CLK_REQ)"]
            #[inline(always)]
            pub const fn imap_clk_req(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "IMAP Clock Required (IMAP_CLK_REQ)"]
            #[inline(always)]
            pub fn set_imap_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "UART 0 Clock Required"]
            #[inline(always)]
            pub const fn uart0_clk_req(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "UART 0 Clock Required"]
            #[inline(always)]
            pub fn set_uart0_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "UART 1 Clock Required"]
            #[inline(always)]
            pub const fn uart1_clk_req(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "UART 1 Clock Required"]
            #[inline(always)]
            pub fn set_uart1_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "GLBL_CFG (GLBL_CFG_CLK_REQ)"]
            #[inline(always)]
            pub const fn glbl_cfg_clk_req(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "GLBL_CFG (GLBL_CFG_CLK_REQ)"]
            #[inline(always)]
            pub fn set_glbl_cfg_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "RTC Clock Required (RTC_CLK_REQ)"]
            #[inline(always)]
            pub const fn rtc_clk_req(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "RTC Clock Required (RTC_CLK_REQ)"]
            #[inline(always)]
            pub fn set_rtc_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
        }
        impl Default for ClkReq2 {
            #[inline(always)]
            fn default() -> ClkReq2 {
                ClkReq2(0)
            }
        }
        impl core::fmt::Debug for ClkReq2 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("ClkReq2")
                    .field("imap_clk_req", &self.imap_clk_req())
                    .field("uart0_clk_req", &self.uart0_clk_req())
                    .field("uart1_clk_req", &self.uart1_clk_req())
                    .field("glbl_cfg_clk_req", &self.glbl_cfg_clk_req())
                    .field("rtc_clk_req", &self.rtc_clk_req())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for ClkReq2 {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "ClkReq2 {{ imap_clk_req: {=bool:?}, uart0_clk_req: {=bool:?}, uart1_clk_req: {=bool:?}, glbl_cfg_clk_req: {=bool:?}, rtc_clk_req: {=bool:?} }}" , self . imap_clk_req () , self . uart0_clk_req () , self . uart1_clk_req () , self . glbl_cfg_clk_req () , self . rtc_clk_req ())
            }
        }
        #[doc = "Clock Required 3 Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct ClkReq3(pub u32);
        impl ClkReq3 {
            #[doc = "ADC Clock Required (ADC_CLK_REQ)"]
            #[inline(always)]
            pub const fn adc_clk_req(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "ADC Clock Required (ADC_CLK_REQ)"]
            #[inline(always)]
            pub fn set_adc_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "PS2_0 Clock Required (PS2_0_CLK_REQ)"]
            #[inline(always)]
            pub const fn ps2_0_clk_req(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "PS2_0 Clock Required (PS2_0_CLK_REQ)"]
            #[inline(always)]
            pub fn set_ps2_0_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Hibernation TIMER 0 Clock Required (HTM_0_CLK_REQ)"]
            #[inline(always)]
            pub const fn htm0_clk_req(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Hibernation TIMER 0 Clock Required (HTM_0_CLK_REQ)"]
            #[inline(always)]
            pub fn set_htm0_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "SMB1 Clock Required (SMB1_CLK_REQ)"]
            #[inline(always)]
            pub const fn smb1_clk_req(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "SMB1 Clock Required (SMB1_CLK_REQ)"]
            #[inline(always)]
            pub fn set_smb1_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "SMB2 Clock Required (SMB2_CLK_REQ)"]
            #[inline(always)]
            pub const fn smb2_clk_req(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "SMB2 Clock Required (SMB2_CLK_REQ)"]
            #[inline(always)]
            pub fn set_smb2_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "SMB3 Clock Required (SMB3_CLK_REQ)"]
            #[inline(always)]
            pub const fn smb3_clk_req(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "SMB3 Clock Required (SMB3_CLK_REQ)"]
            #[inline(always)]
            pub fn set_smb3_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
            #[doc = "LED0 Clock Required (LED0_CLK_REQ)"]
            #[inline(always)]
            pub const fn led0_clk_req(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "LED0 Clock Required (LED0_CLK_REQ)"]
            #[inline(always)]
            pub fn set_led0_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "SMB 4 Clock Required (SMB_4_CLK_REQ)"]
            #[inline(always)]
            pub const fn smb_4_clk_req(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "SMB 4 Clock Required (SMB_4_CLK_REQ)"]
            #[inline(always)]
            pub fn set_smb_4_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "TIMER32_0 Clock Required (TIMER32_0_CLK_REQ)"]
            #[inline(always)]
            pub const fn tmr32_0_clk_req(&self) -> bool {
                let val = (self.0 >> 23usize) & 0x01;
                val != 0
            }
            #[doc = "TIMER32_0 Clock Required (TIMER32_0_CLK_REQ)"]
            #[inline(always)]
            pub fn set_tmr32_0_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
            }
            #[doc = "TIMER32_1 Clock Required (TIMER32_1_CLK_REQ)"]
            #[inline(always)]
            pub const fn tmr32_1_clk_req(&self) -> bool {
                let val = (self.0 >> 24usize) & 0x01;
                val != 0
            }
            #[doc = "TIMER32_1 Clock Required (TIMER32_1_CLK_REQ)"]
            #[inline(always)]
            pub fn set_tmr32_1_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
            }
            #[doc = "PKE Clock Required"]
            #[inline(always)]
            pub const fn pke_clk_req(&self) -> bool {
                let val = (self.0 >> 26usize) & 0x01;
                val != 0
            }
            #[doc = "PKE Clock Required"]
            #[inline(always)]
            pub fn set_pke_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
            }
            #[doc = "RNG Clock Required"]
            #[inline(always)]
            pub const fn rng_clk_req(&self) -> bool {
                let val = (self.0 >> 27usize) & 0x01;
                val != 0
            }
            #[doc = "RNG Clock Required"]
            #[inline(always)]
            pub fn set_rng_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
            }
            #[doc = "AES_HASH Clock Required"]
            #[inline(always)]
            pub const fn aes_hash_clk_req(&self) -> bool {
                let val = (self.0 >> 28usize) & 0x01;
                val != 0
            }
            #[doc = "AES_HASH Clock Required"]
            #[inline(always)]
            pub fn set_aes_hash_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
            }
            #[doc = "Hibernation TIMER 1 Clock Required (HTM_1_CLK_REQ)"]
            #[inline(always)]
            pub const fn htm_1_clk_req(&self) -> bool {
                let val = (self.0 >> 29usize) & 0x01;
                val != 0
            }
            #[doc = "Hibernation TIMER 1 Clock Required (HTM_1_CLK_REQ)"]
            #[inline(always)]
            pub fn set_htm_1_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
            }
            #[doc = "Capture Compare Timer Clock Required (CCTIMER_CLK_REQ)"]
            #[inline(always)]
            pub const fn cctimer_clk_req(&self) -> bool {
                let val = (self.0 >> 30usize) & 0x01;
                val != 0
            }
            #[doc = "Capture Compare Timer Clock Required (CCTIMER_CLK_REQ)"]
            #[inline(always)]
            pub fn set_cctimer_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
            }
        }
        impl Default for ClkReq3 {
            #[inline(always)]
            fn default() -> ClkReq3 {
                ClkReq3(0)
            }
        }
        impl core::fmt::Debug for ClkReq3 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("ClkReq3")
                    .field("adc_clk_req", &self.adc_clk_req())
                    .field("ps2_0_clk_req", &self.ps2_0_clk_req())
                    .field("htm0_clk_req", &self.htm0_clk_req())
                    .field("smb1_clk_req", &self.smb1_clk_req())
                    .field("smb2_clk_req", &self.smb2_clk_req())
                    .field("smb3_clk_req", &self.smb3_clk_req())
                    .field("led0_clk_req", &self.led0_clk_req())
                    .field("smb_4_clk_req", &self.smb_4_clk_req())
                    .field("tmr32_0_clk_req", &self.tmr32_0_clk_req())
                    .field("tmr32_1_clk_req", &self.tmr32_1_clk_req())
                    .field("pke_clk_req", &self.pke_clk_req())
                    .field("rng_clk_req", &self.rng_clk_req())
                    .field("aes_hash_clk_req", &self.aes_hash_clk_req())
                    .field("htm_1_clk_req", &self.htm_1_clk_req())
                    .field("cctimer_clk_req", &self.cctimer_clk_req())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for ClkReq3 {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "ClkReq3 {{ adc_clk_req: {=bool:?}, ps2_0_clk_req: {=bool:?}, htm0_clk_req: {=bool:?}, smb1_clk_req: {=bool:?}, smb2_clk_req: {=bool:?}, smb3_clk_req: {=bool:?}, led0_clk_req: {=bool:?}, smb_4_clk_req: {=bool:?}, tmr32_0_clk_req: {=bool:?}, tmr32_1_clk_req: {=bool:?}, pke_clk_req: {=bool:?}, rng_clk_req: {=bool:?}, aes_hash_clk_req: {=bool:?}, htm_1_clk_req: {=bool:?}, cctimer_clk_req: {=bool:?} }}" , self . adc_clk_req () , self . ps2_0_clk_req () , self . htm0_clk_req () , self . smb1_clk_req () , self . smb2_clk_req () , self . smb3_clk_req () , self . led0_clk_req () , self . smb_4_clk_req () , self . tmr32_0_clk_req () , self . tmr32_1_clk_req () , self . pke_clk_req () , self . rng_clk_req () , self . aes_hash_clk_req () , self . htm_1_clk_req () , self . cctimer_clk_req ())
            }
        }
        #[doc = "Clock Required 4 Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct ClkReq4(pub u32);
        impl ClkReq4 {
            #[doc = "RTOS Clock Required (RTOS_CLK_REQ)"]
            #[inline(always)]
            pub const fn rtos_clk_req(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "RTOS Clock Required (RTOS_CLK_REQ)"]
            #[inline(always)]
            pub fn set_rtos_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Quad Master SPI Clock Required"]
            #[inline(always)]
            pub const fn qmspi_clk_req(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Quad Master SPI Clock Required"]
            #[inline(always)]
            pub fn set_qmspi_clk_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
        }
        impl Default for ClkReq4 {
            #[inline(always)]
            fn default() -> ClkReq4 {
                ClkReq4(0)
            }
        }
        impl core::fmt::Debug for ClkReq4 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("ClkReq4")
                    .field("rtos_clk_req", &self.rtos_clk_req())
                    .field("qmspi_clk_req", &self.qmspi_clk_req())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for ClkReq4 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "ClkReq4 {{ rtos_clk_req: {=bool:?}, qmspi_clk_req: {=bool:?} }}",
                    self.rtos_clk_req(),
                    self.qmspi_clk_req()
                )
            }
        }
        #[doc = "LOCK Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct LockReg(pub u32);
        impl LockReg {
            #[doc = "PCR Reset Enable Lock Register."]
            #[inline(always)]
            pub const fn pcr_rst_en_lock(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "PCR Reset Enable Lock Register."]
            #[inline(always)]
            pub fn set_pcr_rst_en_lock(&mut self, val: u32) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for LockReg {
            #[inline(always)]
            fn default() -> LockReg {
                LockReg(0)
            }
        }
        impl core::fmt::Debug for LockReg {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("LockReg")
                    .field("pcr_rst_en_lock", &self.pcr_rst_en_lock())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for LockReg {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "LockReg {{ pcr_rst_en_lock: {=u32:?} }}", self.pcr_rst_en_lock())
            }
        }
        #[doc = "Oscillator ID Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct OscId(pub u32);
        impl OscId {
            #[doc = "Test bits"]
            #[inline(always)]
            pub const fn test(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Test bits"]
            #[inline(always)]
            pub fn set_test(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "PLL Lock Status"]
            #[inline(always)]
            pub const fn pll_lock(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "PLL Lock Status"]
            #[inline(always)]
            pub fn set_pll_lock(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
        }
        impl Default for OscId {
            #[inline(always)]
            fn default() -> OscId {
                OscId(0)
            }
        }
        impl core::fmt::Debug for OscId {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("OscId")
                    .field("test", &self.test())
                    .field("pll_lock", &self.pll_lock())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for OscId {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "OscId {{ test: {=u8:?}, pll_lock: {=bool:?} }}",
                    self.test(),
                    self.pll_lock()
                )
            }
        }
        #[doc = "Processor Clock Control Register \\[7:0\\] Processor Clock Divide Value (PROC_DIV)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct ProcClkCtrl(pub u32);
        impl ProcClkCtrl {
            #[doc = "Selects the EC clock rate"]
            #[inline(always)]
            pub const fn div(&self) -> super::vals::Div {
                let val = (self.0 >> 0usize) & 0xff;
                super::vals::Div::from_bits(val as u8)
            }
            #[doc = "Selects the EC clock rate"]
            #[inline(always)]
            pub fn set_div(&mut self, val: super::vals::Div) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
            }
        }
        impl Default for ProcClkCtrl {
            #[inline(always)]
            fn default() -> ProcClkCtrl {
                ProcClkCtrl(0)
            }
        }
        impl core::fmt::Debug for ProcClkCtrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("ProcClkCtrl").field("div", &self.div()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for ProcClkCtrl {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "ProcClkCtrl {{ div: {:?} }}", self.div())
            }
        }
        #[doc = "Power Reset Control Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct PwrRstCtrl(pub u32);
        impl PwrRstCtrl {
            #[doc = "Used by FW to control internal RESET_VCC signal function and external PWROK pin. This bit is read-only when VCC_PWRGD is de-asserted low."]
            #[inline(always)]
            pub const fn pwr_inv(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Used by FW to control internal RESET_VCC signal function and external PWROK pin. This bit is read-only when VCC_PWRGD is de-asserted low."]
            #[inline(always)]
            pub fn set_pwr_inv(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Determines what generates the internal platform reset signal. 1=LRESET# pin; 0=eSPI PLTRST# VWire"]
            #[inline(always)]
            pub const fn h_rst_sel(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Determines what generates the internal platform reset signal. 1=LRESET# pin; 0=eSPI PLTRST# VWire"]
            #[inline(always)]
            pub fn set_h_rst_sel(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
        }
        impl Default for PwrRstCtrl {
            #[inline(always)]
            fn default() -> PwrRstCtrl {
                PwrRstCtrl(0)
            }
        }
        impl core::fmt::Debug for PwrRstCtrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("PwrRstCtrl")
                    .field("pwr_inv", &self.pwr_inv())
                    .field("h_rst_sel", &self.h_rst_sel())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for PwrRstCtrl {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "PwrRstCtrl {{ pwr_inv: {=bool:?}, h_rst_sel: {=bool:?} }}",
                    self.pwr_inv(),
                    self.h_rst_sel()
                )
            }
        }
        #[doc = "PCR Power Reset Status Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct PwrRstSts(pub u32);
        impl PwrRstSts {
            #[doc = "Indicates the status of VCC_PWRGD. 0 = PWRGD not asserted. 1 = PWRGD asserte."]
            #[inline(always)]
            pub const fn vcc_pwrgd_sts(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Indicates the status of VCC_PWRGD. 0 = PWRGD not asserted. 1 = PWRGD asserte."]
            #[inline(always)]
            pub fn set_vcc_pwrgd_sts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Indicates the status of RESET_VCC. 0 = reset active. 1 = reset not active."]
            #[inline(always)]
            pub const fn rst_h_sts(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Indicates the status of RESET_VCC. 0 = reset active. 1 = reset not active."]
            #[inline(always)]
            pub fn set_rst_h_sts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Indicates the status of RESET_VTR. 0 = reset active. 1 = reset not active.(R/W1C)"]
            #[inline(always)]
            pub const fn rst_vtr_sts(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Indicates the status of RESET_VTR. 0 = reset active. 1 = reset not active.(R/W1C)"]
            #[inline(always)]
            pub fn set_rst_vtr_sts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "VBAT reset status 0 = No reset occurred while VTR was off or since the last time this bit was cleared. 1 = A reset occurred.(R/WC)"]
            #[inline(always)]
            pub const fn vbat_rst_sts(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "VBAT reset status 0 = No reset occurred while VTR was off or since the last time this bit was cleared. 1 = A reset occurred.(R/WC)"]
            #[inline(always)]
            pub fn set_vbat_rst_sts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Indicates the status of RESET_SYS.(R/W1C) 0 = No reset occurred since the last time this bit was cleared. 1 = A reset occurred."]
            #[inline(always)]
            pub const fn rst_sys_sts(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Indicates the status of RESET_SYS.(R/W1C) 0 = No reset occurred since the last time this bit was cleared. 1 = A reset occurred."]
            #[inline(always)]
            pub fn set_rst_sys_sts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Indicates status of JTAG_TRST# pin. 0 = No JTAG reset occurred since the last time this bit was cleared. 1 = A reset occurred because of a JTAG command."]
            #[inline(always)]
            pub const fn jtag_rst_sts(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Indicates status of JTAG_TRST# pin. 0 = No JTAG reset occurred since the last time this bit was cleared. 1 = A reset occurred because of a JTAG command."]
            #[inline(always)]
            pub fn set_jtag_rst_sts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Indicates that a WDT_EVENT happened. (R/W1C) 0 = Not active. 1 = A WDT_EVENT occured."]
            #[inline(always)]
            pub const fn wdt_event(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Indicates that a WDT_EVENT happened. (R/W1C) 0 = Not active. 1 = A WDT_EVENT occured."]
            #[inline(always)]
            pub fn set_wdt_event(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "ACTIVE_32K (32K_ACTIVE)"]
            #[inline(always)]
            pub const fn active_32k(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "ACTIVE_32K (32K_ACTIVE)"]
            #[inline(always)]
            pub fn set_active_32k(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "PCICLK_ACTIVE (PCICLK_ACTIVE)"]
            #[inline(always)]
            pub const fn pciclk_active(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "PCICLK_ACTIVE (PCICLK_ACTIVE)"]
            #[inline(always)]
            pub fn set_pciclk_active(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
        }
        impl Default for PwrRstSts {
            #[inline(always)]
            fn default() -> PwrRstSts {
                PwrRstSts(0)
            }
        }
        impl core::fmt::Debug for PwrRstSts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("PwrRstSts")
                    .field("vcc_pwrgd_sts", &self.vcc_pwrgd_sts())
                    .field("rst_h_sts", &self.rst_h_sts())
                    .field("rst_vtr_sts", &self.rst_vtr_sts())
                    .field("vbat_rst_sts", &self.vbat_rst_sts())
                    .field("rst_sys_sts", &self.rst_sys_sts())
                    .field("jtag_rst_sts", &self.jtag_rst_sts())
                    .field("wdt_event", &self.wdt_event())
                    .field("active_32k", &self.active_32k())
                    .field("pciclk_active", &self.pciclk_active())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for PwrRstSts {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "PwrRstSts {{ vcc_pwrgd_sts: {=bool:?}, rst_h_sts: {=bool:?}, rst_vtr_sts: {=bool:?}, vbat_rst_sts: {=bool:?}, rst_sys_sts: {=bool:?}, jtag_rst_sts: {=bool:?}, wdt_event: {=bool:?}, active_32k: {=bool:?}, pciclk_active: {=bool:?} }}" , self . vcc_pwrgd_sts () , self . rst_h_sts () , self . rst_vtr_sts () , self . vbat_rst_sts () , self . rst_sys_sts () , self . jtag_rst_sts () , self . wdt_event () , self . active_32k () , self . pciclk_active ())
            }
        }
        #[doc = "Reset Enable 0 Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RstEn0(pub u32);
        impl RstEn0 {
            #[doc = "OTP Reset Enable"]
            #[inline(always)]
            pub const fn otp_rst_en(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "OTP Reset Enable"]
            #[inline(always)]
            pub fn set_otp_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
        }
        impl Default for RstEn0 {
            #[inline(always)]
            fn default() -> RstEn0 {
                RstEn0(0)
            }
        }
        impl core::fmt::Debug for RstEn0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RstEn0")
                    .field("otp_rst_en", &self.otp_rst_en())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RstEn0 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "RstEn0 {{ otp_rst_en: {=bool:?} }}", self.otp_rst_en())
            }
        }
        #[doc = "Reset Enable 1 Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RstEn1(pub u32);
        impl RstEn1 {
            #[doc = "Interrupt Reset Enable"]
            #[inline(always)]
            pub const fn int_rst_en(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Interrupt Reset Enable"]
            #[inline(always)]
            pub fn set_int_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "TACH0 Reset Enable (TACH0_RST_EN)"]
            #[inline(always)]
            pub const fn tach0_rst_en(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "TACH0 Reset Enable (TACH0_RST_EN)"]
            #[inline(always)]
            pub fn set_tach0_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "PWM0 Reset Enable (PWM0_RST_EN)"]
            #[inline(always)]
            pub const fn pwm0_rst_en(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "PWM0 Reset Enable (PWM0_RST_EN)"]
            #[inline(always)]
            pub fn set_pwm0_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "DMA Reset Enable (DMA_RST_EN)"]
            #[inline(always)]
            pub const fn dma_rst_en(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "DMA Reset Enable (DMA_RST_EN)"]
            #[inline(always)]
            pub fn set_dma_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "TFDP Reset Enable (TFDP_RST_EN)"]
            #[inline(always)]
            pub const fn tfdp_rst_en(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "TFDP Reset Enable (TFDP_RST_EN)"]
            #[inline(always)]
            pub fn set_tfdp_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "WDT Reset Enable (WDT_RST_EN)"]
            #[inline(always)]
            pub const fn wdt_rst_en(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "WDT Reset Enable (WDT_RST_EN)"]
            #[inline(always)]
            pub fn set_wdt_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "SMB0 Reset Enable (SMB0_RST_EN)"]
            #[inline(always)]
            pub const fn smb0_rst_en(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "SMB0 Reset Enable (SMB0_RST_EN)"]
            #[inline(always)]
            pub fn set_smb0_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "TACH2 Reset Enable (TACH2_RST_EN)"]
            #[inline(always)]
            pub const fn tach2_rst_en(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "TACH2 Reset Enable (TACH2_RST_EN)"]
            #[inline(always)]
            pub fn set_tach2_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "PWM1 Reset Enable (PWM1_RST_EN)"]
            #[inline(always)]
            pub const fn pwm1_rst_en(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "PWM1 Reset Enable (PWM1_RST_EN)"]
            #[inline(always)]
            pub fn set_pwm1_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "PWM4 Reset Enable (PWM4_RST_EN)"]
            #[inline(always)]
            pub const fn pwm4_rst_en(&self) -> bool {
                let val = (self.0 >> 23usize) & 0x01;
                val != 0
            }
            #[doc = "PWM4 Reset Enable (PWM4_RST_EN)"]
            #[inline(always)]
            pub fn set_pwm4_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
            }
            #[doc = "PWM6 Reset Enable (PWM6_RST_EN)"]
            #[inline(always)]
            pub const fn pwm6_rst_en(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[doc = "PWM6 Reset Enable (PWM6_RST_EN)"]
            #[inline(always)]
            pub fn set_pwm6_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
            }
            #[doc = "PWM7 Reset Enable (PWM7_RST_EN)"]
            #[inline(always)]
            pub const fn pwm7_rst_en(&self) -> bool {
                let val = (self.0 >> 26usize) & 0x01;
                val != 0
            }
            #[doc = "PWM7 Reset Enable (PWM7_RST_EN)"]
            #[inline(always)]
            pub fn set_pwm7_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
            }
            #[doc = "TIMER16_0 Reset Enable (TIMER16_0_RST_EN)"]
            #[inline(always)]
            pub const fn tmr16_0_rst_en(&self) -> bool {
                let val = (self.0 >> 30usize) & 0x01;
                val != 0
            }
            #[doc = "TIMER16_0 Reset Enable (TIMER16_0_RST_EN)"]
            #[inline(always)]
            pub fn set_tmr16_0_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
            }
            #[doc = "TIMER16_1 Reset Enable (TIMER16_1_RST_EN)"]
            #[inline(always)]
            pub const fn tmr16_1_rst_en(&self) -> bool {
                let val = (self.0 >> 31usize) & 0x01;
                val != 0
            }
            #[doc = "TIMER16_1 Reset Enable (TIMER16_1_RST_EN)"]
            #[inline(always)]
            pub fn set_tmr16_1_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
            }
        }
        impl Default for RstEn1 {
            #[inline(always)]
            fn default() -> RstEn1 {
                RstEn1(0)
            }
        }
        impl core::fmt::Debug for RstEn1 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RstEn1")
                    .field("int_rst_en", &self.int_rst_en())
                    .field("tach0_rst_en", &self.tach0_rst_en())
                    .field("pwm0_rst_en", &self.pwm0_rst_en())
                    .field("dma_rst_en", &self.dma_rst_en())
                    .field("tfdp_rst_en", &self.tfdp_rst_en())
                    .field("wdt_rst_en", &self.wdt_rst_en())
                    .field("smb0_rst_en", &self.smb0_rst_en())
                    .field("tach2_rst_en", &self.tach2_rst_en())
                    .field("pwm1_rst_en", &self.pwm1_rst_en())
                    .field("pwm4_rst_en", &self.pwm4_rst_en())
                    .field("pwm6_rst_en", &self.pwm6_rst_en())
                    .field("pwm7_rst_en", &self.pwm7_rst_en())
                    .field("tmr16_0_rst_en", &self.tmr16_0_rst_en())
                    .field("tmr16_1_rst_en", &self.tmr16_1_rst_en())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RstEn1 {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "RstEn1 {{ int_rst_en: {=bool:?}, tach0_rst_en: {=bool:?}, pwm0_rst_en: {=bool:?}, dma_rst_en: {=bool:?}, tfdp_rst_en: {=bool:?}, wdt_rst_en: {=bool:?}, smb0_rst_en: {=bool:?}, tach2_rst_en: {=bool:?}, pwm1_rst_en: {=bool:?}, pwm4_rst_en: {=bool:?}, pwm6_rst_en: {=bool:?}, pwm7_rst_en: {=bool:?}, tmr16_0_rst_en: {=bool:?}, tmr16_1_rst_en: {=bool:?} }}" , self . int_rst_en () , self . tach0_rst_en () , self . pwm0_rst_en () , self . dma_rst_en () , self . tfdp_rst_en () , self . wdt_rst_en () , self . smb0_rst_en () , self . tach2_rst_en () , self . pwm1_rst_en () , self . pwm4_rst_en () , self . pwm6_rst_en () , self . pwm7_rst_en () , self . tmr16_0_rst_en () , self . tmr16_1_rst_en ())
            }
        }
        #[doc = "Reset Enable 2 Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RstEn2(pub u32);
        impl RstEn2 {
            #[doc = "IMAP Reset Enable (IMAP_RST_EN)"]
            #[inline(always)]
            pub const fn imap_rst_en(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "IMAP Reset Enable (IMAP_RST_EN)"]
            #[inline(always)]
            pub fn set_imap_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "UART 0 Reset Enable"]
            #[inline(always)]
            pub const fn uart0_rst_en(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "UART 0 Reset Enable"]
            #[inline(always)]
            pub fn set_uart0_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "UART 1 Reset Enable"]
            #[inline(always)]
            pub const fn uart1_rst_en(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "UART 1 Reset Enable"]
            #[inline(always)]
            pub fn set_uart1_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "ACPI EC 0 Reset Enable (ACPI_EC_0_RST_EN)"]
            #[inline(always)]
            pub const fn acpi_ec0_rst_en(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "ACPI EC 0 Reset Enable (ACPI_EC_0_RST_EN)"]
            #[inline(always)]
            pub fn set_acpi_ec0_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "ACPI EC 1 Reset Enable (ACPI_EC_1_RST_EN)"]
            #[inline(always)]
            pub const fn acpi_ec1_rst_en(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "ACPI EC 1 Reset Enable (ACPI_EC_1_RST_EN)"]
            #[inline(always)]
            pub fn set_acpi_ec1_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "ACPI PM1 Reset Enable (ACPI_PM1_RST_EN)"]
            #[inline(always)]
            pub const fn acpi_pm1_rst_en(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "ACPI PM1 Reset Enable (ACPI_PM1_RST_EN)"]
            #[inline(always)]
            pub fn set_acpi_pm1_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
            #[doc = "Mailbox Reset Enable (MBX_RST_EN)"]
            #[inline(always)]
            pub const fn mbx_rst_en(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Mailbox Reset Enable (MBX_RST_EN)"]
            #[inline(always)]
            pub fn set_mbx_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "RTC Reset Enable (RTC_RST_EN)"]
            #[inline(always)]
            pub const fn rtc_rst_en(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "RTC Reset Enable (RTC_RST_EN)"]
            #[inline(always)]
            pub fn set_rtc_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "SCRATCH 32 Reset Enable"]
            #[inline(always)]
            pub const fn scratch_32_rst_en(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "SCRATCH 32 Reset Enable"]
            #[inline(always)]
            pub fn set_scratch_32_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "ACPI EC 2 Reset Enable (ACPI_EC_2_RST_EN)"]
            #[inline(always)]
            pub const fn acpi_ec_2_rst_en(&self) -> bool {
                let val = (self.0 >> 21usize) & 0x01;
                val != 0
            }
            #[doc = "ACPI EC 2 Reset Enable (ACPI_EC_2_RST_EN)"]
            #[inline(always)]
            pub fn set_acpi_ec_2_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
            }
            #[doc = "ACPI EC 3 Reset Enable (ACPI_EC_3_RST_EN)"]
            #[inline(always)]
            pub const fn acpi_ec_3_rst_en(&self) -> bool {
                let val = (self.0 >> 22usize) & 0x01;
                val != 0
            }
            #[doc = "ACPI EC 3 Reset Enable (ACPI_EC_3_RST_EN)"]
            #[inline(always)]
            pub fn set_acpi_ec_3_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
            }
            #[doc = "Port 80 0 Reset Enable"]
            #[inline(always)]
            pub const fn port_80_0_rst_en(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[doc = "Port 80 0 Reset Enable"]
            #[inline(always)]
            pub fn set_port_80_0_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
            }
            #[doc = "Port 80 1 Reset Enable"]
            #[inline(always)]
            pub const fn port_80_1_rst_en(&self) -> bool {
                let val = (self.0 >> 26usize) & 0x01;
                val != 0
            }
            #[doc = "Port 80 1 Reset Enable"]
            #[inline(always)]
            pub fn set_port_80_1_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
            }
        }
        impl Default for RstEn2 {
            #[inline(always)]
            fn default() -> RstEn2 {
                RstEn2(0)
            }
        }
        impl core::fmt::Debug for RstEn2 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RstEn2")
                    .field("imap_rst_en", &self.imap_rst_en())
                    .field("uart0_rst_en", &self.uart0_rst_en())
                    .field("uart1_rst_en", &self.uart1_rst_en())
                    .field("acpi_ec0_rst_en", &self.acpi_ec0_rst_en())
                    .field("acpi_ec1_rst_en", &self.acpi_ec1_rst_en())
                    .field("acpi_pm1_rst_en", &self.acpi_pm1_rst_en())
                    .field("mbx_rst_en", &self.mbx_rst_en())
                    .field("rtc_rst_en", &self.rtc_rst_en())
                    .field("scratch_32_rst_en", &self.scratch_32_rst_en())
                    .field("acpi_ec_2_rst_en", &self.acpi_ec_2_rst_en())
                    .field("acpi_ec_3_rst_en", &self.acpi_ec_3_rst_en())
                    .field("port_80_0_rst_en", &self.port_80_0_rst_en())
                    .field("port_80_1_rst_en", &self.port_80_1_rst_en())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RstEn2 {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "RstEn2 {{ imap_rst_en: {=bool:?}, uart0_rst_en: {=bool:?}, uart1_rst_en: {=bool:?}, acpi_ec0_rst_en: {=bool:?}, acpi_ec1_rst_en: {=bool:?}, acpi_pm1_rst_en: {=bool:?}, mbx_rst_en: {=bool:?}, rtc_rst_en: {=bool:?}, scratch_32_rst_en: {=bool:?}, acpi_ec_2_rst_en: {=bool:?}, acpi_ec_3_rst_en: {=bool:?}, port_80_0_rst_en: {=bool:?}, port_80_1_rst_en: {=bool:?} }}" , self . imap_rst_en () , self . uart0_rst_en () , self . uart1_rst_en () , self . acpi_ec0_rst_en () , self . acpi_ec1_rst_en () , self . acpi_pm1_rst_en () , self . mbx_rst_en () , self . rtc_rst_en () , self . scratch_32_rst_en () , self . acpi_ec_2_rst_en () , self . acpi_ec_3_rst_en () , self . port_80_0_rst_en () , self . port_80_1_rst_en ())
            }
        }
        #[doc = "Reset Enable 3 Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RstEn3(pub u32);
        impl RstEn3 {
            #[doc = "ADC Reset Enable (ADC_RST_EN)"]
            #[inline(always)]
            pub const fn adc_rst_en(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "ADC Reset Enable (ADC_RST_EN)"]
            #[inline(always)]
            pub fn set_adc_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "PS2_0 Reset Enable (PS2_0_RST_EN)"]
            #[inline(always)]
            pub const fn ps2_0_rst_en(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "PS2_0 Reset Enable (PS2_0_RST_EN)"]
            #[inline(always)]
            pub fn set_ps2_0_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Hibernation TIMER 0 Reset Enable (HTM_0_RST_EN)"]
            #[inline(always)]
            pub const fn htm_0_rst_en(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Hibernation TIMER 0 Reset Enable (HTM_0_RST_EN)"]
            #[inline(always)]
            pub fn set_htm_0_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "SMB1 Reset Enable (SMB1_RST_EN)"]
            #[inline(always)]
            pub const fn smb1_rst_en(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "SMB1 Reset Enable (SMB1_RST_EN)"]
            #[inline(always)]
            pub fn set_smb1_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "SMB2 Reset Enable (SMB2_RST_EN)"]
            #[inline(always)]
            pub const fn smb2_rst_en(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "SMB2 Reset Enable (SMB2_RST_EN)"]
            #[inline(always)]
            pub fn set_smb2_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "SMB3 Reset Enable (SMB3_RST_EN)"]
            #[inline(always)]
            pub const fn smb3_rst_en(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "SMB3 Reset Enable (SMB3_RST_EN)"]
            #[inline(always)]
            pub fn set_smb3_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
            #[doc = "LED0 Reset Enable (LED0_RST_EN)"]
            #[inline(always)]
            pub const fn led0_rst_en(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "LED0 Reset Enable (LED0_RST_EN)"]
            #[inline(always)]
            pub fn set_led0_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "SMB 4 Reset Enable (SMB_4_RST_EN)"]
            #[inline(always)]
            pub const fn smb_4_rst_en(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "SMB 4 Reset Enable (SMB_4_RST_EN)"]
            #[inline(always)]
            pub fn set_smb_4_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "TIMER32_0 Reset Enable (TIMER32_0_RST_EN)"]
            #[inline(always)]
            pub const fn tmr32_0_rst_en(&self) -> bool {
                let val = (self.0 >> 23usize) & 0x01;
                val != 0
            }
            #[doc = "TIMER32_0 Reset Enable (TIMER32_0_RST_EN)"]
            #[inline(always)]
            pub fn set_tmr32_0_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
            }
            #[doc = "TIMER32_1 Reset Enable (TIMER32_1_RST_EN)"]
            #[inline(always)]
            pub const fn tmr32_1_rst_en(&self) -> bool {
                let val = (self.0 >> 24usize) & 0x01;
                val != 0
            }
            #[doc = "TIMER32_1 Reset Enable (TIMER32_1_RST_EN)"]
            #[inline(always)]
            pub fn set_tmr32_1_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
            }
            #[doc = "PKE Reset Enable"]
            #[inline(always)]
            pub const fn pke_rst_en(&self) -> bool {
                let val = (self.0 >> 26usize) & 0x01;
                val != 0
            }
            #[doc = "PKE Reset Enable"]
            #[inline(always)]
            pub fn set_pke_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
            }
            #[doc = "RNG Reset Enable"]
            #[inline(always)]
            pub const fn rng_rst_en(&self) -> bool {
                let val = (self.0 >> 27usize) & 0x01;
                val != 0
            }
            #[doc = "RNG Reset Enable"]
            #[inline(always)]
            pub fn set_rng_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
            }
            #[doc = "AES_HASH Reset Enable"]
            #[inline(always)]
            pub const fn aes_hash_rst_en(&self) -> bool {
                let val = (self.0 >> 28usize) & 0x01;
                val != 0
            }
            #[doc = "AES_HASH Reset Enable"]
            #[inline(always)]
            pub fn set_aes_hash_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
            }
            #[doc = "Hibernation TIMER 1 Reset Enable (HTM_1_RST_EN)"]
            #[inline(always)]
            pub const fn htm_1_rst_en(&self) -> bool {
                let val = (self.0 >> 29usize) & 0x01;
                val != 0
            }
            #[doc = "Hibernation TIMER 1 Reset Enable (HTM_1_RST_EN)"]
            #[inline(always)]
            pub fn set_htm_1_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
            }
            #[doc = "Capture Compare Timer Reset Enable (CCTIMER_RST_EN)"]
            #[inline(always)]
            pub const fn cctimer_rst_en(&self) -> bool {
                let val = (self.0 >> 30usize) & 0x01;
                val != 0
            }
            #[doc = "Capture Compare Timer Reset Enable (CCTIMER_RST_EN)"]
            #[inline(always)]
            pub fn set_cctimer_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
            }
        }
        impl Default for RstEn3 {
            #[inline(always)]
            fn default() -> RstEn3 {
                RstEn3(0)
            }
        }
        impl core::fmt::Debug for RstEn3 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RstEn3")
                    .field("adc_rst_en", &self.adc_rst_en())
                    .field("ps2_0_rst_en", &self.ps2_0_rst_en())
                    .field("htm_0_rst_en", &self.htm_0_rst_en())
                    .field("smb1_rst_en", &self.smb1_rst_en())
                    .field("smb2_rst_en", &self.smb2_rst_en())
                    .field("smb3_rst_en", &self.smb3_rst_en())
                    .field("led0_rst_en", &self.led0_rst_en())
                    .field("smb_4_rst_en", &self.smb_4_rst_en())
                    .field("tmr32_0_rst_en", &self.tmr32_0_rst_en())
                    .field("tmr32_1_rst_en", &self.tmr32_1_rst_en())
                    .field("pke_rst_en", &self.pke_rst_en())
                    .field("rng_rst_en", &self.rng_rst_en())
                    .field("aes_hash_rst_en", &self.aes_hash_rst_en())
                    .field("htm_1_rst_en", &self.htm_1_rst_en())
                    .field("cctimer_rst_en", &self.cctimer_rst_en())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RstEn3 {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "RstEn3 {{ adc_rst_en: {=bool:?}, ps2_0_rst_en: {=bool:?}, htm_0_rst_en: {=bool:?}, smb1_rst_en: {=bool:?}, smb2_rst_en: {=bool:?}, smb3_rst_en: {=bool:?}, led0_rst_en: {=bool:?}, smb_4_rst_en: {=bool:?}, tmr32_0_rst_en: {=bool:?}, tmr32_1_rst_en: {=bool:?}, pke_rst_en: {=bool:?}, rng_rst_en: {=bool:?}, aes_hash_rst_en: {=bool:?}, htm_1_rst_en: {=bool:?}, cctimer_rst_en: {=bool:?} }}" , self . adc_rst_en () , self . ps2_0_rst_en () , self . htm_0_rst_en () , self . smb1_rst_en () , self . smb2_rst_en () , self . smb3_rst_en () , self . led0_rst_en () , self . smb_4_rst_en () , self . tmr32_0_rst_en () , self . tmr32_1_rst_en () , self . pke_rst_en () , self . rng_rst_en () , self . aes_hash_rst_en () , self . htm_1_rst_en () , self . cctimer_rst_en ())
            }
        }
        #[doc = "Reset Enable 4 Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RstEn4(pub u32);
        impl RstEn4 {
            #[doc = "RTOS Reset Enable (RTOS_RST_EN)"]
            #[inline(always)]
            pub const fn rtos_rst_en(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "RTOS Reset Enable (RTOS_RST_EN)"]
            #[inline(always)]
            pub fn set_rtos_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Quad Master SPI Reset Enable"]
            #[inline(always)]
            pub const fn qmspi_rst_en(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Quad Master SPI Reset Enable"]
            #[inline(always)]
            pub fn set_qmspi_rst_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
        }
        impl Default for RstEn4 {
            #[inline(always)]
            fn default() -> RstEn4 {
                RstEn4(0)
            }
        }
        impl core::fmt::Debug for RstEn4 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RstEn4")
                    .field("rtos_rst_en", &self.rtos_rst_en())
                    .field("qmspi_rst_en", &self.qmspi_rst_en())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RstEn4 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "RstEn4 {{ rtos_rst_en: {=bool:?}, qmspi_rst_en: {=bool:?} }}",
                    self.rtos_rst_en(),
                    self.qmspi_rst_en()
                )
            }
        }
        #[doc = "Configures the EC_CLK clock domain"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SlowClkCtrl(pub u32);
        impl SlowClkCtrl {
            #[doc = "SLOW_CLOCK_DIVIDE. n=Divide by n; 0=Clock off"]
            #[inline(always)]
            pub const fn div(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x03ff;
                val as u16
            }
            #[doc = "SLOW_CLOCK_DIVIDE. n=Divide by n; 0=Clock off"]
            #[inline(always)]
            pub fn set_div(&mut self, val: u16) {
                self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
            }
        }
        impl Default for SlowClkCtrl {
            #[inline(always)]
            fn default() -> SlowClkCtrl {
                SlowClkCtrl(0)
            }
        }
        impl core::fmt::Debug for SlowClkCtrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("SlowClkCtrl").field("div", &self.div()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for SlowClkCtrl {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "SlowClkCtrl {{ div: {=u16:?} }}", self.div())
            }
        }
        #[doc = "Sleep Enable 0 Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SlpEn0(pub u32);
        impl SlpEn0 {
            #[doc = "OTP Enable"]
            #[inline(always)]
            pub const fn otp_slp_en(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "OTP Enable"]
            #[inline(always)]
            pub fn set_otp_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
        }
        impl Default for SlpEn0 {
            #[inline(always)]
            fn default() -> SlpEn0 {
                SlpEn0(0)
            }
        }
        impl core::fmt::Debug for SlpEn0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("SlpEn0")
                    .field("otp_slp_en", &self.otp_slp_en())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for SlpEn0 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "SlpEn0 {{ otp_slp_en: {=bool:?} }}", self.otp_slp_en())
            }
        }
        #[doc = "Sleep Enable 1 Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SlpEn1(pub u32);
        impl SlpEn1 {
            #[doc = "Interrupt Sleep Enable"]
            #[inline(always)]
            pub const fn int_slp_en(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Interrupt Sleep Enable"]
            #[inline(always)]
            pub fn set_int_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "TACH0 Sleep Enable (TACH0_SLP_EN)"]
            #[inline(always)]
            pub const fn tach0_slp_en(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "TACH0 Sleep Enable (TACH0_SLP_EN)"]
            #[inline(always)]
            pub fn set_tach0_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "PWM0 Sleep Enable (PWM0_SLP_EN)"]
            #[inline(always)]
            pub const fn pwm0_slp_en(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "PWM0 Sleep Enable (PWM0_SLP_EN)"]
            #[inline(always)]
            pub fn set_pwm0_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "PMC Sleep Enable (PMC_SLP_EN)"]
            #[inline(always)]
            pub const fn pmc_slp_en(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "PMC Sleep Enable (PMC_SLP_EN)"]
            #[inline(always)]
            pub fn set_pmc_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "DMA Sleep Enable (DMA_SLP_EN)"]
            #[inline(always)]
            pub const fn dma_slp_en(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "DMA Sleep Enable (DMA_SLP_EN)"]
            #[inline(always)]
            pub fn set_dma_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "TFDP Sleep Enable (TFDP_SLP_EN)"]
            #[inline(always)]
            pub const fn tfdp_slp_en(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "TFDP Sleep Enable (TFDP_SLP_EN)"]
            #[inline(always)]
            pub fn set_tfdp_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "PROCESSOR Sleep Enable (PROCESSOR_SLP_EN)"]
            #[inline(always)]
            pub const fn proc_slp_en(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "PROCESSOR Sleep Enable (PROCESSOR_SLP_EN)"]
            #[inline(always)]
            pub fn set_proc_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "SMB0 Sleep Enable (SMB0_SLP_EN)"]
            #[inline(always)]
            pub const fn smb0_slp_en(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "SMB0 Sleep Enable (SMB0_SLP_EN)"]
            #[inline(always)]
            pub fn set_smb0_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "TACH2 Sleep Enable (TACH2_SLP_EN)"]
            #[inline(always)]
            pub const fn tach2_slp_en(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "TACH2 Sleep Enable (TACH2_SLP_EN)"]
            #[inline(always)]
            pub fn set_tach2_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "PWM1 Sleep Enable (PWM1_SLP_EN)"]
            #[inline(always)]
            pub const fn pwm1_slp_en(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "PWM1 Sleep Enable (PWM1_SLP_EN)"]
            #[inline(always)]
            pub fn set_pwm1_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "PWM4 Sleep Enable (PWM4_SLP_EN)"]
            #[inline(always)]
            pub const fn pwm4_slp_en(&self) -> bool {
                let val = (self.0 >> 23usize) & 0x01;
                val != 0
            }
            #[doc = "PWM4 Sleep Enable (PWM4_SLP_EN)"]
            #[inline(always)]
            pub fn set_pwm4_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
            }
            #[doc = "PWM6 Sleep Enable (PWM6_SLP_EN)"]
            #[inline(always)]
            pub const fn pwm6_slp_en(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[doc = "PWM6 Sleep Enable (PWM6_SLP_EN)"]
            #[inline(always)]
            pub fn set_pwm6_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
            }
            #[doc = "PWM7 Sleep Enable (PWM7_SLP_EN)"]
            #[inline(always)]
            pub const fn pwm7_slp_en(&self) -> bool {
                let val = (self.0 >> 26usize) & 0x01;
                val != 0
            }
            #[doc = "PWM7 Sleep Enable (PWM7_SLP_EN)"]
            #[inline(always)]
            pub fn set_pwm7_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
            }
            #[doc = "EC_REG_BANK Sleep Enable (EC_REG_BANK_SLP_EN)"]
            #[inline(always)]
            pub const fn ec_reg_bank_slp_en(&self) -> bool {
                let val = (self.0 >> 29usize) & 0x01;
                val != 0
            }
            #[doc = "EC_REG_BANK Sleep Enable (EC_REG_BANK_SLP_EN)"]
            #[inline(always)]
            pub fn set_ec_reg_bank_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
            }
            #[doc = "TIMER16_0 Sleep Enable (TIMER16_0_SLP_EN)"]
            #[inline(always)]
            pub const fn tmr16_0_slp_en(&self) -> bool {
                let val = (self.0 >> 30usize) & 0x01;
                val != 0
            }
            #[doc = "TIMER16_0 Sleep Enable (TIMER16_0_SLP_EN)"]
            #[inline(always)]
            pub fn set_tmr16_0_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
            }
            #[doc = "TIMER16_1 Sleep Enable (TIMER16_1_SLP_EN)"]
            #[inline(always)]
            pub const fn tmr16_1_slp_en(&self) -> bool {
                let val = (self.0 >> 31usize) & 0x01;
                val != 0
            }
            #[doc = "TIMER16_1 Sleep Enable (TIMER16_1_SLP_EN)"]
            #[inline(always)]
            pub fn set_tmr16_1_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
            }
        }
        impl Default for SlpEn1 {
            #[inline(always)]
            fn default() -> SlpEn1 {
                SlpEn1(0)
            }
        }
        impl core::fmt::Debug for SlpEn1 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("SlpEn1")
                    .field("int_slp_en", &self.int_slp_en())
                    .field("tach0_slp_en", &self.tach0_slp_en())
                    .field("pwm0_slp_en", &self.pwm0_slp_en())
                    .field("pmc_slp_en", &self.pmc_slp_en())
                    .field("dma_slp_en", &self.dma_slp_en())
                    .field("tfdp_slp_en", &self.tfdp_slp_en())
                    .field("proc_slp_en", &self.proc_slp_en())
                    .field("smb0_slp_en", &self.smb0_slp_en())
                    .field("tach2_slp_en", &self.tach2_slp_en())
                    .field("pwm1_slp_en", &self.pwm1_slp_en())
                    .field("pwm4_slp_en", &self.pwm4_slp_en())
                    .field("pwm6_slp_en", &self.pwm6_slp_en())
                    .field("pwm7_slp_en", &self.pwm7_slp_en())
                    .field("ec_reg_bank_slp_en", &self.ec_reg_bank_slp_en())
                    .field("tmr16_0_slp_en", &self.tmr16_0_slp_en())
                    .field("tmr16_1_slp_en", &self.tmr16_1_slp_en())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for SlpEn1 {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "SlpEn1 {{ int_slp_en: {=bool:?}, tach0_slp_en: {=bool:?}, pwm0_slp_en: {=bool:?}, pmc_slp_en: {=bool:?}, dma_slp_en: {=bool:?}, tfdp_slp_en: {=bool:?}, proc_slp_en: {=bool:?}, smb0_slp_en: {=bool:?}, tach2_slp_en: {=bool:?}, pwm1_slp_en: {=bool:?}, pwm4_slp_en: {=bool:?}, pwm6_slp_en: {=bool:?}, pwm7_slp_en: {=bool:?}, ec_reg_bank_slp_en: {=bool:?}, tmr16_0_slp_en: {=bool:?}, tmr16_1_slp_en: {=bool:?} }}" , self . int_slp_en () , self . tach0_slp_en () , self . pwm0_slp_en () , self . pmc_slp_en () , self . dma_slp_en () , self . tfdp_slp_en () , self . proc_slp_en () , self . smb0_slp_en () , self . tach2_slp_en () , self . pwm1_slp_en () , self . pwm4_slp_en () , self . pwm6_slp_en () , self . pwm7_slp_en () , self . ec_reg_bank_slp_en () , self . tmr16_0_slp_en () , self . tmr16_1_slp_en ())
            }
        }
        #[doc = "Sleep Enable 2 Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SlpEn2(pub u32);
        impl SlpEn2 {
            #[doc = "UART 0 Sleep Enable"]
            #[inline(always)]
            pub const fn uart0_slp_en(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "UART 0 Sleep Enable"]
            #[inline(always)]
            pub fn set_uart0_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "UART 1 Sleep Enable"]
            #[inline(always)]
            pub const fn uart1_slp_en(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "UART 1 Sleep Enable"]
            #[inline(always)]
            pub fn set_uart1_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
        }
        impl Default for SlpEn2 {
            #[inline(always)]
            fn default() -> SlpEn2 {
                SlpEn2(0)
            }
        }
        impl core::fmt::Debug for SlpEn2 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("SlpEn2")
                    .field("uart0_slp_en", &self.uart0_slp_en())
                    .field("uart1_slp_en", &self.uart1_slp_en())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for SlpEn2 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "SlpEn2 {{ uart0_slp_en: {=bool:?}, uart1_slp_en: {=bool:?} }}",
                    self.uart0_slp_en(),
                    self.uart1_slp_en()
                )
            }
        }
        #[doc = "Sleep Enable 3 Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SlpEn3(pub u32);
        impl SlpEn3 {
            #[doc = "ADC Sleep Enable (ADC_SLP_EN)"]
            #[inline(always)]
            pub const fn adc_slp_en(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "ADC Sleep Enable (ADC_SLP_EN)"]
            #[inline(always)]
            pub fn set_adc_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "PS2_0 Sleep Enable (PS2_0_SLP_EN)"]
            #[inline(always)]
            pub const fn ps2_0_slp_en(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "PS2_0 Sleep Enable (PS2_0_SLP_EN)"]
            #[inline(always)]
            pub fn set_ps2_0_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Hibernation Timer 0 Sleep Enable (HTM_0_SLP_EN)"]
            #[inline(always)]
            pub const fn htm_0_slp_en(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Hibernation Timer 0 Sleep Enable (HTM_0_SLP_EN)"]
            #[inline(always)]
            pub fn set_htm_0_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "SMB1 Sleep Enable (SMB1_SLP_EN)"]
            #[inline(always)]
            pub const fn smb1_slp_en(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "SMB1 Sleep Enable (SMB1_SLP_EN)"]
            #[inline(always)]
            pub fn set_smb1_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "SMB2 Sleep Enable (SMB2_SLP_EN)"]
            #[inline(always)]
            pub const fn smb2_slp_en(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "SMB2 Sleep Enable (SMB2_SLP_EN)"]
            #[inline(always)]
            pub fn set_smb2_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "SMB3 Sleep Enable (SMB3_SLP_EN)"]
            #[inline(always)]
            pub const fn smb3_slp_en(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "SMB3 Sleep Enable (SMB3_SLP_EN)"]
            #[inline(always)]
            pub fn set_smb3_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
            #[doc = "LED0 Sleep Enable (LED0_SLP_EN)"]
            #[inline(always)]
            pub const fn led0_slp_en(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "LED0 Sleep Enable (LED0_SLP_EN)"]
            #[inline(always)]
            pub fn set_led0_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "SMB4 Sleep Enable (SMB4_SLP_EN)"]
            #[inline(always)]
            pub const fn smb4_slp_en(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "SMB4 Sleep Enable (SMB4_SLP_EN)"]
            #[inline(always)]
            pub fn set_smb4_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "TIMER32_0 Sleep Enable (TIMER32_0_SLP_EN)"]
            #[inline(always)]
            pub const fn tmr32_0_slp_en(&self) -> bool {
                let val = (self.0 >> 23usize) & 0x01;
                val != 0
            }
            #[doc = "TIMER32_0 Sleep Enable (TIMER32_0_SLP_EN)"]
            #[inline(always)]
            pub fn set_tmr32_0_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
            }
            #[doc = "TIMER32_1 Sleep Enable (TIMER32_1_SLP_EN)"]
            #[inline(always)]
            pub const fn tmr32_1_slp_en(&self) -> bool {
                let val = (self.0 >> 24usize) & 0x01;
                val != 0
            }
            #[doc = "TIMER32_1 Sleep Enable (TIMER32_1_SLP_EN)"]
            #[inline(always)]
            pub fn set_tmr32_1_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
            }
            #[doc = "PKE Sleep Enable"]
            #[inline(always)]
            pub const fn pke_slp_en(&self) -> bool {
                let val = (self.0 >> 26usize) & 0x01;
                val != 0
            }
            #[doc = "PKE Sleep Enable"]
            #[inline(always)]
            pub fn set_pke_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
            }
            #[doc = "RNG Sleep Enable"]
            #[inline(always)]
            pub const fn rng_slp_en(&self) -> bool {
                let val = (self.0 >> 27usize) & 0x01;
                val != 0
            }
            #[doc = "RNG Sleep Enable"]
            #[inline(always)]
            pub fn set_rng_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
            }
            #[doc = "AES_HASH Sleep Enable"]
            #[inline(always)]
            pub const fn aes_hash_slp_en(&self) -> bool {
                let val = (self.0 >> 28usize) & 0x01;
                val != 0
            }
            #[doc = "AES_HASH Sleep Enable"]
            #[inline(always)]
            pub fn set_aes_hash_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
            }
            #[doc = "Hibernation TIMER 1 Sleep Enable (HTM_1_SLP_EN)"]
            #[inline(always)]
            pub const fn htm_1_slp_en(&self) -> bool {
                let val = (self.0 >> 29usize) & 0x01;
                val != 0
            }
            #[doc = "Hibernation TIMER 1 Sleep Enable (HTM_1_SLP_EN)"]
            #[inline(always)]
            pub fn set_htm_1_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
            }
            #[doc = "Capture Compare Timer Sleep Enable (CCTIMER_SLP_EN)"]
            #[inline(always)]
            pub const fn cct_slp_en(&self) -> bool {
                let val = (self.0 >> 30usize) & 0x01;
                val != 0
            }
            #[doc = "Capture Compare Timer Sleep Enable (CCTIMER_SLP_EN)"]
            #[inline(always)]
            pub fn set_cct_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
            }
        }
        impl Default for SlpEn3 {
            #[inline(always)]
            fn default() -> SlpEn3 {
                SlpEn3(0)
            }
        }
        impl core::fmt::Debug for SlpEn3 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("SlpEn3")
                    .field("adc_slp_en", &self.adc_slp_en())
                    .field("ps2_0_slp_en", &self.ps2_0_slp_en())
                    .field("htm_0_slp_en", &self.htm_0_slp_en())
                    .field("smb1_slp_en", &self.smb1_slp_en())
                    .field("smb2_slp_en", &self.smb2_slp_en())
                    .field("smb3_slp_en", &self.smb3_slp_en())
                    .field("led0_slp_en", &self.led0_slp_en())
                    .field("smb4_slp_en", &self.smb4_slp_en())
                    .field("tmr32_0_slp_en", &self.tmr32_0_slp_en())
                    .field("tmr32_1_slp_en", &self.tmr32_1_slp_en())
                    .field("pke_slp_en", &self.pke_slp_en())
                    .field("rng_slp_en", &self.rng_slp_en())
                    .field("aes_hash_slp_en", &self.aes_hash_slp_en())
                    .field("htm_1_slp_en", &self.htm_1_slp_en())
                    .field("cct_slp_en", &self.cct_slp_en())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for SlpEn3 {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "SlpEn3 {{ adc_slp_en: {=bool:?}, ps2_0_slp_en: {=bool:?}, htm_0_slp_en: {=bool:?}, smb1_slp_en: {=bool:?}, smb2_slp_en: {=bool:?}, smb3_slp_en: {=bool:?}, led0_slp_en: {=bool:?}, smb4_slp_en: {=bool:?}, tmr32_0_slp_en: {=bool:?}, tmr32_1_slp_en: {=bool:?}, pke_slp_en: {=bool:?}, rng_slp_en: {=bool:?}, aes_hash_slp_en: {=bool:?}, htm_1_slp_en: {=bool:?}, cct_slp_en: {=bool:?} }}" , self . adc_slp_en () , self . ps2_0_slp_en () , self . htm_0_slp_en () , self . smb1_slp_en () , self . smb2_slp_en () , self . smb3_slp_en () , self . led0_slp_en () , self . smb4_slp_en () , self . tmr32_0_slp_en () , self . tmr32_1_slp_en () , self . pke_slp_en () , self . rng_slp_en () , self . aes_hash_slp_en () , self . htm_1_slp_en () , self . cct_slp_en ())
            }
        }
        #[doc = "Sleep Enable 4 Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SlpEn4(pub u32);
        impl SlpEn4 {
            #[doc = "Quad Master SPI Sleep Enable"]
            #[inline(always)]
            pub const fn qmspi_slp_en(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Quad Master SPI Sleep Enable"]
            #[inline(always)]
            pub fn set_qmspi_slp_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
        }
        impl Default for SlpEn4 {
            #[inline(always)]
            fn default() -> SlpEn4 {
                SlpEn4(0)
            }
        }
        impl core::fmt::Debug for SlpEn4 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("SlpEn4")
                    .field("qmspi_slp_en", &self.qmspi_slp_en())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for SlpEn4 {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "SlpEn4 {{ qmspi_slp_en: {=bool:?} }}", self.qmspi_slp_en())
            }
        }
        #[doc = "System Reset Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SysRst(pub u32);
        impl SysRst {
            #[doc = "A write of a 1 forces an assertion of the RESET_SYS reset signal, resetting the device. A write of 0 has no effect."]
            #[inline(always)]
            pub const fn soft_sys_rst(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "A write of a 1 forces an assertion of the RESET_SYS reset signal, resetting the device. A write of 0 has no effect."]
            #[inline(always)]
            pub fn set_soft_sys_rst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
        }
        impl Default for SysRst {
            #[inline(always)]
            fn default() -> SysRst {
                SysRst(0)
            }
        }
        impl core::fmt::Debug for SysRst {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("SysRst")
                    .field("soft_sys_rst", &self.soft_sys_rst())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for SysRst {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "SysRst {{ soft_sys_rst: {=bool:?} }}", self.soft_sys_rst())
            }
        }
        #[doc = "System Sleep Control"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SysSlpCtrl(pub u32);
        impl SysSlpCtrl {
            #[doc = "Selects the System Sleep mode"]
            #[inline(always)]
            pub const fn slp_mod(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Selects the System Sleep mode"]
            #[inline(always)]
            pub fn set_slp_mod(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Test bit"]
            #[inline(always)]
            pub const fn test(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Test bit"]
            #[inline(always)]
            pub fn set_test(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Initiates the System Sleep mode"]
            #[inline(always)]
            pub const fn slp_all(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Initiates the System Sleep mode"]
            #[inline(always)]
            pub fn set_slp_all(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
        }
        impl Default for SysSlpCtrl {
            #[inline(always)]
            fn default() -> SysSlpCtrl {
                SysSlpCtrl(0)
            }
        }
        impl core::fmt::Debug for SysSlpCtrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("SysSlpCtrl")
                    .field("slp_mod", &self.slp_mod())
                    .field("test", &self.test())
                    .field("slp_all", &self.slp_all())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for SysSlpCtrl {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "SysSlpCtrl {{ slp_mod: {=bool:?}, test: {=bool:?}, slp_all: {=bool:?} }}",
                    self.slp_mod(),
                    self.test(),
                    self.slp_all()
                )
            }
        }
    }
    pub mod vals {
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Div(u8);
        impl Div {
            #[doc = "1: divide 48 MHz Ring Oscillator by 1"]
            pub const DIVIDE_BY_1: Self = Self(0x01);
            #[doc = "3: divide 48 MHz Ring Oscillator by 3"]
            pub const DIVIDE_BY_3: Self = Self(0x03);
            #[doc = "4: divide 48 MHz Ring Oscillator by 4"]
            pub const DIVIDE_BY_4: Self = Self(0x04);
            #[doc = "16: divide 48 MHz Ring Oscillator by 16"]
            pub const DIVIDE_BY_16: Self = Self(0x10);
            #[doc = "48: divide 48 MHz Ring Oscillator by 48"]
            pub const DIVIDE_BY_48: Self = Self(0x30);
        }
        impl Div {
            pub const fn from_bits(val: u8) -> Div {
                Self(val & 0xff)
            }
            pub const fn to_bits(self) -> u8 {
                self.0
            }
        }
        impl core::fmt::Debug for Div {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self.0 {
                    0x01 => f.write_str("DIVIDE_BY_1"),
                    0x03 => f.write_str("DIVIDE_BY_3"),
                    0x04 => f.write_str("DIVIDE_BY_4"),
                    0x10 => f.write_str("DIVIDE_BY_16"),
                    0x30 => f.write_str("DIVIDE_BY_48"),
                    other => core::write!(f, "0x{:02X}", other),
                }
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Div {
            fn format(&self, f: defmt::Formatter) {
                match self.0 {
                    0x01 => defmt::write!(f, "DIVIDE_BY_1"),
                    0x03 => defmt::write!(f, "DIVIDE_BY_3"),
                    0x04 => defmt::write!(f, "DIVIDE_BY_4"),
                    0x10 => defmt::write!(f, "DIVIDE_BY_16"),
                    0x30 => defmt::write!(f, "DIVIDE_BY_48"),
                    other => defmt::write!(f, "0x{:02X}", other),
                }
            }
        }
        impl From<u8> for Div {
            #[inline(always)]
            fn from(val: u8) -> Div {
                Div::from_bits(val)
            }
        }
        impl From<Div> for u8 {
            #[inline(always)]
            fn from(val: Div) -> u8 {
                Div::to_bits(val)
            }
        }
    }
}
pub mod pwm0 {
    #[doc = "The PWM block generates an arbitrary duty cycle output at frequencies from less than 0.1 Hz to 24 MHz"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwm0 {
        ptr: *mut u8,
    }
    unsafe impl Send for Pwm0 {}
    unsafe impl Sync for Pwm0 {}
    impl Pwm0 {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "This field determines both the frequency and duty cycle of the PWM signal. Setting this field to a value of n will cause the On time of the PWM to be n+1 cycles of the PWM Clock Source. When this field is set to zero and the PWMX_COUNTER_OFF_TIME is not set to zero, the PWM_OUTPUT is held low (Full Off)."]
        #[inline(always)]
        pub const fn cnt_on(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "This field determine both the frequency and duty cycle of the PWM signal. Setting this field to a value of n will cause the Off time of the PWM to be n+1 cycles of the PWM Clock Source. When this field is set to zero, the PWM_OUTPUT is held high (Full On)."]
        #[inline(always)]
        pub const fn cnt_off(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "PWMx CONFIGURATION REGISTER"]
        #[inline(always)]
        pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "PWMx CONFIGURATION REGISTER"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cfg(pub u32);
        impl Cfg {
            #[doc = "When the PWM_ENABLE is set to 0 the internal counters are reset and the internal state machine is set to the OFF state. In addition, the PWM_OUTPUT signal is set to the inactive state as determined by the Invert bit. The PWMx Counter ON Time Register and PWMx Counter OFF Time Register are not affected by the PWM_ENABLE bit and may be read and written while the PWM enable bit is 0. 1=Enabled (default); 0=Disabled (gates clocks to save power)."]
            #[inline(always)]
            pub const fn pwm_en(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "When the PWM_ENABLE is set to 0 the internal counters are reset and the internal state machine is set to the OFF state. In addition, the PWM_OUTPUT signal is set to the inactive state as determined by the Invert bit. The PWMx Counter ON Time Register and PWMx Counter OFF Time Register are not affected by the PWM_ENABLE bit and may be read and written while the PWM enable bit is 0. 1=Enabled (default); 0=Disabled (gates clocks to save power)."]
            #[inline(always)]
            pub fn set_pwm_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "This bit determines the clock source used by the PWM duty cycle and frequency control logic. 1=CLOCK_LOW 0=CLOCK_HIGH"]
            #[inline(always)]
            pub const fn clk_sel(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "This bit determines the clock source used by the PWM duty cycle and frequency control logic. 1=CLOCK_LOW 0=CLOCK_HIGH"]
            #[inline(always)]
            pub fn set_clk_sel(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "1= PWM_OUTPUT ON State is active low; 0=PWM_OUTPUT ON State is active high."]
            #[inline(always)]
            pub const fn inv(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "1= PWM_OUTPUT ON State is active low; 0=PWM_OUTPUT ON State is active high."]
            #[inline(always)]
            pub fn set_inv(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "The Clock source for the 16-bit down counter (see PWMx Counter ON Time Register and PWMx Counter OFF Time Register) is determined by bit D1 of this register. The Clock source is then divided by the value of Pre-Divider+1 and the resulting signal determines the rate at which the down counter will be decremented. For example, a Pre-Divider value of 1 divides the input clock by 2 and a value of 2 divides the input clock by 3. A Pre-Divider of 0 will disable the Pre-Divider option."]
            #[inline(always)]
            pub const fn clk_pre_div(&self) -> u8 {
                let val = (self.0 >> 3usize) & 0x0f;
                val as u8
            }
            #[doc = "The Clock source for the 16-bit down counter (see PWMx Counter ON Time Register and PWMx Counter OFF Time Register) is determined by bit D1 of this register. The Clock source is then divided by the value of Pre-Divider+1 and the resulting signal determines the rate at which the down counter will be decremented. For example, a Pre-Divider value of 1 divides the input clock by 2 and a value of 2 divides the input clock by 3. A Pre-Divider of 0 will disable the Pre-Divider option."]
            #[inline(always)]
            pub fn set_clk_pre_div(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
            }
        }
        impl Default for Cfg {
            #[inline(always)]
            fn default() -> Cfg {
                Cfg(0)
            }
        }
        impl core::fmt::Debug for Cfg {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Cfg")
                    .field("pwm_en", &self.pwm_en())
                    .field("clk_sel", &self.clk_sel())
                    .field("inv", &self.inv())
                    .field("clk_pre_div", &self.clk_pre_div())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Cfg {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Cfg {{ pwm_en: {=bool:?}, clk_sel: {=bool:?}, inv: {=bool:?}, clk_pre_div: {=u8:?} }}",
                    self.pwm_en(),
                    self.clk_sel(),
                    self.inv(),
                    self.clk_pre_div()
                )
            }
        }
    }
}
pub mod qmspi {
    #[doc = "The QMSPI may be used to communicate with various peripheral devices that use a Serial Peripheral Interface"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmspi {
        ptr: *mut u8,
    }
    unsafe impl Send for Qmspi {}
    unsafe impl Sync for Qmspi {}
    impl Qmspi {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "QMSPI Mode Register"]
        #[inline(always)]
        pub const fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "QMSPI SPI Control"]
        #[inline(always)]
        pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "QMSPI Execute Register"]
        #[inline(always)]
        pub const fn exe(self) -> crate::common::Reg<regs::Exe, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "QMSPI Interface Control Register"]
        #[inline(always)]
        pub const fn ifctrl(self) -> crate::common::Reg<regs::Ifctrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "QMSPI Status Register"]
        #[inline(always)]
        pub const fn sts(self) -> crate::common::Reg<regs::Sts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "QMSPI Buffer Count Status Register"]
        #[inline(always)]
        pub const fn buf_cnt_sts(self) -> crate::common::Reg<regs::BufCntSts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "QMSPI Interrupt Enable Register"]
        #[inline(always)]
        pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
        }
        #[doc = "QMSPI Buffer Count Trigger Register"]
        #[inline(always)]
        pub const fn buf_cnt_trig(self) -> crate::common::Reg<regs::BufCntTrig, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
        #[doc = "QMSPI Transmit Buffer Register"]
        #[inline(always)]
        pub const fn tx_fifo(self) -> crate::common::Reg<regs::TxFifo, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }
        #[doc = "QMSPI Receive Buffer Register"]
        #[inline(always)]
        pub const fn rx_fifo(self) -> crate::common::Reg<regs::RxFifo, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
        }
        #[doc = "QMSPI Chip Select Timing Register"]
        #[inline(always)]
        pub const fn cstm(self) -> crate::common::Reg<regs::Cstm, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
        }
        #[doc = "QMSPI Description Buffer 0 Register"]
        #[inline(always)]
        pub const fn descr(self, n: usize) -> crate::common::Reg<regs::Descr, crate::common::RW> {
            assert!(n < 16usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize + n * 4usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "QMSPI Buffer Count Status Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct BufCntSts(pub u32);
        impl BufCntSts {
            #[doc = "This is a count of the number of bytes currently valid in the Transmit Buffer."]
            #[inline(always)]
            pub const fn tx_buff_cnt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "This is a count of the number of bytes currently valid in the Transmit Buffer."]
            #[inline(always)]
            pub fn set_tx_buff_cnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
            #[doc = "This is a count of the number of bytes currently valid in the Receive Buffer."]
            #[inline(always)]
            pub const fn rx_buff_cnt(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0xffff;
                val as u16
            }
            #[doc = "This is a count of the number of bytes currently valid in the Receive Buffer."]
            #[inline(always)]
            pub fn set_rx_buff_cnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
            }
        }
        impl Default for BufCntSts {
            #[inline(always)]
            fn default() -> BufCntSts {
                BufCntSts(0)
            }
        }
        impl core::fmt::Debug for BufCntSts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("BufCntSts")
                    .field("tx_buff_cnt", &self.tx_buff_cnt())
                    .field("rx_buff_cnt", &self.rx_buff_cnt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for BufCntSts {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "BufCntSts {{ tx_buff_cnt: {=u16:?}, rx_buff_cnt: {=u16:?} }}",
                    self.tx_buff_cnt(),
                    self.rx_buff_cnt()
                )
            }
        }
        #[doc = "QMSPI Buffer Count Trigger Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct BufCntTrig(pub u32);
        impl BufCntTrig {
            #[doc = "An interrupt is triggered if the TRANSMIT_BUFFER_COUNT field is less than or equal to this value. A value of 0 disables the interrupt."]
            #[inline(always)]
            pub const fn tx_buf_trig(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "An interrupt is triggered if the TRANSMIT_BUFFER_COUNT field is less than or equal to this value. A value of 0 disables the interrupt."]
            #[inline(always)]
            pub fn set_tx_buf_trig(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
            #[doc = "An interrupt is triggered if the RECEIVE_BUFFER_COUNT field is greater than or equal to this value. A value of 0 disables the interrupt."]
            #[inline(always)]
            pub const fn rx_buf_trig(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0xffff;
                val as u16
            }
            #[doc = "An interrupt is triggered if the RECEIVE_BUFFER_COUNT field is greater than or equal to this value. A value of 0 disables the interrupt."]
            #[inline(always)]
            pub fn set_rx_buf_trig(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
            }
        }
        impl Default for BufCntTrig {
            #[inline(always)]
            fn default() -> BufCntTrig {
                BufCntTrig(0)
            }
        }
        impl core::fmt::Debug for BufCntTrig {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("BufCntTrig")
                    .field("tx_buf_trig", &self.tx_buf_trig())
                    .field("rx_buf_trig", &self.rx_buf_trig())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for BufCntTrig {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "BufCntTrig {{ tx_buf_trig: {=u16:?}, rx_buf_trig: {=u16:?} }}",
                    self.tx_buf_trig(),
                    self.rx_buf_trig()
                )
            }
        }
        #[doc = "QMSPI Chip Select Timing Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cstm(pub u32);
        impl Cstm {
            #[inline(always)]
            pub const fn dly_cs_on_clk_strt(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[inline(always)]
            pub fn set_dly_cs_on_clk_strt(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
            #[doc = "This selects the number of system clock cycles between the last clock edge and the deassertion of CS."]
            #[inline(always)]
            pub const fn dly_clk_stop_cs_off(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0x0f;
                val as u8
            }
            #[doc = "This selects the number of system clock cycles between the last clock edge and the deassertion of CS."]
            #[inline(always)]
            pub fn set_dly_clk_stop_cs_off(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
            }
            #[doc = "This selects the number of system clock cycles between CS deassertion to the data ports for WP and HOLD switching from input to output. This is only used if the WP/HOLD functions are in use and only on IO2/WP and IO3/HOLD pins."]
            #[inline(always)]
            pub const fn dly_last_dat_hld(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0x0f;
                val as u8
            }
            #[doc = "This selects the number of system clock cycles between CS deassertion to the data ports for WP and HOLD switching from input to output. This is only used if the WP/HOLD functions are in use and only on IO2/WP and IO3/HOLD pins."]
            #[inline(always)]
            pub fn set_dly_last_dat_hld(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
            }
            #[doc = "This selects the number of system clock cycles between CS deassertion to CS assertion. This is the minimum pulse width of CS deassertion."]
            #[inline(always)]
            pub const fn dly_off_to_on(&self) -> u16 {
                let val = (self.0 >> 23usize) & 0x01ff;
                val as u16
            }
            #[doc = "This selects the number of system clock cycles between CS deassertion to CS assertion. This is the minimum pulse width of CS deassertion."]
            #[inline(always)]
            pub fn set_dly_off_to_on(&mut self, val: u16) {
                self.0 = (self.0 & !(0x01ff << 23usize)) | (((val as u32) & 0x01ff) << 23usize);
            }
        }
        impl Default for Cstm {
            #[inline(always)]
            fn default() -> Cstm {
                Cstm(0)
            }
        }
        impl core::fmt::Debug for Cstm {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Cstm")
                    .field("dly_cs_on_clk_strt", &self.dly_cs_on_clk_strt())
                    .field("dly_clk_stop_cs_off", &self.dly_clk_stop_cs_off())
                    .field("dly_last_dat_hld", &self.dly_last_dat_hld())
                    .field("dly_off_to_on", &self.dly_off_to_on())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Cstm {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Cstm {{ dly_cs_on_clk_strt: {=u8:?}, dly_clk_stop_cs_off: {=u8:?}, dly_last_dat_hld: {=u8:?}, dly_off_to_on: {=u16:?} }}" , self . dly_cs_on_clk_strt () , self . dly_clk_stop_cs_off () , self . dly_last_dat_hld () , self . dly_off_to_on ())
            }
        }
        #[doc = "QMSPI SPI Control"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctrl(pub u32);
        impl Ctrl {
            #[doc = "This field sets the transmission mode. If this field is set for Dual Mode or Quad Mode then either TX_TRANSFER_ENABLE or RX_TRANSFER_ENABLE must be 0. 3=Reserved; 2=Quad Mode; 1=Dual Mode; 0=Single/Duplex Mode."]
            #[inline(always)]
            pub const fn tx_mode(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x03;
                val as u8
            }
            #[doc = "This field sets the transmission mode. If this field is set for Dual Mode or Quad Mode then either TX_TRANSFER_ENABLE or RX_TRANSFER_ENABLE must be 0. 3=Reserved; 2=Quad Mode; 1=Dual Mode; 0=Single/Duplex Mode."]
            #[inline(always)]
            pub fn set_tx_mode(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
            }
            #[doc = "This field bit selects the transmit function of the SPI interface. 3=Transmit Enabled in 1 Mode. The MOSI or IO Bus will send out only 1's. The Transmit Buffer will not be used. 2=Transmit Enabled in 0 Mode. The MOSI or IO Bus will send out only 0's. The Transmit Buffer will not be used. 1=Transmit Enabled. Data will be fetched from the Transmit Buffer and sent out on the MOSI or IO Bus. 0=Transmit is Disabled. Not data is sent. This will cause the MOSI be to be undriven, or the IO bus to be undriven if Receive is also disabled."]
            #[inline(always)]
            pub const fn tx_trans_en(&self) -> u8 {
                let val = (self.0 >> 2usize) & 0x03;
                val as u8
            }
            #[doc = "This field bit selects the transmit function of the SPI interface. 3=Transmit Enabled in 1 Mode. The MOSI or IO Bus will send out only 1's. The Transmit Buffer will not be used. 2=Transmit Enabled in 0 Mode. The MOSI or IO Bus will send out only 0's. The Transmit Buffer will not be used. 1=Transmit Enabled. Data will be fetched from the Transmit Buffer and sent out on the MOSI or IO Bus. 0=Transmit is Disabled. Not data is sent. This will cause the MOSI be to be undriven, or the IO bus to be undriven if Receive is also disabled."]
            #[inline(always)]
            pub fn set_tx_trans_en(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
            }
            #[doc = "This bit enables DMA support for Transmit Transfer. If enabled, DMA will be requested to fill the FIFO until either the interface reaches TRANSFER_LENGTH or the DMA sends a termination request. The size defined here must match DMA programmed access size. 1=DMA is enabled.and set to 1 Byte 2=DMA is enabled and set to 2 Bytes 3=DMA is enabled and set to 4 Bytes. 0=DMA is disabled. All data in the Transmit Buffer must be emptied by firmware"]
            #[inline(always)]
            pub const fn tx_dma_en(&self) -> u8 {
                let val = (self.0 >> 4usize) & 0x03;
                val as u8
            }
            #[doc = "This bit enables DMA support for Transmit Transfer. If enabled, DMA will be requested to fill the FIFO until either the interface reaches TRANSFER_LENGTH or the DMA sends a termination request. The size defined here must match DMA programmed access size. 1=DMA is enabled.and set to 1 Byte 2=DMA is enabled and set to 2 Bytes 3=DMA is enabled and set to 4 Bytes. 0=DMA is disabled. All data in the Transmit Buffer must be emptied by firmware"]
            #[inline(always)]
            pub fn set_tx_dma_en(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
            }
            #[doc = "This bit enables the receive function of the SPI interface. 1=Receive is enabled. Data received from the SPI Slave is stored in the Receive Buffer 0=Receive is disabled"]
            #[inline(always)]
            pub const fn rx_trans_en(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "This bit enables the receive function of the SPI interface. 1=Receive is enabled. Data received from the SPI Slave is stored in the Receive Buffer 0=Receive is disabled"]
            #[inline(always)]
            pub fn set_rx_trans_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "This bit enables DMA support for Receive Transfer. If enabled, DMA will be requested to empty the FIFO until either the interface reaches TRANSFER_LENGTH or the DMA sends a termination request. The size defined here must match DMA programmed access size. 1=DMA is enabled.and set to 1 Byte 2=DMA is enabled and set to 2 Bytes 3=DMA is enabled and set to 4 Bytes 0=DMA is disabled. All data in the Receive Buffer must be emptied by firmware"]
            #[inline(always)]
            pub const fn rx_dma_en(&self) -> u8 {
                let val = (self.0 >> 7usize) & 0x03;
                val as u8
            }
            #[doc = "This bit enables DMA support for Receive Transfer. If enabled, DMA will be requested to empty the FIFO until either the interface reaches TRANSFER_LENGTH or the DMA sends a termination request. The size defined here must match DMA programmed access size. 1=DMA is enabled.and set to 1 Byte 2=DMA is enabled and set to 2 Bytes 3=DMA is enabled and set to 4 Bytes 0=DMA is disabled. All data in the Receive Buffer must be emptied by firmware"]
            #[inline(always)]
            pub fn set_rx_dma_en(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
            }
            #[doc = "This selects what action is taken at the end of a transfer. When the transaction closes, the Chip Select de-asserts, the SPI interface returns to IDLE and the DMA interface terminates When Description Buffers are in use this bit must be set only on the Last Buffer. 1=The transaction is terminated 0=The transaction is not terminated"]
            #[inline(always)]
            pub const fn close_trans_en(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "This selects what action is taken at the end of a transfer. When the transaction closes, the Chip Select de-asserts, the SPI interface returns to IDLE and the DMA interface terminates When Description Buffers are in use this bit must be set only on the Last Buffer. 1=The transaction is terminated 0=The transaction is not terminated"]
            #[inline(always)]
            pub fn set_close_trans_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "3=TRANSFER_LENGTH defined in units of 16-byte segments 2=TRANSFER_LENGTH defined in units of 4-byte segments 1=TRANSFER_LENGTH defined in units of bytes 0=TRANSFER_LENGTH defined in units of bits."]
            #[inline(always)]
            pub const fn trans_units(&self) -> u8 {
                let val = (self.0 >> 10usize) & 0x03;
                val as u8
            }
            #[doc = "3=TRANSFER_LENGTH defined in units of 16-byte segments 2=TRANSFER_LENGTH defined in units of 4-byte segments 1=TRANSFER_LENGTH defined in units of bytes 0=TRANSFER_LENGTH defined in units of bits."]
            #[inline(always)]
            pub fn set_trans_units(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
            }
            #[doc = "This field selects the first buffer used if Description Buffers are enabled."]
            #[inline(always)]
            pub const fn descr_buff_ptr(&self) -> u8 {
                let val = (self.0 >> 12usize) & 0x0f;
                val as u8
            }
            #[doc = "This field selects the first buffer used if Description Buffers are enabled."]
            #[inline(always)]
            pub fn set_descr_buff_ptr(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
            }
            #[doc = "This enables the Description Buffers to be used. 1=Description Buffers in use. The first buffer is defined in DESCRIPTION_BUFFER_POINTER 0=Description Buffers disabled."]
            #[inline(always)]
            pub const fn descr_buff_en(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "This enables the Description Buffers to be used. 1=Description Buffers in use. The first buffer is defined in DESCRIPTION_BUFFER_POINTER 0=Description Buffers disabled."]
            #[inline(always)]
            pub fn set_descr_buff_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "The length of the SPI transfer. The count is in bytes or bits, depending on the value of TRANSFER_LENGTH_BITS. A value of 0 means an infinite length transfer."]
            #[inline(always)]
            pub const fn trans_len(&self) -> u16 {
                let val = (self.0 >> 17usize) & 0x7fff;
                val as u16
            }
            #[doc = "The length of the SPI transfer. The count is in bytes or bits, depending on the value of TRANSFER_LENGTH_BITS. A value of 0 means an infinite length transfer."]
            #[inline(always)]
            pub fn set_trans_len(&mut self, val: u16) {
                self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
            }
        }
        impl Default for Ctrl {
            #[inline(always)]
            fn default() -> Ctrl {
                Ctrl(0)
            }
        }
        impl core::fmt::Debug for Ctrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ctrl")
                    .field("tx_mode", &self.tx_mode())
                    .field("tx_trans_en", &self.tx_trans_en())
                    .field("tx_dma_en", &self.tx_dma_en())
                    .field("rx_trans_en", &self.rx_trans_en())
                    .field("rx_dma_en", &self.rx_dma_en())
                    .field("close_trans_en", &self.close_trans_en())
                    .field("trans_units", &self.trans_units())
                    .field("descr_buff_ptr", &self.descr_buff_ptr())
                    .field("descr_buff_en", &self.descr_buff_en())
                    .field("trans_len", &self.trans_len())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ctrl {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Ctrl {{ tx_mode: {=u8:?}, tx_trans_en: {=u8:?}, tx_dma_en: {=u8:?}, rx_trans_en: {=bool:?}, rx_dma_en: {=u8:?}, close_trans_en: {=bool:?}, trans_units: {=u8:?}, descr_buff_ptr: {=u8:?}, descr_buff_en: {=bool:?}, trans_len: {=u16:?} }}" , self . tx_mode () , self . tx_trans_en () , self . tx_dma_en () , self . rx_trans_en () , self . rx_dma_en () , self . close_trans_en () , self . trans_units () , self . descr_buff_ptr () , self . descr_buff_en () , self . trans_len ())
            }
        }
        #[doc = "QMSPI Description Buffer 0 Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Descr(pub u32);
        impl Descr {
            #[doc = "This field sets the transmission mode. If this field is set for Dual Mode or Quad Mode then either TX_TRANSFER_ENABLE or RX_TRANSFER_ENABLE must be 0. 3=Reserved; 2=Quad Mode; 1=Dual Mode; 0=Single/Duplex Mode."]
            #[inline(always)]
            pub const fn inface_mod(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x03;
                val as u8
            }
            #[doc = "This field sets the transmission mode. If this field is set for Dual Mode or Quad Mode then either TX_TRANSFER_ENABLE or RX_TRANSFER_ENABLE must be 0. 3=Reserved; 2=Quad Mode; 1=Dual Mode; 0=Single/Duplex Mode."]
            #[inline(always)]
            pub fn set_inface_mod(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
            }
            #[doc = "This field bit selects the transmit function of the SPI interface. 3=Transmit Enabled in 1 Mode. The MOSI or IO Bus will send out only 1's. The Transmit Buffer will not be used 2=Transmit Enabled in 0 Mode. The MOSI or IO Bus will send out only 0's. The Transmit Buffer will not be used. 1=Transmit Enabled. Data will be fetched from the Transmit Buffer and sent out on the MOSI or IO Bus. 0=Transmit is Disabled. No data is sent. This will cause the MOSI be to be undriven, or the IO bus to be undriven if Receive is also disabled."]
            #[inline(always)]
            pub const fn tx_trans_en(&self) -> u8 {
                let val = (self.0 >> 2usize) & 0x03;
                val as u8
            }
            #[doc = "This field bit selects the transmit function of the SPI interface. 3=Transmit Enabled in 1 Mode. The MOSI or IO Bus will send out only 1's. The Transmit Buffer will not be used 2=Transmit Enabled in 0 Mode. The MOSI or IO Bus will send out only 0's. The Transmit Buffer will not be used. 1=Transmit Enabled. Data will be fetched from the Transmit Buffer and sent out on the MOSI or IO Bus. 0=Transmit is Disabled. No data is sent. This will cause the MOSI be to be undriven, or the IO bus to be undriven if Receive is also disabled."]
            #[inline(always)]
            pub fn set_tx_trans_en(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
            }
            #[doc = "This bit enables DMA support for Transmit Transfer. If enabled, DMA will be requested to fill the FIFO until either the interface reaches TRANSFER_LENGTH or the DMA sends a termination request. The size defined here must match DMA programmed access size. 1=DMA is enabled.and set to 1 Byte 2=DMA is enabled and set to 2 Bytes 3=DMA is enabled and set to 4 Bytes 0=DMA is disabled. All data in the Transmit Buffer must be emptied by firmware."]
            #[inline(always)]
            pub const fn tx_dma_en(&self) -> u8 {
                let val = (self.0 >> 4usize) & 0x03;
                val as u8
            }
            #[doc = "This bit enables DMA support for Transmit Transfer. If enabled, DMA will be requested to fill the FIFO until either the interface reaches TRANSFER_LENGTH or the DMA sends a termination request. The size defined here must match DMA programmed access size. 1=DMA is enabled.and set to 1 Byte 2=DMA is enabled and set to 2 Bytes 3=DMA is enabled and set to 4 Bytes 0=DMA is disabled. All data in the Transmit Buffer must be emptied by firmware."]
            #[inline(always)]
            pub fn set_tx_dma_en(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
            }
            #[doc = "This bit enables the receive function of the SPI interface. 1=Receive is enabled. Data received from the SPI Slave is stored in the Receive Buffer; 0=Receive is disabled."]
            #[inline(always)]
            pub const fn rx_trans_en(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "This bit enables the receive function of the SPI interface. 1=Receive is enabled. Data received from the SPI Slave is stored in the Receive Buffer; 0=Receive is disabled."]
            #[inline(always)]
            pub fn set_rx_trans_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "This bit enables DMA support for Receive Transfer. If enabled, DMA will be requested to empty the FIFO until either the interface reaches TRANSFER_LENGTH or the DMA sends a termination request. The size defined here must match DMA programmed access size. 1=DMA is enabled.and set to 1 Byte 2=DMA is enabled and set to 2 Bytes 3=DMA is enabled and set to 4 Bytes 0=DMA is disabled. All data in the Receive Buffer must be emptied by firmware."]
            #[inline(always)]
            pub const fn rx_dma_en(&self) -> u8 {
                let val = (self.0 >> 7usize) & 0x03;
                val as u8
            }
            #[doc = "This bit enables DMA support for Receive Transfer. If enabled, DMA will be requested to empty the FIFO until either the interface reaches TRANSFER_LENGTH or the DMA sends a termination request. The size defined here must match DMA programmed access size. 1=DMA is enabled.and set to 1 Byte 2=DMA is enabled and set to 2 Bytes 3=DMA is enabled and set to 4 Bytes 0=DMA is disabled. All data in the Receive Buffer must be emptied by firmware."]
            #[inline(always)]
            pub fn set_rx_dma_en(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
            }
            #[doc = "This selects what action is taken at the end of a transfer. This bit must be set only on the Last Buffer. 1=The transfer is terminated. The Chip Select de-asserts, the SPI interface returns to IDLE and the DMA interface completes the transfer. 0=The transfer is not closed. Chip Select remains asserted and the DMA interface and the SPI interface remain active"]
            #[inline(always)]
            pub const fn close_trans_en(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "This selects what action is taken at the end of a transfer. This bit must be set only on the Last Buffer. 1=The transfer is terminated. The Chip Select de-asserts, the SPI interface returns to IDLE and the DMA interface completes the transfer. 0=The transfer is not closed. Chip Select remains asserted and the DMA interface and the SPI interface remain active"]
            #[inline(always)]
            pub fn set_close_trans_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "1=TRANSFER_LENGTH defined in bits 0=TRANSFER_LENGTH defined in bytes"]
            #[inline(always)]
            pub const fn trans_len_bits(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "1=TRANSFER_LENGTH defined in bits 0=TRANSFER_LENGTH defined in bytes"]
            #[inline(always)]
            pub fn set_trans_len_bits(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "If this bit is 1 then this is the last Description Buffer in the chain. When the transfer described by this buffer completes the TRANSFER_COMPLETE status will be set to 1. If this bit is 0, then this is not the last buffer in use. When the transfer completes the next buffer will be activated, and no additional status will be asserted."]
            #[inline(always)]
            pub const fn descr_buf_last(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "If this bit is 1 then this is the last Description Buffer in the chain. When the transfer described by this buffer completes the TRANSFER_COMPLETE status will be set to 1. If this bit is 0, then this is not the last buffer in use. When the transfer completes the next buffer will be activated, and no additional status will be asserted."]
            #[inline(always)]
            pub fn set_descr_buf_last(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "This defines the next buffer to be used if Description Buffers are enabled and this is not the last buffer. This can point to the current buffer, creating an infinite loop."]
            #[inline(always)]
            pub const fn descr_buf_nxt_ptr(&self) -> u8 {
                let val = (self.0 >> 12usize) & 0x0f;
                val as u8
            }
            #[doc = "This defines the next buffer to be used if Description Buffers are enabled and this is not the last buffer. This can point to the current buffer, creating an infinite loop."]
            #[inline(always)]
            pub fn set_descr_buf_nxt_ptr(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
            }
            #[doc = "The length of the SPI transfer. The count is in bytes or bits, depending on the value of TRANSFER_LENGTH_BITS. A value of 0 means an infinite length transfer."]
            #[inline(always)]
            pub const fn tx_len(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0xffff;
                val as u16
            }
            #[doc = "The length of the SPI transfer. The count is in bytes or bits, depending on the value of TRANSFER_LENGTH_BITS. A value of 0 means an infinite length transfer."]
            #[inline(always)]
            pub fn set_tx_len(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
            }
        }
        impl Default for Descr {
            #[inline(always)]
            fn default() -> Descr {
                Descr(0)
            }
        }
        impl core::fmt::Debug for Descr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Descr")
                    .field("inface_mod", &self.inface_mod())
                    .field("tx_trans_en", &self.tx_trans_en())
                    .field("tx_dma_en", &self.tx_dma_en())
                    .field("rx_trans_en", &self.rx_trans_en())
                    .field("rx_dma_en", &self.rx_dma_en())
                    .field("close_trans_en", &self.close_trans_en())
                    .field("trans_len_bits", &self.trans_len_bits())
                    .field("descr_buf_last", &self.descr_buf_last())
                    .field("descr_buf_nxt_ptr", &self.descr_buf_nxt_ptr())
                    .field("tx_len", &self.tx_len())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Descr {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Descr {{ inface_mod: {=u8:?}, tx_trans_en: {=u8:?}, tx_dma_en: {=u8:?}, rx_trans_en: {=bool:?}, rx_dma_en: {=u8:?}, close_trans_en: {=bool:?}, trans_len_bits: {=bool:?}, descr_buf_last: {=bool:?}, descr_buf_nxt_ptr: {=u8:?}, tx_len: {=u16:?} }}" , self . inface_mod () , self . tx_trans_en () , self . tx_dma_en () , self . rx_trans_en () , self . rx_dma_en () , self . close_trans_en () , self . trans_len_bits () , self . descr_buf_last () , self . descr_buf_nxt_ptr () , self . tx_len ())
            }
        }
        #[doc = "QMSPI Execute Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Exe(pub u32);
        impl Exe {
            #[doc = "Writing a 1 to this bit will start the SPI transfer. Writing a 0 to this bit has no effect. This bit is self-clearing. This bit must not be set to 1 if the field STOP in this register is set to 1."]
            #[inline(always)]
            pub const fn start(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Writing a 1 to this bit will start the SPI transfer. Writing a 0 to this bit has no effect. This bit is self-clearing. This bit must not be set to 1 if the field STOP in this register is set to 1."]
            #[inline(always)]
            pub fn set_start(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Writing a 1 to this bit will stop any transfer in progress at the next byte boundary. Writing a 0 to this bit has no effect. This bit is self clearing. This bit must not be set to 1 if the field START in this register is set to 1."]
            #[inline(always)]
            pub const fn stop(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Writing a 1 to this bit will stop any transfer in progress at the next byte boundary. Writing a 0 to this bit has no effect. This bit is self clearing. This bit must not be set to 1 if the field START in this register is set to 1."]
            #[inline(always)]
            pub fn set_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Writing a 1 to this bit will clear out the Transmit and Receive FIFOs. Any data stored in the FIFOs is discarded and all count fields are reset. Writing a 0 to this bit has no effect. This bit is self clearing."]
            #[inline(always)]
            pub const fn clr_dat_buff(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Writing a 1 to this bit will clear out the Transmit and Receive FIFOs. Any data stored in the FIFOs is discarded and all count fields are reset. Writing a 0 to this bit has no effect. This bit is self clearing."]
            #[inline(always)]
            pub fn set_clr_dat_buff(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
        }
        impl Default for Exe {
            #[inline(always)]
            fn default() -> Exe {
                Exe(0)
            }
        }
        impl core::fmt::Debug for Exe {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Exe")
                    .field("start", &self.start())
                    .field("stop", &self.stop())
                    .field("clr_dat_buff", &self.clr_dat_buff())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Exe {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Exe {{ start: {=bool:?}, stop: {=bool:?}, clr_dat_buff: {=bool:?} }}",
                    self.start(),
                    self.stop(),
                    self.clr_dat_buff()
                )
            }
        }
        #[doc = "QMSPI Interrupt Enable Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ien(pub u32);
        impl Ien {
            #[doc = "1=Enable an interrupt if TRANSFER_COMPLETE is asserted 0=Disable the interrupt."]
            #[inline(always)]
            pub const fn trans_compl_en(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "1=Enable an interrupt if TRANSFER_COMPLETE is asserted 0=Disable the interrupt."]
            #[inline(always)]
            pub fn set_trans_compl_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "1=Enable an interrupt if DMA_COMPLETE is asserted 0=Disable the interrupt."]
            #[inline(always)]
            pub const fn dma_compl_en(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "1=Enable an interrupt if DMA_COMPLETE is asserted 0=Disable the interrupt."]
            #[inline(always)]
            pub fn set_dma_compl_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "1=Enable an interrupt if TRANSMIT_BUFFER_ERROR is asserted 0=Disable the interrupt."]
            #[inline(always)]
            pub const fn tx_buf_err_en(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "1=Enable an interrupt if TRANSMIT_BUFFER_ERROR is asserted 0=Disable the interrupt."]
            #[inline(always)]
            pub fn set_tx_buf_err_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "1=Enable an interrupt if RECEIVE_BUFFER_ERROR is asserted 0=Disable the interrupt."]
            #[inline(always)]
            pub const fn rx_buf_err_en(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "1=Enable an interrupt if RECEIVE_BUFFER_ERROR is asserted 0=Disable the interrupt."]
            #[inline(always)]
            pub fn set_rx_buf_err_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "1=Enable an interrupt if PROGRAMMING_ERROR is asserted 0=Disable the interrupt."]
            #[inline(always)]
            pub const fn prgm_err_en(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "1=Enable an interrupt if PROGRAMMING_ERROR is asserted 0=Disable the interrupt."]
            #[inline(always)]
            pub fn set_prgm_err_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "1=Enable an interrupt if TRANSMIT_BUFFER_FULL is asserted 0=Disable the interrupt."]
            #[inline(always)]
            pub const fn tx_buf_full_en(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "1=Enable an interrupt if TRANSMIT_BUFFER_FULL is asserted 0=Disable the interrupt."]
            #[inline(always)]
            pub fn set_tx_buf_full_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "1=Enable an interrupt if TRANSMIT_BUFFER_EMPTY is asserted 0=Disable the interrupt."]
            #[inline(always)]
            pub const fn tx_buf_empty_en(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "1=Enable an interrupt if TRANSMIT_BUFFER_EMPTY is asserted 0=Disable the interrupt."]
            #[inline(always)]
            pub fn set_tx_buf_empty_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "1=Enable an interrupt if TRANSMIT_BUFFER_REQUEST is asserted 0=Disable the interrupt."]
            #[inline(always)]
            pub const fn tx_buf_req_en(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "1=Enable an interrupt if TRANSMIT_BUFFER_REQUEST is asserted 0=Disable the interrupt."]
            #[inline(always)]
            pub fn set_tx_buf_req_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "1=Enable an interrupt if RECEIVE_BUFFER_FULL is asserted 0=Disable the interrupt."]
            #[inline(always)]
            pub const fn rx_buf_ful_en(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "1=Enable an interrupt if RECEIVE_BUFFER_FULL is asserted 0=Disable the interrupt."]
            #[inline(always)]
            pub fn set_rx_buf_ful_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "1=Enable an interrupt if RECEIVE_BUFFER_EMPTY is asserted 0=Disable the interrupt."]
            #[inline(always)]
            pub const fn rx_buf_empty_en(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "1=Enable an interrupt if RECEIVE_BUFFER_EMPTY is asserted 0=Disable the interrupt."]
            #[inline(always)]
            pub fn set_rx_buf_empty_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "1=Enable an interrupt if RECEIVE_BUFFER_REQUEST is asserted 0=Disable the interrupt."]
            #[inline(always)]
            pub const fn rx_buf_req_en(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "1=Enable an interrupt if RECEIVE_BUFFER_REQUEST is asserted 0=Disable the interrupt."]
            #[inline(always)]
            pub fn set_rx_buf_req_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
        }
        impl Default for Ien {
            #[inline(always)]
            fn default() -> Ien {
                Ien(0)
            }
        }
        impl core::fmt::Debug for Ien {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ien")
                    .field("trans_compl_en", &self.trans_compl_en())
                    .field("dma_compl_en", &self.dma_compl_en())
                    .field("tx_buf_err_en", &self.tx_buf_err_en())
                    .field("rx_buf_err_en", &self.rx_buf_err_en())
                    .field("prgm_err_en", &self.prgm_err_en())
                    .field("tx_buf_full_en", &self.tx_buf_full_en())
                    .field("tx_buf_empty_en", &self.tx_buf_empty_en())
                    .field("tx_buf_req_en", &self.tx_buf_req_en())
                    .field("rx_buf_ful_en", &self.rx_buf_ful_en())
                    .field("rx_buf_empty_en", &self.rx_buf_empty_en())
                    .field("rx_buf_req_en", &self.rx_buf_req_en())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ien {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Ien {{ trans_compl_en: {=bool:?}, dma_compl_en: {=bool:?}, tx_buf_err_en: {=bool:?}, rx_buf_err_en: {=bool:?}, prgm_err_en: {=bool:?}, tx_buf_full_en: {=bool:?}, tx_buf_empty_en: {=bool:?}, tx_buf_req_en: {=bool:?}, rx_buf_ful_en: {=bool:?}, rx_buf_empty_en: {=bool:?}, rx_buf_req_en: {=bool:?} }}" , self . trans_compl_en () , self . dma_compl_en () , self . tx_buf_err_en () , self . rx_buf_err_en () , self . prgm_err_en () , self . tx_buf_full_en () , self . tx_buf_empty_en () , self . tx_buf_req_en () , self . rx_buf_ful_en () , self . rx_buf_empty_en () , self . rx_buf_req_en ())
            }
        }
        #[doc = "QMSPI Interface Control Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ifctrl(pub u32);
        impl Ifctrl {
            #[doc = "This bit sets the value on the WRITE PROTECT SPI Output Port if it is driven. 1=WRITE PROTECT is driven to 1; 0=WRITE PROTECT is driven to 0"]
            #[inline(always)]
            pub const fn wr_prct_out_val(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This bit sets the value on the WRITE PROTECT SPI Output Port if it is driven. 1=WRITE PROTECT is driven to 1; 0=WRITE PROTECT is driven to 0"]
            #[inline(always)]
            pub fn set_wr_prct_out_val(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "1=WRITE PROTECT SPI Output Port is driven 0=WRITE PROTECT SPI Output Port is not driven"]
            #[inline(always)]
            pub const fn wr_prct_out_en(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "1=WRITE PROTECT SPI Output Port is driven 0=WRITE PROTECT SPI Output Port is not driven"]
            #[inline(always)]
            pub fn set_wr_prct_out_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "This bit sets the value on the HOLD SPI Output Port if it is driven. 1=HOLD is driven to 1; 0=HOLD is driven to 0."]
            #[inline(always)]
            pub const fn hld_out_val(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "This bit sets the value on the HOLD SPI Output Port if it is driven. 1=HOLD is driven to 1; 0=HOLD is driven to 0."]
            #[inline(always)]
            pub fn set_hld_out_val(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "1=HOLD SPI Output Port is driven 0=HOLD SPI Output Port is not driven."]
            #[inline(always)]
            pub const fn hld_out_en(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "1=HOLD SPI Output Port is driven 0=HOLD SPI Output Port is not driven."]
            #[inline(always)]
            pub fn set_hld_out_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "1=Enable pull-down resistors on Receive pins while the SPI Chip Select signal is not asserted 0=No pull-down resistors enabled on Receive pins"]
            #[inline(always)]
            pub const fn pd_on_not_sel(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "1=Enable pull-down resistors on Receive pins while the SPI Chip Select signal is not asserted 0=No pull-down resistors enabled on Receive pins"]
            #[inline(always)]
            pub fn set_pd_on_not_sel(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "1=Enable pull-up resistors on Receive pins while the SPI Chip Select signal is not asserted 0=No pull-up resistors enabled on Receive pins."]
            #[inline(always)]
            pub const fn pu_on_notsel(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "1=Enable pull-up resistors on Receive pins while the SPI Chip Select signal is not asserted 0=No pull-up resistors enabled on Receive pins."]
            #[inline(always)]
            pub fn set_pu_on_notsel(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "1=Enable pull-down resistors on Transmit pins while the pins are not driven 0=No pull-down resistors enabled ion Transmit pins."]
            #[inline(always)]
            pub const fn pd_on_notdriven(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "1=Enable pull-down resistors on Transmit pins while the pins are not driven 0=No pull-down resistors enabled ion Transmit pins."]
            #[inline(always)]
            pub fn set_pd_on_notdriven(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "1=Enable pull-up resistors on Transmit pins while the pins are not driven 0=No pull-up resistors enabled ion Transmit pins."]
            #[inline(always)]
            pub const fn pu_on_notdriven(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "1=Enable pull-up resistors on Transmit pins while the pins are not driven 0=No pull-up resistors enabled ion Transmit pins."]
            #[inline(always)]
            pub fn set_pu_on_notdriven(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
        }
        impl Default for Ifctrl {
            #[inline(always)]
            fn default() -> Ifctrl {
                Ifctrl(0)
            }
        }
        impl core::fmt::Debug for Ifctrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ifctrl")
                    .field("wr_prct_out_val", &self.wr_prct_out_val())
                    .field("wr_prct_out_en", &self.wr_prct_out_en())
                    .field("hld_out_val", &self.hld_out_val())
                    .field("hld_out_en", &self.hld_out_en())
                    .field("pd_on_not_sel", &self.pd_on_not_sel())
                    .field("pu_on_notsel", &self.pu_on_notsel())
                    .field("pd_on_notdriven", &self.pd_on_notdriven())
                    .field("pu_on_notdriven", &self.pu_on_notdriven())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ifctrl {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Ifctrl {{ wr_prct_out_val: {=bool:?}, wr_prct_out_en: {=bool:?}, hld_out_val: {=bool:?}, hld_out_en: {=bool:?}, pd_on_not_sel: {=bool:?}, pu_on_notsel: {=bool:?}, pd_on_notdriven: {=bool:?}, pu_on_notdriven: {=bool:?} }}" , self . wr_prct_out_val () , self . wr_prct_out_en () , self . hld_out_val () , self . hld_out_en () , self . pd_on_not_sel () , self . pu_on_notsel () , self . pd_on_notdriven () , self . pu_on_notdriven ())
            }
        }
        #[doc = "QMSPI Mode Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Mode(pub u32);
        impl Mode {
            #[doc = "This bit is used to activate the QMSPI block. 1=Enabled. The block is fully operational 0=Disabled. Clocks are gated to conserve power and the output signals are set to their inactive state."]
            #[inline(always)]
            pub const fn act(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This bit is used to activate the QMSPI block. 1=Enabled. The block is fully operational 0=Disabled. Clocks are gated to conserve power and the output signals are set to their inactive state."]
            #[inline(always)]
            pub fn set_act(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Writing this bit with a 1 will reset the Quad SPI block. It is self-clearing."]
            #[inline(always)]
            pub const fn soft_rst(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Writing this bit with a 1 will reset the Quad SPI block. It is self-clearing."]
            #[inline(always)]
            pub fn set_soft_rst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Polarity of the SPI clock line when there are no transactions in process. 1=SPI Clock starts High; 0=SPI Clock starts Low."]
            #[inline(always)]
            pub const fn cpol(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Polarity of the SPI clock line when there are no transactions in process. 1=SPI Clock starts High; 0=SPI Clock starts Low."]
            #[inline(always)]
            pub fn set_cpol(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Clock phase of the Master data out. Common SPI modes require this field to be programmed with the same value as CHPA_MISO in this register. e.g. Mode 0: CPOL=0; CHPA_MISO=0; CHPA_MOSI=0; Mode 3: CPOL=1; CHPA_MISO=1; CHPA_MOSI=1. See datasheet for alternate mode settings. If CPOL=1: 1=Data changes on the falling edge of the SPI clock; 0=Data changes on the rising edge of the SPI clock If CPOL=0: 1=Data changes on the rising edge of the SPI clock; 0=Data changes on the falling edge of the SPI clock"]
            #[inline(always)]
            pub const fn chpa_mosi(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Clock phase of the Master data out. Common SPI modes require this field to be programmed with the same value as CHPA_MISO in this register. e.g. Mode 0: CPOL=0; CHPA_MISO=0; CHPA_MOSI=0; Mode 3: CPOL=1; CHPA_MISO=1; CHPA_MOSI=1. See datasheet for alternate mode settings. If CPOL=1: 1=Data changes on the falling edge of the SPI clock; 0=Data changes on the rising edge of the SPI clock If CPOL=0: 1=Data changes on the rising edge of the SPI clock; 0=Data changes on the falling edge of the SPI clock"]
            #[inline(always)]
            pub fn set_chpa_mosi(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Clock phase of the Master data in. Common SPI modes require this field to be programmed with the same value as CHPA_MOSI in this register. e.g. Mode 0: CPOL=0; CHPA_MISO=0; CHPA_MOSI=0; Mode 3: CPOL=1; CHPA_MISO=1; CHPA_MOSI=1. See datasheet for alternate mode settings. If CPOL=1: 1=Data are captured on the rising edge of the SPI clock; 0=Data are captured on the falling edge of the SPI clock If CPOL=0: 1=Data are captured on the falling edge of the SPI clock; 0=Data are captured on the rising edge of the SPI clock"]
            #[inline(always)]
            pub const fn chpa_miso(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Clock phase of the Master data in. Common SPI modes require this field to be programmed with the same value as CHPA_MOSI in this register. e.g. Mode 0: CPOL=0; CHPA_MISO=0; CHPA_MOSI=0; Mode 3: CPOL=1; CHPA_MISO=1; CHPA_MOSI=1. See datasheet for alternate mode settings. If CPOL=1: 1=Data are captured on the rising edge of the SPI clock; 0=Data are captured on the falling edge of the SPI clock If CPOL=0: 1=Data are captured on the falling edge of the SPI clock; 0=Data are captured on the rising edge of the SPI clock"]
            #[inline(always)]
            pub fn set_chpa_miso(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "The SPI clock divide in number of system clocks. A value of 1 divides the master clock by 1, a value of 255 divides the master clock by 255. A value of 0 divides the master clock by 256."]
            #[inline(always)]
            pub const fn clk_div(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0x01ff;
                val as u16
            }
            #[doc = "The SPI clock divide in number of system clocks. A value of 1 divides the master clock by 1, a value of 255 divides the master clock by 255. A value of 0 divides the master clock by 256."]
            #[inline(always)]
            pub fn set_clk_div(&mut self, val: u16) {
                self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
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
                    .field("act", &self.act())
                    .field("soft_rst", &self.soft_rst())
                    .field("cpol", &self.cpol())
                    .field("chpa_mosi", &self.chpa_mosi())
                    .field("chpa_miso", &self.chpa_miso())
                    .field("clk_div", &self.clk_div())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Mode {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Mode {{ act: {=bool:?}, soft_rst: {=bool:?}, cpol: {=bool:?}, chpa_mosi: {=bool:?}, chpa_miso: {=bool:?}, clk_div: {=u16:?} }}" , self . act () , self . soft_rst () , self . cpol () , self . chpa_mosi () , self . chpa_miso () , self . clk_div ())
            }
        }
        #[doc = "QMSPI Receive Buffer Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxFifo(pub u32);
        impl RxFifo {
            #[doc = "Buffer that stores data from the external SPI Slave device to the SPI Master (this block), which is received over MISO or IO. Reads from this register will empty the Rx FIFO. A 1 Byte read will have valid data on bits \\[7:0\\] and a Word read will have data on bits \\[15:0\\]. It is possible to request more data than the FIFO has (underflow condition), but this will cause an error (Rx Buffer Error). Read accesses to this register decrement the RECEIVE_BUFFER_COUNT field."]
            #[inline(always)]
            pub const fn rx_buf(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "Buffer that stores data from the external SPI Slave device to the SPI Master (this block), which is received over MISO or IO. Reads from this register will empty the Rx FIFO. A 1 Byte read will have valid data on bits \\[7:0\\] and a Word read will have data on bits \\[15:0\\]. It is possible to request more data than the FIFO has (underflow condition), but this will cause an error (Rx Buffer Error). Read accesses to this register decrement the RECEIVE_BUFFER_COUNT field."]
            #[inline(always)]
            pub fn set_rx_buf(&mut self, val: u32) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for RxFifo {
            #[inline(always)]
            fn default() -> RxFifo {
                RxFifo(0)
            }
        }
        impl core::fmt::Debug for RxFifo {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("RxFifo").field("rx_buf", &self.rx_buf()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for RxFifo {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "RxFifo {{ rx_buf: {=u32:?} }}", self.rx_buf())
            }
        }
        #[doc = "QMSPI Status Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sts(pub u32);
        impl Sts {
            #[doc = "In Manual Mode (neither DMA nor Description Buffers are enabled), this bit will be set to 1 when the transfer matches TRANSFER_LENGTH. If DMA Mode is enabled, this bit will be set to 1 when DMA_COMPLETE is set to 1. In Description Buffer Mode, this bit will be set to 1 only when the Last Buffer completes its transfer. In all cases, this bit will be set to 1 if the STOP bit is set to 1 and the controller has completed the current 8 bits being copied. 1=Transfer completed; 0=Transfer not complete."]
            #[inline(always)]
            pub const fn trans_compl(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "In Manual Mode (neither DMA nor Description Buffers are enabled), this bit will be set to 1 when the transfer matches TRANSFER_LENGTH. If DMA Mode is enabled, this bit will be set to 1 when DMA_COMPLETE is set to 1. In Description Buffer Mode, this bit will be set to 1 only when the Last Buffer completes its transfer. In all cases, this bit will be set to 1 if the STOP bit is set to 1 and the controller has completed the current 8 bits being copied. 1=Transfer completed; 0=Transfer not complete."]
            #[inline(always)]
            pub fn set_trans_compl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "This field has no meaning if DMA is not enabled. This bit will be set to 1 when the DMA controller asserts the DONE signal to the SPI controller. This occurs either when the SPI controller has closed the DMA transfer, or the DMA channel has completed its count. If both Transmit and Receive DMA transfers are active, then this bit will only assert after both have completed. If CLOSE_TRANSFER_ENABLE is enabled, DMA_COMPLETE and TRANSFER_COMPLETE will be asserted simultaneously. This status is not inhibited by the description buffers, so it can fire on all valid description buffers while operating in that mode. 1=DMA completed; 0=DMA not completed."]
            #[inline(always)]
            pub const fn dma_compl(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "This field has no meaning if DMA is not enabled. This bit will be set to 1 when the DMA controller asserts the DONE signal to the SPI controller. This occurs either when the SPI controller has closed the DMA transfer, or the DMA channel has completed its count. If both Transmit and Receive DMA transfers are active, then this bit will only assert after both have completed. If CLOSE_TRANSFER_ENABLE is enabled, DMA_COMPLETE and TRANSFER_COMPLETE will be asserted simultaneously. This status is not inhibited by the description buffers, so it can fire on all valid description buffers while operating in that mode. 1=DMA completed; 0=DMA not completed."]
            #[inline(always)]
            pub fn set_dma_compl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "1=Overflow error occurred (attempt to write to a full Transmit Buffer) 0=No overflow occurred."]
            #[inline(always)]
            pub const fn tx_buff_err(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "1=Overflow error occurred (attempt to write to a full Transmit Buffer) 0=No overflow occurred."]
            #[inline(always)]
            pub fn set_tx_buff_err(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "1=Underflow error occurred (attempt to read from an empty Receive Buffer) 0=No underflow occurred."]
            #[inline(always)]
            pub const fn rx_buff_err(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "1=Underflow error occurred (attempt to read from an empty Receive Buffer) 0=No underflow occurred."]
            #[inline(always)]
            pub fn set_rx_buff_err(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "This bit if a programming error is detected. 1=Programming Error detected; 0=No programming error detected."]
            #[inline(always)]
            pub const fn prgm_err(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "This bit if a programming error is detected. 1=Programming Error detected; 0=No programming error detected."]
            #[inline(always)]
            pub fn set_prgm_err(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "1=The Transmit Buffer is full 0=The Transmit Buffer is not full."]
            #[inline(always)]
            pub const fn tx_buff_full(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "1=The Transmit Buffer is full 0=The Transmit Buffer is not full."]
            #[inline(always)]
            pub fn set_tx_buff_full(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "1=The Transmit Buffer is empty 0=The Transmit Buffer is not empty."]
            #[inline(always)]
            pub const fn tx_buff_emp(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "1=The Transmit Buffer is empty 0=The Transmit Buffer is not empty."]
            #[inline(always)]
            pub fn set_tx_buff_emp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "This status is asserted if the Transmit Buffer reaches a high water mark established by the TRANSMIT_BUFFER_TRIGGER field. 1=TRANSMIT_BUFFER_COUNT is less than or equal to TRANSMIT_BUFFER_TRIGGER; 0=TRANSMIT_BUFFER_COUNT is greater than TRANSMIT_BUFFER_TRIGGER."]
            #[inline(always)]
            pub const fn tx_buff_req(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "This status is asserted if the Transmit Buffer reaches a high water mark established by the TRANSMIT_BUFFER_TRIGGER field. 1=TRANSMIT_BUFFER_COUNT is less than or equal to TRANSMIT_BUFFER_TRIGGER; 0=TRANSMIT_BUFFER_COUNT is greater than TRANSMIT_BUFFER_TRIGGER."]
            #[inline(always)]
            pub fn set_tx_buff_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "1=The SPI interface had been stalled due to a flow issue (an attempt by the interface to read from an empty Transmit Buffer) 0=No stalls occurred."]
            #[inline(always)]
            pub const fn tx_buff_stall(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "1=The SPI interface had been stalled due to a flow issue (an attempt by the interface to read from an empty Transmit Buffer) 0=No stalls occurred."]
            #[inline(always)]
            pub fn set_tx_buff_stall(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "1=The Receive Buffer is full 0=The Receive Buffer is not full."]
            #[inline(always)]
            pub const fn rx_buff_full(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "1=The Receive Buffer is full 0=The Receive Buffer is not full."]
            #[inline(always)]
            pub fn set_rx_buff_full(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "1=The Receive Buffer is empty 0=The Receive Buffer is not empty."]
            #[inline(always)]
            pub const fn rx_buff_emp(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "1=The Receive Buffer is empty 0=The Receive Buffer is not empty."]
            #[inline(always)]
            pub fn set_rx_buff_emp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "This status is asserted if the Receive Buffer reaches a high water mark established by the RECEIVE_BUFFER_TRIGGER field. 1=RECEIVE_BUFFER_COUNT is greater than or equal to RECEIVE_BUFFER_TRIGGER 0=RECEIVE_BUFFER_COUNT is less than RECEIVE_BUFFER_TRIGGER."]
            #[inline(always)]
            pub const fn rx_buff_req(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "This status is asserted if the Receive Buffer reaches a high water mark established by the RECEIVE_BUFFER_TRIGGER field. 1=RECEIVE_BUFFER_COUNT is greater than or equal to RECEIVE_BUFFER_TRIGGER 0=RECEIVE_BUFFER_COUNT is less than RECEIVE_BUFFER_TRIGGER."]
            #[inline(always)]
            pub fn set_rx_buff_req(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "1=The SPI interface had been stalled due to a flow issue (an attempt by the interface to write to a full Receive Buffer) 0=No stalls occurred."]
            #[inline(always)]
            pub const fn rx_buff_stall(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "1=The SPI interface had been stalled due to a flow issue (an attempt by the interface to write to a full Receive Buffer) 0=No stalls occurred."]
            #[inline(always)]
            pub fn set_rx_buff_stall(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
            #[doc = "1=A transfer is currently executing 0=No transfer currently in progress."]
            #[inline(always)]
            pub const fn trans_activ(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "1=A transfer is currently executing 0=No transfer currently in progress."]
            #[inline(always)]
            pub fn set_trans_activ(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "This field shows the Description Buffer currently active. This field has no meaning if Description Buffers are not enabled."]
            #[inline(always)]
            pub const fn cur_descr_buf(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0x0f;
                val as u8
            }
            #[doc = "This field shows the Description Buffer currently active. This field has no meaning if Description Buffers are not enabled."]
            #[inline(always)]
            pub fn set_cur_descr_buf(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
            }
        }
        impl Default for Sts {
            #[inline(always)]
            fn default() -> Sts {
                Sts(0)
            }
        }
        impl core::fmt::Debug for Sts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Sts")
                    .field("trans_compl", &self.trans_compl())
                    .field("dma_compl", &self.dma_compl())
                    .field("tx_buff_err", &self.tx_buff_err())
                    .field("rx_buff_err", &self.rx_buff_err())
                    .field("prgm_err", &self.prgm_err())
                    .field("tx_buff_full", &self.tx_buff_full())
                    .field("tx_buff_emp", &self.tx_buff_emp())
                    .field("tx_buff_req", &self.tx_buff_req())
                    .field("tx_buff_stall", &self.tx_buff_stall())
                    .field("rx_buff_full", &self.rx_buff_full())
                    .field("rx_buff_emp", &self.rx_buff_emp())
                    .field("rx_buff_req", &self.rx_buff_req())
                    .field("rx_buff_stall", &self.rx_buff_stall())
                    .field("trans_activ", &self.trans_activ())
                    .field("cur_descr_buf", &self.cur_descr_buf())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Sts {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Sts {{ trans_compl: {=bool:?}, dma_compl: {=bool:?}, tx_buff_err: {=bool:?}, rx_buff_err: {=bool:?}, prgm_err: {=bool:?}, tx_buff_full: {=bool:?}, tx_buff_emp: {=bool:?}, tx_buff_req: {=bool:?}, tx_buff_stall: {=bool:?}, rx_buff_full: {=bool:?}, rx_buff_emp: {=bool:?}, rx_buff_req: {=bool:?}, rx_buff_stall: {=bool:?}, trans_activ: {=bool:?}, cur_descr_buf: {=u8:?} }}" , self . trans_compl () , self . dma_compl () , self . tx_buff_err () , self . rx_buff_err () , self . prgm_err () , self . tx_buff_full () , self . tx_buff_emp () , self . tx_buff_req () , self . tx_buff_stall () , self . rx_buff_full () , self . rx_buff_emp () , self . rx_buff_req () , self . rx_buff_stall () , self . trans_activ () , self . cur_descr_buf ())
            }
        }
        #[doc = "QMSPI Transmit Buffer Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxFifo(pub u32);
        impl TxFifo {
            #[doc = "Writes to this register store data to be transmitted from the SPI Master to the external SPI Slave. Writes to this block will be written to the Transmit FIFO. A 1 Byte write fills 1 byte of the FIFO. A Word write fills 2 Bytes and a Doubleword write fills 4 bytes. The data must always be aligned to the bottom most byte (so 1 byte write is on bits \\[7:0\\] and Word write is on \\[15:0\\]). An overflow condition, TRANSMIT_BUFFER_ERROR, if a write to a full FIFO occurs. Write accesses to this register increment the TRANSMIT_BUFFER_COUNT field."]
            #[inline(always)]
            pub const fn tx_buf(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "Writes to this register store data to be transmitted from the SPI Master to the external SPI Slave. Writes to this block will be written to the Transmit FIFO. A 1 Byte write fills 1 byte of the FIFO. A Word write fills 2 Bytes and a Doubleword write fills 4 bytes. The data must always be aligned to the bottom most byte (so 1 byte write is on bits \\[7:0\\] and Word write is on \\[15:0\\]). An overflow condition, TRANSMIT_BUFFER_ERROR, if a write to a full FIFO occurs. Write accesses to this register increment the TRANSMIT_BUFFER_COUNT field."]
            #[inline(always)]
            pub fn set_tx_buf(&mut self, val: u32) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for TxFifo {
            #[inline(always)]
            fn default() -> TxFifo {
                TxFifo(0)
            }
        }
        impl core::fmt::Debug for TxFifo {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("TxFifo").field("tx_buf", &self.tx_buf()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for TxFifo {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "TxFifo {{ tx_buf: {=u32:?} }}", self.tx_buf())
            }
        }
    }
}
pub mod rtc {
    #[doc = "This is the set of registers that are automatically counted by hardware every 1 second while the block is enabled"]
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
        #[doc = "Seconds Register"]
        #[inline(always)]
        pub const fn sec(self) -> crate::common::Reg<u8, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Seconds Alarm Register"]
        #[inline(always)]
        pub const fn sec_alarm(self) -> crate::common::Reg<u8, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01usize) as _) }
        }
        #[doc = "Minutes Register"]
        #[inline(always)]
        pub const fn min(self) -> crate::common::Reg<u8, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
        }
        #[doc = "Minutes Alarm Register"]
        #[inline(always)]
        pub const fn min_alarm(self) -> crate::common::Reg<u8, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03usize) as _) }
        }
        #[doc = "Hours Register"]
        #[inline(always)]
        pub const fn hr(self) -> crate::common::Reg<u8, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Hours Alarm Register"]
        #[inline(always)]
        pub const fn hr_alarm(self) -> crate::common::Reg<u8, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05usize) as _) }
        }
        #[doc = "Day of Week Register"]
        #[inline(always)]
        pub const fn day_of_wk(self) -> crate::common::Reg<u8, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
        }
        #[doc = "Day of Month Register"]
        #[inline(always)]
        pub const fn day_of_mon(self) -> crate::common::Reg<u8, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07usize) as _) }
        }
        #[doc = "Month Register"]
        #[inline(always)]
        pub const fn month(self) -> crate::common::Reg<u8, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Year Register"]
        #[inline(always)]
        pub const fn year(self) -> crate::common::Reg<u8, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09usize) as _) }
        }
        #[doc = "Register A"]
        #[inline(always)]
        pub const fn rega(self) -> crate::common::Reg<u8, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ausize) as _) }
        }
        #[doc = "Register B"]
        #[inline(always)]
        pub const fn regb(self) -> crate::common::Reg<u8, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0busize) as _) }
        }
        #[doc = "Register C"]
        #[inline(always)]
        pub const fn regc(self) -> crate::common::Reg<u8, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Register D"]
        #[inline(always)]
        pub const fn regd(self) -> crate::common::Reg<u8, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0dusize) as _) }
        }
        #[doc = "RTC Control Register"]
        #[inline(always)]
        pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "Week Alarm Register\\[7:0\\] - ALARM_DAY_OF_WEEK This register, if written to a value in the range 1- -7, will inhibit the Alarm interrupt unless this field matches the contents of the Day of Week Register also."]
        #[inline(always)]
        pub const fn wk_alarm(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "Daylight Savings Forward Register"]
        #[inline(always)]
        pub const fn daylt_savf(self) -> crate::common::Reg<regs::DayltSavf, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
        }
        #[doc = "Daylight Savings Backward Register"]
        #[inline(always)]
        pub const fn daylt_savb(self) -> crate::common::Reg<regs::DayltSavb, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "RTC Control Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctrl(pub u32);
        impl Ctrl {
            #[doc = "BLOCK_ENABLE This bit must be '1' in order for the block to function internally. Registers may be initialized first, before setting this bit to '1' to start operation."]
            #[inline(always)]
            pub const fn blk_en(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "BLOCK_ENABLE This bit must be '1' in order for the block to function internally. Registers may be initialized first, before setting this bit to '1' to start operation."]
            #[inline(always)]
            pub fn set_blk_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "SOFT_RESET A '1' written to this bit position will trigger the RTC_RST reset, resetting the block and all registers except this one and the Test Register. This bit is self-clearing at the end of the reset, one cycle of Host Bus Clock later, and so requires no waiting."]
            #[inline(always)]
            pub const fn soft_rst(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "SOFT_RESET A '1' written to this bit position will trigger the RTC_RST reset, resetting the block and all registers except this one and the Test Register. This bit is self-clearing at the end of the reset, one cycle of Host Bus Clock later, and so requires no waiting."]
            #[inline(always)]
            pub fn set_soft_rst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "TEST"]
            #[inline(always)]
            pub const fn test(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "TEST"]
            #[inline(always)]
            pub fn set_test(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "ALARM_ENABLE 1=Enables the Alarm features, 0=Disables the Alarm features"]
            #[inline(always)]
            pub const fn alm_en(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "ALARM_ENABLE 1=Enables the Alarm features, 0=Disables the Alarm features"]
            #[inline(always)]
            pub fn set_alm_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Ctrl {
            #[inline(always)]
            fn default() -> Ctrl {
                Ctrl(0)
            }
        }
        impl core::fmt::Debug for Ctrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ctrl")
                    .field("blk_en", &self.blk_en())
                    .field("soft_rst", &self.soft_rst())
                    .field("test", &self.test())
                    .field("alm_en", &self.alm_en())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ctrl {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Ctrl {{ blk_en: {=bool:?}, soft_rst: {=bool:?}, test: {=bool:?}, alm_en: {=bool:?} }}",
                    self.blk_en(),
                    self.soft_rst(),
                    self.test(),
                    self.alm_en()
                )
            }
        }
        #[doc = "Daylight Savings Backward Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DayltSavb(pub u32);
        impl DayltSavb {
            #[doc = "This field matches the Month Register."]
            #[inline(always)]
            pub const fn dst_mon(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "This field matches the Month Register."]
            #[inline(always)]
            pub fn set_dst_mon(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "This field matches the Day of Week Register bits\\[2:0\\]."]
            #[inline(always)]
            pub const fn dst_day_of_wk(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0x07;
                val as u8
            }
            #[doc = "This field matches the Day of Week Register bits\\[2:0\\]."]
            #[inline(always)]
            pub fn set_dst_day_of_wk(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
            }
            #[doc = "5=Last week of month, 4 =Fourth week of month, 3=Third week of month, 2=Second week of month, 1=First week of month"]
            #[inline(always)]
            pub const fn dst_wk(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0x07;
                val as u8
            }
            #[doc = "5=Last week of month, 4 =Fourth week of month, 3=Third week of month, 2=Second week of month, 1=First week of month"]
            #[inline(always)]
            pub fn set_dst_wk(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
            }
            #[doc = "This field holds the matching value for bits\\[6:0\\] of the Hours register. The written value will be interpreted according to the 24/12 Hour mode and DM mode settings at the time of writing."]
            #[inline(always)]
            pub const fn dst_hr(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0x7f;
                val as u8
            }
            #[doc = "This field holds the matching value for bits\\[6:0\\] of the Hours register. The written value will be interpreted according to the 24/12 Hour mode and DM mode settings at the time of writing."]
            #[inline(always)]
            pub fn set_dst_hr(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
            }
            #[doc = "This bit selects AM vs. PM, to match bit\\[7\\] of the Hours Register if 12-Hour mode is selected in Register B at the time of writing."]
            #[inline(always)]
            pub const fn dst_am_pm(&self) -> bool {
                let val = (self.0 >> 31usize) & 0x01;
                val != 0
            }
            #[doc = "This bit selects AM vs. PM, to match bit\\[7\\] of the Hours Register if 12-Hour mode is selected in Register B at the time of writing."]
            #[inline(always)]
            pub fn set_dst_am_pm(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
            }
        }
        impl Default for DayltSavb {
            #[inline(always)]
            fn default() -> DayltSavb {
                DayltSavb(0)
            }
        }
        impl core::fmt::Debug for DayltSavb {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DayltSavb")
                    .field("dst_mon", &self.dst_mon())
                    .field("dst_day_of_wk", &self.dst_day_of_wk())
                    .field("dst_wk", &self.dst_wk())
                    .field("dst_hr", &self.dst_hr())
                    .field("dst_am_pm", &self.dst_am_pm())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DayltSavb {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "DayltSavb {{ dst_mon: {=u8:?}, dst_day_of_wk: {=u8:?}, dst_wk: {=u8:?}, dst_hr: {=u8:?}, dst_am_pm: {=bool:?} }}" , self . dst_mon () , self . dst_day_of_wk () , self . dst_wk () , self . dst_hr () , self . dst_am_pm ())
            }
        }
        #[doc = "Daylight Savings Forward Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DayltSavf(pub u32);
        impl DayltSavf {
            #[doc = "This field matches the Month Register."]
            #[inline(always)]
            pub const fn dst_mon(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "This field matches the Month Register."]
            #[inline(always)]
            pub fn set_dst_mon(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "This field matches the Day of Week Register bits\\[2:0\\]."]
            #[inline(always)]
            pub const fn dst_day_of_wk(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0x07;
                val as u8
            }
            #[doc = "This field matches the Day of Week Register bits\\[2:0\\]."]
            #[inline(always)]
            pub fn set_dst_day_of_wk(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
            }
            #[doc = "5=Last week of month, 4 =Fourth week of month, 3=Third week of month, 2=Second week of month, 1=First week of month"]
            #[inline(always)]
            pub const fn dst_wk(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0x07;
                val as u8
            }
            #[doc = "5=Last week of month, 4 =Fourth week of month, 3=Third week of month, 2=Second week of month, 1=First week of month"]
            #[inline(always)]
            pub fn set_dst_wk(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
            }
            #[doc = "This field holds the matching value for bits\\[6:0\\] of the Hours register. The written value will be interpreted according to the 24/12 Hour mode and DM mode settings at the time of writing."]
            #[inline(always)]
            pub const fn dst_hr(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0x7f;
                val as u8
            }
            #[doc = "This field holds the matching value for bits\\[6:0\\] of the Hours register. The written value will be interpreted according to the 24/12 Hour mode and DM mode settings at the time of writing."]
            #[inline(always)]
            pub fn set_dst_hr(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
            }
            #[doc = "This bit selects AM vs. PM, to match bit\\[7\\] of the Hours Register if 12-Hour mode is selected in Register B at the time of writing."]
            #[inline(always)]
            pub const fn dst_am_pm(&self) -> bool {
                let val = (self.0 >> 31usize) & 0x01;
                val != 0
            }
            #[doc = "This bit selects AM vs. PM, to match bit\\[7\\] of the Hours Register if 12-Hour mode is selected in Register B at the time of writing."]
            #[inline(always)]
            pub fn set_dst_am_pm(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
            }
        }
        impl Default for DayltSavf {
            #[inline(always)]
            fn default() -> DayltSavf {
                DayltSavf(0)
            }
        }
        impl core::fmt::Debug for DayltSavf {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DayltSavf")
                    .field("dst_mon", &self.dst_mon())
                    .field("dst_day_of_wk", &self.dst_day_of_wk())
                    .field("dst_wk", &self.dst_wk())
                    .field("dst_hr", &self.dst_hr())
                    .field("dst_am_pm", &self.dst_am_pm())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DayltSavf {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "DayltSavf {{ dst_mon: {=u8:?}, dst_day_of_wk: {=u8:?}, dst_wk: {=u8:?}, dst_hr: {=u8:?}, dst_am_pm: {=bool:?} }}" , self . dst_mon () , self . dst_day_of_wk () , self . dst_wk () , self . dst_hr () , self . dst_am_pm ())
            }
        }
    }
}
pub mod rtos {
    #[doc = "RTOS is a 32-bit timer designed to operate on the 32kHz oscillator which is available during all chip sleep states."]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rtos {
        ptr: *mut u8,
    }
    unsafe impl Send for Rtos {}
    unsafe impl Sync for Rtos {}
    impl Rtos {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "RTOS Timer Count Register."]
        #[inline(always)]
        pub const fn cnt(self) -> crate::common::Reg<regs::Cnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "RTOS Timer Preload Register"]
        #[inline(always)]
        pub const fn prld(self) -> crate::common::Reg<regs::Prld, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "RTOS Timer Control Register"]
        #[inline(always)]
        pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Soft Interrupt Register"]
        #[inline(always)]
        pub const fn softirq(self) -> crate::common::Reg<regs::Softirq, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "RTOS Timer Count Register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cnt(pub u32);
        impl Cnt {
            #[doc = "This register contains the current value of the RTOS Timer counter. This register should be read as a DWORD. There is no latching mechanism of the upper bytes implemented if the register is accessed as a byte or word. Reading the register with byte or word operations may give incorrect results."]
            #[inline(always)]
            pub const fn cntr(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "This register contains the current value of the RTOS Timer counter. This register should be read as a DWORD. There is no latching mechanism of the upper bytes implemented if the register is accessed as a byte or word. Reading the register with byte or word operations may give incorrect results."]
            #[inline(always)]
            pub fn set_cntr(&mut self, val: u32) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Cnt {
            #[inline(always)]
            fn default() -> Cnt {
                Cnt(0)
            }
        }
        impl core::fmt::Debug for Cnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Cnt").field("cntr", &self.cntr()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Cnt {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Cnt {{ cntr: {=u32:?} }}", self.cntr())
            }
        }
        #[doc = "RTOS Timer Control Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctrl(pub u32);
        impl Ctrl {
            #[doc = "1=RTOS timer counter is enabled 0=RTOS timer disabled. All register bits are reset to their default state"]
            #[inline(always)]
            pub const fn blk_en(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "1=RTOS timer counter is enabled 0=RTOS timer disabled. All register bits are reset to their default state"]
            #[inline(always)]
            pub fn set_blk_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "1=The the RTOS Timer Preload Register is loaded into the timer counter and the counter is restarted when the counter transitions from 1 to 0 0=The timer counter halts when it transitions from 1 to 0 and will not restart."]
            #[inline(always)]
            pub const fn au_reload(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "1=The the RTOS Timer Preload Register is loaded into the timer counter and the counter is restarted when the counter transitions from 1 to 0 0=The timer counter halts when it transitions from 1 to 0 and will not restart."]
            #[inline(always)]
            pub fn set_au_reload(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Writing a 1 to this bit will load the timer counter with the RTOS Timer Preload Register and start counting. If the Preload Register is 0, counting will not start and this bit will be cleared to 0. Writing a 0 to this bit will halt the counter and clear its contents to 0. The RTOS timer interrupt will not be generated. This bit is automatically cleared if the AUTO_RELOAD bit is 0 and the timer counter transitions from 1 to 0."]
            #[inline(always)]
            pub const fn tmr_strt(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Writing a 1 to this bit will load the timer counter with the RTOS Timer Preload Register and start counting. If the Preload Register is 0, counting will not start and this bit will be cleared to 0. Writing a 0 to this bit will halt the counter and clear its contents to 0. The RTOS timer interrupt will not be generated. This bit is automatically cleared if the AUTO_RELOAD bit is 0 and the timer counter transitions from 1 to 0."]
            #[inline(always)]
            pub fn set_tmr_strt(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "1=The timer counter is halted when the external HALT signal is asserted. Counting is always enabled if HALT is de-asserted. 0=The HALT signal does not affect the RTOS Timer"]
            #[inline(always)]
            pub const fn ext_hw_halt_en(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "1=The timer counter is halted when the external HALT signal is asserted. Counting is always enabled if HALT is de-asserted. 0=The HALT signal does not affect the RTOS Timer"]
            #[inline(always)]
            pub fn set_ext_hw_halt_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "1=The timer counter is halted. If the counter was running, clearing this bit will restart the counter from the value at which it halted 0=The timer counter, if enabled, will continue to run"]
            #[inline(always)]
            pub const fn fw_tmr_halt(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "1=The timer counter is halted. If the counter was running, clearing this bit will restart the counter from the value at which it halted 0=The timer counter, if enabled, will continue to run"]
            #[inline(always)]
            pub fn set_fw_tmr_halt(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
        }
        impl Default for Ctrl {
            #[inline(always)]
            fn default() -> Ctrl {
                Ctrl(0)
            }
        }
        impl core::fmt::Debug for Ctrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ctrl")
                    .field("blk_en", &self.blk_en())
                    .field("au_reload", &self.au_reload())
                    .field("tmr_strt", &self.tmr_strt())
                    .field("ext_hw_halt_en", &self.ext_hw_halt_en())
                    .field("fw_tmr_halt", &self.fw_tmr_halt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ctrl {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Ctrl {{ blk_en: {=bool:?}, au_reload: {=bool:?}, tmr_strt: {=bool:?}, ext_hw_halt_en: {=bool:?}, fw_tmr_halt: {=bool:?} }}" , self . blk_en () , self . au_reload () , self . tmr_strt () , self . ext_hw_halt_en () , self . fw_tmr_halt ())
            }
        }
        #[doc = "RTOS Timer Preload Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Prld(pub u32);
        impl Prld {
            #[doc = "The this register is loaded into the RTOS Timer counter either when the TIMER_START bit is written with a 1, or when the timer counter counts down to 0 and the AUTO_RELOAD bit is 1. This register must be programmed with a new count value before the TIMER_START bit is set to 1. If this register is updated while the counter is operating, the new count value will only take effect if the counter transitions form 1 to 0 while the AUTO_RELOAD bit is set."]
            #[inline(always)]
            pub const fn preload(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "The this register is loaded into the RTOS Timer counter either when the TIMER_START bit is written with a 1, or when the timer counter counts down to 0 and the AUTO_RELOAD bit is 1. This register must be programmed with a new count value before the TIMER_START bit is set to 1. If this register is updated while the counter is operating, the new count value will only take effect if the counter transitions form 1 to 0 while the AUTO_RELOAD bit is set."]
            #[inline(always)]
            pub fn set_preload(&mut self, val: u32) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for Prld {
            #[inline(always)]
            fn default() -> Prld {
                Prld(0)
            }
        }
        impl core::fmt::Debug for Prld {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Prld").field("preload", &self.preload()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Prld {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Prld {{ preload: {=u32:?} }}", self.preload())
            }
        }
        #[doc = "Soft Interrupt Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Softirq(pub u32);
        impl Softirq {
            #[doc = "Software Interrupt. A write of a '1' to this bit will generate an SWI interrupt to the EC. Writes of a '0' have no effect. Reads return '0'."]
            #[inline(always)]
            pub const fn swi0(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Software Interrupt. A write of a '1' to this bit will generate an SWI interrupt to the EC. Writes of a '0' have no effect. Reads return '0'."]
            #[inline(always)]
            pub fn set_swi0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Software Interrupt. A write of a '1' to this bit will generate an SWI interrupt to the EC. Writes of a '0' have no effect. Reads return '0'."]
            #[inline(always)]
            pub const fn swi1(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Software Interrupt. A write of a '1' to this bit will generate an SWI interrupt to the EC. Writes of a '0' have no effect. Reads return '0'."]
            #[inline(always)]
            pub fn set_swi1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Software Interrupt. A write of a '1' to this bit will generate an SWI interrupt to the EC. Writes of a '0' have no effect. Reads return '0'."]
            #[inline(always)]
            pub const fn swi2(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Software Interrupt. A write of a '1' to this bit will generate an SWI interrupt to the EC. Writes of a '0' have no effect. Reads return '0'."]
            #[inline(always)]
            pub fn set_swi2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Software Interrupt. A write of a '1' to this bit will generate an SWI interrupt to the EC. Writes of a '0' have no effect. Reads return '0'."]
            #[inline(always)]
            pub const fn swi3(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Software Interrupt. A write of a '1' to this bit will generate an SWI interrupt to the EC. Writes of a '0' have no effect. Reads return '0'."]
            #[inline(always)]
            pub fn set_swi3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Softirq {
            #[inline(always)]
            fn default() -> Softirq {
                Softirq(0)
            }
        }
        impl core::fmt::Debug for Softirq {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Softirq")
                    .field("swi0", &self.swi0())
                    .field("swi1", &self.swi1())
                    .field("swi2", &self.swi2())
                    .field("swi3", &self.swi3())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Softirq {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Softirq {{ swi0: {=bool:?}, swi1: {=bool:?}, swi2: {=bool:?}, swi3: {=bool:?} }}",
                    self.swi0(),
                    self.swi1(),
                    self.swi2(),
                    self.swi3()
                )
            }
        }
    }
}
pub mod smb0 {
    #[doc = "The SMBus interface can handle standard SMBus 2.0 protocols as well as I2C interface."]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smb0 {
        ptr: *mut u8,
    }
    unsafe impl Send for Smb0 {}
    unsafe impl Sync for Smb0 {}
    impl Smb0 {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Status Register"]
        #[inline(always)]
        pub const fn rsts(self) -> crate::common::Reg<regs::Rsts, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Control Register"]
        #[inline(always)]
        pub const fn wctrl(self) -> crate::common::Reg<regs::Wctrl, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Own Address Register Note that the Data Register and Own Address fields are offset by one bit, so that programming Own Address 1 with a value of 55h will result in the value AAh being recognized as the SMB Controller Core slave address."]
        #[inline(always)]
        pub const fn own_addr(self) -> crate::common::Reg<regs::OwnAddr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "This register holds the data that are either shifted out to or shifted in from the I2C port."]
        #[inline(always)]
        pub const fn i2cdata(self) -> crate::common::Reg<u8, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Reserved"]
        #[inline(always)]
        pub const fn rsvd1(self, n: usize) -> crate::common::Reg<u8, crate::common::R> {
            assert!(n < 3usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09usize + n * 1usize) as _) }
        }
        #[doc = "SMBus Master Command Register"]
        #[inline(always)]
        pub const fn mcmd(self) -> crate::common::Reg<regs::Mcmd, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "SMBus Slave Command Register"]
        #[inline(always)]
        pub const fn scmd(self) -> crate::common::Reg<regs::Scmd, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "Packet Error Check (PEC) Register"]
        #[inline(always)]
        pub const fn pec(self) -> crate::common::Reg<regs::Pec, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "Repeated Start Hold Time Register"]
        #[inline(always)]
        pub const fn rshtm(self) -> crate::common::Reg<regs::Rshtm, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
        }
        #[doc = "Reserved"]
        #[inline(always)]
        pub const fn rsvd2(self) -> crate::common::Reg<u32, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
        #[doc = "Completion Register"]
        #[inline(always)]
        pub const fn compl(self) -> crate::common::Reg<regs::Compl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }
        #[doc = "Idle Scaling Register"]
        #[inline(always)]
        pub const fn idlsc(self) -> crate::common::Reg<regs::Idlsc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
        }
        #[doc = "Configuration Register"]
        #[inline(always)]
        pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
        }
        #[doc = "Bus Clock Register"]
        #[inline(always)]
        pub const fn busclk(self) -> crate::common::Reg<regs::Busclk, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
        }
        #[doc = "Block ID Register"]
        #[inline(always)]
        pub const fn blkid(self) -> crate::common::Reg<regs::Blkid, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
        }
        #[doc = "Revision Register"]
        #[inline(always)]
        pub const fn blkrev(self) -> crate::common::Reg<regs::Blkrev, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
        }
        #[doc = "Bit-Bang Control Register"]
        #[inline(always)]
        pub const fn bbctrl(self) -> crate::common::Reg<regs::Bbctrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
        }
        #[doc = "Test"]
        #[inline(always)]
        pub const fn test(self) -> crate::common::Reg<regs::Test, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
        }
        #[doc = "Data Timing Register"]
        #[inline(always)]
        pub const fn datatm(self) -> crate::common::Reg<regs::Datatm, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
        }
        #[doc = "Time-Out Scaling Register"]
        #[inline(always)]
        pub const fn tmoutsc(self) -> crate::common::Reg<regs::Tmoutsc, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
        }
        #[doc = "SMBus Slave Transmit Buffer Register"]
        #[inline(always)]
        pub const fn slv_txb(self) -> crate::common::Reg<regs::SlvTxb, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
        }
        #[doc = "SMBus Slave Receive Buffer Register"]
        #[inline(always)]
        pub const fn slv_rxb(self) -> crate::common::Reg<regs::SlvRxb, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
        }
        #[doc = "SMBus Master Transmit Buffer Register"]
        #[inline(always)]
        pub const fn mtr_txb(self) -> crate::common::Reg<regs::MtrTxb, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
        }
        #[doc = "SMBus Master Receive Buffer Register"]
        #[inline(always)]
        pub const fn mtr_rxb(self) -> crate::common::Reg<regs::MtrRxb, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
        }
        #[doc = "WAKE STATUS Register"]
        #[inline(always)]
        pub const fn wake_sts(self) -> crate::common::Reg<regs::WakeSts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
        }
        #[doc = "WAKE ENABLE Register"]
        #[inline(always)]
        pub const fn wake_en(self) -> crate::common::Reg<regs::WakeEn, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
        }
        #[doc = "This is the Slave Address Register"]
        #[inline(always)]
        pub const fn slv_addr(self) -> crate::common::Reg<regs::SlvAddr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
        }
        #[doc = "This is the Promiscuous Interrupt Register"]
        #[inline(always)]
        pub const fn prm_sts(self) -> crate::common::Reg<regs::PrmSts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
        }
        #[doc = "This is the Promiscuous Interrupt Enable Register"]
        #[inline(always)]
        pub const fn prm_ien(self) -> crate::common::Reg<regs::PrmIen, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
        }
        #[doc = "This is the Promiscuous Control Register"]
        #[inline(always)]
        pub const fn prm_ctrl(self) -> crate::common::Reg<regs::PrmCtrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Bit-Bang Control Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Bbctrl(pub u32);
        impl Bbctrl {
            #[doc = "Bit-Bang Mode Enable. 0 - Bit Bang Mode Disabled. 1 - Bit Bang Mode Enabled"]
            #[inline(always)]
            pub const fn bben(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Bit-Bang Mode Enable. 0 - Bit Bang Mode Disabled. 1 - Bit Bang Mode Enabled"]
            #[inline(always)]
            pub fn set_bben(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Bit-Bang Clock Direction. The CLDIR bit controls the direction of SCLK. 0 - Input, 1 - Output"]
            #[inline(always)]
            pub const fn cldir(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Bit-Bang Clock Direction. The CLDIR bit controls the direction of SCLK. 0 - Input, 1 - Output"]
            #[inline(always)]
            pub fn set_cldir(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Bit-Bang Data Direction. The DADIR bit controls the direction of SDAT. 0 - Input. 1 - Output"]
            #[inline(always)]
            pub const fn dadir(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Bit-Bang Data Direction. The DADIR bit controls the direction of SDAT. 0 - Input. 1 - Output"]
            #[inline(always)]
            pub fn set_dadir(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Bit-Bang Clock. The BBCLK bit controls the state of SCLK when BBEN = and CLDIR = '1'"]
            #[inline(always)]
            pub const fn bbclk(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Bit-Bang Clock. The BBCLK bit controls the state of SCLK when BBEN = and CLDIR = '1'"]
            #[inline(always)]
            pub fn set_bbclk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Bit-Bang Data. The BBDAT bit controls the state of SDAT when BBEN = and DADIR = '1'"]
            #[inline(always)]
            pub const fn bbdat(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Bit-Bang Data. The BBDAT bit controls the state of SDAT when BBEN = and DADIR = '1'"]
            #[inline(always)]
            pub fn set_bbdat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Bit-Bang Clock In. The BBCLKI bit always returns the state of SCLK."]
            #[inline(always)]
            pub const fn bbclki(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Bit-Bang Clock In. The BBCLKI bit always returns the state of SCLK."]
            #[inline(always)]
            pub fn set_bbclki(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Bit-Bang Data In. The BBDATI bit always returns the state of SDAT"]
            #[inline(always)]
            pub const fn bbdati(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Bit-Bang Data In. The BBDATI bit always returns the state of SDAT"]
            #[inline(always)]
            pub fn set_bbdati(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
        }
        impl Default for Bbctrl {
            #[inline(always)]
            fn default() -> Bbctrl {
                Bbctrl(0)
            }
        }
        impl core::fmt::Debug for Bbctrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Bbctrl")
                    .field("bben", &self.bben())
                    .field("cldir", &self.cldir())
                    .field("dadir", &self.dadir())
                    .field("bbclk", &self.bbclk())
                    .field("bbdat", &self.bbdat())
                    .field("bbclki", &self.bbclki())
                    .field("bbdati", &self.bbdati())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Bbctrl {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Bbctrl {{ bben: {=bool:?}, cldir: {=bool:?}, dadir: {=bool:?}, bbclk: {=bool:?}, bbdat: {=bool:?}, bbclki: {=bool:?}, bbdati: {=bool:?} }}" , self . bben () , self . cldir () , self . dadir () , self . bbclk () , self . bbdat () , self . bbclki () , self . bbdati ())
            }
        }
        #[doc = "Block ID Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Blkid(pub u8);
        impl Blkid {
            #[doc = "Block ID."]
            #[inline(always)]
            pub const fn id(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Block ID."]
            #[inline(always)]
            pub fn set_id(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
            }
        }
        impl Default for Blkid {
            #[inline(always)]
            fn default() -> Blkid {
                Blkid(0)
            }
        }
        impl core::fmt::Debug for Blkid {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Blkid").field("id", &self.id()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Blkid {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Blkid {{ id: {=u8:?} }}", self.id())
            }
        }
        #[doc = "Revision Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Blkrev(pub u8);
        impl Blkrev {
            #[doc = "Block Revision Number"]
            #[inline(always)]
            pub const fn rev(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Block Revision Number"]
            #[inline(always)]
            pub fn set_rev(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
            }
        }
        impl Default for Blkrev {
            #[inline(always)]
            fn default() -> Blkrev {
                Blkrev(0)
            }
        }
        impl core::fmt::Debug for Blkrev {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Blkrev").field("rev", &self.rev()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Blkrev {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Blkrev {{ rev: {=u8:?} }}", self.rev())
            }
        }
        #[doc = "Bus Clock Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Busclk(pub u32);
        impl Busclk {
            #[doc = "This field defines the number of I2C Baud Clock periods that make up the low phase of the I2C/SMBus bus clock."]
            #[inline(always)]
            pub const fn low_per(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "This field defines the number of I2C Baud Clock periods that make up the low phase of the I2C/SMBus bus clock."]
            #[inline(always)]
            pub fn set_low_per(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "This field defines the number of I2C Baud Clock periods that make up the high phase of the I2C/SMBus bus clock."]
            #[inline(always)]
            pub const fn high_per(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "This field defines the number of I2C Baud Clock periods that make up the high phase of the I2C/SMBus bus clock."]
            #[inline(always)]
            pub fn set_high_per(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
        }
        impl Default for Busclk {
            #[inline(always)]
            fn default() -> Busclk {
                Busclk(0)
            }
        }
        impl core::fmt::Debug for Busclk {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Busclk")
                    .field("low_per", &self.low_per())
                    .field("high_per", &self.high_per())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Busclk {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Busclk {{ low_per: {=u8:?}, high_per: {=u8:?} }}",
                    self.low_per(),
                    self.high_per()
                )
            }
        }
        #[doc = "Configuration Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cfg(pub u32);
        impl Cfg {
            #[doc = "The PORT SEL \\[3:0\\] bits determine which one of 16 possible bus ports apply to the active 2-wire SDAT and SCLK bus pair."]
            #[inline(always)]
            pub const fn port_sel(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "The PORT SEL \\[3:0\\] bits determine which one of 16 possible bus ports apply to the active 2-wire SDAT and SCLK bus pair."]
            #[inline(always)]
            pub fn set_port_sel(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
            #[doc = "When the Timing Check Enable bit (TCEN) is asserted ('1'), Bus Time-Outs are enabled"]
            #[inline(always)]
            pub const fn tcen(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "When the Timing Check Enable bit (TCEN) is asserted ('1'), Bus Time-Outs are enabled"]
            #[inline(always)]
            pub fn set_tcen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "When this bit is 1, the base period for the Bus Clock Register is multiplied by 4, and thus the frequency is divided by 4."]
            #[inline(always)]
            pub const fn slow_clk(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "When this bit is 1, the base period for the Bus Clock Register is multiplied by 4, and thus the frequency is divided by 4."]
            #[inline(always)]
            pub fn set_slow_clk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Must be always written with 0."]
            #[inline(always)]
            pub const fn test(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Must be always written with 0."]
            #[inline(always)]
            pub fn set_test(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "When the PEC Enable bit (PECEN) is asserted ('1'), Hardware PEC Support is enabled"]
            #[inline(always)]
            pub const fn pecen(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "When the PEC Enable bit (PECEN) is asserted ('1'), Hardware PEC Support is enabled"]
            #[inline(always)]
            pub fn set_pecen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "Input filtering enable. Input filtering is required by the I2C specification if external filtering is not available. 1=Input filtering is enabled; 0=Input filtering is disabled."]
            #[inline(always)]
            pub const fn fen(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Input filtering enable. Input filtering is required by the I2C specification if external filtering is not available. 1=Input filtering is enabled; 0=Input filtering is disabled."]
            #[inline(always)]
            pub fn set_fen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "When RESET is asserted ('1'), all logic and registers except for the RESET bit itself are initialized to the power-on default state."]
            #[inline(always)]
            pub const fn rst(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "When RESET is asserted ('1'), all logic and registers except for the RESET bit itself are initialized to the power-on default state."]
            #[inline(always)]
            pub fn set_rst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "When ENAB (Enable) is not asserted ('0') (default), the SMB Controller Core is disabled and in the lowest power consumption state (Disabled State). The ENAB bit must be asserted ('1') for normal operation."]
            #[inline(always)]
            pub const fn en(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "When ENAB (Enable) is not asserted ('0') (default), the SMB Controller Core is disabled and in the lowest power consumption state (Disabled State). The ENAB bit must be asserted ('1') for normal operation."]
            #[inline(always)]
            pub fn set_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "0: Slave Address I2C Compatibility Mode (default). 1: SMBus Address Decode Mode"]
            #[inline(always)]
            pub const fn dsa(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "0: Slave Address I2C Compatibility Mode (default). 1: SMBus Address Decode Mode"]
            #[inline(always)]
            pub fn set_dsa(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "If this bit is 1, the MCTP Fairness protocol is in effect."]
            #[inline(always)]
            pub const fn fair(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "If this bit is 1, the MCTP Fairness protocol is in effect."]
            #[inline(always)]
            pub fn set_fair(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "Must be always written with 0."]
            #[inline(always)]
            pub const fn test0(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "Must be always written with 0."]
            #[inline(always)]
            pub fn set_test0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "This is the General Call Disable bit. 0: the response to the General Call address as a slave is enabled 1: the response to the General Call address as a slave is disabled."]
            #[inline(always)]
            pub const fn gc_dis(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "This is the General Call Disable bit. 0: the response to the General Call address as a slave is enabled 1: the response to the General Call address as a slave is disabled."]
            #[inline(always)]
            pub fn set_gc_dis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "This bit define the Mode of SM Bus Controler Mode of operation. 0= Normal Operation. 1= Promiscuous Mode Enable."]
            #[inline(always)]
            pub const fn cfg_promis(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "This bit define the Mode of SM Bus Controler Mode of operation. 0= Normal Operation. 1= Promiscuous Mode Enable."]
            #[inline(always)]
            pub fn set_cfg_promis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
            #[doc = "A write of a 1 to this bit forces the SMBus Slave Transmit Buffer Register to be marked empty. A write of 0 has no effect. This is a self-clearing bit."]
            #[inline(always)]
            pub const fn flush_sxbuf(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "A write of a 1 to this bit forces the SMBus Slave Transmit Buffer Register to be marked empty. A write of 0 has no effect. This is a self-clearing bit."]
            #[inline(always)]
            pub fn set_flush_sxbuf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "A write of a 1 to this bit forces the SMBus Slave Receive Buffer Register to be marked empty. A write of 0 has no effect. This is a self-clearing bit."]
            #[inline(always)]
            pub const fn flush_srbuf(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "A write of a 1 to this bit forces the SMBus Slave Receive Buffer Register to be marked empty. A write of 0 has no effect. This is a self-clearing bit."]
            #[inline(always)]
            pub fn set_flush_srbuf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "A write of a 1 to this bit forces the SMBus Master Transmit Buffer Register to be marked empty. A write of 0 has no effect. This is a self-clearing bit."]
            #[inline(always)]
            pub const fn flush_mxbuf(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "A write of a 1 to this bit forces the SMBus Master Transmit Buffer Register to be marked empty. A write of 0 has no effect. This is a self-clearing bit."]
            #[inline(always)]
            pub fn set_flush_mxbuf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "A write of a 1 to this bit forces the SMBus Master Receive Buffer Register to be marked empty. A write of 0 has no effect. This is a self-clearing bit."]
            #[inline(always)]
            pub const fn flush_mrbuf(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "A write of a 1 to this bit forces the SMBus Master Receive Buffer Register to be marked empty. A write of 0 has no effect. This is a self-clearing bit."]
            #[inline(always)]
            pub fn set_flush_mrbuf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "0: Disable the AAS, 1: Enable the AAS Interrupt"]
            #[inline(always)]
            pub const fn en_aas(&self) -> bool {
                let val = (self.0 >> 28usize) & 0x01;
                val != 0
            }
            #[doc = "0: Disable the AAS, 1: Enable the AAS Interrupt"]
            #[inline(always)]
            pub fn set_en_aas(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
            }
            #[doc = "If this bit is 1, the Idle interrupt is enabled. If this bit is 0, the Idle interrupt is disabled."]
            #[inline(always)]
            pub const fn enidi(&self) -> bool {
                let val = (self.0 >> 29usize) & 0x01;
                val != 0
            }
            #[doc = "If this bit is 1, the Idle interrupt is enabled. If this bit is 0, the Idle interrupt is disabled."]
            #[inline(always)]
            pub fn set_enidi(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
            }
            #[doc = "If this bit is 1, the Master Done interrupt is enabled. If this bit is 0, the Master Done interrupt is disabled."]
            #[inline(always)]
            pub const fn enmi(&self) -> bool {
                let val = (self.0 >> 30usize) & 0x01;
                val != 0
            }
            #[doc = "If this bit is 1, the Master Done interrupt is enabled. If this bit is 0, the Master Done interrupt is disabled."]
            #[inline(always)]
            pub fn set_enmi(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
            }
            #[doc = "If this bit is 1, the Slave Done interrupt is enabled. If this bit is 0, the Slave Done interrupt is disabled"]
            #[inline(always)]
            pub const fn ensi(&self) -> bool {
                let val = (self.0 >> 31usize) & 0x01;
                val != 0
            }
            #[doc = "If this bit is 1, the Slave Done interrupt is enabled. If this bit is 0, the Slave Done interrupt is disabled"]
            #[inline(always)]
            pub fn set_ensi(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
            }
        }
        impl Default for Cfg {
            #[inline(always)]
            fn default() -> Cfg {
                Cfg(0)
            }
        }
        impl core::fmt::Debug for Cfg {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Cfg")
                    .field("port_sel", &self.port_sel())
                    .field("tcen", &self.tcen())
                    .field("slow_clk", &self.slow_clk())
                    .field("test", &self.test())
                    .field("pecen", &self.pecen())
                    .field("fen", &self.fen())
                    .field("rst", &self.rst())
                    .field("en", &self.en())
                    .field("dsa", &self.dsa())
                    .field("fair", &self.fair())
                    .field("test0", &self.test0())
                    .field("gc_dis", &self.gc_dis())
                    .field("cfg_promis", &self.cfg_promis())
                    .field("flush_sxbuf", &self.flush_sxbuf())
                    .field("flush_srbuf", &self.flush_srbuf())
                    .field("flush_mxbuf", &self.flush_mxbuf())
                    .field("flush_mrbuf", &self.flush_mrbuf())
                    .field("en_aas", &self.en_aas())
                    .field("enidi", &self.enidi())
                    .field("enmi", &self.enmi())
                    .field("ensi", &self.ensi())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Cfg {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Cfg {{ port_sel: {=u8:?}, tcen: {=bool:?}, slow_clk: {=bool:?}, test: {=bool:?}, pecen: {=bool:?}, fen: {=bool:?}, rst: {=bool:?}, en: {=bool:?}, dsa: {=bool:?}, fair: {=bool:?}, test0: {=bool:?}, gc_dis: {=bool:?}, cfg_promis: {=bool:?}, flush_sxbuf: {=bool:?}, flush_srbuf: {=bool:?}, flush_mxbuf: {=bool:?}, flush_mrbuf: {=bool:?}, en_aas: {=bool:?}, enidi: {=bool:?}, enmi: {=bool:?}, ensi: {=bool:?} }}" , self . port_sel () , self . tcen () , self . slow_clk () , self . test () , self . pecen () , self . fen () , self . rst () , self . en () , self . dsa () , self . fair () , self . test0 () , self . gc_dis () , self . cfg_promis () , self . flush_sxbuf () , self . flush_srbuf () , self . flush_mxbuf () , self . flush_mrbuf () , self . en_aas () , self . enidi () , self . enmi () , self . ensi ())
            }
        }
        #[doc = "Completion Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Compl(pub u32);
        impl Compl {
            #[doc = "When DTEN is asserted ('1'), Device Time-out checking is enabled. When DTEN is not asserted ('0'), Device Time-out checking is disabled."]
            #[inline(always)]
            pub const fn dten(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "When DTEN is asserted ('1'), Device Time-out checking is enabled. When DTEN is not asserted ('0'), Device Time-out checking is disabled."]
            #[inline(always)]
            pub fn set_dten(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "When MCEN is asserted ('1'), Master Cumulative Time-Out checking is enabled. When MCEN is not asserted ('0'), Master Cumulative Time-Out checking is disabled."]
            #[inline(always)]
            pub const fn mcen(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "When MCEN is asserted ('1'), Master Cumulative Time-Out checking is enabled. When MCEN is not asserted ('0'), Master Cumulative Time-Out checking is disabled."]
            #[inline(always)]
            pub fn set_mcen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "When SCEN is asserted ('1'), Slave Cumulative Time-Out checking is enabled. When SCEN is not asserted ('0'), Slave Cumulative Time-Out checking is disabled."]
            #[inline(always)]
            pub const fn scen(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "When SCEN is asserted ('1'), Slave Cumulative Time-Out checking is enabled. When SCEN is not asserted ('0'), Slave Cumulative Time-Out checking is disabled."]
            #[inline(always)]
            pub fn set_scen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "When BIDEN is asserted ('1'), Bus Idle Detect Time-Out checking is enabled. When BIDEN is not asserted ('0'), Bus Idle Detect Time-Out checking is disabled."]
            #[inline(always)]
            pub const fn biden(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "When BIDEN is asserted ('1'), Bus Idle Detect Time-Out checking is enabled. When BIDEN is not asserted ('0'), Bus Idle Detect Time-Out checking is disabled."]
            #[inline(always)]
            pub fn set_biden(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "The Time-out Error Detected bit (TIMERR) is asserted ('1') whenever any of the enabled time-out error detect status bits (CHDH, CHDL, SCTO, MCTO and DTO) are asserted."]
            #[inline(always)]
            pub const fn timerr(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "The Time-out Error Detected bit (TIMERR) is asserted ('1') whenever any of the enabled time-out error detect status bits (CHDH, CHDL, SCTO, MCTO and DTO) are asserted."]
            #[inline(always)]
            pub fn set_timerr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "DTO is the Device Time-out bit. (R/WC)"]
            #[inline(always)]
            pub const fn dto(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "DTO is the Device Time-out bit. (R/WC)"]
            #[inline(always)]
            pub fn set_dto(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "MCTO is the Master Cumulative Time-out bit. (R/WC)"]
            #[inline(always)]
            pub const fn mcto(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "MCTO is the Master Cumulative Time-out bit. (R/WC)"]
            #[inline(always)]
            pub fn set_mcto(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "SCTO is the Slave Cumulative Time-out bit(R/WC)"]
            #[inline(always)]
            pub const fn scto(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "SCTO is the Slave Cumulative Time-out bit(R/WC)"]
            #[inline(always)]
            pub fn set_scto(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "CHDL is the clock high time-out detect bit(R/WC)"]
            #[inline(always)]
            pub const fn chdl(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "CHDL is the clock high time-out detect bit(R/WC)"]
            #[inline(always)]
            pub fn set_chdl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "CHDH is the bus idle time-out detect bit(R/WC)"]
            #[inline(always)]
            pub const fn chdh(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "CHDH is the bus idle time-out detect bit(R/WC)"]
            #[inline(always)]
            pub fn set_chdh(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "If this bit is 1, the BER bit in the Status register was set while either the Slave state machine or the Master state machine was not in the Idle state.(R/WC)"]
            #[inline(always)]
            pub const fn ber(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "If this bit is 1, the BER bit in the Status register was set while either the Slave state machine or the Master state machine was not in the Idle state.(R/WC)"]
            #[inline(always)]
            pub fn set_ber(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "If this bit is 1, the LAB bit in the Status register was set while either the Slave state machine or the Master state machine was not in the Idle state.(R/WC)"]
            #[inline(always)]
            pub const fn lab(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "If this bit is 1, the LAB bit in the Status register was set while either the Slave state machine or the Master state machine was not in the Idle state.(R/WC)"]
            #[inline(always)]
            pub fn set_lab(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "If this bit is 1, the Slave state machine sent a NACK to the transmitting Master while the Slave was receiving data from the SMBus interface."]
            #[inline(always)]
            pub const fn snakr(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "If this bit is 1, the Slave state machine sent a NACK to the transmitting Master while the Slave was receiving data from the SMBus interface."]
            #[inline(always)]
            pub fn set_snakr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "0: Slave has just finished the receive phase of a transaction. 1: Slave has just finished the transmit phase of a transaction."]
            #[inline(always)]
            pub const fn str(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "0: Slave has just finished the receive phase of a transaction. 1: Slave has just finished the transmit phase of a transaction."]
            #[inline(always)]
            pub fn set_str(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "If this bit is 1, the WriteCount\\[7:0\\] counter in the Slave state machine either counted down to 0 before the Master sent a NACK signal, or the Slave received a NACK signal before the counter reached 0."]
            #[inline(always)]
            pub const fn sprot(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "If this bit is 1, the WriteCount\\[7:0\\] counter in the Slave state machine either counted down to 0 before the Master sent a NACK signal, or the Slave received a NACK signal before the counter reached 0."]
            #[inline(always)]
            pub fn set_sprot(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
            #[doc = "If this bit is 1, the Slave State Machine stopped because it detected a Repeat Start bit, with bit\\[0\\] of the byte containing the slave address equal to 1, indicating that the Master requested a Read operation."]
            #[inline(always)]
            pub const fn rep_rd(&self) -> bool {
                let val = (self.0 >> 20usize) & 0x01;
                val != 0
            }
            #[doc = "If this bit is 1, the Slave State Machine stopped because it detected a Repeat Start bit, with bit\\[0\\] of the byte containing the slave address equal to 1, indicating that the Master requested a Read operation."]
            #[inline(always)]
            pub fn set_rep_rd(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
            }
            #[doc = "If this bit is 1, the Slave State Machine stopped because it detected a Repeat Start bit, with bit\\[0\\] of the byte containing the slave address equal to 0, indicating that the Master requested a Write operation."]
            #[inline(always)]
            pub const fn rep_wr(&self) -> bool {
                let val = (self.0 >> 21usize) & 0x01;
                val != 0
            }
            #[doc = "If this bit is 1, the Slave State Machine stopped because it detected a Repeat Start bit, with bit\\[0\\] of the byte containing the slave address equal to 0, indicating that the Master requested a Write operation."]
            #[inline(always)]
            pub fn set_rep_wr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
            }
            #[doc = "If this bit is 1, the Master state machine received a NACK from the receiving Slave while the Master was transmitting data over the SMBus interface. (R/WC)"]
            #[inline(always)]
            pub const fn mnakx(&self) -> bool {
                let val = (self.0 >> 24usize) & 0x01;
                val != 0
            }
            #[doc = "If this bit is 1, the Master state machine received a NACK from the receiving Slave while the Master was transmitting data over the SMBus interface. (R/WC)"]
            #[inline(always)]
            pub fn set_mnakx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
            }
            #[doc = "0: Master has just finished the receive phase of a transaction. 1: Master has just finished the transmit phase of a transaction."]
            #[inline(always)]
            pub const fn mtr(&self) -> bool {
                let val = (self.0 >> 25usize) & 0x01;
                val != 0
            }
            #[doc = "0: Master has just finished the receive phase of a transaction. 1: Master has just finished the transmit phase of a transaction."]
            #[inline(always)]
            pub fn set_mtr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
            }
            #[doc = "This bit is set when the I2C bus becomes idle (on the rising edge of nBB). (R/WC)"]
            #[inline(always)]
            pub const fn idle(&self) -> bool {
                let val = (self.0 >> 29usize) & 0x01;
                val != 0
            }
            #[doc = "This bit is set when the I2C bus becomes idle (on the rising edge of nBB). (R/WC)"]
            #[inline(always)]
            pub fn set_idle(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
            }
            #[doc = "If this bit is 1, Master State machine completed operation and returned to the Idle state. It is cleared when written with a 1. Writes of a 0 have no effect. (R/WC)"]
            #[inline(always)]
            pub const fn mdone(&self) -> bool {
                let val = (self.0 >> 30usize) & 0x01;
                val != 0
            }
            #[doc = "If this bit is 1, Master State machine completed operation and returned to the Idle state. It is cleared when written with a 1. Writes of a 0 have no effect. (R/WC)"]
            #[inline(always)]
            pub fn set_mdone(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
            }
            #[doc = "If this bit is 1, Slave State machine completed operation and returned to the Idle state. It is cleared when written with a 1. Writes of a 0 have no effect.(R/WC)"]
            #[inline(always)]
            pub const fn sdone(&self) -> bool {
                let val = (self.0 >> 31usize) & 0x01;
                val != 0
            }
            #[doc = "If this bit is 1, Slave State machine completed operation and returned to the Idle state. It is cleared when written with a 1. Writes of a 0 have no effect.(R/WC)"]
            #[inline(always)]
            pub fn set_sdone(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
            }
        }
        impl Default for Compl {
            #[inline(always)]
            fn default() -> Compl {
                Compl(0)
            }
        }
        impl core::fmt::Debug for Compl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Compl")
                    .field("dten", &self.dten())
                    .field("mcen", &self.mcen())
                    .field("scen", &self.scen())
                    .field("biden", &self.biden())
                    .field("timerr", &self.timerr())
                    .field("dto", &self.dto())
                    .field("mcto", &self.mcto())
                    .field("scto", &self.scto())
                    .field("chdl", &self.chdl())
                    .field("chdh", &self.chdh())
                    .field("ber", &self.ber())
                    .field("lab", &self.lab())
                    .field("snakr", &self.snakr())
                    .field("str", &self.str())
                    .field("sprot", &self.sprot())
                    .field("rep_rd", &self.rep_rd())
                    .field("rep_wr", &self.rep_wr())
                    .field("mnakx", &self.mnakx())
                    .field("mtr", &self.mtr())
                    .field("idle", &self.idle())
                    .field("mdone", &self.mdone())
                    .field("sdone", &self.sdone())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Compl {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Compl {{ dten: {=bool:?}, mcen: {=bool:?}, scen: {=bool:?}, biden: {=bool:?}, timerr: {=bool:?}, dto: {=bool:?}, mcto: {=bool:?}, scto: {=bool:?}, chdl: {=bool:?}, chdh: {=bool:?}, ber: {=bool:?}, lab: {=bool:?}, snakr: {=bool:?}, str: {=bool:?}, sprot: {=bool:?}, rep_rd: {=bool:?}, rep_wr: {=bool:?}, mnakx: {=bool:?}, mtr: {=bool:?}, idle: {=bool:?}, mdone: {=bool:?}, sdone: {=bool:?} }}" , self . dten () , self . mcen () , self . scen () , self . biden () , self . timerr () , self . dto () , self . mcto () , self . scto () , self . chdl () , self . chdh () , self . ber () , self . lab () , self . snakr () , self . str () , self . sprot () , self . rep_rd () , self . rep_wr () , self . mnakx () , self . mtr () , self . idle () , self . mdone () , self . sdone ())
            }
        }
        #[doc = "Data Timing Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Datatm(pub u32);
        impl Datatm {
            #[doc = "The Data Hold \\[7:0\\] timer determines the SDAT hold time following SCLK driven low."]
            #[inline(always)]
            pub const fn data_hold(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "The Data Hold \\[7:0\\] timer determines the SDAT hold time following SCLK driven low."]
            #[inline(always)]
            pub fn set_data_hold(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "The Restart Setup \\[7:0\\] timer determines the SDAT setup time from the rising edge of SCLK for a repeated START condition."]
            #[inline(always)]
            pub const fn restart_setup(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "The Restart Setup \\[7:0\\] timer determines the SDAT setup time from the rising edge of SCLK for a repeated START condition."]
            #[inline(always)]
            pub fn set_restart_setup(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
            #[doc = "The Stop Setup \\[7:0\\] timer determines the SDAT setup time from the rising edge of SCLK for a STOP condition."]
            #[inline(always)]
            pub const fn stop_setup(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0xff;
                val as u8
            }
            #[doc = "The Stop Setup \\[7:0\\] timer determines the SDAT setup time from the rising edge of SCLK for a STOP condition."]
            #[inline(always)]
            pub fn set_stop_setup(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
            }
            #[doc = "This field determines the SCL hold time following SDA driven low during the first START bit in a transfer. It is the parameter THD:STA in the I2C Specification for an initial START bit. Repeated START hold time is determined by the Repeated START Hold Time Register."]
            #[inline(always)]
            pub const fn first_start_hold(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0xff;
                val as u8
            }
            #[doc = "This field determines the SCL hold time following SDA driven low during the first START bit in a transfer. It is the parameter THD:STA in the I2C Specification for an initial START bit. Repeated START hold time is determined by the Repeated START Hold Time Register."]
            #[inline(always)]
            pub fn set_first_start_hold(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
            }
        }
        impl Default for Datatm {
            #[inline(always)]
            fn default() -> Datatm {
                Datatm(0)
            }
        }
        impl core::fmt::Debug for Datatm {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Datatm")
                    .field("data_hold", &self.data_hold())
                    .field("restart_setup", &self.restart_setup())
                    .field("stop_setup", &self.stop_setup())
                    .field("first_start_hold", &self.first_start_hold())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Datatm {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Datatm {{ data_hold: {=u8:?}, restart_setup: {=u8:?}, stop_setup: {=u8:?}, first_start_hold: {=u8:?} }}" , self . data_hold () , self . restart_setup () , self . stop_setup () , self . first_start_hold ())
            }
        }
        #[doc = "Idle Scaling Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Idlsc(pub u32);
        impl Idlsc {
            #[doc = "This field defines the number of ticks of the baud clock required to satisfy the fairness protocol. The default value for this field sets the idle window to 31us, which is the appropriate value for a 100 KHz bus"]
            #[inline(always)]
            pub const fn fair_bus_idl_min(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x0fff;
                val as u16
            }
            #[doc = "This field defines the number of ticks of the baud clock required to satisfy the fairness protocol. The default value for this field sets the idle window to 31us, which is the appropriate value for a 100 KHz bus"]
            #[inline(always)]
            pub fn set_fair_bus_idl_min(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
            }
            #[doc = "This field defines the number of ticks of the baud clock required to program the delay. The default value for this field sets the delay period to 32us, which is the appropriate value for a 100 KHz bus"]
            #[inline(always)]
            pub const fn fair_idl_dly(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0x0fff;
                val as u16
            }
            #[doc = "This field defines the number of ticks of the baud clock required to program the delay. The default value for this field sets the delay period to 32us, which is the appropriate value for a 100 KHz bus"]
            #[inline(always)]
            pub fn set_fair_idl_dly(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
            }
        }
        impl Default for Idlsc {
            #[inline(always)]
            fn default() -> Idlsc {
                Idlsc(0)
            }
        }
        impl core::fmt::Debug for Idlsc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Idlsc")
                    .field("fair_bus_idl_min", &self.fair_bus_idl_min())
                    .field("fair_idl_dly", &self.fair_idl_dly())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Idlsc {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Idlsc {{ fair_bus_idl_min: {=u16:?}, fair_idl_dly: {=u16:?} }}",
                    self.fair_bus_idl_min(),
                    self.fair_idl_dly()
                )
            }
        }
        #[doc = "SMBus Master Command Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Mcmd(pub u32);
        impl Mcmd {
            #[doc = "While this bit is 1, transfer bytes over SMBus. As long as WriteCount is non-zero, a byte from the Master Transmit Buffer is transmitted to the slave device and WriteCount is decremented."]
            #[inline(always)]
            pub const fn mrun(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "While this bit is 1, transfer bytes over SMBus. As long as WriteCount is non-zero, a byte from the Master Transmit Buffer is transmitted to the slave device and WriteCount is decremented."]
            #[inline(always)]
            pub fn set_mrun(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "When this bit is 0, the Master State Machine does not transition out of the IDLE or PAUSE states. When this bit is 1, the Master State Machine immediately transitions to the WAIT-BUSBUSY and MRUN-RECEIVE states, respectively."]
            #[inline(always)]
            pub const fn mproceed(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "When this bit is 0, the Master State Machine does not transition out of the IDLE or PAUSE states. When this bit is 1, the Master State Machine immediately transitions to the WAIT-BUSBUSY and MRUN-RECEIVE states, respectively."]
            #[inline(always)]
            pub fn set_mproceed(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "If this bit is 1, send a Start bit on the SMBus before the first byte of the WriteCount is sent to the SMBus transmitter."]
            #[inline(always)]
            pub const fn start0(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "If this bit is 1, send a Start bit on the SMBus before the first byte of the WriteCount is sent to the SMBus transmitter."]
            #[inline(always)]
            pub fn set_start0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "If this bit is 1, send a Start bit just before the last byte of the WriteCount is sent to the SMBus transmitter."]
            #[inline(always)]
            pub const fn startn(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "If this bit is 1, send a Start bit just before the last byte of the WriteCount is sent to the SMBus transmitter."]
            #[inline(always)]
            pub fn set_startn(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "If this bit is 1, send a Stop bit after the transaction completes."]
            #[inline(always)]
            pub const fn stop(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "If this bit is 1, send a Stop bit after the transaction completes."]
            #[inline(always)]
            pub fn set_stop(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "If this bit is 1, a copy of the PEC register is transmitted when WriteCount is 0. After the PEC register is read, both the PEC register and this bit are cleared to 0."]
            #[inline(always)]
            pub const fn pec_term(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "If this bit is 1, a copy of the PEC register is transmitted when WriteCount is 0. After the PEC register is read, both the PEC register and this bit are cleared to 0."]
            #[inline(always)]
            pub fn set_pec_term(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "If this bit is 1, then the ReadCount field is replaced by the byte that is read from the SMBus when ReadCount\\[7:0\\] is 1. After ReadCount\\[7:0\\] is updated, this bit is cleared to 0."]
            #[inline(always)]
            pub const fn readm(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "If this bit is 1, then the ReadCount field is replaced by the byte that is read from the SMBus when ReadCount\\[7:0\\] is 1. After ReadCount\\[7:0\\] is updated, this bit is cleared to 0."]
            #[inline(always)]
            pub fn set_readm(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "If this bit is 0, reading from the SMBus stops when ReadCount reaches 0. If this bit is 1, reading continues when ReadCount is 0 for one more byte."]
            #[inline(always)]
            pub const fn rd_pec(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "If this bit is 0, reading from the SMBus stops when ReadCount reaches 0. If this bit is 1, reading continues when ReadCount is 0 for one more byte."]
            #[inline(always)]
            pub fn set_rd_pec(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "This field is a count of the number of bytes to transmit to the SMBus from the SMBus Master Transmit Buffer Register It is decremented by 1 for each byte written to the SMBus from the SMBus Master Transmit Buffer Register."]
            #[inline(always)]
            pub const fn wr_cnt(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0xff;
                val as u8
            }
            #[doc = "This field is a count of the number of bytes to transmit to the SMBus from the SMBus Master Transmit Buffer Register It is decremented by 1 for each byte written to the SMBus from the SMBus Master Transmit Buffer Register."]
            #[inline(always)]
            pub fn set_wr_cnt(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
            }
            #[doc = "This field is a count of the number of bytes to read in from the SMBus to the SMBus Master Receive Buffer Register and must be greater than 0 in order for the Master State Machine to initiate a read phase. It is decremented by 1 for each byte read from the SMBus into the SMBus Master Receive Buffer Register. It can be overwritten by the first byte read in from the SMBus."]
            #[inline(always)]
            pub const fn rd_cnt(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0xff;
                val as u8
            }
            #[doc = "This field is a count of the number of bytes to read in from the SMBus to the SMBus Master Receive Buffer Register and must be greater than 0 in order for the Master State Machine to initiate a read phase. It is decremented by 1 for each byte read from the SMBus into the SMBus Master Receive Buffer Register. It can be overwritten by the first byte read in from the SMBus."]
            #[inline(always)]
            pub fn set_rd_cnt(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
            }
        }
        impl Default for Mcmd {
            #[inline(always)]
            fn default() -> Mcmd {
                Mcmd(0)
            }
        }
        impl core::fmt::Debug for Mcmd {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Mcmd")
                    .field("mrun", &self.mrun())
                    .field("mproceed", &self.mproceed())
                    .field("start0", &self.start0())
                    .field("startn", &self.startn())
                    .field("stop", &self.stop())
                    .field("pec_term", &self.pec_term())
                    .field("readm", &self.readm())
                    .field("rd_pec", &self.rd_pec())
                    .field("wr_cnt", &self.wr_cnt())
                    .field("rd_cnt", &self.rd_cnt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Mcmd {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Mcmd {{ mrun: {=bool:?}, mproceed: {=bool:?}, start0: {=bool:?}, startn: {=bool:?}, stop: {=bool:?}, pec_term: {=bool:?}, readm: {=bool:?}, rd_pec: {=bool:?}, wr_cnt: {=u8:?}, rd_cnt: {=u8:?} }}" , self . mrun () , self . mproceed () , self . start0 () , self . startn () , self . stop () , self . pec_term () , self . readm () , self . rd_pec () , self . wr_cnt () , self . rd_cnt ())
            }
        }
        #[doc = "SMBus Master Receive Buffer Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct MtrRxb(pub u32);
        impl MtrRxb {
            #[doc = "MASTER_RECEIVE_BUFFER"]
            #[inline(always)]
            pub const fn mrxb(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "MASTER_RECEIVE_BUFFER"]
            #[inline(always)]
            pub fn set_mrxb(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for MtrRxb {
            #[inline(always)]
            fn default() -> MtrRxb {
                MtrRxb(0)
            }
        }
        impl core::fmt::Debug for MtrRxb {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("MtrRxb").field("mrxb", &self.mrxb()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for MtrRxb {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "MtrRxb {{ mrxb: {=u8:?} }}", self.mrxb())
            }
        }
        #[doc = "SMBus Master Transmit Buffer Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct MtrTxb(pub u32);
        impl MtrTxb {
            #[doc = "MASTER_TRANSMIT_BUFFER"]
            #[inline(always)]
            pub const fn mtxb(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "MASTER_TRANSMIT_BUFFER"]
            #[inline(always)]
            pub fn set_mtxb(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for MtrTxb {
            #[inline(always)]
            fn default() -> MtrTxb {
                MtrTxb(0)
            }
        }
        impl core::fmt::Debug for MtrTxb {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("MtrTxb").field("mtxb", &self.mtxb()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for MtrTxb {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "MtrTxb {{ mtxb: {=u8:?} }}", self.mtxb())
            }
        }
        #[doc = "Own Address Register Note that the Data Register and Own Address fields are offset by one bit, so that programming Own Address 1 with a value of 55h will result in the value AAh being recognized as the SMB Controller Core slave address."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct OwnAddr(pub u32);
        impl OwnAddr {
            #[doc = "The Own Address 1 bits configure one of the two addresses to which the SMB Controller Core will respond when addressed as a slave."]
            #[inline(always)]
            pub const fn addr1(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x7f;
                val as u8
            }
            #[doc = "The Own Address 1 bits configure one of the two addresses to which the SMB Controller Core will respond when addressed as a slave."]
            #[inline(always)]
            pub fn set_addr1(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
            }
            #[doc = "The Own Address 2 bits configure one of the two addresses to which the SMB Controller Core will respond when addressed as a slave."]
            #[inline(always)]
            pub const fn addr2(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0x7f;
                val as u8
            }
            #[doc = "The Own Address 2 bits configure one of the two addresses to which the SMB Controller Core will respond when addressed as a slave."]
            #[inline(always)]
            pub fn set_addr2(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
            }
        }
        impl Default for OwnAddr {
            #[inline(always)]
            fn default() -> OwnAddr {
                OwnAddr(0)
            }
        }
        impl core::fmt::Debug for OwnAddr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("OwnAddr")
                    .field("addr1", &self.addr1())
                    .field("addr2", &self.addr2())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for OwnAddr {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "OwnAddr {{ addr1: {=u8:?}, addr2: {=u8:?} }}",
                    self.addr1(),
                    self.addr2()
                )
            }
        }
        #[doc = "Packet Error Check (PEC) Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pec(pub u32);
        impl Pec {
            #[doc = "The SMBus Packet Error Check (PEC) byte."]
            #[inline(always)]
            pub const fn pec(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "The SMBus Packet Error Check (PEC) byte."]
            #[inline(always)]
            pub fn set_pec(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Pec {
            #[inline(always)]
            fn default() -> Pec {
                Pec(0)
            }
        }
        impl core::fmt::Debug for Pec {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Pec").field("pec", &self.pec()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Pec {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Pec {{ pec: {=u8:?} }}", self.pec())
            }
        }
        #[doc = "This is the Promiscuous Control Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct PrmCtrl(pub u8);
        impl PrmCtrl {
            #[doc = "This is the Promiscuous ACK / NAK response register. 1= ACK. 0= NAK."]
            #[inline(always)]
            pub const fn ack_nak(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This is the Promiscuous ACK / NAK response register. 1= ACK. 0= NAK."]
            #[inline(always)]
            pub fn set_ack_nak(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
        }
        impl Default for PrmCtrl {
            #[inline(always)]
            fn default() -> PrmCtrl {
                PrmCtrl(0)
            }
        }
        impl core::fmt::Debug for PrmCtrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("PrmCtrl").field("ack_nak", &self.ack_nak()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for PrmCtrl {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "PrmCtrl {{ ack_nak: {=bool:?} }}", self.ack_nak())
            }
        }
        #[doc = "This is the Promiscuous Interrupt Enable Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct PrmIen(pub u8);
        impl PrmIen {
            #[doc = "This is the Promiscuous interrupt enable register. 1= Interrupt Enable. 0= Interrupt Disabled."]
            #[inline(always)]
            pub const fn addr(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This is the Promiscuous interrupt enable register. 1= Interrupt Enable. 0= Interrupt Disabled."]
            #[inline(always)]
            pub fn set_addr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
        }
        impl Default for PrmIen {
            #[inline(always)]
            fn default() -> PrmIen {
                PrmIen(0)
            }
        }
        impl core::fmt::Debug for PrmIen {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("PrmIen").field("addr", &self.addr()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for PrmIen {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "PrmIen {{ addr: {=bool:?} }}", self.addr())
            }
        }
        #[doc = "This is the Promiscuous Interrupt Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct PrmSts(pub u8);
        impl PrmSts {
            #[doc = "This is the Promiscuous Address Status interrupt and is set on the 8th clock of the I2C Address. This bit will hold the Clock line low till this register bit is cleared. This is Write 1 to clear. R/W1C."]
            #[inline(always)]
            pub const fn addr_intr(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This is the Promiscuous Address Status interrupt and is set on the 8th clock of the I2C Address. This bit will hold the Clock line low till this register bit is cleared. This is Write 1 to clear. R/W1C."]
            #[inline(always)]
            pub fn set_addr_intr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
        }
        impl Default for PrmSts {
            #[inline(always)]
            fn default() -> PrmSts {
                PrmSts(0)
            }
        }
        impl core::fmt::Debug for PrmSts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("PrmSts").field("addr_intr", &self.addr_intr()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for PrmSts {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "PrmSts {{ addr_intr: {=bool:?} }}", self.addr_intr())
            }
        }
        #[doc = "Repeated Start Hold Time Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rshtm(pub u32);
        impl Rshtm {
            #[doc = "This is the value of the timing requirement tHd:Sta in the I2C specification for a repeated START bit. This is used to hold the clock until the Hold Time for the repeated Start Bit has been satisfied."]
            #[inline(always)]
            pub const fn rshtm(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "This is the value of the timing requirement tHd:Sta in the I2C specification for a repeated START bit. This is used to hold the clock until the Hold Time for the repeated Start Bit has been satisfied."]
            #[inline(always)]
            pub fn set_rshtm(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for Rshtm {
            #[inline(always)]
            fn default() -> Rshtm {
                Rshtm(0)
            }
        }
        impl core::fmt::Debug for Rshtm {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Rshtm").field("rshtm", &self.rshtm()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rshtm {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Rshtm {{ rshtm: {=u8:?} }}", self.rshtm())
            }
        }
        #[doc = "Status Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rsts(pub u32);
        impl Rsts {
            #[doc = "The Bus Busy bit (NBB) is a read-only flag indicating when the bus is in use. A zero indicates that the bus is busy and access is not possible."]
            #[inline(always)]
            pub const fn nbb(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "The Bus Busy bit (NBB) is a read-only flag indicating when the bus is in use. A zero indicates that the bus is busy and access is not possible."]
            #[inline(always)]
            pub fn set_nbb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "The Lost Arbitration Bit (LAB) is set when, in multi-master operation, arbitration is lost to another master on the bus"]
            #[inline(always)]
            pub const fn lab(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "The Lost Arbitration Bit (LAB) is set when, in multi-master operation, arbitration is lost to another master on the bus"]
            #[inline(always)]
            pub fn set_lab(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "The Addressed As Slave bit (AAS) is valid only when PIN is asserted ('0'). When acting as slave, AAS is set when an incoming address over the bus matches the value in the Own Address Register or if the 'general call' address (00h) has been received"]
            #[inline(always)]
            pub const fn aas(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "The Addressed As Slave bit (AAS) is valid only when PIN is asserted ('0'). When acting as slave, AAS is set when an incoming address over the bus matches the value in the Own Address Register or if the 'general call' address (00h) has been received"]
            #[inline(always)]
            pub fn set_aas(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "The Last Received Bit or Address 0 (general call) bit (LRB/AD0) serves a dual function and is valid only while the PIN bit is asserted ('0'). When the AAS bit is not asserted ('0') (i.e., not addressed as a slave), the LRB/AD0 holds the value of the last received bit over the bus. When the AAS bit is asserted ('1') (i.e., addressed as slave), the SMB Controller Core has been addressed as a slave."]
            #[inline(always)]
            pub const fn lrb_ad0(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "The Last Received Bit or Address 0 (general call) bit (LRB/AD0) serves a dual function and is valid only while the PIN bit is asserted ('0'). When the AAS bit is not asserted ('0') (i.e., not addressed as a slave), the LRB/AD0 holds the value of the last received bit over the bus. When the AAS bit is asserted ('1') (i.e., addressed as slave), the SMB Controller Core has been addressed as a slave."]
            #[inline(always)]
            pub fn set_lrb_ad0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "When Bus Error (BER) is asserted, a misplaced START or STOP condition or Bus Time-Outs have been detected."]
            #[inline(always)]
            pub const fn ber(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "When Bus Error (BER) is asserted, a misplaced START or STOP condition or Bus Time-Outs have been detected."]
            #[inline(always)]
            pub fn set_ber(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "When in slave receiver mode, STS is asserted ('1') when an externally generated STOP condition is detected. Note that STS is used only in slave receiver mode."]
            #[inline(always)]
            pub const fn sts(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "When in slave receiver mode, STS is asserted ('1') when an externally generated STOP condition is detected. Note that STS is used only in slave receiver mode."]
            #[inline(always)]
            pub fn set_sts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "SMBus Address Decoded (SAD)"]
            #[inline(always)]
            pub const fn sad(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "SMBus Address Decoded (SAD)"]
            #[inline(always)]
            pub fn set_sad(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Pending Interrupt bit"]
            #[inline(always)]
            pub const fn pin(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Pending Interrupt bit"]
            #[inline(always)]
            pub fn set_pin(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
        }
        impl Default for Rsts {
            #[inline(always)]
            fn default() -> Rsts {
                Rsts(0)
            }
        }
        impl core::fmt::Debug for Rsts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Rsts")
                    .field("nbb", &self.nbb())
                    .field("lab", &self.lab())
                    .field("aas", &self.aas())
                    .field("lrb_ad0", &self.lrb_ad0())
                    .field("ber", &self.ber())
                    .field("sts", &self.sts())
                    .field("sad", &self.sad())
                    .field("pin", &self.pin())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Rsts {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Rsts {{ nbb: {=bool:?}, lab: {=bool:?}, aas: {=bool:?}, lrb_ad0: {=bool:?}, ber: {=bool:?}, sts: {=bool:?}, sad: {=bool:?}, pin: {=bool:?} }}" , self . nbb () , self . lab () , self . aas () , self . lrb_ad0 () , self . ber () , self . sts () , self . sad () , self . pin ())
            }
        }
        #[doc = "SMBus Slave Command Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Scmd(pub u32);
        impl Scmd {
            #[doc = "Setting this bit to 1 enables the Slave State Machine to operate."]
            #[inline(always)]
            pub const fn srun(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Setting this bit to 1 enables the Slave State Machine to operate."]
            #[inline(always)]
            pub fn set_srun(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "When this bit is 0, the Slave State Machine does not transition out of the IDLE, REPEAT_START_WRITE or REPEAT_START_READ states. When this bit is 1, the Slave State Machine immediately transitions to the START_WAIT, RECEIVE and TRANSMIT states, respectively."]
            #[inline(always)]
            pub const fn sproceed(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "When this bit is 0, the Slave State Machine does not transition out of the IDLE, REPEAT_START_WRITE or REPEAT_START_READ states. When this bit is 1, the Slave State Machine immediately transitions to the START_WAIT, RECEIVE and TRANSMIT states, respectively."]
            #[inline(always)]
            pub fn set_sproceed(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "If Slave_WriteCount is 0 and Slave_PEC is 1 when the Master requests data, the PEC Register is copied to the DATA register. After the PEC Register is copied to the SMBus, the PEC Register is cleared and Slave_PEC is set to 0."]
            #[inline(always)]
            pub const fn pec(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "If Slave_WriteCount is 0 and Slave_PEC is 1 when the Master requests data, the PEC Register is copied to the DATA register. After the PEC Register is copied to the SMBus, the PEC Register is cleared and Slave_PEC is set to 0."]
            #[inline(always)]
            pub fn set_pec(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "This field is set to the number of bytes software expects to send to the Master."]
            #[inline(always)]
            pub const fn wr_cnt(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "This field is set to the number of bytes software expects to send to the Master."]
            #[inline(always)]
            pub fn set_wr_cnt(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
            #[doc = "This field is decremented each time a byte is copied from DATA to the SMBus Slave Receive Buffer Register."]
            #[inline(always)]
            pub const fn rd_cnt(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0xff;
                val as u8
            }
            #[doc = "This field is decremented each time a byte is copied from DATA to the SMBus Slave Receive Buffer Register."]
            #[inline(always)]
            pub fn set_rd_cnt(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
            }
        }
        impl Default for Scmd {
            #[inline(always)]
            fn default() -> Scmd {
                Scmd(0)
            }
        }
        impl core::fmt::Debug for Scmd {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Scmd")
                    .field("srun", &self.srun())
                    .field("sproceed", &self.sproceed())
                    .field("pec", &self.pec())
                    .field("wr_cnt", &self.wr_cnt())
                    .field("rd_cnt", &self.rd_cnt())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Scmd {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Scmd {{ srun: {=bool:?}, sproceed: {=bool:?}, pec: {=bool:?}, wr_cnt: {=u8:?}, rd_cnt: {=u8:?} }}",
                    self.srun(),
                    self.sproceed(),
                    self.pec(),
                    self.wr_cnt(),
                    self.rd_cnt()
                )
            }
        }
        #[doc = "This is the Slave Address Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SlvAddr(pub u8);
        impl SlvAddr {
            #[doc = "This register stores value of address + LSB direction after the 8th clock of the Address Byte."]
            #[inline(always)]
            pub const fn saddr(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "This register stores value of address + LSB direction after the 8th clock of the Address Byte."]
            #[inline(always)]
            pub fn set_saddr(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
            }
        }
        impl Default for SlvAddr {
            #[inline(always)]
            fn default() -> SlvAddr {
                SlvAddr(0)
            }
        }
        impl core::fmt::Debug for SlvAddr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("SlvAddr").field("saddr", &self.saddr()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for SlvAddr {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "SlvAddr {{ saddr: {=u8:?} }}", self.saddr())
            }
        }
        #[doc = "SMBus Slave Receive Buffer Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SlvRxb(pub u32);
        impl SlvRxb {
            #[doc = "SLAVE_RECEIVE_BUFFER"]
            #[inline(always)]
            pub const fn srxb(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "SLAVE_RECEIVE_BUFFER"]
            #[inline(always)]
            pub fn set_srxb(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for SlvRxb {
            #[inline(always)]
            fn default() -> SlvRxb {
                SlvRxb(0)
            }
        }
        impl core::fmt::Debug for SlvRxb {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("SlvRxb").field("srxb", &self.srxb()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for SlvRxb {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "SlvRxb {{ srxb: {=u8:?} }}", self.srxb())
            }
        }
        #[doc = "SMBus Slave Transmit Buffer Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SlvTxb(pub u32);
        impl SlvTxb {
            #[doc = "SLAVE_TRANSMIT_BUFFER"]
            #[inline(always)]
            pub const fn stxb(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "SLAVE_TRANSMIT_BUFFER"]
            #[inline(always)]
            pub fn set_stxb(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for SlvTxb {
            #[inline(always)]
            fn default() -> SlvTxb {
                SlvTxb(0)
            }
        }
        impl core::fmt::Debug for SlvTxb {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("SlvTxb").field("stxb", &self.stxb()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for SlvTxb {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "SlvTxb {{ stxb: {=u8:?} }}", self.stxb())
            }
        }
        #[doc = "Test"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Test(pub u8);
        impl Test {
            #[doc = "This register must not be written, or undesirable results may occur."]
            #[inline(always)]
            pub const fn test(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "This register must not be written, or undesirable results may occur."]
            #[inline(always)]
            pub fn set_test(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
            }
        }
        impl Default for Test {
            #[inline(always)]
            fn default() -> Test {
                Test(0)
            }
        }
        impl core::fmt::Debug for Test {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Test").field("test", &self.test()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Test {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Test {{ test: {=u8:?} }}", self.test())
            }
        }
        #[doc = "Time-Out Scaling Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Tmoutsc(pub u32);
        impl Tmoutsc {
            #[doc = "Clock High time out period = Clock High Time-Out \\[7:0\\] x Baud_Clock_Period x 2"]
            #[inline(always)]
            pub const fn clk_high_tim_out(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Clock High time out period = Clock High Time-Out \\[7:0\\] x Baud_Clock_Period x 2"]
            #[inline(always)]
            pub fn set_clk_high_tim_out(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "Slave Cumulative Time-Out duration = Slave Cum Time-Out \\[7:0\\] x Baud_Clock_Period x 1024"]
            #[inline(always)]
            pub const fn slv_cum_tim_out(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "Slave Cumulative Time-Out duration = Slave Cum Time-Out \\[7:0\\] x Baud_Clock_Period x 1024"]
            #[inline(always)]
            pub fn set_slv_cum_tim_out(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
            #[doc = "Master Cumulative Time-Out duration = Master Cum Time-Out \\[7:0\\] x Baud_Clock_Period x 512"]
            #[inline(always)]
            pub const fn mast_cum_tim_out(&self) -> u8 {
                let val = (self.0 >> 16usize) & 0xff;
                val as u8
            }
            #[doc = "Master Cumulative Time-Out duration = Master Cum Time-Out \\[7:0\\] x Baud_Clock_Period x 512"]
            #[inline(always)]
            pub fn set_mast_cum_tim_out(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
            }
            #[doc = "Bus Idle Minimum time = Bus Idle Min \\[7:0\\] x Baud_Clock_Period"]
            #[inline(always)]
            pub const fn bus_idle_min(&self) -> u8 {
                let val = (self.0 >> 24usize) & 0xff;
                val as u8
            }
            #[doc = "Bus Idle Minimum time = Bus Idle Min \\[7:0\\] x Baud_Clock_Period"]
            #[inline(always)]
            pub fn set_bus_idle_min(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
            }
        }
        impl Default for Tmoutsc {
            #[inline(always)]
            fn default() -> Tmoutsc {
                Tmoutsc(0)
            }
        }
        impl core::fmt::Debug for Tmoutsc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Tmoutsc")
                    .field("clk_high_tim_out", &self.clk_high_tim_out())
                    .field("slv_cum_tim_out", &self.slv_cum_tim_out())
                    .field("mast_cum_tim_out", &self.mast_cum_tim_out())
                    .field("bus_idle_min", &self.bus_idle_min())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Tmoutsc {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Tmoutsc {{ clk_high_tim_out: {=u8:?}, slv_cum_tim_out: {=u8:?}, mast_cum_tim_out: {=u8:?}, bus_idle_min: {=u8:?} }}" , self . clk_high_tim_out () , self . slv_cum_tim_out () , self . mast_cum_tim_out () , self . bus_idle_min ())
            }
        }
        #[doc = "WAKE ENABLE Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct WakeEn(pub u32);
        impl WakeEn {
            #[doc = "Enable Start Bit Detection Interrupt. The Start Bit Detection Interrupt is wake-capable. 1=Start Bit Detection Interrupt enabled; 0=Start Bit Detection Interrupt disabled"]
            #[inline(always)]
            pub const fn start_det_int_en(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable Start Bit Detection Interrupt. The Start Bit Detection Interrupt is wake-capable. 1=Start Bit Detection Interrupt enabled; 0=Start Bit Detection Interrupt disabled"]
            #[inline(always)]
            pub fn set_start_det_int_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for WakeEn {
            #[inline(always)]
            fn default() -> WakeEn {
                WakeEn(0)
            }
        }
        impl core::fmt::Debug for WakeEn {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("WakeEn")
                    .field("start_det_int_en", &self.start_det_int_en())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for WakeEn {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "WakeEn {{ start_det_int_en: {=bool:?} }}", self.start_det_int_en())
            }
        }
        #[doc = "WAKE STATUS Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct WakeSts(pub u32);
        impl WakeSts {
            #[doc = "This bit is set to '1' when a START bit is detected while the controller is enabled. This bit is cleared to '0' when written with a '1'. Writes of '0' have no effect. (R/WC)"]
            #[inline(always)]
            pub const fn start_bit_det(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This bit is set to '1' when a START bit is detected while the controller is enabled. This bit is cleared to '0' when written with a '1'. Writes of '0' have no effect. (R/WC)"]
            #[inline(always)]
            pub fn set_start_bit_det(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for WakeSts {
            #[inline(always)]
            fn default() -> WakeSts {
                WakeSts(0)
            }
        }
        impl core::fmt::Debug for WakeSts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("WakeSts")
                    .field("start_bit_det", &self.start_bit_det())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for WakeSts {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "WakeSts {{ start_bit_det: {=bool:?} }}", self.start_bit_det())
            }
        }
        #[doc = "Control Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Wctrl(pub u32);
        impl Wctrl {
            #[doc = "The Acknowledge bit (ACK) must normally be asserted ('1'). This causes the controller to send an acknowledge automatically after each byte (this occurs during the 9th clock pulse). The ACK bit must not be asserted ('0') when the controller is operating in master/receiver mode and requires no further data to be sent from the slave transmitter. This causes a negative acknowledge on the I2C bus, which halts further transmission from the slave device."]
            #[inline(always)]
            pub const fn ack(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "The Acknowledge bit (ACK) must normally be asserted ('1'). This causes the controller to send an acknowledge automatically after each byte (this occurs during the 9th clock pulse). The ACK bit must not be asserted ('0') when the controller is operating in master/receiver mode and requires no further data to be sent from the slave transmitter. This causes a negative acknowledge on the I2C bus, which halts further transmission from the slave device."]
            #[inline(always)]
            pub fn set_ack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "See STA description"]
            #[inline(always)]
            pub const fn sto(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "See STA description"]
            #[inline(always)]
            pub fn set_sto(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "The STA and STO bits control the generation of the I2C Start condition and the transmission of the Slave Address and R/nW bit (from the Data Register), generation of repeated Start condition, and generation of the Stop condition"]
            #[inline(always)]
            pub const fn sta(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "The STA and STO bits control the generation of the I2C Start condition and the transmission of the Slave Address and R/nW bit (from the Data Register), generation of repeated Start condition, and generation of the Stop condition"]
            #[inline(always)]
            pub fn set_sta(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Enable Interrupt bit (ENI) controls the Interrupt Interface"]
            #[inline(always)]
            pub const fn eni(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Enable Interrupt bit (ENI) controls the Interrupt Interface"]
            #[inline(always)]
            pub fn set_eni(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "The Enable Serial Output bit (ESO) enables and disables the SMB Controller Core serial data output (SDAT)"]
            #[inline(always)]
            pub const fn eso(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "The Enable Serial Output bit (ESO) enables and disables the SMB Controller Core serial data output (SDAT)"]
            #[inline(always)]
            pub fn set_eso(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "The Pending Interrupt Not (PIN) bit serves as a software reset function. Writing the PIN bit to a logic '1' de-asserts all status bits except for the nBB bit which is not affected by the PIN bit. The PIN bit is a self-clearing bit. Writing this bit to a logic '0' has no effect."]
            #[inline(always)]
            pub const fn pin(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "The Pending Interrupt Not (PIN) bit serves as a software reset function. Writing the PIN bit to a logic '1' de-asserts all status bits except for the nBB bit which is not affected by the PIN bit. The PIN bit is a self-clearing bit. Writing this bit to a logic '0' has no effect."]
            #[inline(always)]
            pub fn set_pin(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
        }
        impl Default for Wctrl {
            #[inline(always)]
            fn default() -> Wctrl {
                Wctrl(0)
            }
        }
        impl core::fmt::Debug for Wctrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Wctrl")
                    .field("ack", &self.ack())
                    .field("sto", &self.sto())
                    .field("sta", &self.sta())
                    .field("eni", &self.eni())
                    .field("eso", &self.eso())
                    .field("pin", &self.pin())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Wctrl {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Wctrl {{ ack: {=bool:?}, sto: {=bool:?}, sta: {=bool:?}, eni: {=bool:?}, eso: {=bool:?}, pin: {=bool:?} }}" , self . ack () , self . sto () , self . sta () , self . eni () , self . eso () , self . pin ())
            }
        }
    }
}
pub mod tach0 {
    #[doc = "This block monitors TACH output signals from various types of fans, and determines their speed."]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tach0 {
        ptr: *mut u8,
    }
    unsafe impl Send for Tach0 {}
    unsafe impl Sync for Tach0 {}
    impl Tach0 {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "TACHx Control Register"]
        #[inline(always)]
        pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "TACHx Status Register"]
        #[inline(always)]
        pub const fn sts(self) -> crate::common::Reg<regs::Sts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "TACH HIGH LIMIT Register"]
        #[inline(always)]
        pub const fn lim_hi(self) -> crate::common::Reg<regs::LimHi, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "TACHx Low Limit Register"]
        #[inline(always)]
        pub const fn lim_lo(self) -> crate::common::Reg<regs::LimLo, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "TACHx Control Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctrl(pub u32);
        impl Ctrl {
            #[doc = "TACH_OUT_OF_LIMIT_ENABLE This bit is used to enable the TACH_OUT_OF_LIMIT_STATUS bit in the TACHx Status Register to generate an interrupt event. 1=Enable interrupt output from Tach block 0=Disable interrupt output from Tach block (default)"]
            #[inline(always)]
            pub const fn outof_lim_en(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "TACH_OUT_OF_LIMIT_ENABLE This bit is used to enable the TACH_OUT_OF_LIMIT_STATUS bit in the TACHx Status Register to generate an interrupt event. 1=Enable interrupt output from Tach block 0=Disable interrupt output from Tach block (default)"]
            #[inline(always)]
            pub fn set_outof_lim_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "TACH_ENABLE 1= TACH Monitoring enabled, clocks enabled. 0= TACH Idle, clocks gated"]
            #[inline(always)]
            pub const fn en(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "TACH_ENABLE 1= TACH Monitoring enabled, clocks enabled. 0= TACH Idle, clocks gated"]
            #[inline(always)]
            pub fn set_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "FILTER_ENABLE This filter is used to remove high frequency glitches from Tach Input. When this filter is enabled, Tach input pulses less than two 100kHz_- Clk periods wide get filtered. 1= Filter enabled 0= Filter disabled (default) It is recommended that the Tach input filter always be enabled."]
            #[inline(always)]
            pub const fn filt_en(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "FILTER_ENABLE This filter is used to remove high frequency glitches from Tach Input. When this filter is enabled, Tach input pulses less than two 100kHz_- Clk periods wide get filtered. 1= Filter enabled 0= Filter disabled (default) It is recommended that the Tach input filter always be enabled."]
            #[inline(always)]
            pub fn set_filt_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "TACH_READING_MODE_SELECT 1=Counter is incremented on the rising edge of the 100kHz_Clk input. The counter is latched into the TACHX_COUNTER field and reset when the programmed number of edges is detected. 0=Counter is incremented when Tach Input transitions from low-tohigh state (default)"]
            #[inline(always)]
            pub const fn rd_mod_sel(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "TACH_READING_MODE_SELECT 1=Counter is incremented on the rising edge of the 100kHz_Clk input. The counter is latched into the TACHX_COUNTER field and reset when the programmed number of edges is detected. 0=Counter is incremented when Tach Input transitions from low-tohigh state (default)"]
            #[inline(always)]
            pub fn set_rd_mod_sel(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "TACH_EDGES A Tach signal is a square wave with a 50 percent duty cycle. Typically, two Tach periods represents one revolution of the fan. A Tach period consists of three Tach edges. This programmed value represents the number of Tach edges that will be used to determine the interval for which the number of 100kHz_Clk pulses will be counted 11b=9 Tach edges (4 Tach periods) 10b=5 Tach edges (2 Tach periods) 01b=3 Tach edges (1 Tach period) 00b=2 Tach edges (1/2 Tach period)"]
            #[inline(always)]
            pub const fn edges(&self) -> u8 {
                let val = (self.0 >> 11usize) & 0x03;
                val as u8
            }
            #[doc = "TACH_EDGES A Tach signal is a square wave with a 50 percent duty cycle. Typically, two Tach periods represents one revolution of the fan. A Tach period consists of three Tach edges. This programmed value represents the number of Tach edges that will be used to determine the interval for which the number of 100kHz_Clk pulses will be counted 11b=9 Tach edges (4 Tach periods) 10b=5 Tach edges (2 Tach periods) 01b=3 Tach edges (1 Tach period) 00b=2 Tach edges (1/2 Tach period)"]
            #[inline(always)]
            pub fn set_edges(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
            }
            #[doc = "COUNT_READY_INT_EN 1=Enable Count Ready interrupt from Tach block, 0=Disable Count Ready interrupt from Tach block"]
            #[inline(always)]
            pub const fn cnt_rdy_int_en(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "COUNT_READY_INT_EN 1=Enable Count Ready interrupt from Tach block, 0=Disable Count Ready interrupt from Tach block"]
            #[inline(always)]
            pub fn set_cnt_rdy_int_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
            #[doc = "TACH_INPUT_INT_EN 1=Enable Tach Input toggle interrupt from Tach block, 0=Disable Tach Input toggle interrupt from Tach block"]
            #[inline(always)]
            pub const fn in_int_en(&self) -> bool {
                let val = (self.0 >> 15usize) & 0x01;
                val != 0
            }
            #[doc = "TACH_INPUT_INT_EN 1=Enable Tach Input toggle interrupt from Tach block, 0=Disable Tach Input toggle interrupt from Tach block"]
            #[inline(always)]
            pub fn set_in_int_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
            }
            #[doc = "This 16-bit field contains the latched value of the internal Tach pulse counter, which may be configured by the Tach Reading Mode Select field to operate as a free-running counter or to be gated by the Tach input signal."]
            #[inline(always)]
            pub const fn cntr(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0xffff;
                val as u16
            }
            #[doc = "This 16-bit field contains the latched value of the internal Tach pulse counter, which may be configured by the Tach Reading Mode Select field to operate as a free-running counter or to be gated by the Tach input signal."]
            #[inline(always)]
            pub fn set_cntr(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
            }
        }
        impl Default for Ctrl {
            #[inline(always)]
            fn default() -> Ctrl {
                Ctrl(0)
            }
        }
        impl core::fmt::Debug for Ctrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ctrl")
                    .field("outof_lim_en", &self.outof_lim_en())
                    .field("en", &self.en())
                    .field("filt_en", &self.filt_en())
                    .field("rd_mod_sel", &self.rd_mod_sel())
                    .field("edges", &self.edges())
                    .field("cnt_rdy_int_en", &self.cnt_rdy_int_en())
                    .field("in_int_en", &self.in_int_en())
                    .field("cntr", &self.cntr())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ctrl {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Ctrl {{ outof_lim_en: {=bool:?}, en: {=bool:?}, filt_en: {=bool:?}, rd_mod_sel: {=bool:?}, edges: {=u8:?}, cnt_rdy_int_en: {=bool:?}, in_int_en: {=bool:?}, cntr: {=u16:?} }}" , self . outof_lim_en () , self . en () , self . filt_en () , self . rd_mod_sel () , self . edges () , self . cnt_rdy_int_en () , self . in_int_en () , self . cntr ())
            }
        }
        #[doc = "TACH HIGH LIMIT Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct LimHi(pub u32);
        impl LimHi {
            #[doc = "This value is compared with the value in the TACHX_COUNTER field. If the value in the counter is greater than the value programmed in this register, the TACH_OUT_OF_LIMIT_STATUS bit will be set. The TACH_OUT_OF_LIMIT_STATUS status event may be enabled to generate an interrupt to the embedded controller via the TACH_OUT_OF_LIMIT_ENABLE bit in the TACHx Control Register."]
            #[inline(always)]
            pub const fn t_high(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "This value is compared with the value in the TACHX_COUNTER field. If the value in the counter is greater than the value programmed in this register, the TACH_OUT_OF_LIMIT_STATUS bit will be set. The TACH_OUT_OF_LIMIT_STATUS status event may be enabled to generate an interrupt to the embedded controller via the TACH_OUT_OF_LIMIT_ENABLE bit in the TACHx Control Register."]
            #[inline(always)]
            pub fn set_t_high(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for LimHi {
            #[inline(always)]
            fn default() -> LimHi {
                LimHi(0)
            }
        }
        impl core::fmt::Debug for LimHi {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("LimHi").field("t_high", &self.t_high()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for LimHi {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "LimHi {{ t_high: {=u16:?} }}", self.t_high())
            }
        }
        #[doc = "TACHx Low Limit Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct LimLo(pub u32);
        impl LimLo {
            #[doc = "This value is compared with the value in the TACHX_COUNTER field of the TACHx Control Register. If the value in the counter is less than the value programmed in this register, the TACH_OUT_OF_LIMIT_STATUS bit will be set. The TACH_OUT_OF_LIMIT_STATUS status event may be enabled to generate an interrupt to the embedded controller via the TACH_OUT_OF_LIMIT_ENABLE bit in the TACHx Control Register To disable the TACH_OUT_OF_LIMIT_STATUS low event, program 0000h into this register."]
            #[inline(always)]
            pub const fn t_low(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "This value is compared with the value in the TACHX_COUNTER field of the TACHx Control Register. If the value in the counter is less than the value programmed in this register, the TACH_OUT_OF_LIMIT_STATUS bit will be set. The TACH_OUT_OF_LIMIT_STATUS status event may be enabled to generate an interrupt to the embedded controller via the TACH_OUT_OF_LIMIT_ENABLE bit in the TACHx Control Register To disable the TACH_OUT_OF_LIMIT_STATUS low event, program 0000h into this register."]
            #[inline(always)]
            pub fn set_t_low(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for LimLo {
            #[inline(always)]
            fn default() -> LimLo {
                LimLo(0)
            }
        }
        impl core::fmt::Debug for LimLo {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("LimLo").field("t_low", &self.t_low()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for LimLo {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "LimLo {{ t_low: {=u16:?} }}", self.t_low())
            }
        }
        #[doc = "TACHx Status Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sts(pub u32);
        impl Sts {
            #[doc = "TACH_OUT_OF_LIMIT_STATUS 1=Tach is outside of limits, 0=Tach is within limits (R/WC)"]
            #[inline(always)]
            pub const fn outof_lim_sts(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "TACH_OUT_OF_LIMIT_STATUS 1=Tach is outside of limits, 0=Tach is within limits (R/WC)"]
            #[inline(always)]
            pub fn set_outof_lim_sts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "TACH_PIN_STATUS 1= Tach Input is high, 0= Tach Input is low"]
            #[inline(always)]
            pub const fn pin_sts(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "TACH_PIN_STATUS 1= Tach Input is high, 0= Tach Input is low"]
            #[inline(always)]
            pub fn set_pin_sts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "TOGGLE_STATUS 1=Tach Input changed state (this bit is set on a low-to-high or high-tolow transition), 0=Tach stable (R/WC)"]
            #[inline(always)]
            pub const fn tog_sts(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "TOGGLE_STATUS 1=Tach Input changed state (this bit is set on a low-to-high or high-tolow transition), 0=Tach stable (R/WC)"]
            #[inline(always)]
            pub fn set_tog_sts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "COUNT_READY_STATUS 1=Reading ready, 0=Reading not ready"]
            #[inline(always)]
            pub const fn cnt_rdy_sts(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "COUNT_READY_STATUS 1=Reading ready, 0=Reading not ready"]
            #[inline(always)]
            pub fn set_cnt_rdy_sts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Sts {
            #[inline(always)]
            fn default() -> Sts {
                Sts(0)
            }
        }
        impl core::fmt::Debug for Sts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Sts")
                    .field("outof_lim_sts", &self.outof_lim_sts())
                    .field("pin_sts", &self.pin_sts())
                    .field("tog_sts", &self.tog_sts())
                    .field("cnt_rdy_sts", &self.cnt_rdy_sts())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Sts {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Sts {{ outof_lim_sts: {=bool:?}, pin_sts: {=bool:?}, tog_sts: {=bool:?}, cnt_rdy_sts: {=bool:?} }}" , self . outof_lim_sts () , self . pin_sts () , self . tog_sts () , self . cnt_rdy_sts ())
            }
        }
    }
}
pub mod tfdp {
    #[doc = "The TFDP serially transmits EC-originated diagnostic vectors to an external debug trace system."]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tfdp {
        ptr: *mut u8,
    }
    unsafe impl Send for Tfdp {}
    unsafe impl Sync for Tfdp {}
    impl Tfdp {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Debug data to be shifted out on the TFDP Debug port. While data is being shifted out, the Host Interface will 'hold-off' additional writes to the data register until the transfer is complete."]
        #[inline(always)]
        pub const fn msdata(self) -> crate::common::Reg<u8, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Debug Control Register"]
        #[inline(always)]
        pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Debug Control Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctrl(pub u8);
        impl Ctrl {
            #[doc = "Enable. 1=Clock enabled, 0=Clock is disabled (Default)"]
            #[inline(always)]
            pub const fn en(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Enable. 1=Clock enabled, 0=Clock is disabled (Default)"]
            #[inline(always)]
            pub fn set_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
            #[doc = "1= Data is shifted out on the falling edge of the debug clock, 0= Data is shifted out on the rising edge of the debug clock (Default)"]
            #[inline(always)]
            pub const fn edge_sel(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "1= Data is shifted out on the falling edge of the debug clock, 0= Data is shifted out on the rising edge of the debug clock (Default)"]
            #[inline(always)]
            pub fn set_edge_sel(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
            }
            #[doc = "Clock Divider Select."]
            #[inline(always)]
            pub const fn divsel(&self) -> u8 {
                let val = (self.0 >> 2usize) & 0x03;
                val as u8
            }
            #[doc = "Clock Divider Select."]
            #[inline(always)]
            pub fn set_divsel(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
            }
            #[doc = "Inter-packet Delay. The delay is in terms of TFDP Debug output clocks."]
            #[inline(always)]
            pub const fn ip_dly(&self) -> u8 {
                let val = (self.0 >> 4usize) & 0x07;
                val as u8
            }
            #[doc = "Inter-packet Delay. The delay is in terms of TFDP Debug output clocks."]
            #[inline(always)]
            pub fn set_ip_dly(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u8) & 0x07) << 4usize);
            }
        }
        impl Default for Ctrl {
            #[inline(always)]
            fn default() -> Ctrl {
                Ctrl(0)
            }
        }
        impl core::fmt::Debug for Ctrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ctrl")
                    .field("en", &self.en())
                    .field("edge_sel", &self.edge_sel())
                    .field("divsel", &self.divsel())
                    .field("ip_dly", &self.ip_dly())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ctrl {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Ctrl {{ en: {=bool:?}, edge_sel: {=bool:?}, divsel: {=u8:?}, ip_dly: {=u8:?} }}",
                    self.en(),
                    self.edge_sel(),
                    self.divsel(),
                    self.ip_dly()
                )
            }
        }
    }
}
pub mod timer16_0 {
    #[doc = "This 16-bit timer block offers a simple mechanism for firmware to maintain a time base"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer160 {
        ptr: *mut u8,
    }
    unsafe impl Send for Timer160 {}
    unsafe impl Sync for Timer160 {}
    impl Timer160 {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "This is the value of the Timer counter. This is updated by Hardware but may be set by Firmware."]
        #[inline(always)]
        pub const fn cnt(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "This is the value of the Timer pre-load for the counter. This is used by H/W when the counter is to be restarted automatically; this will become the new value of the counter upon restart."]
        #[inline(always)]
        pub const fn prld(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "This is the interrupt status that fires when the timer reaches its limit"]
        #[inline(always)]
        pub const fn sts(self) -> crate::common::Reg<regs::Sts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "This is the interrupt enable for the status EVENT_INTERRUPT bit in the Timer Status Register"]
        #[inline(always)]
        pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Timer Control Register"]
        #[inline(always)]
        pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Timer Control Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctrl(pub u32);
        impl Ctrl {
            #[doc = "This enables the block for operation. 1=This block will function normally; 0=This block will gate its clock and go into its lowest power state"]
            #[inline(always)]
            pub const fn en(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This enables the block for operation. 1=This block will function normally; 0=This block will gate its clock and go into its lowest power state"]
            #[inline(always)]
            pub fn set_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "This selects the counter direction. When the counter in incrementing the counter will saturate and trigger the event when it reaches all F's. When the counter is decrementing the counter will saturate when it reaches 0h. 1=The counter will increment; 0=The counter will decrement"]
            #[inline(always)]
            pub const fn cnt_up(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "This selects the counter direction. When the counter in incrementing the counter will saturate and trigger the event when it reaches all F's. When the counter is decrementing the counter will saturate when it reaches 0h. 1=The counter will increment; 0=The counter will decrement"]
            #[inline(always)]
            pub fn set_cnt_up(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "This will select the action taken upon completing a count. 1=The counter will automatically restart the count, using the contents of the Timer Preload Register to load the Timer Count Register. The interrupt will be set in edge mode 0=The counter will simply enter a done state and wait for further control inputs. The interrupt will be set in level mode."]
            #[inline(always)]
            pub const fn au_restrt(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "This will select the action taken upon completing a count. 1=The counter will automatically restart the count, using the contents of the Timer Preload Register to load the Timer Count Register. The interrupt will be set in edge mode 0=The counter will simply enter a done state and wait for further control inputs. The interrupt will be set in level mode."]
            #[inline(always)]
            pub fn set_au_restrt(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "This is a soft reset. This is self clearing 1 cycle after it is written. Firmware does not need to wait before reconfiguring the Basic Timer following soft reset."]
            #[inline(always)]
            pub const fn sft_rst(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "This is a soft reset. This is self clearing 1 cycle after it is written. Firmware does not need to wait before reconfiguring the Basic Timer following soft reset."]
            #[inline(always)]
            pub fn set_sft_rst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "This bit triggers the timer counter. The counter will operate until it hits its terminating condition. This will clear this bit. It should be noted that when operating in restart mode, there is no terminating condition for the counter, so this bit will never clear. Clearing this bit will halt the timer counter."]
            #[inline(always)]
            pub const fn strt(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "This bit triggers the timer counter. The counter will operate until it hits its terminating condition. This will clear this bit. It should be noted that when operating in restart mode, there is no terminating condition for the counter, so this bit will never clear. Clearing this bit will halt the timer counter."]
            #[inline(always)]
            pub fn set_strt(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "This bit reloads the counter without interrupting it operation. This will not function if the timer has already completed (when the START bit in this register is '0'). This is used to periodically prevent the timer from firing when an event occurs. Usage while the timer is off may result in erroneous behaviour."]
            #[inline(always)]
            pub const fn rld(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "This bit reloads the counter without interrupting it operation. This will not function if the timer has already completed (when the START bit in this register is '0'). This is used to periodically prevent the timer from firing when an event occurs. Usage while the timer is off may result in erroneous behaviour."]
            #[inline(always)]
            pub fn set_rld(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "This is a halt bit. This will halt the timer as long as it is active. Once the halt is inactive, the timer will start from where it left off. 1=Timer is halted. It stops counting. The clock divider will also be reset. 0=Timer runs normally."]
            #[inline(always)]
            pub const fn hlt(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "This is a halt bit. This will halt the timer as long as it is active. Once the halt is inactive, the timer will start from where it left off. 1=Timer is halted. It stops counting. The clock divider will also be reset. 0=Timer runs normally."]
            #[inline(always)]
            pub fn set_hlt(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "This is used to divide down the system clock through clock enables to lower the power consumption of the block and allow slow timers. Updating this value during operation may result in erroneous clock enable pulses until the clock divider restarts. The number of clocks per clock enable pulse is (Value + 1); a setting of 0 runs at the full clock speed, while a setting of 1 runs at half speed."]
            #[inline(always)]
            pub const fn prescale(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0xffff;
                val as u16
            }
            #[doc = "This is used to divide down the system clock through clock enables to lower the power consumption of the block and allow slow timers. Updating this value during operation may result in erroneous clock enable pulses until the clock divider restarts. The number of clocks per clock enable pulse is (Value + 1); a setting of 0 runs at the full clock speed, while a setting of 1 runs at half speed."]
            #[inline(always)]
            pub fn set_prescale(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
            }
        }
        impl Default for Ctrl {
            #[inline(always)]
            fn default() -> Ctrl {
                Ctrl(0)
            }
        }
        impl core::fmt::Debug for Ctrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ctrl")
                    .field("en", &self.en())
                    .field("cnt_up", &self.cnt_up())
                    .field("au_restrt", &self.au_restrt())
                    .field("sft_rst", &self.sft_rst())
                    .field("strt", &self.strt())
                    .field("rld", &self.rld())
                    .field("hlt", &self.hlt())
                    .field("prescale", &self.prescale())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ctrl {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Ctrl {{ en: {=bool:?}, cnt_up: {=bool:?}, au_restrt: {=bool:?}, sft_rst: {=bool:?}, strt: {=bool:?}, rld: {=bool:?}, hlt: {=bool:?}, prescale: {=u16:?} }}" , self . en () , self . cnt_up () , self . au_restrt () , self . sft_rst () , self . strt () , self . rld () , self . hlt () , self . prescale ())
            }
        }
        #[doc = "This is the interrupt enable for the status EVENT_INTERRUPT bit in the Timer Status Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ien(pub u32);
        impl Ien {
            #[doc = "This is the interrupt enable for the status EVENT_INTERRUPT bit in the Timer Status Register."]
            #[inline(always)]
            pub const fn en(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This is the interrupt enable for the status EVENT_INTERRUPT bit in the Timer Status Register."]
            #[inline(always)]
            pub fn set_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Ien {
            #[inline(always)]
            fn default() -> Ien {
                Ien(0)
            }
        }
        impl core::fmt::Debug for Ien {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ien").field("en", &self.en()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ien {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Ien {{ en: {=bool:?} }}", self.en())
            }
        }
        #[doc = "This is the interrupt status that fires when the timer reaches its limit"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sts(pub u32);
        impl Sts {
            #[doc = "This is the interrupt status that fires when the timer reaches its limit. This is the interrupt status that fires when the timer reaches its limit. This may be level or a self clearing signal cycle pulse, based on the AUTO_RESTART bit in the Timer Control Register. If the timer is set to automatically restart, it will provide a pulse, otherwise a level is provided.(R/WC)"]
            #[inline(always)]
            pub const fn evt_int(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This is the interrupt status that fires when the timer reaches its limit. This is the interrupt status that fires when the timer reaches its limit. This may be level or a self clearing signal cycle pulse, based on the AUTO_RESTART bit in the Timer Control Register. If the timer is set to automatically restart, it will provide a pulse, otherwise a level is provided.(R/WC)"]
            #[inline(always)]
            pub fn set_evt_int(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Sts {
            #[inline(always)]
            fn default() -> Sts {
                Sts(0)
            }
        }
        impl core::fmt::Debug for Sts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Sts").field("evt_int", &self.evt_int()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Sts {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Sts {{ evt_int: {=bool:?} }}", self.evt_int())
            }
        }
    }
}
pub mod timer32_0 {
    #[doc = "This 32-bit timer block offers a simple mechanism for firmware to maintain a time base"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer320 {
        ptr: *mut u8,
    }
    unsafe impl Send for Timer320 {}
    unsafe impl Sync for Timer320 {}
    impl Timer320 {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "This is the value of the Timer counter. This is updated by Hardware but may be set by Firmware."]
        #[inline(always)]
        pub const fn cnt(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "This is the value of the Timer pre-load for the counter. This is used by H/W when the counter is to be restarted automatically; this will become the new value of the counter upon restart."]
        #[inline(always)]
        pub const fn prld(self) -> crate::common::Reg<u32, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "This is the interrupt status that fires when the timer reaches its limit"]
        #[inline(always)]
        pub const fn sts(self) -> crate::common::Reg<regs::Sts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "This is the interrupt enable for the status EVENT_INTERRUPT bit in the Timer Status Register"]
        #[inline(always)]
        pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Timer Control Register"]
        #[inline(always)]
        pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Timer Control Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctrl(pub u32);
        impl Ctrl {
            #[doc = "This enables the block for operation. 1=This block will function normally; 0=This block will gate its clock and go into its lowest power state"]
            #[inline(always)]
            pub const fn en(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This enables the block for operation. 1=This block will function normally; 0=This block will gate its clock and go into its lowest power state"]
            #[inline(always)]
            pub fn set_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "This selects the counter direction. When the counter in incrementing the counter will saturate and trigger the event when it reaches all F's. When the counter is decrementing the counter will saturate when it reaches 0h. 1=The counter will increment; 0=The counter will decrement"]
            #[inline(always)]
            pub const fn cnt_up(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "This selects the counter direction. When the counter in incrementing the counter will saturate and trigger the event when it reaches all F's. When the counter is decrementing the counter will saturate when it reaches 0h. 1=The counter will increment; 0=The counter will decrement"]
            #[inline(always)]
            pub fn set_cnt_up(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "This will select the action taken upon completing a count. 1=The counter will automatically restart the count, using the contents of the Timer Preload Register to load the Timer Count Register. The interrupt will be set in edge mode 0=The counter will simply enter a done state and wait for further control inputs. The interrupt will be set in level mode."]
            #[inline(always)]
            pub const fn au_restrt(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "This will select the action taken upon completing a count. 1=The counter will automatically restart the count, using the contents of the Timer Preload Register to load the Timer Count Register. The interrupt will be set in edge mode 0=The counter will simply enter a done state and wait for further control inputs. The interrupt will be set in level mode."]
            #[inline(always)]
            pub fn set_au_restrt(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "This is a soft reset. This is self clearing 1 cycle after it is written. Firmware does not need to wait before reconfiguring the Basic Timer following soft reset."]
            #[inline(always)]
            pub const fn sft_rst(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "This is a soft reset. This is self clearing 1 cycle after it is written. Firmware does not need to wait before reconfiguring the Basic Timer following soft reset."]
            #[inline(always)]
            pub fn set_sft_rst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "This bit triggers the timer counter. The counter will operate until it hits its terminating condition. This will clear this bit. It should be noted that when operating in restart mode, there is no terminating condition for the counter, so this bit will never clear. Clearing this bit will halt the timer counter."]
            #[inline(always)]
            pub const fn strt(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "This bit triggers the timer counter. The counter will operate until it hits its terminating condition. This will clear this bit. It should be noted that when operating in restart mode, there is no terminating condition for the counter, so this bit will never clear. Clearing this bit will halt the timer counter."]
            #[inline(always)]
            pub fn set_strt(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "This bit reloads the counter without interrupting it operation. This will not function if the timer has already completed (when the START bit in this register is '0'). This is used to periodically prevent the timer from firing when an event occurs. Usage while the timer is off may result in erroneous behaviour."]
            #[inline(always)]
            pub const fn rld(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "This bit reloads the counter without interrupting it operation. This will not function if the timer has already completed (when the START bit in this register is '0'). This is used to periodically prevent the timer from firing when an event occurs. Usage while the timer is off may result in erroneous behaviour."]
            #[inline(always)]
            pub fn set_rld(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "This is a halt bit. This will halt the timer as long as it is active. Once the halt is inactive, the timer will start from where it left off. 1=Timer is halted. It stops counting. The clock divider will also be reset. 0=Timer runs normally."]
            #[inline(always)]
            pub const fn hlt(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "This is a halt bit. This will halt the timer as long as it is active. Once the halt is inactive, the timer will start from where it left off. 1=Timer is halted. It stops counting. The clock divider will also be reset. 0=Timer runs normally."]
            #[inline(always)]
            pub fn set_hlt(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "This is used to divide down the system clock through clock enables to lower the power consumption of the block and allow slow timers. Updating this value during operation may result in erroneous clock enable pulses until the clock divider restarts. The number of clocks per clock enable pulse is (Value + 1); a setting of 0 runs at the full clock speed, while a setting of 1 runs at half speed."]
            #[inline(always)]
            pub const fn prescale(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0xffff;
                val as u16
            }
            #[doc = "This is used to divide down the system clock through clock enables to lower the power consumption of the block and allow slow timers. Updating this value during operation may result in erroneous clock enable pulses until the clock divider restarts. The number of clocks per clock enable pulse is (Value + 1); a setting of 0 runs at the full clock speed, while a setting of 1 runs at half speed."]
            #[inline(always)]
            pub fn set_prescale(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
            }
        }
        impl Default for Ctrl {
            #[inline(always)]
            fn default() -> Ctrl {
                Ctrl(0)
            }
        }
        impl core::fmt::Debug for Ctrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ctrl")
                    .field("en", &self.en())
                    .field("cnt_up", &self.cnt_up())
                    .field("au_restrt", &self.au_restrt())
                    .field("sft_rst", &self.sft_rst())
                    .field("strt", &self.strt())
                    .field("rld", &self.rld())
                    .field("hlt", &self.hlt())
                    .field("prescale", &self.prescale())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ctrl {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Ctrl {{ en: {=bool:?}, cnt_up: {=bool:?}, au_restrt: {=bool:?}, sft_rst: {=bool:?}, strt: {=bool:?}, rld: {=bool:?}, hlt: {=bool:?}, prescale: {=u16:?} }}" , self . en () , self . cnt_up () , self . au_restrt () , self . sft_rst () , self . strt () , self . rld () , self . hlt () , self . prescale ())
            }
        }
        #[doc = "This is the interrupt enable for the status EVENT_INTERRUPT bit in the Timer Status Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ien(pub u32);
        impl Ien {
            #[doc = "This is the interrupt enable for the status EVENT_INTERRUPT bit in the Timer Status Register."]
            #[inline(always)]
            pub const fn en(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This is the interrupt enable for the status EVENT_INTERRUPT bit in the Timer Status Register."]
            #[inline(always)]
            pub fn set_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Ien {
            #[inline(always)]
            fn default() -> Ien {
                Ien(0)
            }
        }
        impl core::fmt::Debug for Ien {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ien").field("en", &self.en()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ien {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Ien {{ en: {=bool:?} }}", self.en())
            }
        }
        #[doc = "This is the interrupt status that fires when the timer reaches its limit"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sts(pub u32);
        impl Sts {
            #[doc = "This is the interrupt status that fires when the timer reaches its limit. This is the interrupt status that fires when the timer reaches its limit. This may be level or a self clearing signal cycle pulse, based on the AUTO_RESTART bit in the Timer Control Register. If the timer is set to automatically restart, it will provide a pulse, otherwise a level is provided.(R/WC)"]
            #[inline(always)]
            pub const fn evt_int(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This is the interrupt status that fires when the timer reaches its limit. This is the interrupt status that fires when the timer reaches its limit. This may be level or a self clearing signal cycle pulse, based on the AUTO_RESTART bit in the Timer Control Register. If the timer is set to automatically restart, it will provide a pulse, otherwise a level is provided.(R/WC)"]
            #[inline(always)]
            pub fn set_evt_int(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Sts {
            #[inline(always)]
            fn default() -> Sts {
                Sts(0)
            }
        }
        impl core::fmt::Debug for Sts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Sts").field("evt_int", &self.evt_int()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Sts {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Sts {{ evt_int: {=bool:?} }}", self.evt_int())
            }
        }
    }
}
pub mod uart0 {
    #[doc = "UART when DLAB=0"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Data {
        ptr: *mut u8,
    }
    unsafe impl Send for Data {}
    unsafe impl Sync for Data {}
    impl Data {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "UART Receive (Read) Buffer Register (DLAB=0)"]
        #[inline(always)]
        pub const fn rx_dat(self) -> crate::common::Reg<u8, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "UART Transmit (Write) Buffer Register (DLAB=0)"]
        #[inline(always)]
        pub const fn tx_dat(self) -> crate::common::Reg<u8, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "UART Interrupt Enable Register (DLAB=0)"]
        #[inline(always)]
        pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01usize) as _) }
        }
        #[doc = "UART FIFO Control Register"]
        #[inline(always)]
        pub const fn fifo_cr(self) -> crate::common::Reg<regs::DataFifoCr, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
        }
        #[doc = "UART Interrupt Identification Register"]
        #[inline(always)]
        pub const fn int_id(self) -> crate::common::Reg<regs::DataIntId, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
        }
        #[doc = "UART Line Control Register"]
        #[inline(always)]
        pub const fn lcr(self) -> crate::common::Reg<regs::DataLcr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03usize) as _) }
        }
        #[doc = "UART Modem Control Register"]
        #[inline(always)]
        pub const fn mcr(self) -> crate::common::Reg<regs::DataMcr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "UART Line Status Register"]
        #[inline(always)]
        pub const fn lsr(self) -> crate::common::Reg<regs::DataLsr, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05usize) as _) }
        }
        #[doc = "UART Modem Status Register"]
        #[inline(always)]
        pub const fn msr(self) -> crate::common::Reg<regs::DataMsr, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
        }
        #[doc = "UART Scratchpad Register This 8 bit read/write register has no effect on the operation of the Serial Port. It is intended as a scratchpad register to be used by the programmer to hold data temporarily."]
        #[inline(always)]
        pub const fn scr(self) -> crate::common::Reg<u8, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07usize) as _) }
        }
        #[doc = "UART Activate Register. \\[0:0\\] ACTIVATE When this bit is 1, the UART logical device is powered and functional. When this bit is 0, the UART logical device is powered down and inactive."]
        #[inline(always)]
        pub const fn activate(self) -> crate::common::Reg<u8, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0330usize) as _) }
        }
        #[doc = "UART Config Select Register"]
        #[inline(always)]
        pub const fn cfg_sel(self) -> crate::common::Reg<regs::DataCfgSel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03f0usize) as _) }
        }
    }
    #[doc = "UART when DLAB=1"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlab {
        ptr: *mut u8,
    }
    unsafe impl Send for Dlab {}
    unsafe impl Sync for Dlab {}
    impl Dlab {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "UART Programmable BAUD Rate Generator (LSB) Register (DLAB=1)"]
        #[inline(always)]
        pub const fn baudrt_lsb(self) -> crate::common::Reg<u8, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "UART Programmable BAUD Rate Generator (MSB) Register (DLAB=1). \\[6:0\\] BAUD_RATE_DIVISOR_MSB, \\[7:7\\] BAUD_CLK_SEL 1=If CLK_SRC is '0', the baud clock is derived from the 1.8432MHz_Clk. If CLK_SRC is '1', this bit has no effect 0=If CLK_SRC is '0', the baud clock is derived from the 24MHz_Clk. If CLK_SRC is '1', this bit has no effect"]
        #[inline(always)]
        pub const fn baudrt_msb(self) -> crate::common::Reg<u8, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01usize) as _) }
        }
        #[doc = "UART FIFO Control Register"]
        #[inline(always)]
        pub const fn fifo_cr(self) -> crate::common::Reg<regs::DlabFifoCr, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
        }
        #[doc = "UART Interrupt Identification Register"]
        #[inline(always)]
        pub const fn int_id(self) -> crate::common::Reg<regs::DlabIntId, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
        }
        #[doc = "UART Line Control Register"]
        #[inline(always)]
        pub const fn lcr(self) -> crate::common::Reg<regs::DlabLcr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03usize) as _) }
        }
        #[doc = "UART Modem Control Register"]
        #[inline(always)]
        pub const fn mcr(self) -> crate::common::Reg<regs::DlabMcr, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "UART Line Status Register"]
        #[inline(always)]
        pub const fn lsr(self) -> crate::common::Reg<regs::DlabLsr, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05usize) as _) }
        }
        #[doc = "UART Modem Status Register"]
        #[inline(always)]
        pub const fn msr(self) -> crate::common::Reg<regs::DlabMsr, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
        }
        #[doc = "UART Scratchpad Register This 8 bit read/write register has no effect on the operation of the Serial Port. It is intended as a scratchpad register to be used by the programmer to hold data temporarily."]
        #[inline(always)]
        pub const fn scr(self) -> crate::common::Reg<u8, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07usize) as _) }
        }
        #[doc = "UART Activate Register. \\[0:0\\] ACTIVATE When this bit is 1, the UART logical device is powered and functional. When this bit is 0, the UART logical device is powered down and inactive."]
        #[inline(always)]
        pub const fn activate(self) -> crate::common::Reg<u8, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0330usize) as _) }
        }
        #[doc = "UART Config Select Register"]
        #[inline(always)]
        pub const fn cfg_sel(self) -> crate::common::Reg<regs::DlabCfgSel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03f0usize) as _) }
        }
    }
    #[doc = "The 16550 UART is a full-function Two Pin Serial Port that supports the standard RS-232 Interface."]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uart0 {
        ptr: *mut u8,
    }
    unsafe impl Send for Uart0 {}
    unsafe impl Sync for Uart0 {}
    impl Uart0 {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "UART when DLAB=0"]
        #[inline(always)]
        pub const fn data(self) -> Data {
            unsafe { Data::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "UART when DLAB=1"]
        #[inline(always)]
        pub const fn dlab(self) -> Dlab {
            unsafe { Dlab::from_ptr(self.ptr.add(0x0usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "UART Config Select Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DataCfgSel(pub u8);
        impl DataCfgSel {
            #[doc = "CLK_SRC 1=The UART Baud Clock is derived from an external clock source, 0=The UART Baud Clock is derived from one of the two internal clock sources"]
            #[inline(always)]
            pub const fn clk_src(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "CLK_SRC 1=The UART Baud Clock is derived from an external clock source, 0=The UART Baud Clock is derived from one of the two internal clock sources"]
            #[inline(always)]
            pub fn set_clk_src(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
            #[doc = "POWER 1=The RESET reset signal is derived from nSIO_RESET, 0=The RESET reset signal is derived from VCC1_RESET"]
            #[inline(always)]
            pub const fn pwr(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "POWER 1=The RESET reset signal is derived from nSIO_RESET, 0=The RESET reset signal is derived from VCC1_RESET"]
            #[inline(always)]
            pub fn set_pwr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
            }
            #[doc = "POLARITY 1=The UART_TX and UART_RX pins functions are inverted, 0=The UART_TX and UART_RX pins functions are not inverted"]
            #[inline(always)]
            pub const fn polar(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "POLARITY 1=The UART_TX and UART_RX pins functions are inverted, 0=The UART_TX and UART_RX pins functions are not inverted"]
            #[inline(always)]
            pub fn set_polar(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
            }
        }
        impl Default for DataCfgSel {
            #[inline(always)]
            fn default() -> DataCfgSel {
                DataCfgSel(0)
            }
        }
        impl core::fmt::Debug for DataCfgSel {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DataCfgSel")
                    .field("clk_src", &self.clk_src())
                    .field("pwr", &self.pwr())
                    .field("polar", &self.polar())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DataCfgSel {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "DataCfgSel {{ clk_src: {=bool:?}, pwr: {=bool:?}, polar: {=bool:?} }}",
                    self.clk_src(),
                    self.pwr(),
                    self.polar()
                )
            }
        }
        #[doc = "UART FIFO Control Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DataFifoCr(pub u8);
        impl DataFifoCr {
            #[doc = "EXRF Enable XMIT and RECV FIFO."]
            #[inline(always)]
            pub const fn exrf(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "EXRF Enable XMIT and RECV FIFO."]
            #[inline(always)]
            pub fn set_exrf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
            #[doc = "CLEAR_RECV_FIFO Setting this bit to a logic '1' clears all bytes in the RCVR FIFO and resets its counter logic to '0'."]
            #[inline(always)]
            pub const fn clr_recv_fifo(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "CLEAR_RECV_FIFO Setting this bit to a logic '1' clears all bytes in the RCVR FIFO and resets its counter logic to '0'."]
            #[inline(always)]
            pub fn set_clr_recv_fifo(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
            }
            #[doc = "CLEAR_XMIT_FIFO Setting this bit to a logic '1' clears all bytes in the XMIT FIFO and resets its counter logic to '0' . The shift register is not cleared. This bit is self-clearing."]
            #[inline(always)]
            pub const fn clr_xmit_fifo(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "CLEAR_XMIT_FIFO Setting this bit to a logic '1' clears all bytes in the XMIT FIFO and resets its counter logic to '0' . The shift register is not cleared. This bit is self-clearing."]
            #[inline(always)]
            pub fn set_clr_xmit_fifo(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
            }
            #[doc = "DMA_MODE_SELECT Writing to this bit has no effect on the operation of the UART. The RXRDY and TXRDY pins are not available on this chip."]
            #[inline(always)]
            pub const fn dma_mode_sel(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "DMA_MODE_SELECT Writing to this bit has no effect on the operation of the UART. The RXRDY and TXRDY pins are not available on this chip."]
            #[inline(always)]
            pub fn set_dma_mode_sel(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
            }
            #[doc = "RECV_FIFO_TRIGGER_LEVEL These bits are used to set the trigger level for the RCVR FIFO interrupt."]
            #[inline(always)]
            pub const fn recv_fifo_trig_lvl(&self) -> u8 {
                let val = (self.0 >> 6usize) & 0x03;
                val as u8
            }
            #[doc = "RECV_FIFO_TRIGGER_LEVEL These bits are used to set the trigger level for the RCVR FIFO interrupt."]
            #[inline(always)]
            pub fn set_recv_fifo_trig_lvl(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
            }
        }
        impl Default for DataFifoCr {
            #[inline(always)]
            fn default() -> DataFifoCr {
                DataFifoCr(0)
            }
        }
        impl core::fmt::Debug for DataFifoCr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DataFifoCr")
                    .field("exrf", &self.exrf())
                    .field("clr_recv_fifo", &self.clr_recv_fifo())
                    .field("clr_xmit_fifo", &self.clr_xmit_fifo())
                    .field("dma_mode_sel", &self.dma_mode_sel())
                    .field("recv_fifo_trig_lvl", &self.recv_fifo_trig_lvl())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DataFifoCr {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "DataFifoCr {{ exrf: {=bool:?}, clr_recv_fifo: {=bool:?}, clr_xmit_fifo: {=bool:?}, dma_mode_sel: {=bool:?}, recv_fifo_trig_lvl: {=u8:?} }}" , self . exrf () , self . clr_recv_fifo () , self . clr_xmit_fifo () , self . dma_mode_sel () , self . recv_fifo_trig_lvl ())
            }
        }
        #[doc = "UART Interrupt Identification Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DataIntId(pub u8);
        impl DataIntId {
            #[doc = "IPEND This bit can be used in either a hardwired prioritized or polled environment to indicate whether an interrupt is pending."]
            #[inline(always)]
            pub const fn ipend(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "IPEND This bit can be used in either a hardwired prioritized or polled environment to indicate whether an interrupt is pending."]
            #[inline(always)]
            pub fn set_ipend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
            #[doc = "INTID These bits identify the highest priority interrupt pending"]
            #[inline(always)]
            pub const fn intid(&self) -> u8 {
                let val = (self.0 >> 1usize) & 0x07;
                val as u8
            }
            #[doc = "INTID These bits identify the highest priority interrupt pending"]
            #[inline(always)]
            pub fn set_intid(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u8) & 0x07) << 1usize);
            }
            #[doc = "These two bits are set when the FIFO CONTROL Register bit 0 equals 1."]
            #[inline(always)]
            pub const fn fifo_en(&self) -> u8 {
                let val = (self.0 >> 6usize) & 0x03;
                val as u8
            }
            #[doc = "These two bits are set when the FIFO CONTROL Register bit 0 equals 1."]
            #[inline(always)]
            pub fn set_fifo_en(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
            }
        }
        impl Default for DataIntId {
            #[inline(always)]
            fn default() -> DataIntId {
                DataIntId(0)
            }
        }
        impl core::fmt::Debug for DataIntId {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DataIntId")
                    .field("ipend", &self.ipend())
                    .field("intid", &self.intid())
                    .field("fifo_en", &self.fifo_en())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DataIntId {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "DataIntId {{ ipend: {=bool:?}, intid: {=u8:?}, fifo_en: {=u8:?} }}",
                    self.ipend(),
                    self.intid(),
                    self.fifo_en()
                )
            }
        }
        #[doc = "UART Line Control Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DataLcr(pub u8);
        impl DataLcr {
            #[doc = "WORD_LENGTH These two bits specify the number of bits in each transmitted or received serial character."]
            #[inline(always)]
            pub const fn word_len(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x03;
                val as u8
            }
            #[doc = "WORD_LENGTH These two bits specify the number of bits in each transmitted or received serial character."]
            #[inline(always)]
            pub fn set_word_len(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
            }
            #[doc = "STOP_BITS This bit specifies the number of stop bits in each transmitted or received serial character."]
            #[inline(always)]
            pub const fn stop_bits(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "STOP_BITS This bit specifies the number of stop bits in each transmitted or received serial character."]
            #[inline(always)]
            pub fn set_stop_bits(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
            }
            #[doc = "ENABLE_PARITY Parity Enable bit."]
            #[inline(always)]
            pub const fn en_par(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "ENABLE_PARITY Parity Enable bit."]
            #[inline(always)]
            pub fn set_en_par(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
            }
            #[doc = "PARITY_SELECT Even Parity Select bit."]
            #[inline(always)]
            pub const fn par_sel(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "PARITY_SELECT Even Parity Select bit."]
            #[inline(always)]
            pub fn set_par_sel(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
            }
            #[doc = "STICK_PARITY Stick Parity bit."]
            #[inline(always)]
            pub const fn stick_par(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "STICK_PARITY Stick Parity bit."]
            #[inline(always)]
            pub fn set_stick_par(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
            }
            #[doc = "BREAK_CONTROL Set Break Control bit"]
            #[inline(always)]
            pub const fn brk_ctrl(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "BREAK_CONTROL Set Break Control bit"]
            #[inline(always)]
            pub fn set_brk_ctrl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
            }
            #[doc = "DLAB Divisor Latch Access Bit (DLAB)."]
            #[inline(always)]
            pub const fn dlab(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "DLAB Divisor Latch Access Bit (DLAB)."]
            #[inline(always)]
            pub fn set_dlab(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
            }
        }
        impl Default for DataLcr {
            #[inline(always)]
            fn default() -> DataLcr {
                DataLcr(0)
            }
        }
        impl core::fmt::Debug for DataLcr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DataLcr")
                    .field("word_len", &self.word_len())
                    .field("stop_bits", &self.stop_bits())
                    .field("en_par", &self.en_par())
                    .field("par_sel", &self.par_sel())
                    .field("stick_par", &self.stick_par())
                    .field("brk_ctrl", &self.brk_ctrl())
                    .field("dlab", &self.dlab())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DataLcr {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "DataLcr {{ word_len: {=u8:?}, stop_bits: {=bool:?}, en_par: {=bool:?}, par_sel: {=bool:?}, stick_par: {=bool:?}, brk_ctrl: {=bool:?}, dlab: {=bool:?} }}" , self . word_len () , self . stop_bits () , self . en_par () , self . par_sel () , self . stick_par () , self . brk_ctrl () , self . dlab ())
            }
        }
        #[doc = "UART Line Status Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DataLsr(pub u8);
        impl DataLsr {
            #[doc = "DATA_READY Data Ready. It is set to a logic '1' whenever a complete incoming character has been received and transferred into the Receiver Buffer Register or the FIFO"]
            #[inline(always)]
            pub const fn data_ready(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "DATA_READY Data Ready. It is set to a logic '1' whenever a complete incoming character has been received and transferred into the Receiver Buffer Register or the FIFO"]
            #[inline(always)]
            pub fn set_data_ready(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
            #[doc = "OVERRUN Overrun Error."]
            #[inline(always)]
            pub const fn overrun(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "OVERRUN Overrun Error."]
            #[inline(always)]
            pub fn set_overrun(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
            }
            #[doc = "PARITY ERROR Parity Error."]
            #[inline(always)]
            pub const fn pe(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "PARITY ERROR Parity Error."]
            #[inline(always)]
            pub fn set_pe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
            }
            #[doc = "FRAME_ERROR Framing Error."]
            #[inline(always)]
            pub const fn frame_err(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "FRAME_ERROR Framing Error."]
            #[inline(always)]
            pub fn set_frame_err(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
            }
            #[doc = "BREAK_INTERRUPT Break Interrupt."]
            #[inline(always)]
            pub const fn brk_intr(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "BREAK_INTERRUPT Break Interrupt."]
            #[inline(always)]
            pub fn set_brk_intr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
            }
            #[doc = "TRANSMIT_EMPTY Transmitter Holding Register Empty Bit 5 indicates that the Serial Port is ready to accept a new character for transmission."]
            #[inline(always)]
            pub const fn trans_empty(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "TRANSMIT_EMPTY Transmitter Holding Register Empty Bit 5 indicates that the Serial Port is ready to accept a new character for transmission."]
            #[inline(always)]
            pub fn set_trans_empty(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
            }
            #[doc = "Transmitter Empty. Bit 6 is set to a logic '1' whenever the Transmitter Holding Register (THR) and Transmitter Shift Register (TSR) are both empty."]
            #[inline(always)]
            pub const fn trans_err(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Transmitter Empty. Bit 6 is set to a logic '1' whenever the Transmitter Holding Register (THR) and Transmitter Shift Register (TSR) are both empty."]
            #[inline(always)]
            pub fn set_trans_err(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
            }
            #[doc = "FIFO_ERROR"]
            #[inline(always)]
            pub const fn fifo_err(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "FIFO_ERROR"]
            #[inline(always)]
            pub fn set_fifo_err(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
            }
        }
        impl Default for DataLsr {
            #[inline(always)]
            fn default() -> DataLsr {
                DataLsr(0)
            }
        }
        impl core::fmt::Debug for DataLsr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DataLsr")
                    .field("data_ready", &self.data_ready())
                    .field("overrun", &self.overrun())
                    .field("pe", &self.pe())
                    .field("frame_err", &self.frame_err())
                    .field("brk_intr", &self.brk_intr())
                    .field("trans_empty", &self.trans_empty())
                    .field("trans_err", &self.trans_err())
                    .field("fifo_err", &self.fifo_err())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DataLsr {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "DataLsr {{ data_ready: {=bool:?}, overrun: {=bool:?}, pe: {=bool:?}, frame_err: {=bool:?}, brk_intr: {=bool:?}, trans_empty: {=bool:?}, trans_err: {=bool:?}, fifo_err: {=bool:?} }}" , self . data_ready () , self . overrun () , self . pe () , self . frame_err () , self . brk_intr () , self . trans_empty () , self . trans_err () , self . fifo_err ())
            }
        }
        #[doc = "UART Modem Control Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DataMcr(pub u8);
        impl DataMcr {
            #[doc = "DTR This bit controls the Data Terminal Ready (nDTR) output."]
            #[inline(always)]
            pub const fn dtr(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "DTR This bit controls the Data Terminal Ready (nDTR) output."]
            #[inline(always)]
            pub fn set_dtr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
            #[doc = "RTS This bit controls the Request To Send (nRTS) output."]
            #[inline(always)]
            pub const fn rts(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "RTS This bit controls the Request To Send (nRTS) output."]
            #[inline(always)]
            pub fn set_rts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
            }
            #[doc = "OUT1 This bit controls the Output 1 (OUT1) bit."]
            #[inline(always)]
            pub const fn out1(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "OUT1 This bit controls the Output 1 (OUT1) bit."]
            #[inline(always)]
            pub fn set_out1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
            }
            #[doc = "OUT2 This bit is used to enable an UART interrupt."]
            #[inline(always)]
            pub const fn out2(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "OUT2 This bit is used to enable an UART interrupt."]
            #[inline(always)]
            pub fn set_out2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
            }
            #[doc = "LOOPBACK This bit provides the loopback feature for diagnostic testing of the Serial Port."]
            #[inline(always)]
            pub const fn loopback(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "LOOPBACK This bit provides the loopback feature for diagnostic testing of the Serial Port."]
            #[inline(always)]
            pub fn set_loopback(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
            }
        }
        impl Default for DataMcr {
            #[inline(always)]
            fn default() -> DataMcr {
                DataMcr(0)
            }
        }
        impl core::fmt::Debug for DataMcr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DataMcr")
                    .field("dtr", &self.dtr())
                    .field("rts", &self.rts())
                    .field("out1", &self.out1())
                    .field("out2", &self.out2())
                    .field("loopback", &self.loopback())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DataMcr {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "DataMcr {{ dtr: {=bool:?}, rts: {=bool:?}, out1: {=bool:?}, out2: {=bool:?}, loopback: {=bool:?} }}" , self . dtr () , self . rts () , self . out1 () , self . out2 () , self . loopback ())
            }
        }
        #[doc = "UART Modem Status Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DataMsr(pub u8);
        impl DataMsr {
            #[doc = "CTS Delta Clear To Send (DCTS)."]
            #[inline(always)]
            pub const fn cts(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "CTS Delta Clear To Send (DCTS)."]
            #[inline(always)]
            pub fn set_cts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
            #[doc = "DSR Delta Data Set Ready (DDSR)."]
            #[inline(always)]
            pub const fn dsr(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "DSR Delta Data Set Ready (DDSR)."]
            #[inline(always)]
            pub fn set_dsr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
            }
            #[doc = "RI Trailing Edge of Ring Indicator (TERI)."]
            #[inline(always)]
            pub const fn ri(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "RI Trailing Edge of Ring Indicator (TERI)."]
            #[inline(always)]
            pub fn set_ri(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
            }
            #[doc = "DCD Delta Data Carrier Detect (DDCD)."]
            #[inline(always)]
            pub const fn dcd(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "DCD Delta Data Carrier Detect (DDCD)."]
            #[inline(always)]
            pub fn set_dcd(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
            }
            #[doc = "nCTS This bit is the complement of the Clear To Send (nCTS) input."]
            #[inline(always)]
            pub const fn n_cts(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "nCTS This bit is the complement of the Clear To Send (nCTS) input."]
            #[inline(always)]
            pub fn set_n_cts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
            }
            #[doc = "This bit is the complement of the Data Set Ready (nDSR) input."]
            #[inline(always)]
            pub const fn n_dsr(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "This bit is the complement of the Data Set Ready (nDSR) input."]
            #[inline(always)]
            pub fn set_n_dsr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
            }
            #[doc = "nRI This bit is the complement of the Ring Indicator (nRI) input."]
            #[inline(always)]
            pub const fn n_ri(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "nRI This bit is the complement of the Ring Indicator (nRI) input."]
            #[inline(always)]
            pub fn set_n_ri(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
            }
            #[doc = "nDCD This bit is the complement of the Data Carrier Detect (nDCD) input."]
            #[inline(always)]
            pub const fn n_dcd(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "nDCD This bit is the complement of the Data Carrier Detect (nDCD) input."]
            #[inline(always)]
            pub fn set_n_dcd(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
            }
        }
        impl Default for DataMsr {
            #[inline(always)]
            fn default() -> DataMsr {
                DataMsr(0)
            }
        }
        impl core::fmt::Debug for DataMsr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DataMsr")
                    .field("cts", &self.cts())
                    .field("dsr", &self.dsr())
                    .field("ri", &self.ri())
                    .field("dcd", &self.dcd())
                    .field("n_cts", &self.n_cts())
                    .field("n_dsr", &self.n_dsr())
                    .field("n_ri", &self.n_ri())
                    .field("n_dcd", &self.n_dcd())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DataMsr {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "DataMsr {{ cts: {=bool:?}, dsr: {=bool:?}, ri: {=bool:?}, dcd: {=bool:?}, n_cts: {=bool:?}, n_dsr: {=bool:?}, n_ri: {=bool:?}, n_dcd: {=bool:?} }}" , self . cts () , self . dsr () , self . ri () , self . dcd () , self . n_cts () , self . n_dsr () , self . n_ri () , self . n_dcd ())
            }
        }
        #[doc = "UART Config Select Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DlabCfgSel(pub u8);
        impl DlabCfgSel {
            #[doc = "CLK_SRC 1=The UART Baud Clock is derived from an external clock source, 0=The UART Baud Clock is derived from one of the two internal clock sources"]
            #[inline(always)]
            pub const fn clk_src(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "CLK_SRC 1=The UART Baud Clock is derived from an external clock source, 0=The UART Baud Clock is derived from one of the two internal clock sources"]
            #[inline(always)]
            pub fn set_clk_src(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
            #[doc = "POWER 1=The RESET reset signal is derived from nSIO_RESET, 0=The RESET reset signal is derived from VCC1_RESET"]
            #[inline(always)]
            pub const fn pwr(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "POWER 1=The RESET reset signal is derived from nSIO_RESET, 0=The RESET reset signal is derived from VCC1_RESET"]
            #[inline(always)]
            pub fn set_pwr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
            }
            #[doc = "POLARITY 1=The UART_TX and UART_RX pins functions are inverted, 0=The UART_TX and UART_RX pins functions are not inverted"]
            #[inline(always)]
            pub const fn polar(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "POLARITY 1=The UART_TX and UART_RX pins functions are inverted, 0=The UART_TX and UART_RX pins functions are not inverted"]
            #[inline(always)]
            pub fn set_polar(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
            }
        }
        impl Default for DlabCfgSel {
            #[inline(always)]
            fn default() -> DlabCfgSel {
                DlabCfgSel(0)
            }
        }
        impl core::fmt::Debug for DlabCfgSel {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DlabCfgSel")
                    .field("clk_src", &self.clk_src())
                    .field("pwr", &self.pwr())
                    .field("polar", &self.polar())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DlabCfgSel {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "DlabCfgSel {{ clk_src: {=bool:?}, pwr: {=bool:?}, polar: {=bool:?} }}",
                    self.clk_src(),
                    self.pwr(),
                    self.polar()
                )
            }
        }
        #[doc = "UART FIFO Control Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DlabFifoCr(pub u8);
        impl DlabFifoCr {
            #[doc = "EXRF Enable XMIT and RECV FIFO."]
            #[inline(always)]
            pub const fn exrf(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "EXRF Enable XMIT and RECV FIFO."]
            #[inline(always)]
            pub fn set_exrf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
            #[doc = "CLEAR_RECV_FIFO Setting this bit to a logic '1' clears all bytes in the RCVR FIFO and resets its counter logic to '0'."]
            #[inline(always)]
            pub const fn clr_recv_fifo(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "CLEAR_RECV_FIFO Setting this bit to a logic '1' clears all bytes in the RCVR FIFO and resets its counter logic to '0'."]
            #[inline(always)]
            pub fn set_clr_recv_fifo(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
            }
            #[doc = "CLEAR_XMIT_FIFO Setting this bit to a logic '1' clears all bytes in the XMIT FIFO and resets its counter logic to '0' . The shift register is not cleared. This bit is self-clearing."]
            #[inline(always)]
            pub const fn clr_xmit_fifo(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "CLEAR_XMIT_FIFO Setting this bit to a logic '1' clears all bytes in the XMIT FIFO and resets its counter logic to '0' . The shift register is not cleared. This bit is self-clearing."]
            #[inline(always)]
            pub fn set_clr_xmit_fifo(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
            }
            #[doc = "DMA_MODE_SELECT Writing to this bit has no effect on the operation of the UART. The RXRDY and TXRDY pins are not available on this chip."]
            #[inline(always)]
            pub const fn dma_mode_sel(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "DMA_MODE_SELECT Writing to this bit has no effect on the operation of the UART. The RXRDY and TXRDY pins are not available on this chip."]
            #[inline(always)]
            pub fn set_dma_mode_sel(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
            }
            #[doc = "RECV_FIFO_TRIGGER_LEVEL These bits are used to set the trigger level for the RCVR FIFO interrupt."]
            #[inline(always)]
            pub const fn recv_fifo_trig_lvl(&self) -> u8 {
                let val = (self.0 >> 6usize) & 0x03;
                val as u8
            }
            #[doc = "RECV_FIFO_TRIGGER_LEVEL These bits are used to set the trigger level for the RCVR FIFO interrupt."]
            #[inline(always)]
            pub fn set_recv_fifo_trig_lvl(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
            }
        }
        impl Default for DlabFifoCr {
            #[inline(always)]
            fn default() -> DlabFifoCr {
                DlabFifoCr(0)
            }
        }
        impl core::fmt::Debug for DlabFifoCr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DlabFifoCr")
                    .field("exrf", &self.exrf())
                    .field("clr_recv_fifo", &self.clr_recv_fifo())
                    .field("clr_xmit_fifo", &self.clr_xmit_fifo())
                    .field("dma_mode_sel", &self.dma_mode_sel())
                    .field("recv_fifo_trig_lvl", &self.recv_fifo_trig_lvl())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DlabFifoCr {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "DlabFifoCr {{ exrf: {=bool:?}, clr_recv_fifo: {=bool:?}, clr_xmit_fifo: {=bool:?}, dma_mode_sel: {=bool:?}, recv_fifo_trig_lvl: {=u8:?} }}" , self . exrf () , self . clr_recv_fifo () , self . clr_xmit_fifo () , self . dma_mode_sel () , self . recv_fifo_trig_lvl ())
            }
        }
        #[doc = "UART Interrupt Identification Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DlabIntId(pub u8);
        impl DlabIntId {
            #[doc = "IPEND This bit can be used in either a hardwired prioritized or polled environment to indicate whether an interrupt is pending."]
            #[inline(always)]
            pub const fn ipend(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "IPEND This bit can be used in either a hardwired prioritized or polled environment to indicate whether an interrupt is pending."]
            #[inline(always)]
            pub fn set_ipend(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
            #[doc = "INTID These bits identify the highest priority interrupt pending"]
            #[inline(always)]
            pub const fn intid(&self) -> u8 {
                let val = (self.0 >> 1usize) & 0x07;
                val as u8
            }
            #[doc = "INTID These bits identify the highest priority interrupt pending"]
            #[inline(always)]
            pub fn set_intid(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u8) & 0x07) << 1usize);
            }
            #[doc = "These two bits are set when the FIFO CONTROL Register bit 0 equals 1."]
            #[inline(always)]
            pub const fn fifo_en(&self) -> u8 {
                let val = (self.0 >> 6usize) & 0x03;
                val as u8
            }
            #[doc = "These two bits are set when the FIFO CONTROL Register bit 0 equals 1."]
            #[inline(always)]
            pub fn set_fifo_en(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
            }
        }
        impl Default for DlabIntId {
            #[inline(always)]
            fn default() -> DlabIntId {
                DlabIntId(0)
            }
        }
        impl core::fmt::Debug for DlabIntId {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DlabIntId")
                    .field("ipend", &self.ipend())
                    .field("intid", &self.intid())
                    .field("fifo_en", &self.fifo_en())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DlabIntId {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "DlabIntId {{ ipend: {=bool:?}, intid: {=u8:?}, fifo_en: {=u8:?} }}",
                    self.ipend(),
                    self.intid(),
                    self.fifo_en()
                )
            }
        }
        #[doc = "UART Line Control Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DlabLcr(pub u8);
        impl DlabLcr {
            #[doc = "WORD_LENGTH These two bits specify the number of bits in each transmitted or received serial character."]
            #[inline(always)]
            pub const fn word_len(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x03;
                val as u8
            }
            #[doc = "WORD_LENGTH These two bits specify the number of bits in each transmitted or received serial character."]
            #[inline(always)]
            pub fn set_word_len(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
            }
            #[doc = "STOP_BITS This bit specifies the number of stop bits in each transmitted or received serial character."]
            #[inline(always)]
            pub const fn stop_bits(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "STOP_BITS This bit specifies the number of stop bits in each transmitted or received serial character."]
            #[inline(always)]
            pub fn set_stop_bits(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
            }
            #[doc = "ENABLE_PARITY Parity Enable bit."]
            #[inline(always)]
            pub const fn en_par(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "ENABLE_PARITY Parity Enable bit."]
            #[inline(always)]
            pub fn set_en_par(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
            }
            #[doc = "PARITY_SELECT Even Parity Select bit."]
            #[inline(always)]
            pub const fn par_sel(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "PARITY_SELECT Even Parity Select bit."]
            #[inline(always)]
            pub fn set_par_sel(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
            }
            #[doc = "STICK_PARITY Stick Parity bit."]
            #[inline(always)]
            pub const fn stick_par(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "STICK_PARITY Stick Parity bit."]
            #[inline(always)]
            pub fn set_stick_par(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
            }
            #[doc = "BREAK_CONTROL Set Break Control bit"]
            #[inline(always)]
            pub const fn brk_ctrl(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "BREAK_CONTROL Set Break Control bit"]
            #[inline(always)]
            pub fn set_brk_ctrl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
            }
            #[doc = "DLAB Divisor Latch Access Bit (DLAB)."]
            #[inline(always)]
            pub const fn dlab(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "DLAB Divisor Latch Access Bit (DLAB)."]
            #[inline(always)]
            pub fn set_dlab(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
            }
        }
        impl Default for DlabLcr {
            #[inline(always)]
            fn default() -> DlabLcr {
                DlabLcr(0)
            }
        }
        impl core::fmt::Debug for DlabLcr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DlabLcr")
                    .field("word_len", &self.word_len())
                    .field("stop_bits", &self.stop_bits())
                    .field("en_par", &self.en_par())
                    .field("par_sel", &self.par_sel())
                    .field("stick_par", &self.stick_par())
                    .field("brk_ctrl", &self.brk_ctrl())
                    .field("dlab", &self.dlab())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DlabLcr {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "DlabLcr {{ word_len: {=u8:?}, stop_bits: {=bool:?}, en_par: {=bool:?}, par_sel: {=bool:?}, stick_par: {=bool:?}, brk_ctrl: {=bool:?}, dlab: {=bool:?} }}" , self . word_len () , self . stop_bits () , self . en_par () , self . par_sel () , self . stick_par () , self . brk_ctrl () , self . dlab ())
            }
        }
        #[doc = "UART Line Status Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DlabLsr(pub u8);
        impl DlabLsr {
            #[doc = "DATA_READY Data Ready. It is set to a logic '1' whenever a complete incoming character has been received and transferred into the Receiver Buffer Register or the FIFO"]
            #[inline(always)]
            pub const fn data_ready(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "DATA_READY Data Ready. It is set to a logic '1' whenever a complete incoming character has been received and transferred into the Receiver Buffer Register or the FIFO"]
            #[inline(always)]
            pub fn set_data_ready(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
            #[doc = "OVERRUN Overrun Error."]
            #[inline(always)]
            pub const fn overrun(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "OVERRUN Overrun Error."]
            #[inline(always)]
            pub fn set_overrun(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
            }
            #[doc = "PARITY ERROR Parity Error."]
            #[inline(always)]
            pub const fn pe(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "PARITY ERROR Parity Error."]
            #[inline(always)]
            pub fn set_pe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
            }
            #[doc = "FRAME_ERROR Framing Error."]
            #[inline(always)]
            pub const fn frame_err(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "FRAME_ERROR Framing Error."]
            #[inline(always)]
            pub fn set_frame_err(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
            }
            #[doc = "BREAK_INTERRUPT Break Interrupt."]
            #[inline(always)]
            pub const fn brk_intr(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "BREAK_INTERRUPT Break Interrupt."]
            #[inline(always)]
            pub fn set_brk_intr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
            }
            #[doc = "TRANSMIT_EMPTY Transmitter Holding Register Empty Bit 5 indicates that the Serial Port is ready to accept a new character for transmission."]
            #[inline(always)]
            pub const fn trans_empty(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "TRANSMIT_EMPTY Transmitter Holding Register Empty Bit 5 indicates that the Serial Port is ready to accept a new character for transmission."]
            #[inline(always)]
            pub fn set_trans_empty(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
            }
            #[doc = "Transmitter Empty. Bit 6 is set to a logic '1' whenever the Transmitter Holding Register (THR) and Transmitter Shift Register (TSR) are both empty."]
            #[inline(always)]
            pub const fn trans_err(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Transmitter Empty. Bit 6 is set to a logic '1' whenever the Transmitter Holding Register (THR) and Transmitter Shift Register (TSR) are both empty."]
            #[inline(always)]
            pub fn set_trans_err(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
            }
            #[doc = "FIFO_ERROR"]
            #[inline(always)]
            pub const fn fifo_err(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "FIFO_ERROR"]
            #[inline(always)]
            pub fn set_fifo_err(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
            }
        }
        impl Default for DlabLsr {
            #[inline(always)]
            fn default() -> DlabLsr {
                DlabLsr(0)
            }
        }
        impl core::fmt::Debug for DlabLsr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DlabLsr")
                    .field("data_ready", &self.data_ready())
                    .field("overrun", &self.overrun())
                    .field("pe", &self.pe())
                    .field("frame_err", &self.frame_err())
                    .field("brk_intr", &self.brk_intr())
                    .field("trans_empty", &self.trans_empty())
                    .field("trans_err", &self.trans_err())
                    .field("fifo_err", &self.fifo_err())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DlabLsr {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "DlabLsr {{ data_ready: {=bool:?}, overrun: {=bool:?}, pe: {=bool:?}, frame_err: {=bool:?}, brk_intr: {=bool:?}, trans_empty: {=bool:?}, trans_err: {=bool:?}, fifo_err: {=bool:?} }}" , self . data_ready () , self . overrun () , self . pe () , self . frame_err () , self . brk_intr () , self . trans_empty () , self . trans_err () , self . fifo_err ())
            }
        }
        #[doc = "UART Modem Control Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DlabMcr(pub u8);
        impl DlabMcr {
            #[doc = "DTR This bit controls the Data Terminal Ready (nDTR) output."]
            #[inline(always)]
            pub const fn dtr(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "DTR This bit controls the Data Terminal Ready (nDTR) output."]
            #[inline(always)]
            pub fn set_dtr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
            #[doc = "RTS This bit controls the Request To Send (nRTS) output."]
            #[inline(always)]
            pub const fn rts(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "RTS This bit controls the Request To Send (nRTS) output."]
            #[inline(always)]
            pub fn set_rts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
            }
            #[doc = "OUT1 This bit controls the Output 1 (OUT1) bit."]
            #[inline(always)]
            pub const fn out1(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "OUT1 This bit controls the Output 1 (OUT1) bit."]
            #[inline(always)]
            pub fn set_out1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
            }
            #[doc = "OUT2 This bit is used to enable an UART interrupt."]
            #[inline(always)]
            pub const fn out2(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "OUT2 This bit is used to enable an UART interrupt."]
            #[inline(always)]
            pub fn set_out2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
            }
            #[doc = "LOOPBACK This bit provides the loopback feature for diagnostic testing of the Serial Port."]
            #[inline(always)]
            pub const fn loopback(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "LOOPBACK This bit provides the loopback feature for diagnostic testing of the Serial Port."]
            #[inline(always)]
            pub fn set_loopback(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
            }
        }
        impl Default for DlabMcr {
            #[inline(always)]
            fn default() -> DlabMcr {
                DlabMcr(0)
            }
        }
        impl core::fmt::Debug for DlabMcr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DlabMcr")
                    .field("dtr", &self.dtr())
                    .field("rts", &self.rts())
                    .field("out1", &self.out1())
                    .field("out2", &self.out2())
                    .field("loopback", &self.loopback())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DlabMcr {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "DlabMcr {{ dtr: {=bool:?}, rts: {=bool:?}, out1: {=bool:?}, out2: {=bool:?}, loopback: {=bool:?} }}" , self . dtr () , self . rts () , self . out1 () , self . out2 () , self . loopback ())
            }
        }
        #[doc = "UART Modem Status Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DlabMsr(pub u8);
        impl DlabMsr {
            #[doc = "CTS Delta Clear To Send (DCTS)."]
            #[inline(always)]
            pub const fn cts(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "CTS Delta Clear To Send (DCTS)."]
            #[inline(always)]
            pub fn set_cts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
            #[doc = "DSR Delta Data Set Ready (DDSR)."]
            #[inline(always)]
            pub const fn dsr(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "DSR Delta Data Set Ready (DDSR)."]
            #[inline(always)]
            pub fn set_dsr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
            }
            #[doc = "RI Trailing Edge of Ring Indicator (TERI)."]
            #[inline(always)]
            pub const fn ri(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "RI Trailing Edge of Ring Indicator (TERI)."]
            #[inline(always)]
            pub fn set_ri(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
            }
            #[doc = "DCD Delta Data Carrier Detect (DDCD)."]
            #[inline(always)]
            pub const fn dcd(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "DCD Delta Data Carrier Detect (DDCD)."]
            #[inline(always)]
            pub fn set_dcd(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
            }
            #[doc = "nCTS This bit is the complement of the Clear To Send (nCTS) input."]
            #[inline(always)]
            pub const fn n_cts(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "nCTS This bit is the complement of the Clear To Send (nCTS) input."]
            #[inline(always)]
            pub fn set_n_cts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
            }
            #[doc = "This bit is the complement of the Data Set Ready (nDSR) input."]
            #[inline(always)]
            pub const fn n_dsr(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "This bit is the complement of the Data Set Ready (nDSR) input."]
            #[inline(always)]
            pub fn set_n_dsr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
            }
            #[doc = "nRI This bit is the complement of the Ring Indicator (nRI) input."]
            #[inline(always)]
            pub const fn n_ri(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "nRI This bit is the complement of the Ring Indicator (nRI) input."]
            #[inline(always)]
            pub fn set_n_ri(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
            }
            #[doc = "nDCD This bit is the complement of the Data Carrier Detect (nDCD) input."]
            #[inline(always)]
            pub const fn n_dcd(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "nDCD This bit is the complement of the Data Carrier Detect (nDCD) input."]
            #[inline(always)]
            pub fn set_n_dcd(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
            }
        }
        impl Default for DlabMsr {
            #[inline(always)]
            fn default() -> DlabMsr {
                DlabMsr(0)
            }
        }
        impl core::fmt::Debug for DlabMsr {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("DlabMsr")
                    .field("cts", &self.cts())
                    .field("dsr", &self.dsr())
                    .field("ri", &self.ri())
                    .field("dcd", &self.dcd())
                    .field("n_cts", &self.n_cts())
                    .field("n_dsr", &self.n_dsr())
                    .field("n_ri", &self.n_ri())
                    .field("n_dcd", &self.n_dcd())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for DlabMsr {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "DlabMsr {{ cts: {=bool:?}, dsr: {=bool:?}, ri: {=bool:?}, dcd: {=bool:?}, n_cts: {=bool:?}, n_dsr: {=bool:?}, n_ri: {=bool:?}, n_dcd: {=bool:?} }}" , self . cts () , self . dsr () , self . ri () , self . dcd () , self . n_cts () , self . n_dsr () , self . n_ri () , self . n_dcd ())
            }
        }
        #[doc = "UART Interrupt Enable Register (DLAB=0)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ien(pub u8);
        impl Ien {
            #[doc = "ERDAI This bit enables the Received Data Available Interrupt (and timeout interrupts in the FIFO mode) when set to logic '1'."]
            #[inline(always)]
            pub const fn erdai(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "ERDAI This bit enables the Received Data Available Interrupt (and timeout interrupts in the FIFO mode) when set to logic '1'."]
            #[inline(always)]
            pub fn set_erdai(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
            #[doc = "ETHREI This bit enables the Transmitter Holding Register Empty Interrupt when set to logic '1'."]
            #[inline(always)]
            pub const fn ethrei(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "ETHREI This bit enables the Transmitter Holding Register Empty Interrupt when set to logic '1'."]
            #[inline(always)]
            pub fn set_ethrei(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
            }
            #[doc = "ELSI This bit enables the Received Line Status Interrupt when set to logic '1'."]
            #[inline(always)]
            pub const fn elsi(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "ELSI This bit enables the Received Line Status Interrupt when set to logic '1'."]
            #[inline(always)]
            pub fn set_elsi(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
            }
            #[doc = "EMSI This bit enables the MODEM Status Interrupt when set to logic '1'."]
            #[inline(always)]
            pub const fn emsi(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "EMSI This bit enables the MODEM Status Interrupt when set to logic '1'."]
            #[inline(always)]
            pub fn set_emsi(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
            }
        }
        impl Default for Ien {
            #[inline(always)]
            fn default() -> Ien {
                Ien(0)
            }
        }
        impl core::fmt::Debug for Ien {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ien")
                    .field("erdai", &self.erdai())
                    .field("ethrei", &self.ethrei())
                    .field("elsi", &self.elsi())
                    .field("emsi", &self.emsi())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ien {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Ien {{ erdai: {=bool:?}, ethrei: {=bool:?}, elsi: {=bool:?}, emsi: {=bool:?} }}",
                    self.erdai(),
                    self.ethrei(),
                    self.elsi(),
                    self.emsi()
                )
            }
        }
    }
}
pub mod vbat {
    #[doc = "The VBAT Register Bank block is a block implemented for miscellaneous battery-backed registers"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vbat {
        ptr: *mut u8,
    }
    unsafe impl Send for Vbat {}
    unsafe impl Sync for Vbat {}
    impl Vbat {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "The Power-Fail and Reset Status Register collects and retains the VBAT RST and WDT event status when VCC1 is unpowered."]
        #[inline(always)]
        pub const fn pfrs(self) -> crate::common::Reg<regs::Pfrs, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "CLOCK ENABLE"]
        #[inline(always)]
        pub const fn clk32_en(self) -> crate::common::Reg<regs::Clk32en, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "System Shutdown Enable register."]
        #[inline(always)]
        pub const fn sys_shdn(self) -> crate::common::Reg<regs::SysShdn, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "MONOTONIC COUNTER"]
        #[inline(always)]
        pub const fn mcnt_lo(self) -> crate::common::Reg<regs::McntLo, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }
        #[doc = "COUNTER HIWORD"]
        #[inline(always)]
        pub const fn mcnt_hi(self) -> crate::common::Reg<regs::McntHi, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
        }
        #[doc = "VWIRE_BACKUP"]
        #[inline(always)]
        pub const fn vwr_bckp(self) -> crate::common::Reg<regs::VwrBckp, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "CLOCK ENABLE"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Clk32en(pub u32);
        impl Clk32en {
            #[doc = "This bit selects the source for the 32KHz clock domain. 1=The 32KHZ_IN VTR-powered pin is used as a source for the 32KHz clock domain. If an activity detector does not detect a clock on the selected source, the always-on 32KHz internal clock source is automatically selected 0=The always-on32Khz clock source is used as the source for the 32KHz clock domain."]
            #[inline(always)]
            pub const fn ext_32k(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "This bit selects the source for the 32KHz clock domain. 1=The 32KHZ_IN VTR-powered pin is used as a source for the 32KHz clock domain. If an activity detector does not detect a clock on the selected source, the always-on 32KHz internal clock source is automatically selected 0=The always-on32Khz clock source is used as the source for the 32KHz clock domain."]
            #[inline(always)]
            pub fn set_ext_32k(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
        }
        impl Default for Clk32en {
            #[inline(always)]
            fn default() -> Clk32en {
                Clk32en(0)
            }
        }
        impl core::fmt::Debug for Clk32en {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Clk32en").field("ext_32k", &self.ext_32k()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Clk32en {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Clk32en {{ ext_32k: {=bool:?} }}", self.ext_32k())
            }
        }
        #[doc = "COUNTER HIWORD"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct McntHi(pub u32);
        impl McntHi {
            #[doc = "Thirty-two bit read/write register. If software sets this register to an incrementing value, based on an external non-volatile store, this register may be combined with the Monotonic Counter Register to form a 64-bit monotonic counter."]
            #[inline(always)]
            pub const fn cntr(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "Thirty-two bit read/write register. If software sets this register to an incrementing value, based on an external non-volatile store, this register may be combined with the Monotonic Counter Register to form a 64-bit monotonic counter."]
            #[inline(always)]
            pub fn set_cntr(&mut self, val: u32) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for McntHi {
            #[inline(always)]
            fn default() -> McntHi {
                McntHi(0)
            }
        }
        impl core::fmt::Debug for McntHi {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("McntHi").field("cntr", &self.cntr()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for McntHi {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "McntHi {{ cntr: {=u32:?} }}", self.cntr())
            }
        }
        #[doc = "MONOTONIC COUNTER"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct McntLo(pub u32);
        impl McntLo {
            #[doc = "Read-only register that increments by 1 every time it is read. It is reset to 0 on a VBAT Power On Reset."]
            #[inline(always)]
            pub const fn cntr(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0xffff_ffff;
                val as u32
            }
            #[doc = "Read-only register that increments by 1 every time it is read. It is reset to 0 on a VBAT Power On Reset."]
            #[inline(always)]
            pub fn set_cntr(&mut self, val: u32) {
                self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
            }
        }
        impl Default for McntLo {
            #[inline(always)]
            fn default() -> McntLo {
                McntLo(0)
            }
        }
        impl core::fmt::Debug for McntLo {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("McntLo").field("cntr", &self.cntr()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for McntLo {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "McntLo {{ cntr: {=u32:?} }}", self.cntr())
            }
        }
        #[doc = "The Power-Fail and Reset Status Register collects and retains the VBAT RST and WDT event status when VCC1 is unpowered."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pfrs(pub u8);
        impl Pfrs {
            #[doc = "This bit is set to '1b' if a was triggered by an assertion of the SOFT_SYS_RESET bit in the System Reset Register. This bit is cleared to '0b' when written with a '1b'; writes of a '0b' have no effect. (R/WC)"]
            #[inline(always)]
            pub const fn soft(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "This bit is set to '1b' if a was triggered by an assertion of the SOFT_SYS_RESET bit in the System Reset Register. This bit is cleared to '0b' when written with a '1b'; writes of a '0b' have no effect. (R/WC)"]
            #[inline(always)]
            pub fn set_soft(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
            }
            #[doc = "Test"]
            #[inline(always)]
            pub const fn test(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Test"]
            #[inline(always)]
            pub fn set_test(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
            }
            #[doc = "This bit is set to '1b' if a RESET_SYS was triggered by a low signal on the RESETI# input pin. This bit is cleared to '0b' when written with a '1b'; writes of a '0b' have no effect. (R/WC)"]
            #[inline(always)]
            pub const fn rsti(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "This bit is set to '1b' if a RESET_SYS was triggered by a low signal on the RESETI# input pin. This bit is cleared to '0b' when written with a '1b'; writes of a '0b' have no effect. (R/WC)"]
            #[inline(always)]
            pub fn set_rsti(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
            }
            #[doc = "This bit is set to '1b' if a RESET_SYS was triggered by a Watchdog Timer event. This bit is cleared to '0b' when written with a '1b'; writes of a '0b' have no effect. (R/WC)"]
            #[inline(always)]
            pub const fn wdt_evt(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "This bit is set to '1b' if a RESET_SYS was triggered by a Watchdog Timer event. This bit is cleared to '0b' when written with a '1b'; writes of a '0b' have no effect. (R/WC)"]
            #[inline(always)]
            pub fn set_wdt_evt(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
            }
            #[doc = "This bit is set to '1b' if a RESET_SYS was triggered by an ARM SYSRESETREQ event. This bit is cleared to '0b' when written with a '1b'; writes of a '0b' have no effect. (R/WC)"]
            #[inline(always)]
            pub const fn sys_rstreq(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "This bit is set to '1b' if a RESET_SYS was triggered by an ARM SYSRESETREQ event. This bit is cleared to '0b' when written with a '1b'; writes of a '0b' have no effect. (R/WC)"]
            #[inline(always)]
            pub fn set_sys_rstreq(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
            }
            #[doc = "The VBAT RST bit is set to '1' by hardware when a RESET_VBAT is detected. This is the register default value. To clear VBAT RST EC firmware must write a '1' to this bit; writing a '0' to VBAT RST has no affect.(R/WC)"]
            #[inline(always)]
            pub const fn vbat_rst(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "The VBAT RST bit is set to '1' by hardware when a RESET_VBAT is detected. This is the register default value. To clear VBAT RST EC firmware must write a '1' to this bit; writing a '0' to VBAT RST has no affect.(R/WC)"]
            #[inline(always)]
            pub fn set_vbat_rst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
            }
        }
        impl Default for Pfrs {
            #[inline(always)]
            fn default() -> Pfrs {
                Pfrs(0)
            }
        }
        impl core::fmt::Debug for Pfrs {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Pfrs")
                    .field("soft", &self.soft())
                    .field("test", &self.test())
                    .field("rsti", &self.rsti())
                    .field("wdt_evt", &self.wdt_evt())
                    .field("sys_rstreq", &self.sys_rstreq())
                    .field("vbat_rst", &self.vbat_rst())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Pfrs {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Pfrs {{ soft: {=bool:?}, test: {=bool:?}, rsti: {=bool:?}, wdt_evt: {=bool:?}, sys_rstreq: {=bool:?}, vbat_rst: {=bool:?} }}" , self . soft () , self . test () , self . rsti () , self . wdt_evt () , self . sys_rstreq () , self . vbat_rst ())
            }
        }
        #[doc = "System Shutdown Enable register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SysShdn(pub u32);
        impl SysShdn {
            #[doc = "This bit controls the System Shutdown. 0 = Enable System Shutdown (SYS_SHDN#). 1 = Disable System Shutdown (SYS_SHDN#)."]
            #[inline(always)]
            pub const fn dis(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This bit controls the System Shutdown. 0 = Enable System Shutdown (SYS_SHDN#). 1 = Disable System Shutdown (SYS_SHDN#)."]
            #[inline(always)]
            pub fn set_dis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for SysShdn {
            #[inline(always)]
            fn default() -> SysShdn {
                SysShdn(0)
            }
        }
        impl core::fmt::Debug for SysShdn {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("SysShdn").field("dis", &self.dis()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for SysShdn {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "SysShdn {{ dis: {=bool:?} }}", self.dis())
            }
        }
        #[doc = "VWIRE_BACKUP"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct VwrBckp(pub u32);
        impl VwrBckp {
            #[doc = "The Boot ROM firmware will copy this field into the SRC3 to SRC0 bits of the Master-to-Slave Virtual Wire Register that corresponds to Virtual Wire Index 2h on a RESET_SYS. If software always saves the state of the Index 2h SRC bits on the falling edge of the SUSWARN# virtual wire, the state of the four SRC bits will be synchronized to the state of the four bits in the core logic."]
            #[inline(always)]
            pub const fn m2s_2h_bckup(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "The Boot ROM firmware will copy this field into the SRC3 to SRC0 bits of the Master-to-Slave Virtual Wire Register that corresponds to Virtual Wire Index 2h on a RESET_SYS. If software always saves the state of the Index 2h SRC bits on the falling edge of the SUSWARN# virtual wire, the state of the four SRC bits will be synchronized to the state of the four bits in the core logic."]
            #[inline(always)]
            pub fn set_m2s_2h_bckup(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
            #[doc = "The Boot ROM firmware will copy this field into the SRC3 to SRC0 bits of the Master-to-Slave Virtual Wire Register that corresponds to Virtual Wire Index 42h on a RESET_SYS. If software always saves the state of the Index 2h SRC bits on the falling edge of the SUSWARN# virtual wire, the state of the four SRC bits will be synchronized to the state of the four bits in the core logic."]
            #[inline(always)]
            pub const fn m2s_42h_bckup(&self) -> u8 {
                let val = (self.0 >> 4usize) & 0x0f;
                val as u8
            }
            #[doc = "The Boot ROM firmware will copy this field into the SRC3 to SRC0 bits of the Master-to-Slave Virtual Wire Register that corresponds to Virtual Wire Index 42h on a RESET_SYS. If software always saves the state of the Index 2h SRC bits on the falling edge of the SUSWARN# virtual wire, the state of the four SRC bits will be synchronized to the state of the four bits in the core logic."]
            #[inline(always)]
            pub fn set_m2s_42h_bckup(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
            }
        }
        impl Default for VwrBckp {
            #[inline(always)]
            fn default() -> VwrBckp {
                VwrBckp(0)
            }
        }
        impl core::fmt::Debug for VwrBckp {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("VwrBckp")
                    .field("m2s_2h_bckup", &self.m2s_2h_bckup())
                    .field("m2s_42h_bckup", &self.m2s_42h_bckup())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for VwrBckp {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "VwrBckp {{ m2s_2h_bckup: {=u8:?}, m2s_42h_bckup: {=u8:?} }}",
                    self.m2s_2h_bckup(),
                    self.m2s_42h_bckup()
                )
            }
        }
    }
}
pub mod vbat_ram {
    #[doc = "The VBAT RAM is operational while the main power rail is operational, and will retain its values powered by battery power while the main rail is unpowered."]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VbatRam {
        ptr: *mut u8,
    }
    unsafe impl Send for VbatRam {}
    unsafe impl Sync for VbatRam {}
    impl VbatRam {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "32-bits of VBAT powered RAM."]
        #[inline(always)]
        pub const fn mem(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
            assert!(n < 16usize);
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
        }
    }
}
pub mod vci {
    #[doc = "The VBAT-Powered Control Interfaces with the RTC With Date and DST Adjustment as well as the Week Alarm."]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vci {
        ptr: *mut u8,
    }
    unsafe impl Send for Vci {}
    unsafe impl Sync for Vci {}
    impl Vci {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "VCI Register"]
        #[inline(always)]
        pub const fn ctrl_sts(self) -> crate::common::Reg<regs::CtrlSts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Latch Enable Register"]
        #[inline(always)]
        pub const fn latch_en(self) -> crate::common::Reg<regs::LatchEn, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Latch Resets Register"]
        #[inline(always)]
        pub const fn latch_rst(self) -> crate::common::Reg<regs::LatchRst, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "VCI Input Enable Register"]
        #[inline(always)]
        pub const fn input_en(self) -> crate::common::Reg<regs::InputEn, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Holdoff Count Register"]
        #[inline(always)]
        pub const fn hldoff_cnt(self) -> crate::common::Reg<regs::HldoffCnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "VCI Polarity Register"]
        #[inline(always)]
        pub const fn polarity(self) -> crate::common::Reg<regs::Polarity, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "VCI Posedge Detect Register"]
        #[inline(always)]
        pub const fn pedge_det(self) -> crate::common::Reg<regs::PedgeDet, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
        }
        #[doc = "VCI Negedge Detect Register"]
        #[inline(always)]
        pub const fn nedge_det(self) -> crate::common::Reg<regs::NedgeDet, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
        #[doc = "VCI Buffer Enable Register"]
        #[inline(always)]
        pub const fn buffer_en(self) -> crate::common::Reg<regs::BufferEn, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "VCI Buffer Enable Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct BufferEn(pub u32);
        impl BufferEn {
            #[doc = "Input Buffer enable. After changing the buffer enable for a VCI input, firmware should reset the input latch and clear any potential interrupt that may have been triggered by the input, as changing the buffer may cause the internal status to change. This register has no effect when VTR is powered. When VTR is on, the input buffers are enabled only by the IE bit. For each bit in the field: 1=VCI_IN# input buffer enabled independent of the IE bit. The edge detection latches for this input are always enabled 0=VCI_IN# input buffer enabled by the IE bit. The edge detection latches are only enabled when the IE bit is 1 (default)."]
            #[inline(always)]
            pub const fn v_buf(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "Input Buffer enable. After changing the buffer enable for a VCI input, firmware should reset the input latch and clear any potential interrupt that may have been triggered by the input, as changing the buffer may cause the internal status to change. This register has no effect when VTR is powered. When VTR is on, the input buffers are enabled only by the IE bit. For each bit in the field: 1=VCI_IN# input buffer enabled independent of the IE bit. The edge detection latches for this input are always enabled 0=VCI_IN# input buffer enabled by the IE bit. The edge detection latches are only enabled when the IE bit is 1 (default)."]
            #[inline(always)]
            pub fn set_v_buf(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for BufferEn {
            #[inline(always)]
            fn default() -> BufferEn {
                BufferEn(0)
            }
        }
        impl core::fmt::Debug for BufferEn {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("BufferEn").field("v_buf", &self.v_buf()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for BufferEn {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "BufferEn {{ v_buf: {=u8:?} }}", self.v_buf())
            }
        }
        #[doc = "VCI Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct CtrlSts(pub u32);
        impl CtrlSts {
            #[doc = "These bits provide the latched state of the associated VCI_IN# pin, if latching is enabled or the current state of the pin if latching is not enabled. In both cases, the value is determined after the action of the VCI Polarity Register. Note: The VCI_IN\\[6:0\\]# bits default to the state of their respective input pins."]
            #[inline(always)]
            pub const fn vci_in(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "These bits provide the latched state of the associated VCI_IN# pin, if latching is enabled or the current state of the pin if latching is not enabled. In both cases, the value is determined after the action of the VCI Polarity Register. Note: The VCI_IN\\[6:0\\]# bits default to the state of their respective input pins."]
            #[inline(always)]
            pub fn set_vci_in(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
            #[doc = "This bit selects the power source for GPIO outputs. 1= GPIO will be powered by VBAT power well (VCI_OUT functionality). 0= GPIO will be powered by VTR power well."]
            #[inline(always)]
            pub const fn vci_out_gpio_sel(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "This bit selects the power source for GPIO outputs. 1= GPIO will be powered by VBAT power well (VCI_OUT functionality). 0= GPIO will be powered by VTR power well."]
            #[inline(always)]
            pub fn set_vci_out_gpio_sel(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "This bit provides the current status of the VCI_OVRD_IN pin. Note: The VCI_OVRD_IN bit defaults to the state of the respective input pin."]
            #[inline(always)]
            pub const fn vci_ovrd_in(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "This bit provides the current status of the VCI_OVRD_IN pin. Note: The VCI_OVRD_IN bit defaults to the state of the respective input pin."]
            #[inline(always)]
            pub fn set_vci_ovrd_in(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "This bit provides the current status of the VCI_OUT pin."]
            #[inline(always)]
            pub const fn vci_out(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "This bit provides the current status of the VCI_OUT pin."]
            #[inline(always)]
            pub fn set_vci_out(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "This bit can allow EC firmware to control the state of the VCI_OUT pin. For example, when VTR_PWRGD is asserted and the FW_EXT bit is 1, clearing the VCI_FW_CNTRL bit de-asserts the active high VCI_OUT pin. BIOS must set this bit to 1 prior to setting the FW_EXT bit to 1 on power up, in order to avoid glitches on the VCI_OUT pin."]
            #[inline(always)]
            pub const fn vci_fw_ctrl(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "This bit can allow EC firmware to control the state of the VCI_OUT pin. For example, when VTR_PWRGD is asserted and the FW_EXT bit is 1, clearing the VCI_FW_CNTRL bit de-asserts the active high VCI_OUT pin. BIOS must set this bit to 1 prior to setting the FW_EXT bit to 1 on power up, in order to avoid glitches on the VCI_OUT pin."]
            #[inline(always)]
            pub fn set_vci_fw_ctrl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "This bit controls selecting between the external VBAT-Powered Control Interface inputs, or the VCI_FW_CNTRL bit output to control the VCI_OUT pin. 1=VCI_OUT is determined by the VCI_FW_CNTRL field, when VTR is active 0=VCI_OUT is determined by the external inputs. Note: This bit used to be called GPO/nEXT. The name was changed to distinguish it from the BGPOs, which are elsewhere, and to remove a / in a bit name."]
            #[inline(always)]
            pub const fn fw_ext(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "This bit controls selecting between the external VBAT-Powered Control Interface inputs, or the VCI_FW_CNTRL bit output to control the VCI_OUT pin. 1=VCI_OUT is determined by the VCI_FW_CNTRL field, when VTR is active 0=VCI_OUT is determined by the external inputs. Note: This bit used to be called GPO/nEXT. The name was changed to distinguish it from the BGPOs, which are elsewhere, and to remove a / in a bit name."]
            #[inline(always)]
            pub fn set_fw_ext(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "The Filters Bypass bit is used to enable and disable the input filters on the VCI_IN# pins. 1=Filters disabled; 0=Filters enabled (default)."]
            #[inline(always)]
            pub const fn fltrs_bypass(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "The Filters Bypass bit is used to enable and disable the input filters on the VCI_IN# pins. 1=Filters disabled; 0=Filters enabled (default)."]
            #[inline(always)]
            pub fn set_fltrs_bypass(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "If enabled by WEEK_ALRM_LE, this bit is set to 1 if the Week Alarm signal is asserted. It is reset by writes to WEEK_ALRM_LS."]
            #[inline(always)]
            pub const fn wk_alrm(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "If enabled by WEEK_ALRM_LE, this bit is set to 1 if the Week Alarm signal is asserted. It is reset by writes to WEEK_ALRM_LS."]
            #[inline(always)]
            pub fn set_wk_alrm(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "If enabled by RTC_ALRM_LE, this bit is set to 1 if the RTC Alarm signal is asserted. It is reset by writes to RTC_ALRM_LS."]
            #[inline(always)]
            pub const fn rtc_alrm(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "If enabled by RTC_ALRM_LE, this bit is set to 1 if the RTC Alarm signal is asserted. It is reset by writes to RTC_ALRM_LS."]
            #[inline(always)]
            pub fn set_rtc_alrm(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "This is the System power present select bit. 1= VCI_IN3 is used as System power present. 0= VCI_IN3 is used as VCI_IN3."]
            #[inline(always)]
            pub const fn syspwr_pres(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "This is the System power present select bit. 1= VCI_IN3 is used as System power present. 0= VCI_IN3 is used as VCI_IN3."]
            #[inline(always)]
            pub fn set_syspwr_pres(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
        }
        impl Default for CtrlSts {
            #[inline(always)]
            fn default() -> CtrlSts {
                CtrlSts(0)
            }
        }
        impl core::fmt::Debug for CtrlSts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("CtrlSts")
                    .field("vci_in", &self.vci_in())
                    .field("vci_out_gpio_sel", &self.vci_out_gpio_sel())
                    .field("vci_ovrd_in", &self.vci_ovrd_in())
                    .field("vci_out", &self.vci_out())
                    .field("vci_fw_ctrl", &self.vci_fw_ctrl())
                    .field("fw_ext", &self.fw_ext())
                    .field("fltrs_bypass", &self.fltrs_bypass())
                    .field("wk_alrm", &self.wk_alrm())
                    .field("rtc_alrm", &self.rtc_alrm())
                    .field("syspwr_pres", &self.syspwr_pres())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for CtrlSts {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "CtrlSts {{ vci_in: {=u8:?}, vci_out_gpio_sel: {=bool:?}, vci_ovrd_in: {=bool:?}, vci_out: {=bool:?}, vci_fw_ctrl: {=bool:?}, fw_ext: {=bool:?}, fltrs_bypass: {=bool:?}, wk_alrm: {=bool:?}, rtc_alrm: {=bool:?}, syspwr_pres: {=bool:?} }}" , self . vci_in () , self . vci_out_gpio_sel () , self . vci_ovrd_in () , self . vci_out () , self . vci_fw_ctrl () , self . fw_ext () , self . fltrs_bypass () , self . wk_alrm () , self . rtc_alrm () , self . syspwr_pres ())
            }
        }
        #[doc = "Holdoff Count Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct HldoffCnt(pub u32);
        impl HldoffCnt {
            #[doc = "These bits determine the period of time the VCI_OUT logic is inhibited from re-asserting VCI_OUT after a SYS_SHDN# event. FFh-01h=The Power On Inhibit Holdoff Time is set to a period between 125ms and 31.875 seconds. 0=The Power On Inhibit function is disabled."]
            #[inline(always)]
            pub const fn time(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "These bits determine the period of time the VCI_OUT logic is inhibited from re-asserting VCI_OUT after a SYS_SHDN# event. FFh-01h=The Power On Inhibit Holdoff Time is set to a period between 125ms and 31.875 seconds. 0=The Power On Inhibit function is disabled."]
            #[inline(always)]
            pub fn set_time(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
        }
        impl Default for HldoffCnt {
            #[inline(always)]
            fn default() -> HldoffCnt {
                HldoffCnt(0)
            }
        }
        impl core::fmt::Debug for HldoffCnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("HldoffCnt").field("time", &self.time()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for HldoffCnt {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "HldoffCnt {{ time: {=u8:?} }}", self.time())
            }
        }
        #[doc = "VCI Input Enable Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct InputEn(pub u32);
        impl InputEn {
            #[doc = "Input Enables for VCI_IN# signals. After changing the input enable for a VCI input, firmware should reset the input latch and clear any potential interrupt that may have been triggered by the input, as changing the enable may cause the internal status to change. For each bit in the field: 1=Enabled. The corresponding VCI_IN# input is not gated and toggling the pin will affect the VCI_OUT pin 0=Not Enabled. the corresponding VCI_IN# input does not affect the VCI_OUT pin, even if the input is '0'. Unless the corresponding bit in the VCI Buffer Enable Register is 1, latches are not asserted, even if the VCI_IN# pin is low, during a VBAT power transition"]
            #[inline(always)]
            pub const fn ie(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "Input Enables for VCI_IN# signals. After changing the input enable for a VCI input, firmware should reset the input latch and clear any potential interrupt that may have been triggered by the input, as changing the enable may cause the internal status to change. For each bit in the field: 1=Enabled. The corresponding VCI_IN# input is not gated and toggling the pin will affect the VCI_OUT pin 0=Not Enabled. the corresponding VCI_IN# input does not affect the VCI_OUT pin, even if the input is '0'. Unless the corresponding bit in the VCI Buffer Enable Register is 1, latches are not asserted, even if the VCI_IN# pin is low, during a VBAT power transition"]
            #[inline(always)]
            pub fn set_ie(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for InputEn {
            #[inline(always)]
            fn default() -> InputEn {
                InputEn(0)
            }
        }
        impl core::fmt::Debug for InputEn {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("InputEn").field("ie", &self.ie()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for InputEn {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "InputEn {{ ie: {=u8:?} }}", self.ie())
            }
        }
        #[doc = "Latch Enable Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct LatchEn(pub u32);
        impl LatchEn {
            #[doc = "Latching Enables. Latching occurs after the Polarity configuration, so a VCI_INi# pin is asserted when it is '0' if VCI_IN_POL is '0', and asserted when it is '1' if VCI_IN_POL is '1'. For each bit in the field: 1=Enabled. Assertions of the VCI_INi# pin are held until the latch is reset by writing the corresponding LS bit 0=Not Enabled. The VCI_INi# signal is not latched but passed directly to the VCI_OUT logic"]
            #[inline(always)]
            pub const fn le(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "Latching Enables. Latching occurs after the Polarity configuration, so a VCI_INi# pin is asserted when it is '0' if VCI_IN_POL is '0', and asserted when it is '1' if VCI_IN_POL is '1'. For each bit in the field: 1=Enabled. Assertions of the VCI_INi# pin are held until the latch is reset by writing the corresponding LS bit 0=Not Enabled. The VCI_INi# signal is not latched but passed directly to the VCI_OUT logic"]
            #[inline(always)]
            pub fn set_le(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
            #[doc = "Latch enable for the Week Alarm Power-Up signal. 1=Enabled. Assertions of the Week Alarm are held until the latch is reset by writing the corresponding LS bit 0=Not Enabled. The Week Alarm signal is not latched but passed directly to the VCI_OUT logic"]
            #[inline(always)]
            pub const fn wk_alrm_le(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Latch enable for the Week Alarm Power-Up signal. 1=Enabled. Assertions of the Week Alarm are held until the latch is reset by writing the corresponding LS bit 0=Not Enabled. The Week Alarm signal is not latched but passed directly to the VCI_OUT logic"]
            #[inline(always)]
            pub fn set_wk_alrm_le(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "Latch enable for the RTC Power-Up signal. 1=Enabled. Assertions of the RTC Alarm are held until the latch is reset by writing the corresponding LS bit 0=Not Enabled. The RTC Alarm signal is not latched but passed directly to the VCI_OUT logic"]
            #[inline(always)]
            pub const fn rtc_alrm_le(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "Latch enable for the RTC Power-Up signal. 1=Enabled. Assertions of the RTC Alarm are held until the latch is reset by writing the corresponding LS bit 0=Not Enabled. The RTC Alarm signal is not latched but passed directly to the VCI_OUT logic"]
            #[inline(always)]
            pub fn set_rtc_alrm_le(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
        }
        impl Default for LatchEn {
            #[inline(always)]
            fn default() -> LatchEn {
                LatchEn(0)
            }
        }
        impl core::fmt::Debug for LatchEn {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("LatchEn")
                    .field("le", &self.le())
                    .field("wk_alrm_le", &self.wk_alrm_le())
                    .field("rtc_alrm_le", &self.rtc_alrm_le())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for LatchEn {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "LatchEn {{ le: {=u8:?}, wk_alrm_le: {=bool:?}, rtc_alrm_le: {=bool:?} }}",
                    self.le(),
                    self.wk_alrm_le(),
                    self.rtc_alrm_le()
                )
            }
        }
        #[doc = "Latch Resets Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct LatchRst(pub u32);
        impl LatchRst {
            #[doc = "Latch Resets. When a Latch Resets bit is written with a '1', the corresponding VCI_INi# latch is de-asserted ('1'). The VCI_INi# input to the latch has priority over the Latch Reset input, so firmware cannot reset the latch while the VCI_INi# pin is asserted. Firmware should sample the state of the pin in the VCI Register before attempting to reset the latch. As noted in the Latch Enable Register, the assertion level is determined by the VCI_IN_POL bit. Reads of this register are undefined."]
            #[inline(always)]
            pub const fn ls(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "Latch Resets. When a Latch Resets bit is written with a '1', the corresponding VCI_INi# latch is de-asserted ('1'). The VCI_INi# input to the latch has priority over the Latch Reset input, so firmware cannot reset the latch while the VCI_INi# pin is asserted. Firmware should sample the state of the pin in the VCI Register before attempting to reset the latch. As noted in the Latch Enable Register, the assertion level is determined by the VCI_IN_POL bit. Reads of this register are undefined."]
            #[inline(always)]
            pub fn set_ls(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
            #[doc = "Week Alarm Latch Reset. When this bit is written with a '1', the Week Alarm Event latch is reset. The Week Alarm input to the latch has priority over the Reset input Reads of this register are undefined."]
            #[inline(always)]
            pub const fn wk_alrm_ls(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "Week Alarm Latch Reset. When this bit is written with a '1', the Week Alarm Event latch is reset. The Week Alarm input to the latch has priority over the Reset input Reads of this register are undefined."]
            #[inline(always)]
            pub fn set_wk_alrm_ls(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "RTC Alarm Latch Reset. When this bit is written with a '1', the RTC Alarm Event latch is reset. The RTC Alarm input to the latch has priority over the Reset input Reads of this register are undefined."]
            #[inline(always)]
            pub const fn rtc_alrm_ls(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "RTC Alarm Latch Reset. When this bit is written with a '1', the RTC Alarm Event latch is reset. The RTC Alarm input to the latch has priority over the Reset input Reads of this register are undefined."]
            #[inline(always)]
            pub fn set_rtc_alrm_ls(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
        }
        impl Default for LatchRst {
            #[inline(always)]
            fn default() -> LatchRst {
                LatchRst(0)
            }
        }
        impl core::fmt::Debug for LatchRst {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("LatchRst")
                    .field("ls", &self.ls())
                    .field("wk_alrm_ls", &self.wk_alrm_ls())
                    .field("rtc_alrm_ls", &self.rtc_alrm_ls())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for LatchRst {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "LatchRst {{ ls: {=u8:?}, wk_alrm_ls: {=bool:?}, rtc_alrm_ls: {=bool:?} }}",
                    self.ls(),
                    self.wk_alrm_ls(),
                    self.rtc_alrm_ls()
                )
            }
        }
        #[doc = "VCI Negedge Detect Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct NedgeDet(pub u32);
        impl NedgeDet {
            #[doc = "These bits record a high to low transition on the VCI_IN# pins. A 1 indicates a transition occurred. For each bit in the field: 1=Negative Edge Detected; 0=No edge detected."]
            #[inline(always)]
            pub const fn vci_in(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "These bits record a high to low transition on the VCI_IN# pins. A 1 indicates a transition occurred. For each bit in the field: 1=Negative Edge Detected; 0=No edge detected."]
            #[inline(always)]
            pub fn set_vci_in(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for NedgeDet {
            #[inline(always)]
            fn default() -> NedgeDet {
                NedgeDet(0)
            }
        }
        impl core::fmt::Debug for NedgeDet {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("NedgeDet").field("vci_in", &self.vci_in()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for NedgeDet {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "NedgeDet {{ vci_in: {=u8:?} }}", self.vci_in())
            }
        }
        #[doc = "VCI Posedge Detect Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct PedgeDet(pub u32);
        impl PedgeDet {
            #[doc = "These bits record a low to high transition on the VCI_IN# pins. A 1 indicates a transition occurred. For each bit in the field: 1=Positive Edge Detected; 0=No edge detected."]
            #[inline(always)]
            pub const fn vci_in(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "These bits record a low to high transition on the VCI_IN# pins. A 1 indicates a transition occurred. For each bit in the field: 1=Positive Edge Detected; 0=No edge detected."]
            #[inline(always)]
            pub fn set_vci_in(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for PedgeDet {
            #[inline(always)]
            fn default() -> PedgeDet {
                PedgeDet(0)
            }
        }
        impl core::fmt::Debug for PedgeDet {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("PedgeDet").field("vci_in", &self.vci_in()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for PedgeDet {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "PedgeDet {{ vci_in: {=u8:?} }}", self.vci_in())
            }
        }
        #[doc = "VCI Polarity Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Polarity(pub u32);
        impl Polarity {
            #[doc = "These bits determine the polarity of the VCI_IN input signals: For each bit in the field: 1=Active High. The value on the pins is inverted before use 0=Active Low (default)."]
            #[inline(always)]
            pub const fn vci_in(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "These bits determine the polarity of the VCI_IN input signals: For each bit in the field: 1=Active High. The value on the pins is inverted before use 0=Active Low (default)."]
            #[inline(always)]
            pub fn set_vci_in(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for Polarity {
            #[inline(always)]
            fn default() -> Polarity {
                Polarity(0)
            }
        }
        impl core::fmt::Debug for Polarity {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Polarity").field("vci_in", &self.vci_in()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Polarity {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Polarity {{ vci_in: {=u8:?} }}", self.vci_in())
            }
        }
    }
}
pub mod wdt {
    #[doc = "The function of the Watchdog Timer is to provide a mechanism to detect if the internal embedded controller has failed."]
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
        #[doc = "Writing this field reloads the Watch Dog Timer counter."]
        #[inline(always)]
        pub const fn load(self) -> crate::common::Reg<u16, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "WDT Control Register"]
        #[inline(always)]
        pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "The WDT Kick Register is a strobe. Reads of this register return 0. Writes to this register cause the WDT to reload the WDT Load Register value and start decrementing when the WDT_ENABLE bit in the WDT Control Register is set to '1'. When the WDT_ENABLE bit in the WDT Control Register is cleared to '0', writes to the WDT Kick Register have no effect."]
        #[inline(always)]
        pub const fn kick(self) -> crate::common::Reg<u8, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "This read-only register provides the current WDT count."]
        #[inline(always)]
        pub const fn cnt(self) -> crate::common::Reg<u16, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "This register provides the current WDT count."]
        #[inline(always)]
        pub const fn sts(self) -> crate::common::Reg<regs::Sts, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "Watch Dog Interrupt Enable Register."]
        #[inline(always)]
        pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "WDT Control Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctrl(pub u16);
        impl Ctrl {
            #[doc = "WDT Block enabled"]
            #[inline(always)]
            pub const fn wdt_en(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "WDT Block enabled"]
            #[inline(always)]
            pub fn set_wdt_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
            }
            #[doc = "WDT_STATUS is set by hardware if the last reset of the device was caused by an underflow of the WDT. This bit must be cleared by the EC firmware writing a '1' to this bit. Writing a '0' to this bit has no effect."]
            #[inline(always)]
            pub const fn wdt_sts(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "WDT_STATUS is set by hardware if the last reset of the device was caused by an underflow of the WDT. This bit must be cleared by the EC firmware writing a '1' to this bit. Writing a '0' to this bit has no effect."]
            #[inline(always)]
            pub fn set_wdt_sts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
            }
            #[doc = "This bit enables the WDT Stall function if the Hibernation Timer 0 is active. 1=The WDT is stalled while the Hibernation Timer 0 is active 0=The WDT is not affected by Hibernation Timer 0."]
            #[inline(always)]
            pub const fn hib_tmr0_stl(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "This bit enables the WDT Stall function if the Hibernation Timer 0 is active. 1=The WDT is stalled while the Hibernation Timer 0 is active 0=The WDT is not affected by Hibernation Timer 0."]
            #[inline(always)]
            pub fn set_hib_tmr0_stl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
            }
            #[doc = "This bit enables the WDT Stall function if the Week Timer is active. 1=The WDT is stalled while the Week Timer is active 0=The WDT is not affected by the Week Timer."]
            #[inline(always)]
            pub const fn wk_tmr_stl(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "This bit enables the WDT Stall function if the Week Timer is active. 1=The WDT is stalled while the Week Timer is active 0=The WDT is not affected by the Week Timer."]
            #[inline(always)]
            pub fn set_wk_tmr_stl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
            }
            #[doc = "This bit enables the WDT Stall function if JTAG or SWD debug functions are active 1=The WDT is stalled while either JTAG or SWD is active 0=The WDT is not affected by the JTAG debug interface."]
            #[inline(always)]
            pub const fn jtag_stl(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "This bit enables the WDT Stall function if JTAG or SWD debug functions are active 1=The WDT is stalled while either JTAG or SWD is active 0=The WDT is not affected by the JTAG debug interface."]
            #[inline(always)]
            pub fn set_jtag_stl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
            }
            #[doc = "If the WDT_RESET bit is set and the watch dog timer expires, the Watch dog module will generate interrupt and clear the WDT_RESET to 0b."]
            #[inline(always)]
            pub const fn wdt_rst(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "If the WDT_RESET bit is set and the watch dog timer expires, the Watch dog module will generate interrupt and clear the WDT_RESET to 0b."]
            #[inline(always)]
            pub fn set_wdt_rst(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
            }
        }
        impl Default for Ctrl {
            #[inline(always)]
            fn default() -> Ctrl {
                Ctrl(0)
            }
        }
        impl core::fmt::Debug for Ctrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ctrl")
                    .field("wdt_en", &self.wdt_en())
                    .field("wdt_sts", &self.wdt_sts())
                    .field("hib_tmr0_stl", &self.hib_tmr0_stl())
                    .field("wk_tmr_stl", &self.wk_tmr_stl())
                    .field("jtag_stl", &self.jtag_stl())
                    .field("wdt_rst", &self.wdt_rst())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ctrl {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "Ctrl {{ wdt_en: {=bool:?}, wdt_sts: {=bool:?}, hib_tmr0_stl: {=bool:?}, wk_tmr_stl: {=bool:?}, jtag_stl: {=bool:?}, wdt_rst: {=bool:?} }}" , self . wdt_en () , self . wdt_sts () , self . hib_tmr0_stl () , self . wk_tmr_stl () , self . jtag_stl () , self . wdt_rst ())
            }
        }
        #[doc = "Watch Dog Interrupt Enable Register."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ien(pub u8);
        impl Ien {
            #[doc = "WDT_Int_Enable: This is the interrupt enables bit for WDT_INT interrupt. 1= WDT_INT Interrupt Enable 0= WDT_INT Interrupt Disabled"]
            #[inline(always)]
            pub const fn wdt_inten(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "WDT_Int_Enable: This is the interrupt enables bit for WDT_INT interrupt. 1= WDT_INT Interrupt Enable 0= WDT_INT Interrupt Disabled"]
            #[inline(always)]
            pub fn set_wdt_inten(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
        }
        impl Default for Ien {
            #[inline(always)]
            fn default() -> Ien {
                Ien(0)
            }
        }
        impl core::fmt::Debug for Ien {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ien").field("wdt_inten", &self.wdt_inten()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ien {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Ien {{ wdt_inten: {=bool:?} }}", self.wdt_inten())
            }
        }
        #[doc = "This register provides the current WDT count."]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sts(pub u8);
        impl Sts {
            #[doc = "WDT_EVENT_IRQ : This bit indicates the status of interrupt from Watch dog module."]
            #[inline(always)]
            pub const fn wdt_ev_irq(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "WDT_EVENT_IRQ : This bit indicates the status of interrupt from Watch dog module."]
            #[inline(always)]
            pub fn set_wdt_ev_irq(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
        }
        impl Default for Sts {
            #[inline(always)]
            fn default() -> Sts {
                Sts(0)
            }
        }
        impl core::fmt::Debug for Sts {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Sts").field("wdt_ev_irq", &self.wdt_ev_irq()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Sts {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Sts {{ wdt_ev_irq: {=bool:?} }}", self.wdt_ev_irq())
            }
        }
    }
}
pub mod week {
    #[doc = "The Week Timer and the Sub-Week Timer assert the Power-Up Event Output which automatically powers-up the system from the G3 state"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Week {
        ptr: *mut u8,
    }
    unsafe impl Send for Week {}
    unsafe impl Sync for Week {}
    impl Week {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Control Register"]
        #[inline(always)]
        pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Week Alarm Counter Register"]
        #[inline(always)]
        pub const fn alarm_cnt(self) -> crate::common::Reg<regs::AlarmCnt, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Week Timer Compare Register"]
        #[inline(always)]
        pub const fn tmr_comp(self) -> crate::common::Reg<regs::TmrComp, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Clock Divider Register"]
        #[inline(always)]
        pub const fn clkdiv(self) -> crate::common::Reg<regs::Clkdiv, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Sub-Second Programmable Interrupt Select Register"]
        #[inline(always)]
        pub const fn ss_intr_sel(self) -> crate::common::Reg<regs::SsIntrSel, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "Sub-Week Control Register"]
        #[inline(always)]
        pub const fn swk_ctrl(self) -> crate::common::Reg<regs::SwkCtrl, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "Sub-Week Alarm Counter Register"]
        #[inline(always)]
        pub const fn swk_alarm(self) -> crate::common::Reg<regs::SwkAlarm, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Week Alarm Counter Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct AlarmCnt(pub u32);
        impl AlarmCnt {
            #[doc = "While the WT_ENABLE bit is 1, this register is incremented at a 1 Hz rate. Writes of this register may require one second to take effect. Reads return the current state of the register. Reads and writes complete independently of the state of WT_ENABLE."]
            #[inline(always)]
            pub const fn wk_cntr(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x0fff_ffff;
                val as u32
            }
            #[doc = "While the WT_ENABLE bit is 1, this register is incremented at a 1 Hz rate. Writes of this register may require one second to take effect. Reads return the current state of the register. Reads and writes complete independently of the state of WT_ENABLE."]
            #[inline(always)]
            pub fn set_wk_cntr(&mut self, val: u32) {
                self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
            }
        }
        impl Default for AlarmCnt {
            #[inline(always)]
            fn default() -> AlarmCnt {
                AlarmCnt(0)
            }
        }
        impl core::fmt::Debug for AlarmCnt {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("AlarmCnt").field("wk_cntr", &self.wk_cntr()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for AlarmCnt {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "AlarmCnt {{ wk_cntr: {=u32:?} }}", self.wk_cntr())
            }
        }
        #[doc = "Clock Divider Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Clkdiv(pub u32);
        impl Clkdiv {
            #[doc = "Reads of this register return the current state of the Week Timer 15- bit clock divider."]
            #[inline(always)]
            pub const fn div(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x7fff;
                val as u16
            }
            #[doc = "Reads of this register return the current state of the Week Timer 15- bit clock divider."]
            #[inline(always)]
            pub fn set_div(&mut self, val: u16) {
                self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
            }
        }
        impl Default for Clkdiv {
            #[inline(always)]
            fn default() -> Clkdiv {
                Clkdiv(0)
            }
        }
        impl core::fmt::Debug for Clkdiv {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Clkdiv").field("div", &self.div()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Clkdiv {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "Clkdiv {{ div: {=u16:?} }}", self.div())
            }
        }
        #[doc = "Control Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctrl(pub u32);
        impl Ctrl {
            #[doc = "The WT_ENABLE bit is used to start and stop the Week Alarm Counter Register and the Clock Divider Register. The value in the Counter Register is held when the WT_ENABLE bit is not asserted (0) and the count is resumed from the last value when the bit is asserted (1). The 15-Bit Clock Divider is reset to 00h and the Week Alarm Interface is in its lowest power consumption state when the WT_ENABLE bit is not asserted."]
            #[inline(always)]
            pub const fn wt_en(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "The WT_ENABLE bit is used to start and stop the Week Alarm Counter Register and the Clock Divider Register. The value in the Counter Register is held when the WT_ENABLE bit is not asserted (0) and the count is resumed from the last value when the bit is asserted (1). The 15-Bit Clock Divider is reset to 00h and the Week Alarm Interface is in its lowest power consumption state when the WT_ENABLE bit is not asserted."]
            #[inline(always)]
            pub fn set_wt_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "This bit controls the state of the Power-Up Event Output and enables Week POWER-UP Event decoding in the VBAT-Powered Control Interface. 1=Power-Up Event Output Enabled 0=Power-Up Event Output Disabled and Reset"]
            #[inline(always)]
            pub const fn pwrup_en(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "This bit controls the state of the Power-Up Event Output and enables Week POWER-UP Event decoding in the VBAT-Powered Control Interface. 1=Power-Up Event Output Enabled 0=Power-Up Event Output Disabled and Reset"]
            #[inline(always)]
            pub fn set_pwrup_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
        }
        impl Default for Ctrl {
            #[inline(always)]
            fn default() -> Ctrl {
                Ctrl(0)
            }
        }
        impl core::fmt::Debug for Ctrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("Ctrl")
                    .field("wt_en", &self.wt_en())
                    .field("pwrup_en", &self.pwrup_en())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Ctrl {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Ctrl {{ wt_en: {=bool:?}, pwrup_en: {=bool:?} }}",
                    self.wt_en(),
                    self.pwrup_en()
                )
            }
        }
        #[doc = "Sub-Second Programmable Interrupt Select Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SsIntrSel(pub u32);
        impl SsIntrSel {
            #[doc = "This field determines the rate at which Sub-Second interrupt events are generated."]
            #[inline(always)]
            pub const fn spisr(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "This field determines the rate at which Sub-Second interrupt events are generated."]
            #[inline(always)]
            pub fn set_spisr(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
        }
        impl Default for SsIntrSel {
            #[inline(always)]
            fn default() -> SsIntrSel {
                SsIntrSel(0)
            }
        }
        impl core::fmt::Debug for SsIntrSel {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("SsIntrSel").field("spisr", &self.spisr()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for SsIntrSel {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "SsIntrSel {{ spisr: {=u8:?} }}", self.spisr())
            }
        }
        #[doc = "Sub-Week Alarm Counter Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SwkAlarm(pub u32);
        impl SwkAlarm {
            #[doc = "Writes with a non-zero value to this field reload the 9-bit Sub-Week Alarm counter. Writes of 0 disable the counter. If the Sub-Week Alarm counter decrements to 0 and the AUTO_RELOAD bit is set, the value in this field is automatically loaded into the Sub-Week Alarm counter."]
            #[inline(always)]
            pub const fn cntr_load(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x01ff;
                val as u16
            }
            #[doc = "Writes with a non-zero value to this field reload the 9-bit Sub-Week Alarm counter. Writes of 0 disable the counter. If the Sub-Week Alarm counter decrements to 0 and the AUTO_RELOAD bit is set, the value in this field is automatically loaded into the Sub-Week Alarm counter."]
            #[inline(always)]
            pub fn set_cntr_load(&mut self, val: u16) {
                self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
            }
            #[doc = "Reads of this register return the current state of the 9-bit Sub-Week Alarm counter."]
            #[inline(always)]
            pub const fn cntr_sts(&self) -> u16 {
                let val = (self.0 >> 16usize) & 0x01ff;
                val as u16
            }
            #[doc = "Reads of this register return the current state of the 9-bit Sub-Week Alarm counter."]
            #[inline(always)]
            pub fn set_cntr_sts(&mut self, val: u16) {
                self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
            }
        }
        impl Default for SwkAlarm {
            #[inline(always)]
            fn default() -> SwkAlarm {
                SwkAlarm(0)
            }
        }
        impl core::fmt::Debug for SwkAlarm {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("SwkAlarm")
                    .field("cntr_load", &self.cntr_load())
                    .field("cntr_sts", &self.cntr_sts())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for SwkAlarm {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "SwkAlarm {{ cntr_load: {=u16:?}, cntr_sts: {=u16:?} }}",
                    self.cntr_load(),
                    self.cntr_sts()
                )
            }
        }
        #[doc = "Sub-Week Control Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SwkCtrl(pub u32);
        impl SwkCtrl {
            #[doc = "This bit is set to 1 when the Sub-Week Alarm Counter Register decrements from 1 to 0 and the POWERUP_EN is 1. Writes of 1 clear this bit. Writes of 0 have no effect. Note: This bit MUST be cleared to remove a Sub-Week Timer Power-Up Event."]
            #[inline(always)]
            pub const fn swktmr_pwrup_evt_sts(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "This bit is set to 1 when the Sub-Week Alarm Counter Register decrements from 1 to 0 and the POWERUP_EN is 1. Writes of 1 clear this bit. Writes of 0 have no effect. Note: This bit MUST be cleared to remove a Sub-Week Timer Power-Up Event."]
            #[inline(always)]
            pub fn set_swktmr_pwrup_evt_sts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "This bit is set to 1 when the Week Alarm Counter Register is greater than or equal the contents of the Week Timer Compare Register and the POWERUP_EN is 1. Writes of 1 clear this bit. Writes of 0 have no effect. Note: This bit does not have to be cleared to remove a Week Timer Power-Up Event."]
            #[inline(always)]
            pub const fn wktmr_pwrup_evt_sts(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "This bit is set to 1 when the Week Alarm Counter Register is greater than or equal the contents of the Week Timer Compare Register and the POWERUP_EN is 1. Writes of 1 clear this bit. Writes of 0 have no effect. Note: This bit does not have to be cleared to remove a Week Timer Power-Up Event."]
            #[inline(always)]
            pub fn set_wktmr_pwrup_evt_sts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Test"]
            #[inline(always)]
            pub const fn test(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Test"]
            #[inline(always)]
            pub fn set_test(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Test"]
            #[inline(always)]
            pub const fn test0(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Test"]
            #[inline(always)]
            pub fn set_test0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "1= No reload occurs when the Sub-Week Counter expires 0= Reloads the SUBWEEK_COUNTER_LOAD field into the Sub- Week Counter when the counter expires."]
            #[inline(always)]
            pub const fn au_rld(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "1= No reload occurs when the Sub-Week Counter expires 0= Reloads the SUBWEEK_COUNTER_LOAD field into the Sub- Week Counter when the counter expires."]
            #[inline(always)]
            pub fn set_au_rld(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "This field selects the clock source for the Sub-Week Counter."]
            #[inline(always)]
            pub const fn swk_tick(&self) -> u8 {
                let val = (self.0 >> 7usize) & 0x07;
                val as u8
            }
            #[doc = "This field selects the clock source for the Sub-Week Counter."]
            #[inline(always)]
            pub fn set_swk_tick(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u32) & 0x07) << 7usize);
            }
        }
        impl Default for SwkCtrl {
            #[inline(always)]
            fn default() -> SwkCtrl {
                SwkCtrl(0)
            }
        }
        impl core::fmt::Debug for SwkCtrl {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("SwkCtrl")
                    .field("swktmr_pwrup_evt_sts", &self.swktmr_pwrup_evt_sts())
                    .field("wktmr_pwrup_evt_sts", &self.wktmr_pwrup_evt_sts())
                    .field("test", &self.test())
                    .field("test0", &self.test0())
                    .field("au_rld", &self.au_rld())
                    .field("swk_tick", &self.swk_tick())
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for SwkCtrl {
            fn format(&self, f: defmt::Formatter) {
                defmt :: write ! (f , "SwkCtrl {{ swktmr_pwrup_evt_sts: {=bool:?}, wktmr_pwrup_evt_sts: {=bool:?}, test: {=bool:?}, test0: {=bool:?}, au_rld: {=bool:?}, swk_tick: {=u8:?} }}" , self . swktmr_pwrup_evt_sts () , self . wktmr_pwrup_evt_sts () , self . test () , self . test0 () , self . au_rld () , self . swk_tick ())
            }
        }
        #[doc = "Week Timer Compare Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TmrComp(pub u32);
        impl TmrComp {
            #[doc = "A Week Alarm Interrupt and a Week Alarm Power-Up Event are asserted when the Week Alarm Counter Register is greater than or equal to the contents of this register. Reads and writes complete independently of the state of WT_ENABLE."]
            #[inline(always)]
            pub const fn wk_comp(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x0fff_ffff;
                val as u32
            }
            #[doc = "A Week Alarm Interrupt and a Week Alarm Power-Up Event are asserted when the Week Alarm Counter Register is greater than or equal to the contents of this register. Reads and writes complete independently of the state of WT_ENABLE."]
            #[inline(always)]
            pub fn set_wk_comp(&mut self, val: u32) {
                self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
            }
        }
        impl Default for TmrComp {
            #[inline(always)]
            fn default() -> TmrComp {
                TmrComp(0)
            }
        }
        impl core::fmt::Debug for TmrComp {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("TmrComp").field("wk_comp", &self.wk_comp()).finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for TmrComp {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "TmrComp {{ wk_comp: {=u32:?} }}", self.wk_comp())
            }
        }
    }
}
