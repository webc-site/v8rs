use std::result;

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
  #[error("Input queue is full (capacity: {capacity})")]
  QueueFull { capacity: usize },
  #[error("Job not found: {job_id}")]
  JobNotFound { job_id: u64 },
  #[error("Invalid task index: {index}")]
  InvalidTaskIndex { index: usize },
  #[error("Job already finalized: {job_id}")]
  JobAlreadyFinalized { job_id: u64 },
  #[error("Job aborted: {job_id}")]
  JobAborted { job_id: u64 },
  #[error("Dispatcher error: {0}")]
  Message(String),
}

pub type Result<T> = result::Result<T, Error>;
