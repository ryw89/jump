use fuzzywuzzy::fuzz::token_sort_ratio;

use crate::db::Dir;

/// Score a query against a directory
pub fn score(args: &[String], dir: &Dir) -> u32 {
    let fuzz_score = token_sort_ratio(&args.join(" "), &dir.dir.replace('/', " "), true, true);
    fuzz_score.into()
}
