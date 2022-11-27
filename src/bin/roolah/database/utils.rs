use sqlx::{sqlite::SqliteRow, Column, Row, TypeInfo};

#[allow(dead_code)]
pub fn print_row_values(row: &SqliteRow) {
    row.columns()
        .iter()
        .for_each(|col| match col.type_info().name() {
            "TEXT" => {
                let val: Option<String> = row.try_get(col.name()).unwrap();
                println!("{}: {:?}", col.name(), val);
            }
            "INTEGER" => {
                let val: Option<i64> = row.try_get(col.name()).unwrap();
                println!("{}: {:?}", col.name(), val);
            }
            _ => (),
        });
}
