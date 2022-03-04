use fuzzywuzzy::fuzz::token_sort_ratio;

use crate::db::Dir;

/// Score a query against a directory
pub fn score(args: &[String], dir: &Dir) -> u32 {
    let normalized_dir = normalize_directory(args, &dir.dir);
    let fuzz_score = token_sort_ratio(&args.join(" "), &normalized_dir, true, true);
    fuzz_score.into()
}

/// Normalize a directory.
///
/// The idea here is that queries are likely to be for the end bits of
/// a directory.
fn normalize_directory(args: &[String], dir: &str) -> String {
    let num_terms = args.len();
    let dir_split: Vec<String> = dir
        .replace('/', " ")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let mut slice_len = dir_split.len() - num_terms;
    if slice_len > dir_split.len() {
        slice_len = dir_split.len();
    }

    dir_split[(slice_len)..].join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn dir_maker(dir: &str) -> Dir {
        Dir {
            id: 1,
            dir: dir.to_string(),
            access_count: 0,
            last_accessed: 0,
        }
    }

    /// Confirm queries against various target directories are ordered
    /// appropriately. This is for preserving the desired ordering
    /// should the scoring implementation change.
    #[test]
    fn order_queries() {
        let target_dir = dir_maker("/home/ryanw/Dropbox/Projects");

        let queries = vec!["drop proj".to_string(), "proj".into()];

        let mut scores: HashMap<String, u32> = HashMap::new();
        for q in queries {
            let args: Vec<String> = q.split_whitespace().map(|s| s.to_string()).collect();
            let s = score(&args, &target_dir);
            println!("Score for '{}': {}", q, s);
            scores.insert(q, s);
        }

        // Various comparisons -- This is the 'meat' of the test
        assert!(scores.get("drop proj") > scores.get("proj"));
    }
}
