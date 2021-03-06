use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize)]
pub struct UserData
{
    pub _id: String,
    pub name: String,
    pub age: i32,
    pub email: String,
    pub password: String
}

