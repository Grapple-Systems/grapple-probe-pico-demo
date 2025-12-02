//! This example test the RP Pico on board LED.
//!
//! It does not work with the RP Pico W board. See wifi_blinky.rs.

#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::{gpio, uart, peripherals as periphs};
use embassy_time::Timer;
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};

embassy_rp::bind_interrupts!(struct Irqs {
    UART0_IRQ => uart::BufferedInterruptHandler<periphs::UART0>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_25, Level::Low);

    static RX_BUF: static_cell::StaticCell<[u8; 16]> = static_cell::StaticCell::new();
    let rx_buffer = RX_BUF.init([0; 16]).as_mut_slice();
    static TX_BUF: static_cell::StaticCell<[u8; 16]> = static_cell::StaticCell::new();
    let tx_buffer = TX_BUF.init([0; 16]).as_mut_slice();
    let mut config = uart::Config::default();
    config.baudrate = 115200;
    let uart = uart::BufferedUart::new(p.UART0, p.PIN_0, p.PIN_1, Irqs, tx_buffer, rx_buffer, config);

    spawner.must_spawn(uart_echo(uart));

    loop {
        info!("led on!");
        led.set_high();
        Timer::after_secs(1).await;

        info!("led off!");
        led.set_low();
        Timer::after_secs(1).await;
    }
}

#[embassy_executor::task]
async fn uart_echo(mut uart: uart::BufferedUart) {
    use embedded_io_async::{Read, Write};

    let mut buf = [0u8; 16];
    loop {
        if let Ok(read_size) = uart.read(&mut buf).await {
            let _ = uart.write(&buf[..read_size]).await;
        }
    }
}