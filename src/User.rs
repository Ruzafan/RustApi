use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData
{
    pub Name: String,
    pub Age: u16
}