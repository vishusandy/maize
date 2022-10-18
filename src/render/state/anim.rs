use crate::graphs::Node;
use crate::render::opts;
use crate::render::state::graph;
use crate::render::RenderState;
use image::RgbaImage;
use std::borrow::Cow;
use webp_animation::{Encoder, Error, WebPData};

/// Contains state needed to build animations.
///
/// Only allows references to graph state and animation options.
/// This is to prevent unnecssary clones of owned data when building
/// animation frames.
#[derive(Clone, Debug)]
pub struct State<'ao, 'b, 'c, 'e, 'g, 'o, G>
where
    G: crate::render::RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
    graph: &'g graph::State<'b, 'c, 'e, 'g, 'o, G>,
    opts: &'ao opts::AnimOpts,
}

impl<'ao, 'b, 'c, 'e, 'g, 'o, G> State<'ao, 'b, 'c, 'e, 'g, 'o, G>
where
    G: crate::render::RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
}

#[derive(Clone, Debug)]
pub struct Builder {}
impl Builder {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<'b, 'c, 'e, 'g, 'o, G>(
        graph: &'g graph::State<'b, 'c, 'e, 'g, 'o, G>,
    ) -> BuilderGraph<'b, 'c, 'e, 'g, 'o, G>
    where
        G: crate::render::RenderGraph + Clone + std::fmt::Debug,
        <G::Node as Node>::Block: Clone + std::fmt::Debug,
    {
        BuilderGraph { graph }
    }
}

#[derive(Clone, Debug)]
pub struct BuilderGraph<'b, 'c, 'e, 'g, 'o, G>
where
    G: crate::render::RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
    graph: &'g graph::State<'b, 'c, 'e, 'g, 'o, G>,
}

impl<'b, 'c, 'e, 'g, 'o, G> BuilderGraph<'b, 'c, 'e, 'g, 'o, G>
where
    G: crate::render::RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
    fn opts<'ao>(self, opts: &'ao opts::AnimOpts) -> State<'ao, 'b, 'c, 'e, 'g, 'o, G> {
        State {
            graph: self.graph,
            opts,
        }
    }
}

#[derive(Clone, Debug)]
struct StateFrame<S> {
    state: S,
    frame: usize,
    timestamp: i32,
}

impl<S> StateFrame<S> {
    fn new(state: S, frame: usize, timestamp: i32) -> Self {
        Self {
            state,
            frame,
            timestamp,
        }
    }

    fn to_image(&self, image: RgbaImage) -> ImageFrame {
        ImageFrame {
            image,
            frame: self.frame,
            timestamp: self.timestamp,
        }
    }
}

#[derive(Clone, Debug)]
struct ImageFrame {
    image: RgbaImage,
    frame: usize,
    timestamp: i32,
}

#[derive(Clone, Debug)]
pub(crate) struct StateFrames<'ao, S> {
    frames: Vec<StateFrame<S>>,
    opts: Cow<'ao, opts::AnimOpts>,
    /// current frame
    frame: usize,
    /// the next timestamp to use
    timestamp: i32,
}

impl<'ao, 'b, 'c, 'e, 'g, 'o, S> StateFrames<'ao, S>
where
    S: RenderState<'b, 'c, 'e, 'g, 'o>,
{
    pub(crate) fn new(opts: &'ao opts::AnimOpts) -> Self {
        Self {
            frames: Vec::with_capacity(100),
            opts: Cow::Borrowed(opts),
            frame: 0,
            timestamp: 0,
        }
    }

    fn add_frames<It>(&mut self, iter: It)
    where
        It: Iterator<Item = (S, Option<std::num::NonZeroI32>)>,
    {
        for (state, duration) in iter {
            self.add(
                state,
                duration
                    .map(|d| d.get())
                    .unwrap_or_else(|| self.opts.frame_time()),
            );
        }
    }

    fn add(&mut self, state: S, duration: i32) {
        self.frames
            .push(StateFrame::new(state, self.frame, self.timestamp));
        self.timestamp += duration;
        self.frame += 1;
    }

    fn render_single_threaded(self) -> Result<WebPData, Error> {
        let mut encoder =
            Encoder::new_with_options(self.frames[0].state.size(), self.opts.encoder_options())?;

        for frame in self.frames {
            let image = frame.state.render_image();
            encoder.add_frame(&*image, frame.timestamp)?;
        }

        encoder.finalize(self.timestamp + self.opts.repeat_delay())
    }

    fn render_multi_threaded(self) -> Result<WebPData, Error>
    where
        S: Send,
    {
        use std::sync::mpsc::sync_channel;
        use std::thread;

        let len = self.frames.len();
        let mut encoder =
            Encoder::new_with_options(self.frames[0].state.size(), self.opts.encoder_options())?;

        let (tx, rx) = sync_channel(
            std::thread::available_parallelism()
                .map(std::num::NonZeroUsize::get)
                .unwrap_or(8),
        );
        thread::scope(|s| {
            for frame in self.frames {
                s.spawn(|| {
                    let frame = frame;
                    let image = frame.state.render_image();
                    tx.clone().send(frame.to_image(image)).unwrap();
                });
            }
        });

        for _ in 0..len {
            if let Ok(frame) = rx.recv() {
                let rst = encoder.add_frame(&*frame.image, frame.timestamp);
            }
        }

        encoder.finalize(self.timestamp + self.opts.repeat_delay())
    }
}
