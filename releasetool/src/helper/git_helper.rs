use crate::Error;
use anyhow::Result;
use git2::Repository;
use std::path::Path;

pub struct GitRepositoryHelper {
    repository: Repository,
}

pub fn git_repository_open(repository_path: &str) -> Result<GitRepositoryHelper, Error> {
    let repo_path = Path::new(repository_path);
    let repo = Repository::discover(repo_path).map_err(|e| {
        tracing::event!(
            tracing::Level::ERROR,
            msg = "While opening git repository",
            err = e.to_string(),
        );

        Error::ErrRepositoryOpen
    })?;

    let repo_helper = GitRepositoryHelper { repository: repo };
    Ok(repo_helper)
}

impl GitRepositoryHelper {
    pub fn get_last_tag(&self) -> Result<String, Error> {
        let tags = self.repository.tag_names(Some("*")).map_err(|e| {
            tracing::event!(
                tracing::Level::ERROR,
                msg = "While reading tag names from repository",
                err = e.to_string(),
            );

            Error::ErrTagNamesRead
        })?;

        // tracing::event!(tracing::Level::WARN, msg = "What do I have?", tags);

        println!("{:?}", tags.len());

        Ok("".to_string())
    }
}
