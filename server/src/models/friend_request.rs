use chrono::NaiveDateTime;

/// A record in `tbl_friend_request`.
#[derive(Debug, Clone)]
pub struct FriendRequest {
    /// Uuid of the user making the request.
    pub from_id_user: String,
    /// Uuid of the potential friend.
    pub to_id_user: String,
    /// Date that the friend request was sent.
    pub date_sent: NaiveDateTime,
}
