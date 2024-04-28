use mysql::*;
use mysql::prelude::*;
use crate::models::entity::user_entity::FormData;
pub trait UserRepositoryTrait{
    fn insert_user(&self, pool : &Pool, user:&FormData)->Result<(), Box<dyn std::error::Error>>;
}

pub struct UserRepository;
impl UserRepositoryTrait for UserRepository{
    fn insert_user(&self,pool:&Pool,user:&FormData)->Result<() ,Box<dyn std::error::Error>>{
        let mut conn = pool.get_conn().expect("コネクションプールに接続できません");
        conn.exec_drop(
            r"insert into practice.m_user (username, mailaddress, password) VALUES (:username, :mailaddress, :password)",
            params! {
                "username" => &user.username,
                "mailaddress" => &user.mailaddress,
                "password" => &user.password,
            }
        )?;
        Ok(())
    }
}

