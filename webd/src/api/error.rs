use serde::{Deserialize, Serialize};
use utoipa::ToSchema;



/// Todo operation errors
#[derive(Serialize, Deserialize, ToSchema)]
pub enum WebdError {
    #[schema(example = "missing api key")]
    Unauthorized(String),
}