use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct FileStatus {
    pub status: String,
    pub path: String,
}

#[derive(Serialize, Clone)]
pub struct RepoInfo {
    pub branch: String,
    pub statuses: Vec<FileStatus>,
    pub is_clean: bool,
}
