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

    // ==========================================================
    // COMMENTED METHOD - KEPT FOR FUTURE REFERENCE
    // ==========================================================
    // /// Get all rooms for a community
    // /// USAGE: When implementing community chat listings or discovery
    // /// PURPOSE: Show all available chat rooms within a community
    // pub async fn get_community_rooms(&self, community_id: Uuid) -> Result<Vec<ChatRoom>> {
    //     let rooms = sqlx::query_as!(
    //         ChatRoom,
    //         r#"
    //         SELECT id, community_id, name, description, room_type as "room_type: RoomType",
    //                is_private, created_by, created_at, updated_at
    //         FROM chat_rooms
    //         WHERE community_id = $1
    //         ORDER BY created_at ASC
    //         "#,
    //         community_id
    //     )
    //     .fetch_all(self.database.pool())
    //     .await
    //     .map_err(AppError::Database)?;

    //     Ok(rooms)
    // }

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
            "#
        )
        .bind(room_id)
        .bind(user_id)
        .bind(&participant_role)
        .execute(self.database.pool())
        .await
        .map_err(AppError::Database)?;

        info!("User {} added to room {} with role {}", user_id, room_id, participant_role);
        Ok(())
    }

    // ==========================================================
    // COMMENTED METHOD - KEPT FOR FUTURE REFERENCE
    // ==========================================================
    // /// Remove a user from a room
    // /// USAGE: When implementing room management or user kick functionality
    // /// PURPOSE: Remove participants from chat rooms (leave/kick)
    // pub async fn remove_participant(&self, room_id: Uuid, user_id: Uuid) -> Result<()> {
    //     let result = sqlx::query!(
    //         "DELETE FROM room_participants WHERE room_id = $1 AND user_id = $2",
    //         room_id,
    //         user_id
    //     )
    //     .execute(self.database.pool())
    //     .await
    //     .map_err(AppError::Database)?;

    //     if result.rows_affected() == 0 {
    //         return Err(AppError::NotFound(format!(
    //             "User {} not found in room {}",
    //             user_id, room_id
    //         )));
    //     }

    //     info!("User {} removed from room {}", user_id, room_id);
    //     Ok(())
    // }

    // ==========================================================
    // COMMENTED METHOD - KEPT FOR FUTURE REFERENCE
    // ==========================================================
    // /// Get all participants in a room
    // /// USAGE: When implementing room member lists or participant management
    // /// PURPOSE: Show all users in a chat room with their roles
    // pub async fn get_room_participants(&self, room_id: Uuid) -> Result<Vec<RoomParticipant>> {
    //     let rows = sqlx::query!(
    //         r#"
    //         SELECT id, room_id, user_id, role::text as role, joined_at, last_read_at
    //         FROM room_participants
    //         WHERE room_id = $1
    //         ORDER BY joined_at ASC
    //         "#,
    //         room_id
    //     )
    //     .fetch_all(self.database.pool())
    //     .await
    //     .map_err(AppError::Database)?;

    //     let participants = rows.into_iter().map(|row| {
    //         let role = match row.role.as_deref() {
    //             Some("admin") => ParticipantRole::Admin,
    //             Some("moderator") => ParticipantRole::Moderator,
    //             Some("member") | _ => ParticipantRole::Member,
    //         };

    //         RoomParticipant {
    //             id: row.id,
    //             room_id: row.room_id,
    //             user_id: row.user_id,
    //             role,
    //             joined_at: row.joined_at,
    //             last_read_at: row.last_read_at,
    //         }
    //     }).collect();

    //     Ok(participants)
    // }

    // ==========================================================
    // COMMENTED METHOD - KEPT FOR FUTURE REFERENCE
    // ==========================================================
    // /// Get participant count for a room
    // /// USAGE: When implementing room statistics or UI indicators
    // /// PURPOSE: Show number of users in each chat room
    // pub async fn get_participant_count(&self, room_id: Uuid) -> Result<i64> {
    //     let count = sqlx::query!(
    //         "SELECT COUNT(*) as count FROM room_participants WHERE room_id = $1",
    //         room_id
    //     )
    //     .fetch_one(self.database.pool())
    //     .await
    //     .map_err(AppError::Database)?;

    //     Ok(count.count.unwrap_or(0))
    // }

    // ==========================================================
    // COMMENTED METHOD - KEPT FOR FUTURE REFERENCE
    // ==========================================================
    // /// Create a direct message room between two users
    // /// USAGE: When implementing private messaging functionality
    // /// PURPOSE: Enable one-on-one conversations between users
    // pub async fn create_direct_message_room(
    //     &self,
    //     user1_id: Uuid,
    //     user2_id: Uuid,
    //     community_id: Uuid,
    // ) -> Result<Uuid> {
        //     // Check if DM room already exists between these users
    //     let existing_room = sqlx::query!(
    //         r#"
    //         SELECT cr.id
    //         FROM chat_rooms cr
    //         JOIN room_participants rp1 ON cr.id = rp1.room_id
    //         JOIN room_participants rp2 ON cr.id = rp2.room_id
    //         WHERE cr.room_type = 'direct_message'
    //         AND rp1.user_id = $1 AND rp2.user_id = $2
    //         AND cr.id IN (
    //             SELECT room_id FROM room_participants WHERE user_id = $1
    //             INTERSECT
    //             SELECT room_id FROM room_participants WHERE user_id = $2
    //         )
    //         LIMIT 1
    //         "#,
    //         user1_id,
    //         user2_id
    //     )
    //     .fetch_optional(self.database.pool())
    //     .await
    //     .map_err(AppError::Database)?;

    //     if let Some(room) = existing_room {
    //         return Ok(room.id);
    //     }

    //     // Create new DM room
    //     let room_id = Uuid::new_v4();
    //     let room_name = format!("DM: {} <-> {}", user1_id, user2_id);

    //     let mut tx = self.database.pool().begin().await
    //         .map_err(AppError::Database)?;

    //     // Create room
    //     sqlx::query!(
    //         r#"
    //         INSERT INTO chat_rooms (id, community_id, name, description, room_type, is_private, created_by)
    //         VALUES ($1, $2, $3, $4, 'direct_message', true, $5)
    //         "#,
    //         room_id,
    //         community_id,
    //         room_name,
    //         "Direct message room",
    //         user1_id
    //     )
    //     .execute(&mut *tx)
    //     .await
    //     .map_err(AppError::Database)?;

    //     // Add both participants
    //     sqlx::query!(
    //         r#"
    //         INSERT INTO room_participants (room_id, user_id, role, joined_at)
    //         VALUES
    //             ($1, $2, 'member', NOW()),
    //             ($1, $3, 'member', NOW())
    //         "#,
    //         room_id,
    //         user1_id,
    //         user2_id
    //     )
    //     .execute(&mut *tx)
    //     .await
    //     .map_err(AppError::Database)?;

    //     tx.commit().await.map_err(AppError::Database)?;

    //     info!("Created direct message room {} between users {} and {}", room_id, user1_id, user2_id);
    //     Ok(room_id)
    // }

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
                Ok(matches!(required_permission, "send_message" | "delete_message" | "moderate"))
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

// Tests disabled - require full DB setup
// #[cfg(test)]
// mod tests {
//     use super::*;
// 
//     #[test]
//     fn test_room_service_creation() {
//         // Mock database for testing
//         let database_url = "postgresql://test:test@localhost/test";
//         // In real tests, we'd create a test database connection
//         // For now, just test that the service can be created
// 
//         // This is a compile-time test to ensure the interface is correct
//         let _service_constructor: fn(Database) -> RoomService = RoomService::new;
//     }
// 
//     #[test]
//     fn test_permission_logic() {
//         // Test permission matching logic
//         assert!(matches!("send_message", "send_message"));
//         assert!(matches!("delete_message", "delete_message" | "moderate"));
//         assert!(!matches!("manage_participants", "send_message"));
//     }
// 
//     #[test]
//     fn test_room_name_generation() {
//         let user1 = Uuid::new_v4();
//         let user2 = Uuid::new_v4();
//         let room_name = format!("DM: {} & {}", user1, user2);
// 
//         assert!(room_name.starts_with("DM: "));
//         assert!(room_name.contains(&user1.to_string()));
//         assert!(room_name.contains(&user2.to_string()));
//     }
// }