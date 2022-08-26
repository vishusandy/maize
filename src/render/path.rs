use super::state::GraphState;
use super::RenderGraph;
use crate::graphs::Node;

#[derive(Clone, Debug)]
struct PathState<'b, 'c, 'e, 'g, 'o, 'p, 'po, 'r, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
    _state: std::borrow::Cow<'r, GraphState<'b, 'c, 'e, 'g, 'o, G>>,
    _path: std::borrow::Cow<'p, ()>,
    _path_opts: std::borrow::Cow<'po, ()>,
}
