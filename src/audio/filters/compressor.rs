use super::AudioFilter;

fn db_to_gain(db: f32) -> f32 {
    10f32.powf(db / 20.0)
}

fn gain_to_db(gain: f32) -> f32 {
    20.0 * gain.max(1e-10).log10()
}

pub struct CompressorFilter {
    threshold: f32,
    ratio: f32,
    makeup_gain: f32,

    envelope: f32,

    // Precomputed per-parameter coefficients (recomputed only on update).
    attack_coef: f32,
    release_coef: f32,
}

impl CompressorFilter {
    pub fn new(threshold: f32, ratio: f32, attack: f32, release: f32, makeup_gain: f32) -> Self {
        let attack = attack.max(0.001);
        let release = release.max(0.01);
        Self {
            threshold,
            ratio: ratio.max(1.0),
            makeup_gain,
            envelope: 0.0,
            attack_coef: (-1.0 / (attack * 48000.0)).exp(),
            release_coef: (-1.0 / (release * 48000.0)).exp(),
        }
    }
}

impl AudioFilter for CompressorFilter {
    fn process(&mut self, samples: &mut [i16]) {
        let makeup_gain = db_to_gain(self.makeup_gain);

        for chunk in samples.chunks_exact_mut(2) {
            let left_in = chunk[0] as f32 / 32768.0;
            let right_in = chunk[1] as f32 / 32768.0;

            let abs_sample = left_in.abs().max(right_in.abs());

            if abs_sample > self.envelope {
                self.envelope = self.attack_coef * (self.envelope - abs_sample) + abs_sample;
            } else {
                self.envelope = self.release_coef * (self.envelope - abs_sample) + abs_sample;
            }

            let envelope_db = gain_to_db(self.envelope);
            let mut reduction_db = 0.0;

            if envelope_db > self.threshold {
                reduction_db = (self.threshold - envelope_db) * (1.0 - 1.0 / self.ratio);
            }

            let gain = db_to_gain(reduction_db) * makeup_gain;

            chunk[0] = (left_in * gain * 32768.0).clamp(i16::MIN as f32, i16::MAX as f32) as i16;
            chunk[1] = (right_in * gain * 32768.0).clamp(i16::MIN as f32, i16::MAX as f32) as i16;
        }
    }

    fn is_enabled(&self) -> bool {
        self.threshold < 0.0 || self.ratio > 1.0 || self.makeup_gain != 0.0
    }

    fn reset(&mut self) {
        self.envelope = 0.0;
    }
}
