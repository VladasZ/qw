use std::{
    collections::BTreeSet,
    fs::{read_dir, remove_dir_all},
    path::Path,
    process::Command,
};

use inflector::cases::snakecase::to_snake_case;
use qw::{FieldDef, generate_add_column_sql, generate_migration_sql};
use sercli::Migrations;
use sqlx::PgPool;
use testcontainers_modules::{postgres::Postgres, testcontainers::runners::AsyncRunner};

const ENTITIES_DIR: &str = "../test_data/src/entities";

async fn connect() -> (PgPool, impl Drop) {
    let container = Postgres::default().start().await.unwrap();
    let url = format!(
        "postgres://postgres:postgres@127.0.0.1:{}/postgres",
        container.get_host_port_ipv4(5432).await.unwrap()
    );
    let pool = PgPool::connect(&url).await.unwrap();
    (pool, container)
}

#[tokio::test]
async fn all_petuh_tables_are_created() {
    let (pool, _container) = connect().await;
    let mut tx = pool.begin().await.unwrap();

    let migrations = Migrations::get("../test_data/migrations").unwrap();

    for sql in std::fs::read_dir("../test_data/migrations")
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect::<std::collections::BTreeSet<_>>()
    {
        let sql = std::fs::read_to_string(sql).unwrap();
        sqlx::raw_sql(&sql).execute(&mut *tx).await.unwrap();
    }

    let tables: Vec<String> = sqlx::query_scalar(
        r#"SELECT table_name FROM information_schema.tables WHERE table_schema = 'public' ORDER BY table_name"#,
    )
    .fetch_all(&mut *tx)
    .await
    .unwrap();

    let expected: std::collections::BTreeSet<&str> =
        migrations.entities.values().map(|e| e.table_name.as_str()).collect();

    for table in &expected {
        assert!(tables.iter().any(|t| t == *table), "Missing table: {table}");
    }

    tx.rollback().await.unwrap();
}

#[tokio::test]
async fn create_table_migration_runs() {
    let (pool, _container) = connect().await;
    let mut tx = pool.begin().await.unwrap();

    let sql = generate_migration_sql(
        "TestPost",
        true,
        &[FieldDef {
            name:     "title".into(),
            sql_type: "varchar".into(),
            not_null: true,
            default:  None,
        }],
    );
    sqlx::query(&sql).execute(&mut *tx).await.unwrap();

    let columns: Vec<String> = sqlx::query_scalar(
        r#"SELECT column_name FROM information_schema.columns WHERE table_name = 'test_posts' ORDER BY ordinal_position"#,
    )
    .fetch_all(&mut *tx)
    .await
    .unwrap();

    assert_eq!(columns, vec!["id", "title"]);

    tx.rollback().await.unwrap();
}

#[tokio::test]
async fn alter_table_migration_runs() {
    let (pool, _container) = connect().await;
    let mut tx = pool.begin().await.unwrap();

    let create_sql = generate_migration_sql(
        "TestRooster",
        true,
        &[FieldDef {
            name:     "name".into(),
            sql_type: "varchar".into(),
            not_null: true,
            default:  None,
        }],
    );
    sqlx::query(&create_sql).execute(&mut *tx).await.unwrap();

    let alter_sql = generate_add_column_sql(
        "test_roosters",
        &[FieldDef {
            name:     "greben".into(),
            sql_type: "varchar".into(),
            not_null: true,
            default:  Some("'red'".into()),
        }],
    );
    sqlx::query(&alter_sql).execute(&mut *tx).await.unwrap();

    let columns: Vec<String> = sqlx::query_scalar(
        r#"SELECT column_name FROM information_schema.columns WHERE table_name = 'test_roosters' ORDER BY ordinal_position"#,
    )
    .fetch_all(&mut *tx)
    .await
    .unwrap();

    assert_eq!(columns, vec!["id", "name", "greben"]);

    tx.rollback().await.unwrap();
}

#[tokio::test]
async fn model_gen_recreates_entities() {
    let entities_dir = Path::new(ENTITIES_DIR);

    if entities_dir.exists() {
        remove_dir_all(entities_dir).unwrap();
    }
    assert!(!entities_dir.exists());

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();

    let status = Command::new(env!("CARGO_BIN_EXE_qw"))
        .args(["model", "gen"])
        .current_dir(repo_root)
        .status()
        .unwrap();

    assert!(status.success());

    let migrations = Migrations::get("../test_data/migrations").unwrap();

    let mut expected: BTreeSet<String> = migrations
        .entities
        .keys()
        .map(|name| format!("{}.rs", to_snake_case(name)))
        .collect();

    for name in migrations.enums.keys() {
        expected.insert(format!("{}.rs", to_snake_case(name)));
    }

    expected.insert("mod.rs".to_string());
    expected.insert("model.rs".to_string());

    let actual: BTreeSet<String> = read_dir(entities_dir)
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().to_string())
        .collect();

    assert_eq!(expected, actual);
}
