#![cfg_attr(target_arch = "xtensa", no_std)]
#![cfg_attr(target_arch = "xtensa", no_main)]

use blinksy::{
    layout1d,
    patterns::rainbow::{Rainbow, RainbowParams},
    ControlBuilder,
};
use cfg_iif::cfg_iif;

#[cfg(target_arch = "xtensa")]
use blinksy::layout::Layout1d;
#[cfg(target_arch = "xtensa")]
use gledopto::{board, elapsed, main, ws2812};

fn run() {
    #[cfg(target_arch = "xtensa")]
    let p = board!();

    layout1d!(Layout, 5 * 60);

    let mut control = ControlBuilder::new_1d()
        .with_layout::<Layout>()
        .with_pattern::<Rainbow>(RainbowParams {
            position_scalar: 1.,
            ..Default::default()
        })
        .with_driver(cfg_iif!(
            #[cfg(target_arch = "xtensa")] {
                ws2812!(p, Layout::PIXEL_COUNT)
            } else {
                blinksy_desktop::driver::Desktop::new_1d::<Layout>()
            }
        ))
        .build();

    cfg_iif!(
        #[cfg(target_arch = "xtensa")]
        {
            control.set_brightness(0.2);
        }
    );

    loop {
        let elapsed_in_ms = cfg_iif!(
            #[cfg(target_arch = "xtensa")] { elapsed().as_millis() }
            else { blinksy_desktop::time::elapsed_in_ms() }
        );
        control.tick(elapsed_in_ms).unwrap();
    }
}

#[cfg(target_arch = "xtensa")]
#[main]
fn main() -> ! {
    run();
    loop {}
}

#[cfg(not(target_arch = "xtensa"))]
fn main() {
    run();
}
