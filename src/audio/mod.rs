pub trait Audio {
    fn sample(&self) -> f32;
}

pub struct NoAudio;

impl Audio for NoAudio {
    fn sample(&self) -> f32 {
        0.0
    }
}
