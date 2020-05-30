mod helper;

use gluesql::{
    ExecuteError, FilterContextError, FilterError, JoinError, RowError, SelectError, StoreError,
    ValueError,
};
use helper::{Helper, SledHelper};

#[test]
fn error() {
    let helper = SledHelper::new("data.db");

    let sql = "DROP TABLE TableA";
    helper.test_error(sql, ExecuteError::QueryNotSupported.into());

    helper.run_and_print("CREATE TABLE TableA (id INTEGER);");
    helper.run_and_print("INSERT INTO TableA (id) VALUES (1);");

    let test_cases = vec![
        (StoreError::SchemaNotFound.into(), "SELECT * FROM Nothing;"),
        (SelectError::TableNotFound.into(), "SELECT * FROM;"),
        (
            SelectError::TooManyTables.into(),
            "SELECT * FROM TableA, TableB",
        ),
        (
            SelectError::JoinRightSideNotSupported.into(),
            "SELECT * FROM TableA JOIN (SELECT * FROM TableB) as TableC ON 1 = 1",
        ),
        (
            JoinError::UsingOnJoinNotSupported.into(),
            "SELECT * FROM TableA JOIN TableA USING (id)",
        ),
        (
            JoinError::JoinTypeNotSupported.into(),
            "SELECT * FROM TableA CROSS JOIN TableA as A ON 1 = 2;",
        ),
        (
            FilterError::NestedSelectRowNotFound.into(),
            "SELECT * FROM TableA WHERE id = (SELECT id FROM TableA WHERE id = 2);",
        ),
        (
            FilterContextError::ValueNotFound.into(),
            "SELECT * FROM TableA WHERE noname = 1;",
        ),
        (
            ValueError::LiteralNotSupported.into(),
            "UPDATE TableA SET id = 0.11",
        ),
        (
            RowError::LackOfRequiredColumn("id".to_owned()).into(),
            "INSERT INTO TableA () VALUES ();",
        ),
        (
            RowError::LackOfRequiredValue("id".to_owned()).into(),
            "INSERT INTO TableA VALUES ();",
        ),
    ];

    test_cases
        .into_iter()
        .for_each(|(error, sql)| helper.test_error(sql, error));

    helper.run_and_print("CREATE TABLE TableB (id BOOL);");
    helper.test_error(
        "INSERT INTO TableB (id) VALUES (0);",
        ValueError::SqlTypeNotSupported.into(),
    );
}
