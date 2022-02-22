use scrabble::word_tree::WordTree;
use warp::{Filter, Rejection, Reply};

pub fn all(
    word_tree: &WordTree,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone + '_ {
    let check_route = warp::any()
        .map(move || word_tree)
        .and(warp::path::param())
        .and(warp::get())
        .and_then(check);

    let routes = check_route;

    warp::path("words").and(routes)
}

async fn check(word_tree: &WordTree, word: String) -> Result<impl Reply, Rejection> {
    match word_tree.contains(&word) {
        true => Ok(warp::reply::reply()),
        false => Err(warp::reject::not_found()),
    }
}
