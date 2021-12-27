use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[derive(Default)]
pub struct ApplicationError {
    pub errortype : String,
    pub errordescription : String,
    pub functionname : String
}

#[derive(Debug, Deserialize)]
#[derive(RustcDecodable, RustcEncodable)]
#[derive(Default)]
pub struct KeyValueStruct  {
    pub  key: String,
    pub value: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize,Clone, Default)]
pub struct DatabaseSetup {
    pub  databaseurl : String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize,Clone, Default)]
pub  struct BodyParameter {
    pub CustomerId : String,
    // pub Query : serde_json::Value,
}


#[derive(Debug, Serialize, Deserialize, Default)]
pub  struct APIResponse {
    pub response : Vec<String>,
    pub errors : Vec<ApplicationError>
}
