use octocrab::{models::issues::Comment, models::repos::DiffEntry, Error, Octocrab, OctocrabBuilder, Page};
use std::sync::Arc;

pub struct GhAPIClient<'a> {
    owner: &'a str,
    gh_repo: &'a str,
    instance: Arc<Octocrab>,
}

impl<'a> GhAPIClient<'a> {
    pub fn new(gh_token: &'a str, github_repo: &'a str) -> Self {
        let split_values = github_repo.split("/").collect::<Vec<_>>();
        let owner = split_values[0];
        let gh_repo = split_values[1];
        println!("Owner: {owner}, Repo: {gh_repo}");

        let octocrab_builder = OctocrabBuilder::new()
            .personal_token(gh_token)
            .build()
            .expect("Failed to initialize Octocrab");

        GhAPIClient {
            owner,
            gh_repo,
            instance: Arc::new(octocrab_builder),
        }
    }

    pub async fn get_pull_request_files(&self, pr_number: u64) -> Result<Page<DiffEntry>, Error> {
        self.instance
            .pulls(self.owner, self.gh_repo)
            .list_files(pr_number)
            .await
    }

    pub async fn post_issue_comment(&self, pr_number: u64, comment: &str) -> Result<Comment, Error> {
        self.instance
            .issues(self.owner, self.gh_repo)
            .create_comment(pr_number, comment)
            .await
    }
}