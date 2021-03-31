#![no_std]
#![no_main]

use stm32f1xx_hal::gpio::GpioExt;
use stm32f1xx_hal::prelude::_stm32_hal_rcc_RccExt;
use stm32f1xx_hal::flash::FlashExt;
use panic_halt as _;
use cortex_m_semihosting::{debug,hprintln};
use rtic::app;
//use stm32f1::stm32f103;
use hd44780_driver::{Cursor, CursorBlink, Display, DisplayMode, HD44780};
use stm32f1xx_hal::{gpio::{gpioa::PA15,Output,PushPull},
                    stm32::*,
                    pac::*,
                    
};

#[app(device = stm32f1xx_hal::stm32, peripherals = true)]
const APP: () = {
/*
    struct Resources{
        delay:Delay,
    }*/
    #[init]
    fn init(cx:init::Context) //-> init::LateResources 
    {
    
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

    let mut lcd = HD44780::new_4bit(rs, en, b4, b5, b6, b7, &mut delay);

// Unshift display and set cursor to 0
//&lcd.expect("invalid conection").reset(&mut delay); 

// Clear existing characters
//&lcd.expect("invalid conection").clear(&mut delay); 

// Display the following string
lcd.unwrap().write_str("Hello, world!", &mut delay);

// Move the cursor to the second line
//&lcd.expect("invalid conection").set_cursor_pos(40, &mut delay);

// Display the following string on the second line
//&lcd.expect("invalid conection").write_str("I'm on line 2!", &mut delay);


    // lcd.reset();
    // lcd.clear();
    // lcd.set_display_mode(
    //     DisplayMode {
    //         display: Display::On,
    //         cursor_visibility: Cursor::Visible,
    //         cursor_blink: CursorBlink::On,
    //     });
/*
    init::LateResources{
        delay,
    }
    */
}

    #[idle]
    fn idle(cx:idle::Context) -> ! {

    loop{}
    }
};

 