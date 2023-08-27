use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct NationLogin {
    pub nation: String,
    pub secret: String
}

pub struct NsAPI {
    
}