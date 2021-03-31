#[allow(non_snake_case)] fn init(cx : init :: Context)
{
    let cp = cx . core ; let dp = stm32f1xx_hal :: pac :: Peripherals ::
    take() . unwrap() ; let mut flash = dp . FLASH . constrain() ; let mut rcc
    = dp . RCC . constrain() ; let mut gpioa = dp . GPIOA .
    split(& mut rcc . apb2) ; let clocks = rcc . cfgr .
    freeze(& mut flash . acr) ; let mut delay = stm32f1xx_hal :: delay ::
    Delay :: new(cp . SYST, clocks) ; let rs = gpioa . pa1 .
    into_push_pull_output(& mut gpioa . crl) ; let en = gpioa . pa2 .
    into_push_pull_output(& mut gpioa . crl) ; let b4 = gpioa . pa4 .
    into_push_pull_output(& mut gpioa . crl) ; let b5 = gpioa . pa5 .
    into_push_pull_output(& mut gpioa . crl) ; let b6 = gpioa . pa6 .
    into_push_pull_output(& mut gpioa . crl) ; let b7 = gpioa . pa7 .
    into_push_pull_output(& mut gpioa . crl) ; let mut lcd = HD44780 ::
    new_4bit(rs, en, b4, b5, b6, b7, & mut delay) ; lcd . unwrap() .
    write_str("Hello, world!", & mut delay) ;
} #[allow(non_snake_case)] fn idle(cx : idle :: Context) -> !
{ use rtic :: Mutex as _ ; loop { } } #[allow(non_snake_case)]
#[doc = "Initialization function"] pub mod init
{
    #[doc = r" Execution context"] pub struct Context < >
    {
        #[doc = r" Core (Cortex-M) peripherals"] pub core : rtic :: export ::
        Peripherals, #[doc = r" Device peripherals"] pub device :
        stm32f1xx_hal :: stm32 :: Peripherals,
    } impl < > Context < >
    {
        #[inline(always)] pub unsafe fn
        new(core : rtic :: export :: Peripherals,) -> Self
        {
            Context
            {
                device : stm32f1xx_hal :: stm32 :: Peripherals :: steal(),
                core,
            }
        }
    }
} #[allow(non_snake_case)] #[doc = "Idle loop"] pub mod idle
{
    #[doc = r" Execution context"] pub struct Context < > { } impl < > Context
    < >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & rtic :: export :: Priority) -> Self { Context { } }
    }
} #[doc = r" Implementation details"] const APP : () =
{
    #[doc =
      r" Always include the device crate which contains the vector table"] use
    stm32f1xx_hal :: stm32 as _ ; #[cfg(core = "1")] compile_error !
    ("specified 1 core but tried to compile for more than 1 core") ;
    #[no_mangle] unsafe extern "C" fn main() -> !
    {
        let _TODO : () = () ; rtic :: export :: interrupt :: disable() ; let
        mut core : rtic :: export :: Peripherals = rtic :: export ::
        Peripherals :: steal() . into() ; let late = crate ::
        init(init :: Context :: new(core . into())) ; rtic :: export ::
        interrupt :: enable() ; crate ::
        idle(idle :: Context :: new(& rtic :: export :: Priority :: new(0)))
    }
} ;