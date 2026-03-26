use anyhow::Result;
use comfy_table::{Table, presets::UTF8_FULL};
use sercli::Migrations;
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

fn main() -> Result<()> {
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
    }

    Ok(())
}
