use crate::models::repository::user_repository::{UserRepository, UserRepositoryTrait};
use crate::configs::db::establish_connection;
use crate::models::entity::user_entity::FormData;

pub trait RegisterServiceTrait{
    fn register(&self,user:FormData)->Result<(), String>;
}
pub struct RegisterService;
impl RegisterServiceTrait for RegisterService{
    fn register(&self,user: FormData)->Result<(), String>{
        let pool = establish_connection();
        let repository = UserRepository;
        repository.insert_user(&pool,&user)
            .map_err(|e| e.to_string())
    }
}
