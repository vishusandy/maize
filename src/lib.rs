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
pub use crate::drawing::Pt;

pub use crate::algo::dist::Dist;
pub(crate) use crate::error::Error;
pub use crate::graphs::orth::nodes::rect::RectCell;
pub use crate::graphs::orth::Orth;
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
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
