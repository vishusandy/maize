pub(crate) mod anim;
pub(crate) mod dist;
pub(crate) mod graph;
pub(crate) mod path;

use super::{RenderGraph, RenderState};
use image::Rgba;

#[derive(Clone, Debug)]
pub struct NodeState {
    color: Option<Rgba<u8>>,
}
impl NodeState {
    pub fn new(color: &Rgba<u8>) -> Self {
        Self {
            color: Some(*color),
        }
    }
    pub fn vec(len: usize, color: &Rgba<u8>) -> Vec<Self> {
        (0..len).map(|_| Self::new(color)).collect()
    }

    pub fn get(&self) -> Option<Rgba<u8>> {
        self.color
    }

    pub fn set(&mut self, color: Option<Rgba<u8>>) {
        self.color = color;
    }
}
