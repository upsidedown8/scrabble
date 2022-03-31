use crate::{auth::authenticated_user, db::Db, filters::with, fsm::FsmRef, handlers};
use warp::{filters::BoxedFilter, Filter, Reply};

pub fn all(db: &Db, fsm: &FsmRef) -> BoxedFilter<(impl Reply,)> {
    todo!()
}
