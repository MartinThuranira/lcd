#![no_std]
#![no_main]

use cortex_m_semihosting::{debug, hprintln};
use panic_halt as _;
use rtic::app;
use stm32f1xx_hal::flash::FlashExt;
use stm32f1xx_hal::gpio::GpioExt;
use stm32f1xx_hal::prelude::_stm32_hal_rcc_RccExt;
//use stm32f1::stm32f103;
use hd44780_driver::{Cursor, CursorBlink, Display, DisplayMode, HD44780};
use stm32f1xx_hal::{
    gpio::{gpioa::PA15, Output, PushPull},
    pac::*,
    stm32::*,
};

#[app(device = stm32f1xx_hal::stm32, peripherals = true)]
const APP: () = {
    
    #[init]
    fn init(cx: init::Context) //-> init::LateResources
    {
        hprintln!("Before code").unwrap();
        hprintln!("control").unwrap();
        let cp = cx.core;
        //let dp = cx.device;
        let dp = stm32f1xx_hal::pac::Peripherals::take().unwrap();
        //let dp = stm32f1xx_hal::stm32::Peripherals::take().unwrap();
        //hprintln!("Hello, world!").unwrap();

        let mut flash = dp.FLASH.constrain();
        let mut rcc = dp.RCC.constrain();
        let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);

        let clocks = rcc.cfgr.freeze(&mut flash.acr);
        let mut delay = stm32f1xx_hal::delay::Delay::new(cp.SYST, clocks);

        //init pins

        let rs = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);
        let en = gpioa.pa2.into_push_pull_output(&mut gpioa.crl);
        let b4 = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);
        let b5 = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);
        let b6 = gpioa.pa6.into_push_pull_output(&mut gpioa.crl);
        let b7 = gpioa.pa7.into_push_pull_output(&mut gpioa.crl);

        let mut lcd = HD44780::new_4bit(rs, en, b4, b5, b6, b7, &mut delay).unwrap();

        // Unshift display and set cursor to 0
        lcd.reset(&mut delay);

        // Clear existing characters
        lcd.clear(&mut delay);

        // Display the following string
        lcd.write_str("Hello, world!", &mut delay);

        //control code for check
        loop {
            hprintln!("After code").unwrap();
        }
    }

    #[idle]
    fn idle(cx: idle::Context) -> ! {
        loop {}
    }
};
