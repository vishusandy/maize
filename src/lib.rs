#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(const_fn_floating_point_arithmetic)]

mod algo;
mod drawing;
mod edges;
mod error;
mod graphs;
mod render;
pub(crate) mod util;

pub use crate::algo::dist::Dist;
pub use crate::algo::path::Path;
pub use crate::drawing::Pt;
pub use crate::error::Error;
pub use crate::graphs::orth::nodes::rect::RectCell;
pub use crate::graphs::orth::Orth;
pub use crate::graphs::Graph;
pub use crate::render::opts;

const DEFAULT_NEIGHBORS: usize = 6;

#[cfg(test)]
fn logger(level: log::LevelFilter) {
    let _ = env_logger::Builder::new()
        .filter_level(level)
        .format_module_path(false)
        .format_target(false)
        .format_timestamp(None)
        .format_level(false)
        .try_init();
}

#[cfg(test)]
pub(crate) mod test {
    use super::{Orth, RectCell};
    use crate::graphs::Graph;

    #[cfg(test)]
    pub fn blank(size: u32) -> image::RgbaImage {
        image::RgbaImage::from_pixel(size, size, image::Rgba([255, 255, 255, 255]))
    }

    #[cfg(test)]
    pub(crate) fn rect() -> Orth<RectCell> {
        let mut grid: Orth<RectCell> = Orth::new(4, 4);
        grid.link(0, 1).unwrap();
        grid.link(0, 4).unwrap();
        grid.link(4, 8).unwrap();
        grid.link(8, 12).unwrap();
        grid.link(1, 2).unwrap();
        grid.link(2, 3).unwrap();
        grid.link(1, 5).unwrap();
        grid.link(5, 6).unwrap();
        grid.link(6, 7).unwrap();
        grid.link(7, 11).unwrap();
        grid.link(11, 10).unwrap();
        grid.link(10, 9).unwrap();
        grid.link(9, 13).unwrap();
        grid.link(13, 14).unwrap();
        grid.link(14, 15).unwrap();

        grid
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
