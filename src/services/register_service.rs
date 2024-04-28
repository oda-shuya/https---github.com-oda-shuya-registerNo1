use crate::models::repository::user_repository::{UserRepository, UserRepositoryTrait};
use crate::configs::db::establish_connection;
use crate::models::entity::user_entity::FormData;

pub trait RegisterServiceTrait{
    fn register(&self,user:FormData);
}
pub struct RegisterService;
impl RegisterServiceTrait for RegisterService{
    fn register(&self,user: FormData){
        let pool = establish_connection();
        let repository = UserRepository;
        repository.insert_user(&pool,&user);
    }
}
