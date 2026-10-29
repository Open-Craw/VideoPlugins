use super::correlate_algorithm::correlate;

/// How much the delay changes from the start to the end of the stretch the two sounds share, in
/// milliseconds: 0 for a copy at the same speed, more when the video was sped up or slowed down.
/// `delay` is the delay found over the whole length. It listens to `window` samples at each end
/// and looks `search` samples either side of `delay`. `None` when the shared stretch is too short
/// for two windows or a window cannot be told.
pub fn measure_drift_ms(
    video: &[f32],
    music: &[f32],
    delay: isize,
    sample_rate: u32,
    window: usize,
    search: usize,
) -> Option<f64> {
    let start = delay.max(0) as usize;
    let end = ((delay + music.len() as isize).min(video.len() as isize)).max(0) as usize;
    if end < start + 2 * window {
        return None;
    }
    let local = |video_from: usize| -> Option<isize> {
        let slice = &video[video_from..video_from + window];
        let music_from = (video_from as isize - delay) as usize;
        let tune = &music[music_from..music_from + window];
        let scores = correlate(slice, tune);
        let first = -(tune.len() as isize - 1);
        scores
            .iter()
            .enumerate()
            .map(|(i, &score)| (first + i as isize, score))
            .filter(|(lag, _)| lag.unsigned_abs() <= search)
            .max_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(lag, _)| lag)
    };
    let early = local(start)?;
    let late = local(end - window)?;
    Some((late - early) as f64 * 1000.0 / f64::from(sample_rate))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Notes with a drum-like decaying noise on each: pure tones alone leave the timing ambiguous.
    fn tune(length: usize) -> Vec<f32> {
        let mut state = 3u32;
        let mut note = 200.0f32;
        let mut phase = 0.0f32;
        (0..length)
            .map(|i| {
                state = state.wrapping_mul(1664525).wrapping_add(1013904223);
                let hiss = (state >> 8) as f32 / (1u32 << 24) as f32 - 0.5;
                if i % 1500 == 0 {
                    note = 150.0 + (state >> 20) as f32 % 900.0;
                }
                phase += note / 8000.0 * std::f32::consts::TAU;
                let decay = (-((i % 1500) as f32) / 300.0).exp();
                phase.sin() * 0.3 + hiss * decay
            })
            .collect()
    }

    #[test]
    fn a_copy_at_the_same_speed_does_not_drift() {
        let music = tune(160_000);
        let mut video = vec![0.0; 4_000];
        video.extend_from_slice(&music);
        let drift = measure_drift_ms(&video, &music, 4_000, 8000, 16_000, 800).unwrap();
        assert_eq!(drift, 0.0);
    }

    #[test]
    fn a_video_one_percent_slower_drifts_by_a_percent_of_its_length() {
        let music = tune(160_000);
        // The video plays the music 1% slower: sample n of the video is music sample n / 1.01.
        let slow: Vec<f32> = (0..(music.len() as f64 * 1.01) as usize)
            .map(|n| music[((n as f64 / 1.01) as usize).min(music.len() - 1)])
            .collect();
        let drift = measure_drift_ms(&slow, &music, 0, 8000, 16_000, 2_400).unwrap();
        // Over 20 s that is 200 ms less the window, about 180 ms.
        assert!(drift > 100.0 && drift < 250.0, "drift {drift}");
    }

    #[test]
    fn a_stretch_too_short_for_two_windows_cannot_be_measured() {
        let music = tune(20_000);
        assert_eq!(measure_drift_ms(&music, &music, 0, 8000, 16_000, 800), None);
    }
}
