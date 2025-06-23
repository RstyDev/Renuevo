use crate::{
    backend::domain::{
        repositories::user_repository::UserRepository, services::user_service::UserService,
    },
    entities::Persona,
};

pub struct GetUserByEmailUseCase<T: UserRepository> {
    user_service: UserService<T>,
}

impl<T: UserRepository> GetUserByEmailUseCase<T> {
    pub fn new(user_repo: T) -> Self {
        let user_service = UserService::new(user_repo);
        GetUserByEmailUseCase { user_service }
    }

    pub async fn get_by_email(&self, email: &str) -> Option<Persona> {
        self.user_service.get_by_email(email).await
    }
}
