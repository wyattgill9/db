// use execution::execute;
// use types::{OutputTable, PhysicalPlan};
// use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

// #[repr(C)]
// #[derive(Clone, Copy, FromBytes, IntoBytes, Immutable, KnownLayout)]
// struct Vec3 {
//     x: f32,
//     y: f32,
//     z: f32,
// }

// const TABLE_NAME: &str = "vec3";

#[rustfmt::skip]
fn main() {
    let mut db = catalog::Database::new();

    // db.create_table(TABLE_NAME, &[
    //     ("x", "f32"),
    //     ("y", "f32"), ("z", "f32")
    // ]);

    // let points: Vec<Vec3> = (0..5)
    //     .map(|i| {
    //         let t = i as f32;
    //         Vec3 {
    //             x: t,
    //             y: t * 2.0,
    //             z: t * 3.0,
    //         }
    //     })
    //     .collect();

    // db.insert(TABLE_NAME, points.as_bytes());
    // db.flush_table_writes(TABLE_NAME);

    // let result = execute(PhysicalPlan::default(), &db);
    // let output = OutputTable::from_query_result(&result);
    // println!("{output}");

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

fn build_unresolved_plan(sql: &str, _db: &catalog::Database) -> types::UnresolvedPlan {
    let parsed = sql::parse(sql);
    sql::translate(parsed)

    // let logical = optimizer::build_plan(resolved);
    // optimizer::optimize(logical)
}
