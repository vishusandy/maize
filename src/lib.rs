#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(const_fn_floating_point_arithmetic)]

mod drawing;
mod edges;
mod error;
mod graphs;
mod render;

const DEFAULT_NEIGHBORS: usize = 6;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
