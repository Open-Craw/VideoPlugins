/// A sound as the plugins listen to it: one channel, samples between -1 and 1.
#[derive(Debug, Clone, PartialEq)]
pub struct Sound {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
}

impl Sound {
    /// How long it lasts, in whole milliseconds.
    pub fn duration_ms(&self) -> u64 {
        if self.sample_rate == 0 {
            return 0;
        }
        (self.samples.len() as u64 * 1000) / u64::from(self.sample_rate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lasts_as_many_milliseconds_as_its_samples_take_at_its_rate() {
        let sound = Sound {
            samples: vec![0.0; 22_050],
            sample_rate: 44_100,
        };
        assert_eq!(sound.duration_ms(), 500);
        assert_eq!(
            Sound {
                samples: vec![],
                sample_rate: 44_100
            }
            .duration_ms(),
            0
        );
        assert_eq!(
            Sound {
                samples: vec![0.0; 5],
                sample_rate: 0
            }
            .duration_ms(),
            0
        );
    }
}
