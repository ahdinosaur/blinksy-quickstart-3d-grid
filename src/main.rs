#![cfg_attr(feature = "mcu", no_std)]
#![cfg_attr(feature = "mcu", no_main)]

use blinksy::{
    layout1d,
    patterns::rainbow::{Rainbow, RainbowParams},
    ControlBuilder,
};
use cfg_iif::cfg_iif;

#[cfg(feature = "mcu")]
use blinksy::layout::Layout1d;
#[cfg(feature = "mcu")]
use gledopto::{board, elapsed, main, ws2812};

fn run() {
    #[cfg(feature = "mcu")]
    let p = board!();

    layout1d!(Layout, 5 * 60);

    let mut control = ControlBuilder::new_1d()
        .with_layout::<Layout>()
        .with_pattern::<Rainbow>(RainbowParams {
            position_scalar: 1.,
            ..Default::default()
        })
        .with_driver(cfg_iif!(
            #[cfg(feature = "mcu")] {
                ws2812!(p, Layout::PIXEL_COUNT)
            } else {
                blinksy_desktop::driver::Desktop::new_1d::<Layout>()
            }
        ))
        .build();

    cfg_iif!(
        #[cfg(feature = "mcu")]
        {
            control.set_brightness(0.2);
        }
    );

    loop {
        let elapsed_in_ms = cfg_iif!(
            #[cfg(feature = "mcu")] { elapsed().as_millis() }
            else { blinksy_desktop::time::elapsed_in_ms() }
        );
        control.tick(elapsed_in_ms).unwrap();
    }
}

#[cfg(feature = "mcu")]
#[main]
fn main() -> ! {
    run();
    loop {}
}

#[cfg(feature = "desktop")]
fn main() {
    run();
}
