use serde::ser::{Serialize,SerializeStruct, Serializer};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct user_data
{
    pub name: String,
    pub age: i32,
    pub email: String
}

impl Serialize for user_data {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut user = serializer.serialize_struct("user_data", 3)?;
        user.serialize_field("name", &self.name)?;
        user.serialize_field("age", &self.age)?;
        user.end()
    }
}