/// Maximum possible score for a match (100% match)
pub const MAX_SCORE: u8 = 100;
/// Bonus for exact match, case-insensitive
const EXACT_MATCH_BONUS: u8 = 40;
/// Bonus for exact match, case-sensitive
const CASE_SENSITIVE_MATCH_BONUS: u8 = 100;
/// Minimum weight for position-based score
const POSITION_MIN_WEIGHT: u8 = 10;
/// Maximum weight for position-based score
const POSITION_MAX_WEIGHT: u8 = 30;
/// Maximum bonus for fragment frequency
const MAX_FREQUENCY_BONUS: u8 = 20;
/// Length penalty divisor for frequency bonus
const LENGTH_PENALTY_DIVISOR: f32 = 50.0;

/// Represents a search result with a calculated score and the original content.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SearchResults {
    pub results: Vec<SearchResult>,
}

impl SearchResults {
    /// Reduces the score of each search result by a given rate.
    ///
    /// # Parameters
    /// - `rate`: A multiplier (0.0 to 1.0) to reduce the score.
    ///
    /// # Notes
    /// - The resulting score is clamped between 0 and `MAX_SCORE`.
    /// - A negative or excessively high rate will be properly handled.
    pub fn repress(&mut self, rate: f32) {
        self.results.iter_mut().for_each(|res| res.repress(rate));
    }
}

/// Represents a search result with a calculated score and the original content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchResult {
    /// Calculated score for the match, from 0 to 100.
    pub score: u8,
    /// The original input string that contains the match.
    pub content: String,
}

impl SearchResult {
    /// Reduces the score of the search result by a given rate.
    ///
    /// # Parameters
    /// - `rate`: A multiplier (0.0 to 1.0) to reduce the score.
    ///
    /// # Notes
    /// - The resulting score is clamped between 0 and `MAX_SCORE`.
    /// - A negative or excessively high rate will be properly handled.
    pub fn repress(&mut self, rate: f32) {
        // Calculate the new score with clamping
        let adjusted_score = (self.score as f32 * rate).round() as isize;
        self.score = match adjusted_score {
            n if n >= MAX_SCORE as isize => MAX_SCORE,
            n if n <= 0 => 0,
            n => n as u8,
        };
    }
}

/// Finds matches for a given fragment in the provided inputs, ranked by relevance.
///
/// # Parameters
/// - `inputs`: A slice of strings to search through.
/// - `fragment`: The substring to search for.
///
/// # Returns
/// A `Vec<SearchResult>` containing the ranked matches, sorted in descending order of relevance.
///
/// # Example
/// ```rust
/// let inputs = vec!["Rust programming language".to_string(), "Rust".to_string(), "rusty nails".to_string()];
/// let results = find_matches(&inputs, "Rust");
/// for result in results {
///     println!("{:?}", result);
/// }
/// ```
pub fn find_matches<I: AsRef<str>, F: AsRef<str>>(inputs: &[I], fragment: F) -> SearchResults {
    let fragment_ref = fragment.as_ref();
    let mut results: Vec<SearchResult> = inputs
        .iter()
        .filter_map(|input| rank_match(input, fragment_ref))
        .collect();
    // Sort results by rank in descending order (higher scores first)
    results.sort_by(|a, b| b.score.cmp(&a.score));
    SearchResults { results }
}

pub fn rank_match<I: AsRef<str>, F: AsRef<str>>(input: I, fragment: F) -> Option<SearchResult> {
    let fragment_ref = fragment.as_ref();
    let input_ref = input.as_ref();
    let score = calculate_rank(input_ref, fragment_ref);
    if score > 0 {
        Some(SearchResult {
            score,
            content: input_ref.to_owned(),
        })
    } else {
        None
    }
}

/// Calculates the relevance score for a given input string based on the search fragment.
///
/// # Parameters
/// - `content`: The input string to score.
/// - `fragment`: The substring to search for within the input string.
///
/// # Returns
/// A score from 0 to 100, where 100 represents a perfect case-sensitive match.
///
/// # Scoring Logic
/// 1. **Case-Sensitive Match (100 points)**: Returns 100 if the content exactly matches the fragment with the same case.
/// 2. **Case-Insensitive Match (40 points)**: Adds 40 if the content matches the fragment regardless of case.
/// 3. **Position-Based Score (10-30 points)**: Scores higher if the fragment appears earlier in the content.
/// 4. **Frequency Bonus (up to 20 points)**: Adds points based on the number of fragment occurrences, reduced by a length penalty.
///
/// # Example
/// ```rust
/// let score = calculate_rank("Rust programming language", "Rust");
/// assert!(score > 0);
/// ```
fn calculate_rank(content: &str, fragment: &str) -> u8 {
    if fragment.is_empty() || content.is_empty() {
        return 0;
    }

    // 1. Exact case-sensitive match
    if content == fragment {
        return CASE_SENSITIVE_MATCH_BONUS;
    }

    let mut score = 0;
    let lower_content = content.to_lowercase();
    let lower_fragment = fragment.to_lowercase();

    // 2. Exact match bonus (case-insensitive)
    if lower_content == lower_fragment {
        score += EXACT_MATCH_BONUS;
    }

    // 3. Position-based score
    if let Some(pos) = lower_content.find(&lower_fragment) {
        let len = content.len().max(1);
        let position_weight = ((len - pos) as f32 / len as f32
            * (POSITION_MAX_WEIGHT - POSITION_MIN_WEIGHT) as f32
            + POSITION_MIN_WEIGHT as f32) as u8;
        score += position_weight.min(MAX_SCORE);
    }

    // 4. Frequency bonus (limited and with length penalty)
    if score < CASE_SENSITIVE_MATCH_BONUS {
        let frequency = lower_content.matches(&lower_fragment).count() as u8;
        let length_penalty = (content.len() as f32 / LENGTH_PENALTY_DIVISOR) as u8;
        let adjusted_bonus = (frequency * MAX_FREQUENCY_BONUS).saturating_sub(length_penalty);
        score += adjusted_bonus.min(MAX_FREQUENCY_BONUS);
    }

    // 5. Ensure the score doesn't exceed the max
    score.min(MAX_SCORE)
}
#[test]
fn test() {
    let inputs = vec![
        "Rust programming language".to_string(),
        "rust".to_string(),
        "The Rust book".to_string(),
        "rusty nails".to_string(),
        "Learning Rust".to_string(),
        "RUST".to_string(),
        "Rust".to_string(),
        "rust rust rust".to_string(),
    ];

    let results = find_matches(&inputs, "Rust");
    for result in results.results {
        println!("{:?}", result);
    }
}
