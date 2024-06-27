pub mod sessions;
pub mod process_article;

cja::impl_job_registry!(crate::AppState, sessions::Cleanup, process_article::ProcessArticle);
