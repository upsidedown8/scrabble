use warp::{host::Authority, hyper::Uri, path::FullPath, Rejection, Reply};

use crate::error::Error;

pub mod friends;
pub mod games;
pub mod leaderboard;
pub mod live;
pub mod users;

/// Handler that redirects HTTP to HTTPS.
pub fn http_redirect(
    authority: Option<Authority>,
    full_path: FullPath,
) -> Result<impl Reply, Rejection> {
    // Find the `host`. e.g. example.com.
    let authority = authority.ok_or(Error::MissingAuthority)?;
    let host = authority.host();

    // Find the path and query. e.g. /leaderboard?limit=20&offset=10.
    let path_and_query = full_path.as_str();

    // Create a new Uri from the host, path&query with HTTPS.
    let uri = Uri::builder()
        .scheme("https")
        .authority(authority.host())
        .path_and_query(path_and_query)
        .build()
        .map_err(Error::Http)?;
    Ok(warp::redirect(uri))
}
