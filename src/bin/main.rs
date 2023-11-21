#![no_std]
#![no_main]

use core::fmt::Write;
use cortex_m_rt::entry;
use defmt;
use defmt_rtt as _;
use panic_halt as _;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    serial::{Config, Serial},
};
#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let mut afio = dp.AFIO.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut gpioa = dp.GPIOA.split();

    let serial = Serial::new(
        dp.USART1,
        (
            gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh),
            gpioa.pa10,
        ),
        &mut afio.mapr,
        Config::default().baudrate(115200.bps()),
        &clocks,
    );
    let (mut tx, mut _rx) = serial.split();

    let mut led = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);
    let mut delay = cp.SYST.delay(&clocks);

    defmt::info!("This is a Info.");
    defmt::warn!("This is a warn");
    defmt::error!("This is a Error");
    defmt::println!("This is just print");
    writeln!(tx, "Seial.").unwrap();
    loop {
        defmt::info!("Led Toggle");
        led.toggle();
        delay.delay_ms(1000u16);
    }
}
