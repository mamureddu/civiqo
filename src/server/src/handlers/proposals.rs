//! Proposals handlers for governance system
//! Handles CRUD operations for proposals and voting

use crate::auth::AuthUser;
use crate::handlers::pages::{AppError, AppState};
use axum::{
    extract::{Path, Query, State},
    response::Html,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::sync::Arc;
use uuid::Uuid;

// ======================================================================
// REQUEST/RESPONSE STRUCTS
// ======================================================================

#[derive(Debug, Deserialize)]
pub struct CreateProposalRequest {
    pub community_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub proposal_type: Option<String>,
    pub voting_starts_at: Option<String>,
    pub voting_ends_at: Option<String>,
    pub quorum_required: Option<i32>,
    pub options: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct ProposalResponse {
    pub id: String,
    pub community_id: String,
    pub community_name: String,
    pub title: String,
    pub description: Option<String>,
    pub proposal_type: String,
    pub status: String,
    pub vote_count: i64,
    pub author_name: String,
    pub created_at: String,
    pub voting_starts_at: Option<String>,
    pub voting_ends_at: Option<String>,
    pub quorum_required: i32,
}

#[derive(Debug, Deserialize)]
pub struct CastVoteRequest {
    pub vote_value: String, // "yes", "no", "abstain" or option_id for polls
}

#[derive(Debug, Deserialize, Default)]
#[allow(dead_code)]
pub struct ProposalsQuery {
    pub community_id: Option<Uuid>,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct VoteResultsResponse {
    pub proposal_id: String,
    pub total_votes: i64,
    pub results: Vec<VoteOption>,
    pub quorum_required: i32,
    pub quorum_met: bool,
}

#[derive(Debug, Serialize)]
pub struct VoteOption {
    pub value: String,
    pub count: i64,
    pub percentage: f64,
}

// ======================================================================
// API HANDLERS
// ======================================================================

/// List proposals with optional filters
/// GET /api/proposals
pub async fn list_proposals(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ProposalsQuery>,
) -> Result<Json<Vec<ProposalResponse>>, AppError> {
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.page.unwrap_or(0) * limit;

    let proposals = if let Some(community_id) = params.community_id {
        sqlx::query(
            r#"SELECT p.id, p.community_id, p.title, p.description, p.proposal_type, 
                      p.status, p.voting_starts_at, p.voting_ends_at, p.created_at,
                      p.quorum_required,
                      c.name as community_name, 
                      COALESCE(up.name, u.email) as author_name,
                      (SELECT COUNT(*) FROM votes v WHERE v.proposal_id = p.id) as vote_count
               FROM proposals p
               JOIN communities c ON p.community_id = c.id
               JOIN users u ON p.created_by = u.id
               LEFT JOIN user_profiles up ON u.id = up.user_id
               WHERE p.community_id = $1
               ORDER BY p.created_at DESC
               LIMIT $2 OFFSET $3"#,
        )
        .bind(community_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db.pool)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?
    } else {
        sqlx::query(
            r#"SELECT p.id, p.community_id, p.title, p.description, p.proposal_type, 
                      p.status, p.voting_starts_at, p.voting_ends_at, p.created_at,
                      p.quorum_required,
                      c.name as community_name, 
                      COALESCE(up.name, u.email) as author_name,
                      (SELECT COUNT(*) FROM votes v WHERE v.proposal_id = p.id) as vote_count
               FROM proposals p
               JOIN communities c ON p.community_id = c.id
               JOIN users u ON p.created_by = u.id
               LEFT JOIN user_profiles up ON u.id = up.user_id
               ORDER BY p.created_at DESC
               LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db.pool)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?
    };

    let response: Vec<ProposalResponse> = proposals
        .iter()
        .map(|row| ProposalResponse {
            id: row.get::<Uuid, _>("id").to_string(),
            community_id: row.get::<Uuid, _>("community_id").to_string(),
            community_name: row.get::<String, _>("community_name"),
            title: row.get::<String, _>("title"),
            description: row.get::<Option<String>, _>("description"),
            proposal_type: row.get::<String, _>("proposal_type"),
            status: row.get::<String, _>("status"),
            vote_count: row.get::<i64, _>("vote_count"),
            author_name: row.get::<String, _>("author_name"),
            created_at: row
                .get::<chrono::DateTime<chrono::Utc>, _>("created_at")
                .to_rfc3339(),
            voting_starts_at: row
                .get::<Option<chrono::DateTime<chrono::Utc>>, _>("voting_starts_at")
                .map(|dt| dt.to_rfc3339()),
            voting_ends_at: row
                .get::<Option<chrono::DateTime<chrono::Utc>>, _>("voting_ends_at")
                .map(|dt| dt.to_rfc3339()),
            quorum_required: row.get::<Option<i32>, _>("quorum_required").unwrap_or(0),
        })
        .collect();

    Ok(Json(response))
}

/// Get single proposal with details
/// GET /api/proposals/:id
pub async fn get_proposal(
    State(state): State<Arc<AppState>>,
    Path(proposal_id): Path<Uuid>,
) -> Result<Json<ProposalResponse>, AppError> {
    let row = sqlx::query(
        r#"SELECT p.id, p.community_id, p.title, p.description, p.proposal_type, 
                  p.status, p.voting_starts_at, p.voting_ends_at, p.created_at,
                  p.quorum_required,
                  c.name as community_name, 
                  COALESCE(up.name, u.email) as author_name,
                  (SELECT COUNT(*) FROM votes v WHERE v.proposal_id = p.id) as vote_count
           FROM proposals p
           JOIN communities c ON p.community_id = c.id
           JOIN users u ON p.created_by = u.id
           LEFT JOIN user_profiles up ON u.id = up.user_id
           WHERE p.id = $1"#,
    )
    .bind(proposal_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?
    .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Proposal not found".to_string())))?;

    Ok(Json(ProposalResponse {
        id: row.get::<Uuid, _>("id").to_string(),
        community_id: row.get::<Uuid, _>("community_id").to_string(),
        community_name: row.get::<String, _>("community_name"),
        title: row.get::<String, _>("title"),
        description: row.get::<Option<String>, _>("description"),
        proposal_type: row.get::<String, _>("proposal_type"),
        status: row.get::<String, _>("status"),
        vote_count: row.get::<i64, _>("vote_count"),
        author_name: row.get::<String, _>("author_name"),
        created_at: row
            .get::<chrono::DateTime<chrono::Utc>, _>("created_at")
            .to_rfc3339(),
        voting_starts_at: row
            .get::<Option<chrono::DateTime<chrono::Utc>>, _>("voting_starts_at")
            .map(|dt| dt.to_rfc3339()),
        voting_ends_at: row
            .get::<Option<chrono::DateTime<chrono::Utc>>, _>("voting_ends_at")
            .map(|dt| dt.to_rfc3339()),
        quorum_required: row.get::<Option<i32>, _>("quorum_required").unwrap_or(0),
    }))
}

/// Create new proposal (requires auth + community membership)
/// POST /api/proposals
pub async fn create_proposal(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateProposalRequest>,
) -> Result<Json<ProposalResponse>, AppError> {
    // Validate title
    if payload.title.trim().is_empty() {
        return Err(AppError::Internal(anyhow::anyhow!("Title is required")));
    }
    if payload.title.len() > 255 {
        return Err(AppError::Internal(anyhow::anyhow!(
            "Title too long (max 255 chars)"
        )));
    }

    // Verify user is member of community
    let is_member = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM community_members WHERE community_id = $1 AND user_id = $2 AND status = 'active'"
    )
    .bind(payload.community_id)
    .bind(&user.user_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

    if is_member == 0 {
        return Err(AppError::Internal(anyhow::anyhow!(
            "Must be a community member to create proposals".to_string()
        )));
    }

    let proposal_type = payload.proposal_type.unwrap_or_else(|| "text".to_string());
    let quorum = payload.quorum_required.unwrap_or(0);

    // Parse dates if provided
    let voting_starts: Option<chrono::DateTime<chrono::Utc>> = payload
        .voting_starts_at
        .as_ref()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc));

    let voting_ends: Option<chrono::DateTime<chrono::Utc>> = payload
        .voting_ends_at
        .as_ref()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc));

    // Insert proposal as draft - user must review and publish
    let proposal_id = Uuid::now_v7();
    sqlx::query(
        r#"INSERT INTO proposals (id, community_id, created_by, title, description, 
                                   proposal_type, status, voting_starts_at, voting_ends_at, 
                                   quorum_required, created_at, updated_at)
           VALUES ($1, $2, $3, $4, $5, $6, 'draft', $7, $8, $9, NOW(), NOW())"#,
    )
    .bind(proposal_id)
    .bind(payload.community_id)
    .bind(&user.user_id)
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(&proposal_type)
    .bind(voting_starts)
    .bind(voting_ends)
    .bind(quorum as i64)
    .execute(&state.db.pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

    // Insert options if poll type
    if proposal_type == "poll" {
        if let Some(options) = payload.options {
            for (i, option_text) in options.iter().enumerate() {
                sqlx::query(
                    "INSERT INTO proposal_options (proposal_id, option_text, display_order) VALUES ($1, $2, $3)"
                )
                .bind(proposal_id)
                .bind(option_text)
                .bind(i as i32)
                .execute(&state.db.pool)
                .await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;
            }
        }
    }

    // Fetch and return created proposal
    get_proposal(State(state), Path(proposal_id)).await
}

