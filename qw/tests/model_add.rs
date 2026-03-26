use qw::{FieldDef, generate_add_column_sql, generate_migration_sql, write_migration};
use sercli::Migrations;

#[test]
fn generates_sql_with_default_pk() {
    let sql = generate_migration_sql(
        "Post",
        true,
        &[
            FieldDef {
                name:     "title".into(),
                sql_type: "varchar".into(),
                not_null: true,
            },
            FieldDef {
                name:     "views".into(),
                sql_type: "integer".into(),
                not_null: false,
            },
        ],
    );

    assert_eq!(
        sql,
        r#"CREATE TABLE "posts"
(
    "id" bigint NOT NULL PRIMARY KEY,
    "title" varchar NOT NULL,
    "views" integer
);
"#
    );
}

#[test]
fn generates_sql_without_default_pk() {
    let sql = generate_migration_sql(
        "UserStat",
        false,
        &[
            FieldDef {
                name:     "user_id".into(),
                sql_type: "bigint".into(),
                not_null: true,
            },
            FieldDef {
                name:     "score".into(),
                sql_type: "integer".into(),
                not_null: true,
            },
        ],
    );

    assert_eq!(
        sql,
        r#"CREATE TABLE "user_stats"
(
    "user_id" bigint NOT NULL,
    "score" integer NOT NULL
);
"#
    );
}

#[test]
fn generates_add_column_sql() {
    let sql = generate_add_column_sql(
        "posts",
        &FieldDef {
            name:     "body".into(),
            sql_type: "varchar".into(),
            not_null: true,
        },
    );
    assert_eq!(
        sql,
        r#"ALTER TABLE "posts" ADD COLUMN "body" varchar NOT NULL;
"#
    );

    let sql = generate_add_column_sql(
        "posts",
        &FieldDef {
            name:     "views".into(),
            sql_type: "integer".into(),
            not_null: false,
        },
    );
    assert_eq!(
        sql,
        r#"ALTER TABLE "posts" ADD COLUMN "views" integer;
"#
    );
}

#[test]
fn written_migration_is_parseable() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().to_str().unwrap();

    let sql = generate_migration_sql(
        "Article",
        true,
        &[FieldDef {
            name:     "title".into(),
            sql_type: "varchar".into(),
            not_null: true,
        }],
    );

    write_migration(path, "add_articles", &sql).unwrap();

    let migrations = Migrations::get(path).unwrap();
    let entity = migrations.entities.get("Article").expect("Article not found");
    assert_eq!(entity.table_name, "articles");
    assert_eq!(entity.fields.len(), 2); // id + title
}
