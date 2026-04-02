use anyhow::Result;
use comfy_table::{Table, presets::UTF8_FULL};
use dialoguer::{Confirm, Input, Select};
use inflector::Inflector;
use qw::{FieldDef, generate_add_column_sql, generate_migration_sql, quote_default, write_migration};
use sercli::{
    Migrations,
    db::{generate_model, prepare_db},
};
use serde::Deserialize;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Args {
    New { name: String },
    Model(ModelArgs),
}

#[derive(StructOpt, Debug)]
enum ModelArgs {
    Show,
    Add,
    Edit,
    Gen,
}

#[derive(Deserialize)]
struct Config {
    migrations: String,
}

fn find_config() -> Config {
    let mut dir = std::env::current_dir().unwrap();
    loop {
        let candidate = dir.join("qw.toml");
        if candidate.exists() {
            let contents = std::fs::read_to_string(&candidate).unwrap();
            return toml::from_str(&contents).unwrap();
        }
        assert!(dir.pop(), "qw.toml not found");
    }
}

const SQL_TYPES: &[&str] = &[
    "bigint",
    "integer",
    "smallint",
    "varchar",
    "boolean",
    "decimal",
    "timestamp",
    "real",
];

fn prompt_field(name: String) -> Result<FieldDef> {
    let type_idx = Select::new().with_prompt("Type").items(SQL_TYPES).default(0).interact()?;
    let not_null = Confirm::new().with_prompt("NOT NULL").default(true).interact()?;
    let default = if not_null {
        let value: String = Input::new().with_prompt("Default value").interact_text()?;
        Some(quote_default(SQL_TYPES[type_idx], &value))
    } else {
        None
    };
    Ok(FieldDef {
        name,
        sql_type: SQL_TYPES[type_idx].to_string(),
        not_null,
        default,
    })
}

fn add_model(migrations_path: &str) -> Result<()> {
    let model_name: String = Input::new().with_prompt("Model name").interact_text()?;

    let default_pk = Confirm::new()
        .with_prompt("Add default PK: id bigint NOT NULL PRIMARY KEY")
        .default(true)
        .interact()?;

    let mut fields = vec![];

    loop {
        let name: String = Input::new()
            .with_prompt("Field name (empty to finish)")
            .allow_empty(true)
            .interact_text()?;

        if name.is_empty() {
            break;
        }

        fields.push(prompt_field(name)?);
    }

    let sql = generate_migration_sql(&model_name, default_pk, &fields);
    let name = format!("add_{}", model_name.to_snake_case().to_plural());
    let filename = write_migration(migrations_path, &name, &sql)?;

    println!("Created {filename}");
    println!("{sql}");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::from_args();

    let config = find_config();

    match args {
        Args::New { name } => {
            dbg!(name);
        }
        Args::Model(ModelArgs::Show) => {
            let migrations = Migrations::get(&config.migrations)?;

            let mut table = Table::new();
            table.load_preset(UTF8_FULL);
            table.set_header(["Entity", "Table", "Fields", "Relations"]);

            for (name, entity) in &migrations.entities {
                let fields = entity
                    .fields
                    .iter()
                    .map(|f| format!("{}: {}", f.name, f.ty))
                    .collect::<Vec<_>>()
                    .join("\n");
                let relations = entity
                    .relations
                    .iter()
                    .map(|r| format!("{} -> {}", r.field, r.references))
                    .collect::<Vec<_>>()
                    .join("\n");
                table.add_row([name.as_str(), entity.table_name.as_str(), &fields, &relations]);
            }

            for (name, pg_enum) in &migrations.enums {
                table.add_row([name.as_str(), "[enum]", &pg_enum.cases.join(" | "), ""]);
            }

            println!("{table}");
        }
        Args::Model(ModelArgs::Add) => {
            add_model(&config.migrations)?;
        }
        Args::Model(ModelArgs::Gen) => {
            prepare_db().await?;
            generate_model()?;
        }
        Args::Model(ModelArgs::Edit) => {
            let migrations = Migrations::get(&config.migrations)?;
            let entity_names: Vec<&str> = migrations.entities.keys().map(String::as_str).collect();

            let idx = Select::new().with_prompt("Model").items(&entity_names).default(0).interact()?;
            let entity = &migrations.entities[entity_names[idx]];

            let mut fields = vec![];

            loop {
                let field_name: String = Input::new()
                    .with_prompt("New field name (empty to finish)")
                    .allow_empty(true)
                    .interact_text()?;

                if field_name.is_empty() {
                    break;
                }

                fields.push(prompt_field(field_name)?);
            }

            if !fields.is_empty() {
                let sql = generate_add_column_sql(&entity.table_name, &fields);
                let field_names =
                    fields.iter().map(|f| f.name.to_snake_case()).collect::<Vec<_>>().join("_and_");
                let name = format!("add_{field_names}_to_{}", entity.table_name);
                let filename = write_migration(&config.migrations, &name, &sql)?;
                println!("Created {filename}");
                println!("{sql}");
            }
        }
    }

    Ok(())
}
