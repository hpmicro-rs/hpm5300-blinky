#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
use hpm_metapac::{self as pac, gpiom::vals, FGPIO, SYSCTL};
use riscv::delay::McycleDelay;

use defmt_rtt as _;

#[riscv_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, Rust from HPM5301!!!!!");

    // resource, group, and cpu link.=
    SYSCTL.group0(0).value().modify(|w| w.0 = 0xFFFFFFFF);
    SYSCTL.group0(1).value().modify(|w| w.0 = 0xFFFFFFFF);

    SYSCTL.affiliate(0).set().write(|w| w.set_link(1));

    // GPIO, = >> PA10

    const PA: usize = 0;
    pac::GPIOM.assign(PA).pin(10).modify(|w| {
        w.set_select(vals::PinSelect::CPU0_FGPIO);
        w.set_hide(0b01); // invisible to GPIO0
    });

    FGPIO.oe(PA).set().write(|w| w.set_direction(1 << 10));

    let mut delay = McycleDelay::new(720_000_000 / 2);

    loop {
        defmt::info!("tick");

        delay.delay_ms(1000);

        FGPIO.do_(PA).toggle().write(|w| w.set_output(1 << 10));
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
