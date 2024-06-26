pub mod sessions;

cja::impl_job_registry!(crate::AppState, sessions::Cleanup);
