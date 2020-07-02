use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData
{
    pub name: String,
    pub age: u16
}