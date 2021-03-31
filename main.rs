#![no_std]
#![no_main]

use stm32f1xx_hal::gpio::GpioExt;
use stm32f1xx_hal::prelude::_stm32_hal_rcc_RccExt;
use stm32f1xx_hal::flash::FlashExt;
use panic_halt as _;
use cortex_m_semihosting::{debug,hprintln};
use rtic::app;
use hd44780_driver::{Cursor, CursorBlink, Display, DisplayMode, HD44780};
use stm32f1xx_hal::{gpio::{gpioa::PA15,Output,PushPull},
                    stm32::*,
                    pac::*,
                    
};


#[app(device = stm32f1xx_hal::stm32, peripherals = true)]
const APP: () 
    #[init]
    fn init(cx:init::Context)
    {
    
        let cp = cx.core;
        let dp = stm32f1xx_hal::pac::Peripherals::take().unwrap();
       

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



// Display the following string
lcd.expect("invalid conection").write_str("Hello, world!", &mut delay);

}

    #[idle]
    fn idle(cx:idle::Context) -> ! {

    loop{}
    }
};

 
