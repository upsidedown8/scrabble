use chrono::NaiveDateTime;

/// A record in `tbl_friend_request`.
#[derive(Debug, Clone)]
pub struct FriendRequest {
    /// Id of the user making the request.
    pub from_id_user: usize,
    /// Id of the potential friend.
    pub to_id_user: usize,
    /// Date that the friend request was sent.
    pub date_sent: NaiveDateTime,
}
