use serde::{Deserialize, Serialize};
use utoipa::ToSchema;



/// Todo operation errors
#[derive(Serialize, Deserialize, ToSchema)]
pub enum AppdError {
    #[schema(example = "missing api key")]
    Unauthorized(String),
}