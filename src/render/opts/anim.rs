use webp_animation::{EncoderOptions, EncodingConfig, EncodingType, LossyEncodingConfig};

#[derive(Clone, Debug)]
pub struct AnimOpts {
    frame_time: i32,
    /// how long the last frame should be displayed before repeating
    repeat_delay: i32,
    lossy: bool,
    /// Between 0 and 100. For lossy, 0 gives the smallest size and 100 the largest. For lossless, this parameter is the amount of effort put into the compression: 0 is the fastest but gives larger files compared to the slowest, but best, 100.
    quality: f32,
    /// Quality/speed trade-off (0=fast, 6=slower-better)
    encode_method: usize,
}

impl Default for AnimOpts {
    fn default() -> Self {
        Self {
            frame_time: 250,
            repeat_delay: 2500,
            lossy: false,
            quality: 0.00,
            encode_method: 0,
        }
    }
}

impl AnimOpts {
    pub fn frame_time(&self) -> i32 {
        self.frame_time
    }

    pub fn repeat_delay(&self) -> i32 {
        self.repeat_delay
    }

    pub fn encoding_type(&self) -> EncodingType {
        match self.lossy {
            false => EncodingType::Lossless,
            true => EncodingType::Lossy(LossyEncodingConfig::default()),
        }
    }

    pub fn encoder_options(&self) -> EncoderOptions {
        let config = EncodingConfig {
            encoding_type: self.encoding_type(),
            quality: self.quality,
            method: self.encode_method,
        };

        EncoderOptions {
            encoding_config: Some(config),
            ..EncoderOptions::default()
        }
    }
}
