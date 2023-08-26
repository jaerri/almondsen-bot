use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct NationLogin {
    name: String,
    secret: String
}

pub struct NsAPI {
    
}