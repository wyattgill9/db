use catalog::Database;
use types::{ResolvedPlan, UnresolvedPlan};

pub fn bind(_plan: UnresolvedPlan, _database: &Database) -> ResolvedPlan {
    ResolvedPlan::default()
}
