use anyhow::Result;
use inflector::Inflector;

pub struct FieldDef {
    pub name:     String,
    pub sql_type: String,
    pub not_null: bool,
}

pub fn generate_migration_sql(model_name: &str, default_pk: bool, fields: &[FieldDef]) -> String {
    let table_name = model_name.to_snake_case().to_plural();
    let mut columns = vec![];

    if default_pk {
        columns.push(r#"    "id" bigint NOT NULL PRIMARY KEY"#.to_string());
    }

    for f in fields {
        let constraint = if f.not_null { " NOT NULL" } else { "" };
        columns.push(format!(r#"    "{}" {}{}"#, f.name, f.sql_type, constraint));
    }

    format!(
        r#"CREATE TABLE "{table_name}"
(
{}
);
"#,
        columns.join(
            ",
"
        )
    )
}

pub fn write_migration(migrations_path: &str, model_name: &str, sql: &str) -> Result<String> {
    let table_name = model_name.to_snake_case().to_plural();
    let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S");
    let filename = format!("{timestamp}_add_{table_name}.sql");
    let path = std::path::Path::new(migrations_path).join(&filename);
    std::fs::write(&path, sql)?;
    Ok(filename)
}
