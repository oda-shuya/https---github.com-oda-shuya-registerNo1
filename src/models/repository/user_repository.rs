use mysql::*;
use mysql::prelude::*;
use crate::models::entity::user_entity::FormData;
pub trait UserRepositoryTrait{
    fn insert_user(&self, pool : &Pool, user:&FormData)->bool;
}

pub struct UserRepository;
impl UserRepositoryTrait for UserRepository{
    fn insert_user(&self,pool:&Pool,user:&FormData)->bool{
        let mut conn = pool.get_conn().expect("コネクションプールに接続できません");
        conn.exec_drop(
            r"INSERT INTO pokupoku.users (username, mailaddress, password) VALUES (:username, :mailaddress, :password)",
            params! {
                "username" => &user.username,
                "mailaddress" => &user.mailaddress,
                "password" => &user.password,
            }
        ).is_ok()
    }
    
}

