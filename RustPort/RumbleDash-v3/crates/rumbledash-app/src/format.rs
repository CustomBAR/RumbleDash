//! Number formatting shared by the dashboard.
//!
//! The overlays show the goal label in a compact form, and the app has to
//! describe the same value the same way, so both go through [`compact`] here.

/// Formats a number with `,` as the thousands separator: `1,234,567`.
pub fn number(value: u64) -> String {
    let digits = value.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);

    for (index, byte) in digits.bytes().enumerate() {
        if index > 0 && (digits.len() - index) % 3 == 0 {
            out.push(',');
        }
        out.push(byte as char);
    }

    out
}

/// Formats a goal the way the overlay labels read: `999`, `1k`, `1.5k`, `1.2M`.
///
/// The value is truncated at one decimal place instead of rounded, so a goal of
/// `1,999` reads `1.9k` rather than claiming it has already been reached. The
/// decimal is dropped when it is zero, and the unit only changes at exactly
/// `1,000`, `1,000,000`, and so on – `999,999` stays `999.9k`.
pub fn compact(value: u64) -> String {
    const UNITS: [(u64, &str); 4] = [
        (1_000_000_000_000, "T"),
        (1_000_000_000, "B"),
        (1_000_000, "M"),
        (1_000, "k"),
    ];

    for (divisor, suffix) in UNITS {
        if value >= divisor {
            // Work in tenths so the truncation happens exactly, without floats.
            let tenths = value / (divisor / 10);
            let whole = tenths / 10;
            let fraction = tenths % 10;

            return if fraction == 0 {
                format!("{whole}{suffix}")
            } else {
                format!("{whole}.{fraction}{suffix}")
            };
        }
    }

    value.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_get_thousands_separators() {
        assert_eq!(number(0), "0");
        assert_eq!(number(999), "999");
        assert_eq!(number(1_000), "1,000");
        assert_eq!(number(1_234_567), "1,234,567");
    }

    #[test]
    fn goals_below_a_thousand_stay_exact() {
        assert_eq!(compact(0), "0");
        assert_eq!(compact(1), "1");
        assert_eq!(compact(999), "999");
    }

    #[test]
    fn goals_are_compact_from_a_thousand_up() {
        assert_eq!(compact(1_000), "1k");
        assert_eq!(compact(1_050), "1k");
        assert_eq!(compact(1_500), "1.5k");
        assert_eq!(compact(1_999), "1.9k");
        assert_eq!(compact(12_500), "12.5k");
        assert_eq!(compact(999_999), "999.9k");
    }

    #[test]
    fn goals_roll_over_to_the_next_unit() {
        assert_eq!(compact(1_000_000), "1M");
        assert_eq!(compact(1_500_000), "1.5M");
        assert_eq!(compact(2_000_000_000), "2B");
        assert_eq!(compact(3_400_000_000_000), "3.4T");
    }
}
