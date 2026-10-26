use std::collections::BTreeMap;

use super::appearance_contract::{Appearance, Sample};

/// Joins what was found picture by picture into appearances, for each class: the stretch from the
/// picture where it first shows to the last one it is in. A class that is out of the pictures for
/// more than `gap_ms` (counted from the last picture it was in) has left, and when it comes back
/// that is a new appearance. A stretch seen in fewer than `min_samples` pictures was a flicker and
/// is dropped.
///
/// `samples` must be in time order. The appearances come back in the order they start.
pub fn track_appearances(samples: &[Sample], gap_ms: u64, min_samples: usize) -> Vec<Appearance> {
    let mut open: BTreeMap<usize, (Appearance, u64)> = BTreeMap::new();
    let mut done: Vec<Appearance> = Vec::new();
    for sample in samples {
        let mut classes: Vec<usize> = sample.found.iter().map(|found| found.class).collect();
        classes.sort_unstable();
        classes.dedup();
        for class in classes {
            let here: Vec<_> = sample
                .found
                .iter()
                .filter(|found| found.class == class)
                .copied()
                .collect();
            let best = here.iter().map(|found| found.score).fold(0.0, f32::max);
            let end_ms = sample.time_ms + sample.duration_ms;
            if let Some((current, last_seen)) = open.get_mut(&class) {
                if sample.time_ms.saturating_sub(*last_seen) <= gap_ms {
                    current.end_ms = end_ms;
                    current.confidence = current.confidence.max(best);
                    current.peak_count = current.peak_count.max(here.len());
                    current.samples += 1;
                    *last_seen = sample.time_ms;
                    continue;
                }
                done.push(current.clone());
            }
            open.insert(
                class,
                (
                    Appearance {
                        class,
                        first_frame: sample.frame,
                        start_ms: sample.time_ms,
                        end_ms,
                        confidence: best,
                        peak_count: here.len(),
                        samples: 1,
                        first_boxes: here,
                    },
                    sample.time_ms,
                ),
            );
        }
    }
    done.extend(open.into_values().map(|(appearance, _)| appearance));
    done.retain(|appearance| appearance.samples >= min_samples);
    done.sort_by_key(|appearance| (appearance.start_ms, appearance.class));
    done
}

#[cfg(test)]
mod tests {
    use crate::detect::Detection;

    use super::*;

    fn found(class: usize, score: f32) -> Detection {
        Detection {
            class,
            score,
            x1: 0.0,
            y1: 0.0,
            x2: 1.0,
            y2: 1.0,
        }
    }

    /// A picture every 500 ms, with the classes (and scores) found in each.
    fn timeline(pictures: &[&[(usize, f32)]]) -> Vec<Sample> {
        pictures
            .iter()
            .enumerate()
            .map(|(index, classes)| Sample {
                frame: index,
                time_ms: index as u64 * 500,
                duration_ms: 500,
                found: classes
                    .iter()
                    .map(|(class, score)| found(*class, *score))
                    .collect(),
            })
            .collect()
    }

    #[test]
    fn a_cat_that_stays_is_one_appearance_that_starts_at_its_first_picture() {
        let samples = timeline(&[
            &[],
            &[(15, 0.6)],
            &[(15, 0.9)],
            &[(15, 0.7), (15, 0.5)],
            &[],
        ]);
        let found = track_appearances(&samples, 2000, 1);

        assert_eq!(found.len(), 1);
        let cat = &found[0];
        assert_eq!((cat.class, cat.first_frame, cat.start_ms), (15, 1, 500));
        assert_eq!(cat.end_ms, 2000, "the last picture it is in ends at 2000");
        assert_eq!((cat.samples, cat.peak_count), (3, 2));
        assert_eq!(cat.confidence, 0.9);
        assert_eq!(cat.first_boxes.len(), 1);
    }

    #[test]
    fn a_cat_that_leaves_for_long_and_comes_back_is_a_second_appearance() {
        // In at 0, gone from 500 to 2500 (more than 1000), back at 3000.
        let samples = timeline(&[&[(15, 0.8)], &[], &[], &[], &[], &[], &[(15, 0.8)]]);
        let found = track_appearances(&samples, 1000, 1);
        let starts: Vec<u64> = found.iter().map(|a| a.start_ms).collect();
        assert_eq!(starts, [0, 3000]);

        // With a longer gap the same video is one appearance.
        assert_eq!(track_appearances(&samples, 5000, 1).len(), 1);
    }

    #[test]
    fn a_flicker_is_dropped_and_each_class_is_followed_on_its_own() {
        let samples = timeline(&[&[(15, 0.8), (0, 0.9)], &[(0, 0.9)], &[(0, 0.9)]]);
        let found = track_appearances(&samples, 2000, 2);
        assert_eq!(found.len(), 1, "the cat was in one picture only");
        assert_eq!(found[0].class, 0);
        assert_eq!(found[0].samples, 3);
    }

    #[test]
    fn nothing_found_gives_no_appearances() {
        assert!(track_appearances(&timeline(&[&[], &[]]), 2000, 1).is_empty());
        assert!(track_appearances(&[], 2000, 1).is_empty());
    }
}
