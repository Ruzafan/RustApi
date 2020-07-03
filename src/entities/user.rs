use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize)]
pub struct UserData
{
    pub name: String,
    pub age: i32,
    pub email: String
}
