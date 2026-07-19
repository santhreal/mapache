//! Sampling helpers for `verify --sample`.

/// How many packs `--sample PCT%` should verify. `0%` and an empty pack list both yield 0.
pub(crate) fn sample_pack_count(pack_count: usize, sample_pct: f64) -> usize {
    if pack_count == 0 || sample_pct <= 0.0 {
        return 0;
    }
    let rounded = ((pack_count as f64) * (sample_pct / 100.0)).round() as usize;
    rounded.clamp(1, pack_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_zero_percent_verifies_no_packs() {
        assert_eq!(sample_pack_count(2, 0.0), 0);
        assert_eq!(sample_pack_count(10, 0.0), 0);
        assert_eq!(sample_pack_count(1, 0.0), 0);
    }

    #[test]
    fn sample_empty_pack_list_is_zero() {
        assert_eq!(sample_pack_count(0, 10.0), 0);
        assert_eq!(sample_pack_count(0, 0.0), 0);
        assert_eq!(sample_pack_count(0, 100.0), 0);
    }

    #[test]
    fn sample_positive_percent_keeps_at_least_one_when_packs_exist() {
        assert_eq!(sample_pack_count(10, 50.0), 5);
        assert_eq!(sample_pack_count(10, 100.0), 10);
        assert_eq!(sample_pack_count(10, 0.01), 1);
    }
}
