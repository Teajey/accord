use clap::Parser;
use eyre::Result;

use crate::gen::generate_typescript;
use crate::{
    app::{self, cli},
    cross,
};
use crate::{cross_eprint, cross_eprintln, introspection, print_info};

pub async fn run() -> Result<()> {
    let argv: Result<Vec<_>> = cross::env::argv().collect();

    let cli = match cli::Base::try_parse_from(argv?) {
        Ok(cli) => cli,
        Err(err) => {
            cross_eprint!("{err}");
            cross::process::exit(0);
        }
    };

    if let Some(working_dir) = &cli.working_directory {
        cross::env::set_current_dir(working_dir)?;
    }

    let config = app::Config::load(cli.config_location.as_deref()).unwrap_or_else(|err| {
        cross_eprintln!("Failed to load config: {}", err);
        cross::process::exit(1);
    });

    let ctx = app::Context {
        verbose: cli.verbose,
        config,
    };
    print_info!(ctx, 1, "Context generated!");

    print_info!(ctx, 1, "Fetching schema...");
    let schema = introspection::Response::fetch(&ctx.config).await?.schema();
    print_info!(ctx, 1, "Schema fetched!");

    if ctx.config.emit_schema {
        print_info!(ctx, 1, "Emitting schema json");
        // FIXME: Doesn't really make sense to serialize the schema again when we already recieved it in serial form...
        let schema_json =
            serde_json::to_string_pretty(&schema).expect("recieved valid schema json");
        cross::fs::write_to_file("schema.json", &schema_json)?;
    }

    print_info!(ctx, 1, "Generating typescript...");
    let ts = generate_typescript(cli, ctx.config, schema).await?;

    cross::fs::write_to_file("generated.ts", &ts)?;

    Ok(())
}
