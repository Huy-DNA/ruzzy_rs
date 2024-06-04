#[derive(Clone, Copy)]
pub struct FuzzyConfig {
    threshold: usize,
    insertion_penalty: Option<usize>,
    deletion_penalty: Option<usize>,
    substitution_penalty: Option<usize>,
}

pub fn fuzzy_match<'a, Value: ?Sized>(needle: &'a String, haystack: &'a Vec<(String, &'a Value)>, config: FuzzyConfig) -> Option<&'a Value> {
    let mut res = None;
    let mut threshold = config.threshold;
    for hay in haystack {
        let check_res = check(needle, &hay.0, FuzzyConfig { threshold, ..config });
        if check_res.is_some() {
            let check_res = check_res.unwrap();
            threshold = check_res.1;
            res = Some(hay.1);
        }
    }
    res
}

fn check<'a>(needle: &'a String, candidate: &'a String, config: FuzzyConfig) -> Option<(&'a String, usize)> {
    if needle.len() == 0 || candidate.len() == 0 {
        return None;
    }

    let FuzzyConfig {
        threshold,
        insertion_penalty,
        deletion_penalty,
        substitution_penalty,
    } = config;
    let insertion_penalty = insertion_penalty.unwrap_or(1);
    let deletion_penalty = deletion_penalty.unwrap_or(1);
    let substitution_penalty = substitution_penalty.unwrap_or(2);
    
    let mut prev_row = vec![];
    prev_row.push(0);
    for i in 1..=needle.len() {
        prev_row.push(prev_row[i - 1] + deletion_penalty);
    }

    let mut cur_row = vec![];
    
    for candidate_c in candidate.chars() {
        cur_row.push(prev_row[0] + insertion_penalty);
        for (needle_i, needle_c) in needle.chars().enumerate() {
            let min_cost = 
                if needle_c == candidate_c {
                    *prev_row.get(needle_i).unwrap_or(&threshold)
                } else {
                    [
                        *cur_row.get(needle_i).unwrap_or(&threshold) + deletion_penalty,
                        *prev_row.get(needle_i + 1).unwrap_or(&threshold) + insertion_penalty,
                        *prev_row.get(needle_i).unwrap_or(&threshold) + substitution_penalty,
                    ].into_iter().min().unwrap()
            };
            cur_row.push(min_cost);
        }
        prev_row = cur_row;
        cur_row = vec![];
    }

    prev_row.last()
            .and_then(|i| if *i <= threshold { Some((candidate, *i)) } else { None })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact() {
        let needle = "hello".to_string();
        let haystack = vec![
            ("hallo".to_string(), "this is wrong"),
            ("hello".to_string(), "this is true"),
            ("hell".to_string(), "this is wrong"),
        ];
        let config = FuzzyConfig {
            threshold: 2,
            insertion_penalty: None,
            deletion_penalty: None,
            substitution_penalty: None,
        };
        let result = fuzzy_match(&needle, &haystack, config);
        assert_eq!(result, Some("this is true"));
    }

    #[test]
    fn test_near() {
        let needle = "youtube".to_string();
        let haystack = vec![
            ("you".to_string(), "this is wrong"),
            ("yotube".to_string(), "this is true"),
            ("otube".to_string(), "this is wrong"),
        ];
        let config = FuzzyConfig {
            threshold: 2,
            insertion_penalty: None,
            deletion_penalty: None,
            substitution_penalty: None,
        };
        let result = fuzzy_match(&needle, &haystack, config);
        assert_eq!(result, Some("this is true"));
    }

    #[test]
    fn test_near_ambiguous() {
        let needle = "youtube".to_string();
        let haystack = vec![
            ("you".to_string(), "this is wrong"),
            ("yotube".to_string(), "this is true 1"),
            ("outube".to_string(), "this is true 2"),
        ];
        let config = FuzzyConfig {
            threshold: 2,
            insertion_penalty: None,
            deletion_penalty: None,
            substitution_penalty: None,
        };
        let result = fuzzy_match(&needle, &haystack, config);
        assert_eq!(result, Some("this is true 2"));
    }

    #[test]
    fn test_substitution() {
        let needle = "youtube".to_string();
        let haystack = vec![
            ("routube".to_string(), "this is true"),
            ("ytub".to_string(), "this is wrong"),
            ("out".to_string(), "this is wrong"),
        ];
        let config = FuzzyConfig {
            threshold: 2,
            insertion_penalty: None,
            deletion_penalty: None,
            substitution_penalty: None,
        };
        let result = fuzzy_match(&needle, &haystack, config);
        assert_eq!(result, Some("this is true"));
    }

    #[test]
    fn test_substitution_tied_with_insertion() {
        let needle = "youtube".to_string();
        let haystack = vec![
            ("routube".to_string(), "this is true 1"),
            ("ytube".to_string(), "this is true 2"),
            ("oe".to_string(), "this is wrong"),
        ];
        let config = FuzzyConfig {
            threshold: 2,
            insertion_penalty: None,
            deletion_penalty: None,
            substitution_penalty: None,
        };
        let result = fuzzy_match(&needle, &haystack, config);
        assert_eq!(result, Some("this is true 2"));
    }
}
