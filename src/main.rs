#![no_std]
#![no_main]

use wio_terminal_probe_run::{self, display_helper, scd30::{SCD30, SensorData}};
use embedded_graphics as eg;

use wio_terminal as wio;
use wio::hal::{
    delay::Delay,
    clock::GenericClockController,
    gpio::{Pa16, Pa17, PfD},
    sercom::{I2CMaster3, Sercom3Pad0, Sercom3Pad1, PadPin},
};
use wio::pac::{CorePeripherals, Peripherals};
use wio::prelude::*;
use wio::{entry, Pins};

use eg::{pixelcolor::Rgb565, prelude::*, primitives::Rectangle, style::PrimitiveStyleBuilder};

const CO2_POSITION: (i32, i32) = (185, 90);
const CO2_UNIT: &str = "ppm";

const TEMP_POSITION: (i32, i32) = (185, 130);
const TEMP_UNIT: &str = "°C";

const HUMIDITY_POSITION: (i32, i32) = (185, 170);
const HUMIDITY_UNIT: &str = "%";

#[entry]
fn main() -> ! {
    defmt::info!("Hello, world!");

    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let mut sets = Pins::new(peripherals.PORT).split();
    let (i2c1_scl, i2c1_sda) = {
        let p = unsafe { Peripherals::steal() };
        let pins = Pins::new(p.PORT);
        (pins.i2c1_scl, pins.i2c1_sda)
    };

    let gclk0 = clocks.gclk0();
    let i2c: I2CMaster3<Sercom3Pad0<Pa17<PfD>>, Sercom3Pad1<Pa16<PfD>>> = I2CMaster3::new(
        &clocks.sercom3_core(&gclk0).unwrap(),
        400.khz(),
        peripherals.SERCOM3,
        &mut peripherals.MCLK,
        i2c1_sda.into_pad(&mut sets.port),
        i2c1_scl.into_pad(&mut sets.port),
    );

    let mut scd30 = SCD30::init(i2c);
    let firmware_version = scd30.get_firmware_version().unwrap();
    defmt::info!("Firmware Version: {=u8}.{=u8}", firmware_version[0], firmware_version[1]);

    let pressure = 1020_u16;
    scd30.start_continuous_measurement(pressure).unwrap();

    // ディスプレイドライバを初期化する
    let (mut display, _backlight) = sets
        .display
        .init(
            &mut clocks,
            peripherals.SERCOM7,
            &mut peripherals.MCLK,
            &mut sets.port,
            58.mhz(),
            &mut delay,
        )
        .unwrap();

    let style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::BLACK)
        .build();
    let background =
        Rectangle::new(Point::new(0, 0), Point::new(319, 239))
            .into_styled(style);
    background.draw(&mut display).unwrap();

    loop {
        if scd30.data_ready().unwrap() {
            defmt::info!("Data ready");
            break
        }
    }

    loop {
        let SensorData{ co2, temperature, humidity} = scd30.read_measurement().unwrap();
        defmt::info!("
            CO2 {=f32} ppm
            Temperature {=f32} °C
            Humidity {=f32} %
            ", co2, temperature, humidity
        );

        background.draw(&mut display).unwrap();
        display = display_helper::draw_text(display);
        display = display_helper::draw_numbers(co2, CO2_UNIT, CO2_POSITION, display);
        display = display_helper::draw_numbers(temperature, TEMP_UNIT, TEMP_POSITION, display);
        display = display_helper::draw_numbers(humidity, HUMIDITY_UNIT, HUMIDITY_POSITION, display);

        delay.delay_ms(3_000_u32);
    }
}
