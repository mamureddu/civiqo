use shared::{
    database::Database,
    error::{AppError, Result},
    models::chat::{ChatRoom, RoomType},
    // RoomParticipant,      // Uncomment when implementing participant management
    // ParticipantRole,      // Uncomment when implementing role-based permissions
};
use tracing::{debug, info};
use uuid::Uuid;

/// Service for managing chat rooms and participants
pub struct RoomService {
    database: Database,
}

impl RoomService {
    /// Create a new room service
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    /// Get room information by ID
    pub async fn get_room(&self, room_id: Uuid) -> Result<Option<ChatRoom>> {
        let room = sqlx::query_as!(
            ChatRoom,
            r#"
            SELECT id, community_id, name, description, room_type as "room_type: RoomType",
                   is_private, created_by, created_at, updated_at
            FROM chat_rooms
            WHERE id = $1
            "#,
            room_id
        )
        .fetch_optional(self.database.pool())
        .await
        .map_err(AppError::Database)?;

        Ok(room)
    }

    /// Check if a user can access a room
    pub async fn can_user_access_room(&self, user_id: Uuid, room_id: Uuid) -> Result<bool> {
        // First check if user is a participant
        let participant = sqlx::query!(
            "SELECT user_id FROM room_participants WHERE room_id = $1 AND user_id = $2",
            room_id,
            user_id
        )
        .fetch_optional(self.database.pool())
        .await
        .map_err(AppError::Database)?;

        if participant.is_some() {
            return Ok(true);
        }

        // If not a direct participant, check if it's a public community room
        // and user is a member of the community
        let community_access = sqlx::query!(
            r#"
            SELECT cr.id
            FROM chat_rooms cr
            JOIN community_members cm ON cr.community_id = cm.community_id
            WHERE cr.id = $1
              AND cm.user_id = $2
              AND cr.is_private = false
              AND cm.status = 'active'
            "#,
            room_id,
            user_id
        )
        .fetch_optional(self.database.pool())
        .await
        .map_err(AppError::Database)?;

        Ok(community_access.is_some())
    }

    /// Get user's role in a room
    pub async fn get_user_room_role(&self, user_id: Uuid, room_id: Uuid) -> Result<Option<String>> {
        let participant = sqlx::query!(
            "SELECT role::text FROM room_participants WHERE room_id = $1 AND user_id = $2",
            room_id,
            user_id
        )
        .fetch_optional(self.database.pool())
        .await
        .map_err(AppError::Database)?;

        Ok(participant.and_then(|p| p.role))
    }

    /// Add a user to a room
    pub async fn add_participant(
        &self,
        room_id: Uuid,
        user_id: Uuid,
        role: Option<String>,
    ) -> Result<()> {
        // Check if room exists
        let room = self.get_room(room_id).await?;
        if room.is_none() {
            return Err(AppError::NotFound(format!("Room {} not found", room_id)));
        }

        let participant_role = role.unwrap_or_else(|| "member".to_string());

        sqlx::query(
            r#"
            INSERT INTO room_participants (room_id, user_id, role, joined_at)
            VALUES ($1, $2, $3, NOW())
            ON CONFLICT (room_id, user_id) DO UPDATE SET
                role = $3,
                joined_at = NOW()
            "#,
        )
        .bind(room_id)
        .bind(user_id)
        .bind(&participant_role)
        .execute(self.database.pool())
        .await
        .map_err(AppError::Database)?;

        info!(
            "User {} added to room {} with role {}",
            user_id, room_id, participant_role
        );
        Ok(())
    }

    /// Update user's last read timestamp for a room
    pub async fn update_last_read(&self, user_id: Uuid, room_id: Uuid) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE room_participants
            SET last_read_at = NOW()
            WHERE room_id = $1 AND user_id = $2
            "#,
            room_id,
            user_id
        )
        .execute(self.database.pool())
        .await
        .map_err(AppError::Database)?;

        debug!("Updated last read for user {} in room {}", user_id, room_id);
        Ok(())
    }

    /// Check if user has required permission for room action
    pub async fn check_room_permission(
        &self,
        user_id: Uuid,
        room_id: Uuid,
        required_permission: &str,
    ) -> Result<bool> {
        // Get user's role in the room
        let role = self.get_user_room_role(user_id, room_id).await?;

        match role.as_deref() {
            Some("admin") => Ok(true), // Admins can do everything
            Some("moderator") => {
                // Moderators can moderate but not manage participants
                Ok(matches!(
                    required_permission,
                    "send_message" | "delete_message" | "moderate"
                ))
            }
            Some("member") => {
                // Members can only send messages
                Ok(matches!(required_permission, "send_message"))
            }
            None => {
                // Not a participant - check if it's a public room in their community
                if required_permission == "send_message" {
                    self.can_user_access_room(user_id, room_id).await
                } else {
                    Ok(false)
                }
            }
            _ => Ok(false),
        }
    }
}
