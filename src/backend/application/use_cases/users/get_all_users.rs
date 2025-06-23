use crate::{
    backend::domain::{
        repositories::user_repository::UserRepository, services::user_service::UserService,
    },
    entities::Persona,
    error::AppRes,
};

pub struct GetAllUsersUseCase<T: UserRepository> {
    user_service: UserService<T>,
}

impl<T: UserRepository> GetAllUsersUseCase<T> {
    pub fn new(user_repo: T) -> Self {
        let user_service = UserService::new(user_repo);
        GetAllUsersUseCase { user_service }
    }

    pub async fn get_all(&self) -> AppRes<Vec<Persona>> {
        self.user_service.get_all().await
    }
}
