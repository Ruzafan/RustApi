use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize)]
pub struct Login {
    pub email: String,
    pub password: String
}