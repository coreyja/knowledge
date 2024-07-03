pub mod process_article;
pub mod sessions;

cja::impl_job_registry!(
    crate::AppState,
    sessions::Cleanup,
    process_article::ProcessArticle
);
