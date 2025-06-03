use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::time;

mod cmd;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(
        long("build-tmpl"),
        help("Build metadata template. Comma separated list of ID templates."),
        env("$GSG_BUILD_TMPL")
    )]
    build_tmpl: Option<String>,

    #[arg(
        long("bump-commit-tmpl"),
        help("Template for bump commit message"),
        env("$GSG_BUILD_TMPL"),
        default_value("chore: version bump for {{tag}} [skip ci]")
    )]
    bump_commit_tmpl: String,

    #[arg(
        long("debug"),
        help("Enable debugging using extensive logs"),
        env("$GSG_DEBUG")
    )]
    debug: Option<bool>,

    //TODO: add default value
    #[arg(
        long("initial-development"),
        help(
            "set this to false when you're ready to release 1.0.0,
			    ignored if version is already >= 1.0.0"
        ),
        env("$GSG_INITIAL_DEVELOPMENT"),
        default_value_t = true
    )]
    initial_development: bool,

    #[arg(
        long("gl-api"),
        help("gitlab api URL"),
        env("$GITLAB_URL"),
        default_value("https://gitlab.com/api/v4/")
    )]
    server_api: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(
        name("next-version"),
        about("Computes next version based on last tag in tree and provided prefix")
    )]
    NextVersion(cmd::next_version::ComputeNextVersionArgs),
    // #[command(
    //     name("generate-tag"),
    //     about("Generates gitlab tag with provided version")
    // )]
    // GenerateTag,
    //
    // #[command(
    //     name("generate-release"),
    //     about("Generates gitlab tag with provided version")
    // )]
    // GenerateRelease,
}

fn main() -> Result<()> {
    let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stderr());

    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_file(true)
        .with_line_number(true)
        .with_writer(non_blocking)
        .with_timer(time::LocalTime::rfc_3339())
        .init();

    let cli = Cli::parse();
    match &cli.command {
        Commands::NextVersion(args) => {
            tracing::event!(
                tracing::Level::INFO,
                msg = "Starting to compute next version",
            );
            let _ = cmd::next_version::compute_next_version(args)?;
        }
    }
    return Ok(());
}
