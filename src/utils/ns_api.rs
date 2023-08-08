use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct NationLogin {
    name: String,
    x_autologin: String
}

pub struct NsAPI {
    
}