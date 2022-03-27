use chrono::{DateTime, Utc};
use uuid::Uuid;

/// A record in `tbl_password_reset`.
#[derive(Debug)]
pub struct PasswordReset {
    /// The id of the user who made the request.
    pub id_user: Uuid,
    /// A secret which is sent by email to verify the user's
    /// password reset request.
    pub secret_hex: String,
    /// The time at which the `password_reset` is no longer valid.
    pub valid_until: DateTime<Utc>,
}
