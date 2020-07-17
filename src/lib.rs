#[macro_use]
extern crate diesel;

use diesel::{pg::PgConnection, prelude::*};

table! {
    testable (id) {
        id -> Integer,
        value -> Nullable<Integer>,
    }
}
table! {
    testable2 (id) {
        id -> Integer,
        value -> Nullable<Integer>,
    }
}
table! {
    testable3 (id) {
        id -> Integer,
        value -> Nullable<Integer>,
    }
}
table! {
    testable4 (id) {
        id -> Integer,
        value -> Nullable<Integer>,
    }
}
table! {
    testable5 (id) {
        id -> Integer,
        value -> Nullable<Integer>,
    }
}
table! {
    testable6 (id) {
        id -> Integer,
        value -> Nullable<Integer>,
    }
}
table! {
    testable7 (id) {
        id -> Integer,
        value -> Nullable<Integer>,
    }
}
table! {
    testable8 (id) {
        id -> Integer,
        value -> Nullable<Integer>,
    }
}
table! {
    testable9 (id) {
        id -> Integer,
        value -> Nullable<Integer>,
    }
}
table! {
    testable10 (id) {
        id -> Integer,
        value -> Nullable<Integer>,
    }
}

pub fn do_db_things(conn: &PgConnection, id: i32) -> Result<(), diesel::result::Error> {
    diesel::update(testable::table)
        .filter(testable::dsl::id.eq(id))
        .set(testable::dsl::value.eq(None::<i32>))
        .execute(conn)?;

    Ok(())
}
