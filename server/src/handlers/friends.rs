use crate::{auth::Jwt, db::Db, error::Error, models};
use api::{
    auth::AuthWrapper,
    routes::friends::{Friend, FriendRequestsResponse, FriendsResponse},
};
use warp::{Rejection, Reply};

/// POST /api/friends/{username} [+Auth]
pub async fn add(username: String, db: Db, jwt: Jwt) -> Result<impl Reply, Rejection> {
    models::FriendRequest::insert(&db, jwt.id_user(), &username).await?;

    Ok(warp::reply::json(&AuthWrapper {
        token: Some(jwt.token()?),
        response: (),
    }))
}

/// DELETE /api/friends/{username} [+Auth]
pub async fn remove(username: String, db: Db, jwt: Jwt) -> Result<impl Reply, Rejection> {
    models::FriendRequest::delete(&db, jwt.id_user(), &username).await?;

    Ok(warp::reply::json(&AuthWrapper {
        token: Some(jwt.token()?),
        response: (),
    }))
}

/// GET /api/friends [+Auth]
pub async fn list(db: Db, jwt: Jwt) -> Result<impl Reply, Rejection> {
    let friends = sqlx::query_file!("sql/friends/list_friends.sql", jwt.id_user())
        .fetch_all(&db)
        .await
        .map_err(Error::Sqlx)?
        .into_iter()
        .filter_map(|row| {
            row.since.map(|since| Friend {
                username: row.username,
                since,
            })
        })
        .collect();

    Ok(warp::reply::json(&AuthWrapper {
        token: Some(jwt.token()?),
        response: FriendsResponse { friends },
    }))
}

/// GET /api/friends/requests/incoming [+Auth]
pub async fn list_incoming(db: Db, jwt: Jwt) -> Result<impl Reply, Rejection> {
    let requests = sqlx::query_file!("sql/friends/list_incoming.sql", jwt.id_user())
        .fetch_all(&db)
        .await
        .map_err(Error::Sqlx)?
        .into_iter()
        .map(|row| Friend {
            username: row.username,
            since: row.date_sent,
        })
        .collect();

    Ok(warp::reply::json(&AuthWrapper {
        token: Some(jwt.token()?),
        response: FriendRequestsResponse { requests },
    }))
}

/// GET /api/friends/requests/outgoing [+Auth]
pub async fn list_outgoing(db: Db, jwt: Jwt) -> Result<impl Reply, Rejection> {
    let requests = sqlx::query_file!("sql/friends/list_outgoing.sql", jwt.id_user())
        .fetch_all(&db)
        .await
        .map_err(Error::Sqlx)?
        .into_iter()
        .map(|row| Friend {
            username: row.username,
            since: row.date_sent,
        })
        .collect();

    Ok(warp::reply::json(&AuthWrapper {
        token: (Some(jwt.token()?)),
        response: FriendRequestsResponse { requests },
    }))
}
