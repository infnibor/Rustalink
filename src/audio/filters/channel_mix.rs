use super::AudioFilter;

pub struct ChannelMixFilter {
    left_to_left: f32,
    left_to_right: f32,
    right_to_left: f32,
    right_to_right: f32,
}

impl ChannelMixFilter {
    pub fn new(
        left_to_left: f32,
        left_to_right: f32,
        right_to_left: f32,
        right_to_right: f32,
    ) -> Self {
        Self {
            left_to_left: left_to_left.clamp(0.0, 1.0),
            left_to_right: left_to_right.clamp(0.0, 1.0),
            right_to_left: right_to_left.clamp(0.0, 1.0),
            right_to_right: right_to_right.clamp(0.0, 1.0),
        }
    }
}

impl AudioFilter for ChannelMixFilter {
    fn process(&mut self, samples: &mut [i16]) {
        let num_frames = samples.len() / 2;

        for frame in 0..num_frames {
            let offset = frame * 2;
            let left = samples[offset] as f32;
            let right = samples[offset + 1] as f32;

            let new_left = left * self.left_to_left + right * self.right_to_left;
            let new_right = left * self.left_to_right + right * self.right_to_right;

            // Clamp in f32 before truncating to i16 to avoid overflow.
            samples[offset] = new_left.clamp(i16::MIN as f32, i16::MAX as f32) as i16;
            samples[offset + 1] = new_right.clamp(i16::MIN as f32, i16::MAX as f32) as i16;
        }
    }

    fn is_enabled(&self) -> bool {
        (self.left_to_left - 1.0).abs() > f32::EPSILON
            || self.left_to_right.abs() > f32::EPSILON
            || self.right_to_left.abs() > f32::EPSILON
            || (self.right_to_right - 1.0).abs() > f32::EPSILON
    }

    fn reset(&mut self) {}
}
