use crate::helper;
use anyhow::{Ok, Result};
use clap::Args;

#[derive(Debug, Args)]
#[command(author, version, about, long_about = None)]
pub struct ComputeNextVersionArgs {
    #[
        arg(
            long("bump-patch"),
            help("Force patch bump when none of tthe commits would otherwise trigger a bump.\nFirst lines of commits will be listed under 'Fixes' in changelog
                and release note. If there are commits that trigger a bump, this flag is ignored."), 
            env("$GSG_BUMP_PATCH")
        )
    ]
    bump_patch: Option<bool>,

    #[arg(
        long("hotfix-branches"),
        help("Comma separated list of branch names for hotfixes."),
        env("$GSG_HOTFIX_BRANCHES"),
        default_value("^rel-")
    )]
    hotfix_branches: String,

    #[arg(
        long("minor-commit-types"),
        help("comma separated list of commit message types that indicate minor bump"),
        env("$GSG_MINOR_COMMIT_TYPES"),
        default_value("feat")
    )]
    commit_types_minor: String,

    #[arg(
        long("patch-commit-types"),
        help("comma separated list of commit message types that indicate patch bump"),
        env("$GSG_PATCH_COMMIT_TYPES"),
        default_value("fix,refactor,perf,docs,style,test")
    )]
    patch_commit_types: String,

    #[arg(
        long("omit-commit-types"),
        help("comma separated list of commit message types that indicate major bump"),
        env("$GSG_OMIT_COMMIT_TYPES")
    )]
    omit_commit_types: Option<String>,

    #[arg(
        long("pre-tmpl"),
        help("Pre-release version template. Comma separated list of ID templates"),
        env("$GSG_PRE_TMPL"),
        default_value("{{env 'CI_COMMIT_BRANCH'}},{{env 'TIME_NOW'}}")
    )]
    pre_tmpl: String,

    #[arg(
        long("release-branches"),
        help(
            "Comma separated list of branch names. If release-branches is defined, normal
                releases can be done from listed brances. Other brances will produce pre-release
                versions. If release-brances is not defined or is empty string, all branches will
                produce normal releases. WARNING: this is an experimental feature"
        ),
        env("$GSG_RELEASE_BRANCHES")
    )]
    release_branches: Option<String>,

    #[arg(
        long("tag-prefix"),
        help("Prefix to use in version tags"),
        env("$GSG_TAG_PREFIX"),
        default_value("v")
    )]
    tag_prefix: String,
}

pub fn compute_next_version(args: &ComputeNextVersionArgs) -> Result<()> {
    println!("command args: {:?}", args);
    //tracing::event!(tracing::Level::DEBUG, msg = "Command args", args = args,);
    //let git_dir_path = helper::git_heler::detect_git_dir()?;
    let git_repo = helper::git_helper::git_repository_open(".")?;
    let _ = git_repo.get_last_tag()?;
    return Ok(());
}
