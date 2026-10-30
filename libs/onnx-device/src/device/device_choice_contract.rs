/// Where the user wants a model to run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum DeviceChoice {
    /// The GPU when there is a discrete one, the CPU otherwise.
    #[default]
    Auto,
    Cpu,
    /// The best GPU, even an integrated one (`None`), or the adapter with this number.
    Gpu(Option<u32>),
}

impl DeviceChoice {
    /// Reads `auto`, `cpu`, `gpu` or `gpu:N` (N from 0), in any case. `None` when it is anything
    /// else, so a plugin can answer with a 400.
    pub fn parse(text: &str) -> Option<DeviceChoice> {
        let text = text.trim().to_lowercase();
        match text.as_str() {
            "auto" => Some(DeviceChoice::Auto),
            "cpu" => Some(DeviceChoice::Cpu),
            "gpu" => Some(DeviceChoice::Gpu(None)),
            other => other
                .strip_prefix("gpu:")
                .and_then(|number| number.parse().ok())
                .map(|number| DeviceChoice::Gpu(Some(number))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_words_a_user_may_give_are_read_and_anything_else_is_refused() {
        assert_eq!(DeviceChoice::parse("auto"), Some(DeviceChoice::Auto));
        assert_eq!(DeviceChoice::parse(" CPU "), Some(DeviceChoice::Cpu));
        assert_eq!(DeviceChoice::parse("gpu"), Some(DeviceChoice::Gpu(None)));
        assert_eq!(
            DeviceChoice::parse("GPU:1"),
            Some(DeviceChoice::Gpu(Some(1)))
        );
        assert_eq!(DeviceChoice::parse("gpu:x"), None);
        assert_eq!(DeviceChoice::parse("cuda"), None);
    }
}
