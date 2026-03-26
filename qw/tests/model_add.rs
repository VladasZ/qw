use qw::{FieldDef, generate_add_column_sql, generate_migration_sql, quote_default, write_migration};
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
                default:  None,
            },
            FieldDef {
                name:     "views".into(),
                sql_type: "integer".into(),
                not_null: false,
                default:  None,
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
                default:  None,
            },
            FieldDef {
                name:     "score".into(),
                sql_type: "integer".into(),
                not_null: true,
                default:  None,
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
fn generates_add_column_single() {
    let sql = generate_add_column_sql(
        "posts",
        &[FieldDef {
            name:     "body".into(),
            sql_type: "varchar".into(),
            not_null: true,
            default:  Some("''".into()),
        }],
    );
    assert_eq!(
        sql,
        r#"ALTER TABLE "posts"
    ADD COLUMN "body" varchar DEFAULT '' NOT NULL;
"#
    );
}

#[test]
fn generates_add_column_multiple() {
    let sql = generate_add_column_sql(
        "posts",
        &[
            FieldDef {
                name:     "body".into(),
                sql_type: "varchar".into(),
                not_null: true,
                default:  Some("''".into()),
            },
            FieldDef {
                name:     "views".into(),
                sql_type: "integer".into(),
                not_null: false,
                default:  None,
            },
        ],
    );
    assert_eq!(
        sql,
        r#"ALTER TABLE "posts"
    ADD COLUMN "body" varchar DEFAULT '' NOT NULL,
    ADD COLUMN "views" integer;
"#
    );
}

#[test]
fn quotes_varchar_default() {
    assert_eq!(quote_default("varchar", "red grebeshok"), "'red grebeshok'");
}

#[test]
fn quotes_timestamp_default() {
    assert_eq!(
        quote_default("timestamp", "2024-01-01 00:00:00"),
        "'2024-01-01 00:00:00'"
    );
}

#[test]
fn does_not_quote_numeric_defaults() {
    assert_eq!(quote_default("integer", "0"), "0");
    assert_eq!(quote_default("bigint", "42"), "42");
    assert_eq!(quote_default("boolean", "false"), "false");
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
            default:  None,
        }],
    );

    write_migration(path, "add_articles", &sql).unwrap();

    let migrations = Migrations::get(path).unwrap();
    let entity = migrations.entities.get("Article").expect("Article not found");
    assert_eq!(entity.table_name, "articles");
    assert_eq!(entity.fields.len(), 2); // id + title
}