/// Cast vote on proposal
/// POST /api/proposals/:id/vote
pub async fn cast_vote(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path(proposal_id): Path<Uuid>,
    Json(payload): Json<CastVoteRequest>,
) -> Result<Html<String>, AppError> {
    // Verify proposal exists and is active
    let proposal = sqlx::query(
        "SELECT community_id, status, voting_starts_at, voting_ends_at FROM proposals WHERE id = $1"
    )
    .bind(proposal_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?
    .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Proposal not found".to_string())))?;

    let status: String = proposal.get("status");
    if status != "active" {
        return Err(AppError::Internal(anyhow::anyhow!(
            "Voting is not open for this proposal".to_string()
        )));
    }

    // Verify user is community member
    let community_id: Uuid = proposal.get("community_id");
    let is_member = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM community_members WHERE community_id = $1 AND user_id = $2 AND status = 'active'"
    )
    .bind(community_id)
    .bind(&user.user_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

    if is_member == 0 {
        return Err(AppError::Internal(anyhow::anyhow!(
            "Must be a community member to vote".to_string()
        )));
    }

    // Check if user already voted
    let existing_vote = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM votes WHERE proposal_id = $1 AND user_id = $2",
    )
    .bind(proposal_id)
    .bind(&user.user_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

    if existing_vote > 0 {
        // Update existing vote
        sqlx::query(
            "UPDATE votes SET vote_value = $1, created_at = NOW() WHERE proposal_id = $2 AND user_id = $3"
        )
        .bind(&payload.vote_value)
        .bind(proposal_id)
        .bind(&user.user_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;
    } else {
        // Insert new vote
        sqlx::query(
            "INSERT INTO votes (proposal_id, user_id, vote_value, created_at) VALUES ($1, $2, $3, NOW())"
        )
        .bind(proposal_id)
        .bind(&user.user_id)
        .bind(&payload.vote_value)
        .execute(&state.db.pool)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;
    }

    // Return updated results fragment
    get_results_fragment(State(state), Path(proposal_id)).await
}

/// Get voting results
/// GET /api/proposals/:id/results
pub async fn get_results(
    State(state): State<Arc<AppState>>,
    Path(proposal_id): Path<Uuid>,
) -> Result<Json<VoteResultsResponse>, AppError> {
    // Get proposal info
    let proposal = sqlx::query("SELECT quorum_required, community_id FROM proposals WHERE id = $1")
        .bind(proposal_id)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Proposal not found".to_string())))?;

    let quorum_required: i64 = proposal
        .get::<Option<i32>, _>("quorum_required")
        .unwrap_or(0) as i64;
    let community_id: Uuid = proposal.get("community_id");

    // Get vote counts by value
    let votes = sqlx::query(
        r#"SELECT vote_value, COUNT(*) as count 
           FROM votes 
           WHERE proposal_id = $1 AND vote_value IS NOT NULL
           GROUP BY vote_value"#,
    )
    .bind(proposal_id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

    let total_votes: i64 = votes.iter().map(|r| r.get::<i64, _>("count")).sum();

    // Get total community members for quorum calculation
    let total_members = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM community_members WHERE community_id = $1 AND status = 'active'",
    )
    .bind(community_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or(1);

    let quorum_percentage = if total_members > 0 {
        (total_votes as f64 / total_members as f64) * 100.0
    } else {
        0.0
    };

    let quorum_met = quorum_percentage >= quorum_required as f64;

    let results: Vec<VoteOption> = votes
        .iter()
        .map(|row| {
            let count: i64 = row.get("count");
            VoteOption {
                value: row.get::<String, _>("vote_value"),
                count,
                percentage: if total_votes > 0 {
                    (count as f64 / total_votes as f64) * 100.0
                } else {
                    0.0
                },
            }
        })
        .collect();

    Ok(Json(VoteResultsResponse {
        proposal_id: proposal_id.to_string(),
        total_votes,
        results,
        quorum_required: quorum_required as i32,
        quorum_met,
    }))
}

/// Get results as HTML fragment for HTMX
pub async fn get_results_fragment(
    State(state): State<Arc<AppState>>,
    Path(proposal_id): Path<Uuid>,
) -> Result<Html<String>, AppError> {
    let results = get_results(State(state), Path(proposal_id)).await?;
    let data = results.0;

    let mut html = String::new();
    html.push_str(&format!(
        r#"<div class="space-y-4">
            <div class="flex items-center justify-between text-sm text-civiqo-gray-600 mb-2">
                <span>{} voti totali</span>
                <span>Quorum: {:.0}%</span>
            </div>"#,
        data.total_votes,
        if data.total_votes > 0 {
            (data.total_votes as f64 / 100.0) * 100.0
        } else {
            0.0
        }
    ));

    for option in &data.results {
        let color = if option.value == "yes" {
            "bg-civiqo-green"
        } else if option.value == "no" {
            "bg-red-500"
        } else {
            "bg-civiqo-blue"
        };

        html.push_str(&format!(
            r#"<div>
                <div class="flex items-center justify-between mb-1">
                    <span class="text-sm font-medium text-civiqo-gray-900 capitalize">{}</span>
                    <span class="text-sm text-civiqo-gray-600">{:.1}% ({})</span>
                </div>
                <div class="w-full bg-civiqo-gray-200 rounded-full h-3">
                    <div class="h-3 rounded-full transition-all duration-500 {}"
                         style="width: {}%"></div>
                </div>
            </div>"#,
            option.value, option.percentage, option.count, color, option.percentage
        ));
    }

    if data.quorum_met {
        html.push_str(r#"<div class="mt-4 p-3 bg-green-50 border border-green-200 rounded-lg text-sm text-green-800">
            ✓ Quorum raggiunto
        </div>"#);
    }

    html.push_str("</div>");

    Ok(Html(html))
}

/// Activate proposal (start voting) - returns HTML for HTMX
/// POST /api/proposals/:id/activate
pub async fn activate_proposal(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path(proposal_id): Path<Uuid>,
) -> Result<Html<String>, AppError> {
    // Verify user is author
    let proposal = sqlx::query(
        r#"SELECT p.created_by, p.status, p.title, p.description, p.voting_ends_at,
                  c.name as community_name,
                  (SELECT COUNT(*) FROM votes v WHERE v.proposal_id = p.id) as vote_count
           FROM proposals p
           JOIN communities c ON p.community_id = c.id
           WHERE p.id = $1"#,
    )
    .bind(proposal_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?
    .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Proposal not found".to_string())))?;

    let created_by: Uuid = proposal.get("created_by");
    let user_uuid = Uuid::parse_str(&user.user_id)
        .map_err(|_| AppError::Internal(anyhow::anyhow!("Invalid user ID".to_string())))?;

    if created_by != user_uuid {
        return Ok(Html(r#"<div class="p-4 bg-red-100 text-red-700 rounded-lg">Solo l'autore può attivare la proposta</div>"#.to_string()));
    }

    let status: String = proposal.get("status");
    if status != "draft" {
        return Ok(Html(r#"<div class="p-4 bg-red-100 text-red-700 rounded-lg">Solo le bozze possono essere attivate</div>"#.to_string()));
    }

    // Update status to active
    sqlx::query("UPDATE proposals SET status = 'active', updated_at = NOW() WHERE id = $1")
        .bind(proposal_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

    // Return updated card HTML
    let title: String = proposal.get("title");
    let description: Option<String> = proposal.get("description");
    let community_name: String = proposal.get("community_name");
    let vote_count: i64 = proposal.get("vote_count");
    let voting_ends: Option<chrono::DateTime<chrono::Utc>> = proposal.get("voting_ends_at");

    let ends_text = voting_ends
        .map(|dt| {
            let now = chrono::Utc::now();
            let diff = dt.signed_duration_since(now);
            if diff.num_seconds() < 0 {
                "Votazione terminata".to_string()
            } else if diff.num_days() > 0 {
                format!("Termina tra {} giorni", diff.num_days())
            } else if diff.num_hours() > 0 {
                format!("Termina tra {} ore", diff.num_hours())
            } else {
                "In scadenza".to_string()
            }
        })
        .unwrap_or_else(|| "Nessuna scadenza".to_string());

    Ok(Html(format!(
        r#"
    <div class="bg-white rounded-xl shadow-sm p-6 hover:shadow-md transition border border-civiqo-gray-200">
        <div class="flex items-start justify-between mb-4">
            <div>
                <h3 class="text-lg font-semibold text-civiqo-gray-900">{}</h3>
                <p class="text-civiqo-gray-600 text-sm mt-1">{}</p>
            </div>
            <span class="px-3 py-1 bg-civiqo-eco-green/10 text-civiqo-eco-green text-sm rounded-full font-medium">Attiva</span>
        </div>
        <p class="text-civiqo-gray-700 mb-4 line-clamp-2">{}</p>
        <div class="flex items-center justify-between text-sm text-civiqo-gray-600">
            <div class="flex items-center space-x-4">
                <span>{}</span>
                <span>•</span>
                <span>{} voti</span>
            </div>
            <div class="flex items-center space-x-2">
                <a href="/governance/{}" 
                   class="px-3 py-1 bg-civiqo-blue text-white text-sm rounded-lg hover:bg-civiqo-blue-dark transition">
                    Vota
                </a>
            </div>
        </div>
    </div>
    "#,
        title,
        community_name,
        description.unwrap_or_default(),
        ends_text,
        vote_count,
        proposal_id
    )))
}

/// Close proposal
/// POST /api/proposals/:id/close
pub async fn close_proposal(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path(proposal_id): Path<Uuid>,
) -> Result<Json<ProposalResponse>, AppError> {
    // Verify user is author or community admin
    let proposal =
        sqlx::query("SELECT created_by, community_id, status FROM proposals WHERE id = $1")
            .bind(proposal_id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?
            .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Proposal not found".to_string())))?;

    let created_by: Uuid = proposal.get("created_by");
    let community_id: Uuid = proposal.get("community_id");
    let user_uuid = Uuid::parse_str(&user.user_id)
        .map_err(|_| AppError::Internal(anyhow::anyhow!("Invalid user ID".to_string())))?;

    // Check if user is author or admin
    let is_admin = sqlx::query_scalar::<_, i64>(
        r#"SELECT COUNT(*) FROM community_members cm
           JOIN roles r ON cm.role_id = r.id
           WHERE cm.community_id = $1 AND cm.user_id = $2 AND r.name = 'admin'"#,
    )
    .bind(community_id)
    .bind(user_uuid)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or(0);

    if created_by != user_uuid && is_admin == 0 {
        return Err(AppError::Internal(anyhow::anyhow!(
            "Only the author or admin can close the proposal".to_string()
        )));
    }

    // Update status to closed
    sqlx::query("UPDATE proposals SET status = 'closed', updated_at = NOW() WHERE id = $1")
        .bind(proposal_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

    get_proposal(State(state), Path(proposal_id)).await
}
