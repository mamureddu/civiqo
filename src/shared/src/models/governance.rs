use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Poll {
    pub id: Uuid,
    pub community_id: Uuid,
    pub created_by: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub poll_type: PollType,
    pub options: serde_json::Value, // JSON array of options
    pub settings: serde_json::Value, // Poll configuration
    pub status: PollStatus,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "poll_type", rename_all = "snake_case")]
pub enum PollType {
    SingleChoice,
    MultipleChoice,
    RankedChoice,
    YesNo,
    Rating,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "poll_status", rename_all = "snake_case")]
pub enum PollStatus {
    Draft,
    Active,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Vote {
    pub id: Uuid,
    pub poll_id: Uuid,
    pub user_id: Uuid,
    pub vote_data: serde_json::Value, // Encrypted vote data
    pub vote_hash: String, // For verification without revealing vote
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Decision {
    pub id: Uuid,
    pub community_id: Uuid,
    pub created_by: Uuid,
    pub title: String,
    pub description: String,
    pub decision_type: DecisionType,
    pub status: DecisionStatus,
    pub decision_makers: serde_json::Value, // List of user IDs or roles
    pub deadline: Option<DateTime<Utc>>,
    pub result: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "decision_type", rename_all = "snake_case")]
pub enum DecisionType {
    Simple,
    Consensus,
    Majority,
    SuperMajority,
    Unanimous,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "decision_status", rename_all = "snake_case")]
pub enum DecisionStatus {
    Pending,
    InProgress,
    Approved,
    Rejected,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DecisionVote {
    pub id: Uuid,
    pub decision_id: Uuid,
    pub user_id: Uuid,
    pub vote: DecisionVoteType,
    pub comment: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "decision_vote_type", rename_all = "snake_case")]
pub enum DecisionVoteType {
    Approve,
    Reject,
    Abstain,
}

// Request/Response types
#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePollRequest {
    pub title: String,
    pub description: Option<String>,
    pub poll_type: PollType,
    pub options: Vec<String>,
    pub settings: PollSettings,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PollSettings {
    pub anonymous: bool,
    pub allow_multiple: bool,
    pub max_choices: Option<i32>,
    pub required_role: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CastVoteRequest {
    pub choices: Vec<String>, // For multiple choice or ranked
    pub choice: Option<String>, // For single choice
    pub rating: Option<i32>, // For rating polls
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDecisionRequest {
    pub title: String,
    pub description: String,
    pub decision_type: DecisionType,
    pub decision_makers: Vec<Uuid>,
    pub deadline: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionVoteRequest {
    pub vote: DecisionVoteType,
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PollWithResults {
    #[serde(flatten)]
    pub poll: Poll,
    pub total_votes: i64,
    pub results: serde_json::Value,
    pub user_voted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionWithVotes {
    #[serde(flatten)]
    pub decision: Decision,
    pub votes: Vec<DecisionVote>,
    pub can_vote: bool,
    pub user_vote: Option<DecisionVoteType>,
}