use crate::{
    backend::domain::{repositories::UserRepository, services::UserService},
    entities::Persona,
    error::AppRes,
};

pub struct GetUserByIdUseCase<T: UserRepository> {
    user_service: UserService<T>,
}

impl<T: UserRepository> GetUserByIdUseCase<T> {
    pub fn new(user_repo: T) -> Self {
        let user_service = UserService::new(user_repo);
        GetUserByIdUseCase { user_service }
    }

    pub async fn get_by_id(&self, id: &str) -> AppRes<Option<Persona>> {
        self.user_service.get_by_id(id).await
    }

    pub async fn get_by_id_with_password(&self, id: &str) -> AppRes<Option<Persona>> {
        self.user_service.get_by_id_with_password(id).await
    }
}
