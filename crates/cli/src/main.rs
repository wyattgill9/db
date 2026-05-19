use catalog::Database;

#[rustfmt::skip]
#[cfg(target_pointer_width = "64")]
fn main() {
    let mut db = Database::new();

    db.create_table("trips", &[
        ("cab_type", "u8"),
        ("passenger_count", "u8"),
        ("total_amount", "f64"),
    ]);

    let sql = "SELECT passenger_count, avg(total_amount) \
               FROM trips \
               GROUP BY passenger_count";

    let logical_plan = build_unresolved_plan(sql, &db);
    println!("{logical_plan:#?}");
}

fn build_unresolved_plan(sql: &str, _db: &Database) -> types::UnresolvedPlan {
    let parsed = sql::parse(sql);
    sql::translate(parsed)

    // let logical = optimizer::build_plan(resolved);
    // optimizer::optimize(logical)
}
