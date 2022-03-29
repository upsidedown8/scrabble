use crate::{
    auth::{authenticated_user, Jwt},
    error::Error,
    models, with_db, Db,
};
use api::{
    auth::AuthWrapper,
    routes::friends::{Friend, FriendRequestsResponse, FriendsResponse},
};
use warp::{Filter, Rejection, Reply};

/// Filters for the friends routes.
pub fn all(db: &Db) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let add_friend_route = warp::any()
        .and(warp::post())
        .and(with_db(db))
        .and(authenticated_user())
        .and(warp::path::param())
        .and_then(add_friend);
    let remove_friend_route = warp::any()
        .and(warp::delete())
        .and(with_db(db))
        .and(authenticated_user())
        .and(warp::path::param())
        .and_then(remove_friend);
    let list_friends_route = warp::any()
        .and(warp::get())
        .and(with_db(db))
        .and(authenticated_user())
        .and_then(list_friends);
    let list_requests_route = warp::path("requests")
        .and(warp::get())
        .and(with_db(db))
        .and(authenticated_user())
        .and_then(list_requests);

    let routes = add_friend_route
        .or(remove_friend_route)
        .or(list_friends_route)
        .or(list_requests_route);

    warp::path("friends").and(routes)
}

/// POST /api/friends [+Auth]
async fn add_friend(db: Db, jwt: Jwt, username: String) -> Result<impl Reply, Rejection> {
    models::FriendRequest::insert(&db, jwt.id_user(), &username).await?;

    Ok(warp::reply::json(&AuthWrapper {
        auth: Some(jwt.auth()?),
        response: (),
    }))
}

/// DELETE /api/friends/{username} [+Auth]
async fn remove_friend(db: Db, jwt: Jwt, username: String) -> Result<impl Reply, Rejection> {
    models::FriendRequest::delete(&db, jwt.id_user(), &username).await?;

    Ok(warp::reply::json(&AuthWrapper {
        auth: Some(jwt.auth()?),
        response: (),
    }))
}

/// GET /api/friends [+Auth]
async fn list_friends(db: Db, jwt: Jwt) -> Result<impl Reply, Rejection> {
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
        auth: Some(jwt.auth()?),
        response: FriendsResponse { friends },
    }))
}

/// GET /api/friends/requests [+Auth]
async fn list_requests(db: Db, jwt: Jwt) -> Result<impl Reply, Rejection> {
    let requests = sqlx::query_file!("sql/friends/list_requests.sql", jwt.id_user())
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
        auth: Some(jwt.auth()?),
        response: FriendRequestsResponse { requests },
    }))
}
