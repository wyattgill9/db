use catalog::Database;
use types::{PhysicalPlan, QueryResult};

pub fn execute(_pplan: PhysicalPlan, _database: &Database) -> QueryResult {
    QueryResult::default()
}
