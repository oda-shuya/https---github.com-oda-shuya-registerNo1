use serde::{Serialize,Deserialize};
#[derive(Serialize,Deserialize,Clone)]
pub struct FormData{
    pub username : String,
    pub mailaddress : String,
    pub password : String
}