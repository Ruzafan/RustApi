use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct user_data
{
    pub name: String,
    pub age: u16
}

impl user_data {
    
}