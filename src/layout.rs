use blinksy::layout::{Layout3d, Shape3d, Vec3};
use core::iter;

pub struct Layout;

impl Layout3d for Layout {
    const PIXEL_COUNT: usize = 5 * 5 * 5;

    fn shapes() -> impl Iterator<Item = Shape3d> {
        let mut index: usize = 0;

        fn map(n: usize) -> f32 {
            map_range(n as f32, 0., 4., -1., 1.)
        }

        iter::from_fn(move || {
            if index >= 5 * 5 * 5 {
                return None;
            }

            let x = map(index % 5);
            let z = map(index / 5 % 5);
            let y = map(index / 5 / 5);

            index += 1;

            Some(Shape3d::Point(Vec3::new(x, y, z)))
        })
    }
}

fn map_range(value: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    (value - in_min) / (in_max - in_min) * (out_max - out_min) + out_min
}
