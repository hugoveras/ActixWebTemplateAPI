use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub firstname: String,
    pub lastname : String,
    pub email: String,

}

