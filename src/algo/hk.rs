use crate::graphs::{Graph, Node};
use crate::render::state::graph;
use crate::render::RenderGraph;
use rand::seq::SliceRandom;
use rand::Rng;
use webp_animation::{Encoder, WebPData};

pub fn huntkill<G: Graph, R: Rng + ?Sized>(graph: &mut G, rng: &mut R) -> Result<(), crate::Error> {
    if let Some(mut node) = graph.random(rng) {
        loop {
            if let Some(n) = graph.unlinked_neighbors(node).choose(rng) {
                graph.link(node, *n)?;
                node = *n;
            } else if let Some(n) = hunt(graph, rng)? {
                node = n;
            } else {
                break;
            }
        }
    }
    Ok(())
}

fn hunt<G: Graph, R: Rng + ?Sized>(
    graph: &mut G,
    rng: &mut R,
) -> Result<Option<usize>, crate::Error> {
    for node in graph.node_ids() {
        // find first cell without any links
        if graph.node(node).is_empty() {
            // choose a random cell that has links
            if let Some(n) = graph.linked_neighbors(node).choose(rng) {
                return graph.link(node, *n).map(|_| Some(*n));
            }
        }
    }
    Ok(None)
}

const SEL_NODE: image::Rgba<u8> = crate::color::LIGHT_GREEN;
const IN_PATH: image::Rgba<u8> = crate::color::GREEN;

pub fn animated_huntkill<G, R>(
    graph: &mut graph::State<G>,
    opts: &crate::render::opts::AnimOpts,
    rng: &mut R,
) -> Result<WebPData, crate::Error>
where
    R: Rng + ?Sized,
    G: Graph + RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
    use crate::render::RenderState;

    let mut timestamp = 0i32;
    let mut encoder: Encoder = Encoder::new_with_options(graph.size(), opts.encoder_options())?;

    if let Some(mut node) = graph.graph().random(rng) {
        graph.set_bg(node, Some(SEL_NODE));
        let image = graph.render();
        encoder.add_frame(&*image, timestamp)?;
        timestamp += opts.frame_time();
        graph.set_bg(node, Some(IN_PATH));
        let image = graph.render();
        encoder.add_frame(&*image, timestamp)?;
        timestamp += opts.frame_time();

        loop {
            if let Some(n) = graph.graph().unlinked_neighbors(node).choose(rng) {
                graph.set_bg(*n, Some(SEL_NODE));
                let image = graph.render();
                encoder.add_frame(&*image, timestamp)?;
                timestamp += opts.frame_time();
                graph.set_bg(*n, Some(IN_PATH));

                graph.graph_mut().link(node, *n)?;

                let image = graph.render();
                encoder.add_frame(&*image, timestamp)?;
                timestamp += opts.frame_time();

                node = *n;
            } else if let Some(n) = animated_hunt(graph, &mut timestamp, &mut encoder, opts, rng)? {
                graph.set_bg(n, Some(SEL_NODE));
                let image = graph.render();
                encoder.add_frame(&*image, timestamp)?;
                timestamp += opts.frame_time();

                graph.set_bg(n, Some(IN_PATH));
                let image = graph.render();
                encoder.add_frame(&*image, timestamp)?;
                timestamp += opts.frame_time();

                node = n;
            } else {
                break;
            }
        }

        encoder
            .finalize(timestamp + opts.repeat_delay())
            .map_err(crate::Error::from)
    } else {
        Err(crate::Error::ZeroSizedGraph())
    }
}

fn animated_hunt<G, R>(
    graph: &mut graph::State<G>,
    timestamp: &mut i32,
    encoder: &mut Encoder,
    opts: &crate::render::opts::AnimOpts,
    rng: &mut R,
) -> Result<Option<usize>, crate::Error>
where
    R: Rng + ?Sized,
    G: Graph + RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
    for node in graph.graph().node_ids() {
        // find first cell without any links
        if graph.graph().node(node).is_empty() {
            // choose a random cell that has links
            if let Some(n) = graph.graph().linked_neighbors(node).choose(rng) {
                graph.set_bg(node, Some(SEL_NODE));
                let image = graph.render();
                encoder.add_frame(&*image, *timestamp)?;
                *timestamp += opts.frame_time();
                graph.set_bg(node, Some(IN_PATH));

                graph.set_bg(*n, Some(IN_PATH));
                let image = graph.render();
                encoder.add_frame(&*image, *timestamp)?;
                *timestamp += opts.frame_time();

                return graph.graph_mut().link(node, *n).map(|_| Some(*n));
            }
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use crate::render::opts::AnimOpts;
    use rand::SeedableRng;
    use rand_xoshiro::SplitMix64;
    #[test]
    fn algo_huntkill() -> Result<(), image::ImageError> {
        crate::logger(crate::LOG_LEVEL);
        let rng = &mut SplitMix64::seed_from_u64(13131313131313131313);
        let mut graph = crate::Orth::new(8, 8);
        super::huntkill(&mut graph, rng).unwrap();
        graph.render().save("images/tests/rect_huntkill.png")
    }

    #[test]
    fn anim_huntkill() -> Result<(), crate::Error> {
        use std::fs::OpenOptions;
        use std::io::prelude::*;
        crate::logger(crate::LOG_LEVEL);
        let rng = &mut SplitMix64::seed_from_u64(13131313131313131313);
        let mut graph = crate::Orth::new(8, 8).build_render_owned().finish();
        let opts = AnimOpts::default();
        let bytes = super::animated_huntkill(&mut graph, &opts, rng)?;

        // let mut f = File::create("images/tests/rect_huntkill.webp").expect("Error creating file");
        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .open("images/tests/rect_huntkill.webp")
            .unwrap();
        f.write_all(&bytes).expect("Error writing to file");
        Ok(())
    }
}
