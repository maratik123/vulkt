use thiserror::Error;

pub type AppResult<R> = Result<R, AppError>;

#[derive(Error, Debug)]
pub enum AppError {}
