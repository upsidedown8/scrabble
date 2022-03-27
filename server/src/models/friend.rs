use chrono::{DateTime, Utc};

/// A record in `tbl_friend`.
#[derive(Debug, Clone)]
pub struct Friend {
    /// Uuid of the first user.
    pub first_id_user: String,
    /// Uuid of the second user.
    pub second_id_user: String,
    /// Date that the friend was added.
    pub date_added: DateTime<Utc>,
}
