use actix_web_thiserror::ResponseError;
use fuzion_commons::db::{PgClient, PgClientError};
use thiserror::Error;

pub struct UserService;

impl UserService {
  pub async fn upsert_user(_db_client: PgClient<'_>) {}
}

#[derive(Debug, Error, ResponseError)]
pub enum UserServiceError {
  #[error("internal error")]
  InternalError,
  #[error(transparent)]
  PostgresError(#[from] PgClientError),
}
