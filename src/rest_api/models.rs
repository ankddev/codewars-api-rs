//! Models that represent the data returned by the API

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a Codewars user
///
/// Read more at [Codewars documentation](https://dev.codewars.com/#get-user)
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct User {
    /// Username of the user
    pub username: String,
    /// Name of the user
    pub name: String,
    /// Total honor points earned by the user
    pub honor: u64,
    /// Name of the clan
    pub clan: String,
    /// The user's position on the overall leaderboard
    #[serde(rename = "leaderboardPosition")]
    pub leaderboard_position: u64,
    /// Array of skills entered by the user
    pub skills: Vec<String>,
    /// Ranks object with overall and language ranks
    pub ranks: Ranks,
    /// Object with fields for the number of authored and completed kata respectively
    #[serde(rename = "codeChallenges")]
    pub code_challenges: CodeChallenges,
}

/// Represents an object with fields for the number of authored and completed kata respectively
///
/// Read more at [Codewars documentation](https://dev.codewars.com/#get-user)
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Default, Clone)]
pub struct CodeChallenges {
    /// Total number of authored challenges
    #[serde(rename = "totalAuthored")]
    pub total_authored: u64,
    /// Total number of completed challenges
    #[serde(rename = "totalCompleted")]
    pub total_completed: u64,
}

/// Represents a ranks object with overall and language ranks
///
/// Read more at [Codewars documentation](https://dev.codewars.com/#get-user)
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Ranks {
    /// Overall rank
    pub overall: Rank,
    /// Ranks for each language trained
    pub languages: HashMap<String, Rank>,
}

/// Represents a rank object
///
/// Read more at [Codewars documentation](https://dev.codewars.com/#get-user)
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Rank {
    /// Rank in integer. [-8, -1] maps to kyu, [1, 8] maps to dan
    pub rank: i8,
    /// Either {-rank} kyu or {rank} dan
    pub name: String,
    /// The color of the rank. Possible colors are white (7-8 kyu), yellow (5-6 kyu), blue (3-4 kyu), purple (1-2 kyu), black (1-4 dan), and red (5-8 dan)
    pub color: Color,
    /// The total score earned. This is the number that determines the rank
    pub score: u64,
}

/// Represents a color of the rank
///
/// Read more at [Codewars documentation](https://dev.codewars.com/#get-user)
#[derive(
    Serialize,
    Deserialize,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Clone,
    strum::Display,
    strum::IntoStaticStr,
    strum::AsRefStr,
    strum::EnumString,
    strum::EnumCount,
    strum::EnumIter,
)]
pub enum Color {
    /// White (7-8 kyu)
    #[serde(rename = "white")]
    White,
    /// Yellow (5-6 kyu)
    #[serde(rename = "yellow")]
    Yellow,
    /// Blue (3-4 kyu)
    #[serde(rename = "blue")]
    Blue,
    /// Purple (1-2 kyu)
    #[serde(rename = "purple")]
    Purple,
    /// Black (1-4 dan)
    #[serde(rename = "black")]
    Black,
    /// Red (5-8 dan)
    #[serde(rename = "red")]
    Red,
}


/// Represents an authored challenge
///
/// Read more at [Codewars documentation](https://dev.codewars.com/#list-authored-challenges)
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Default, Clone)]
pub struct AuthoredChallenge {
    /// ID of the kata
    pub id: String,
    /// Name of the kata
    pub name: String,
    /// Description of the kata in Markdown
    pub description: String,
    /// Rank of the kata if approved
    #[serde(flatten)]
    pub rank: Option<i8>,
    /// Rank name of the kata if approved
    #[serde(rename = "rankName", flatten)]
    pub rank_name: Option<String>,
    /// Array of tags associated with the kata
    pub tags: Vec<String>,
    /// Array of language names the kata is available in
    pub languages: Vec<String>,
}

/// Represents list of authored challenges
///
/// Read more at [Codewars documentation](https://dev.codewars.com/#list-authored-challenges)
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Default, Clone)]
pub struct AuthoredChallenges {
    /// Vector of authored challenges
    pub data: Vec<AuthoredChallenge>,
}

/// Represents a code challenge
///
/// Read more at [Codewars documentation](https://dev.codewars.com/#get-code-challenge)
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Default, Clone)]
pub struct CodeChallenge {
    /// ID of the kata
    pub id: String,
    /// Name of the kata
    pub name: String,
    /// Slug of the kata
    pub slug: String,
    /// URL of the kata
    pub url: String,
    /// Category of the kata
    pub category: String,
    /// Description of the kata in Markdown
    pub description: String,
    /// Array of tags associated with the kata
    pub tags: Vec<String>,
    /// Array of language names the kata is available in
    pub languages: Vec<String>,
    /// Object describing the rank of the kata if approved
    #[serde(flatten)]
    pub rank: Option<Rank>,
    /// The author of the kata
    #[serde(rename = "createdBy")]
    pub created_by: Author,
    /// Date and time when the kata was first published
    #[serde(rename = "publishedAt")]
    pub published_at: String,
    /// The approver of the kata
    #[serde(rename = "approvedBy", flatten)]
    pub approved_by: Option<Author>,
    /// Date and time when the kata was approved
    #[serde(rename = "approvedAt")]
    pub approved_at: String,
    /// Total number of completions
    #[serde(rename = "totalCompleted")]
    pub total_completed: u64,
    /// Total number of attempts
    #[serde(rename = "totalAttempts")]
    pub total_attempts: u64,
    /// The number of bookmarks
    #[serde(rename = "totalStars")]
    pub total_stars: u64,
    /// The sum of all votes casted
    #[serde(rename = "voteScore")]
    pub vote_score: u64,
    /// Whether to allow contributions
    #[serde(rename = "contributorsWanted")]
    pub contributors_wanted: bool,
    /// Object with fields for the number of unresolved issues and suggestions respectively
    pub unresolved: Unresolved,
}

/// Represents an author of code challenge
///
/// Read more at [Codewars documentation](https://dev.codewars.com/#get-code-challenge)
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Default, Clone)]
pub struct Author {
    /// Username of the user
    pub username: String,
    /// URL of the user's profile
    pub url: String,
}

/// Represents an object with fields for the number of unresolved issues and suggestions respectively
///
/// Read more at [Codewars documentation](https://dev.codewars.com/#get-code-challenge)
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Default, Clone)]
pub struct Unresolved {
    /// Number of unresolved issues
    pub issues: u64,
    /// Number of unresolved suggestions
    pub suggestions: u64,
}

/// Represents a list of completed code challenges
///
/// Read more at [Codewars documentation](https://dev.codewars.com/#list-completed-challenges)
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Default, Clone)]
pub struct CompletedChallenges {
    /// Total number of pages
    #[serde(rename = "totalPages")]
    pub total_pages: u64,
    /// Total number of items
    #[serde(rename = "totalItems")]
    pub total_items: u64,
    /// Array of completed code challenges
    pub data: Vec<CompletedChallenge>,
}

/// Represents a completed code challenge
///
/// Read more at [Codewars documentation](https://dev.codewars.com/#list-completed-challenges)
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Default, Clone)]
pub struct CompletedChallenge {
    /// ID of the kata
    pub id: String,
    /// Name of the kata
    pub name: String,
    /// Slug of the kata
    pub slug: String,
    /// Date and time of the completion
    #[serde(rename = "completedAt")]
    pub completed_at: String,
    /// Array of languages a kata completed in
    #[serde(rename = "completedLanguages")]
    pub completed_languages: Vec<String>,
}
