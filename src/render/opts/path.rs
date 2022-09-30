use super::{Colors, Size, Text};

pub(crate) struct Opts {
    size: Size,
    color: Colors,
    text: Text,
    path: PathOpts,
}

pub(crate) struct PathOpts {
    show_arrows: bool,
    path_color: image::Rgba<u8>,
    path_bg: image::Rgba<u8>,
}
