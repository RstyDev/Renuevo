use crate::{
    backend::domain::{
        repositories::user_repository::UserRepository, services::user_service::UserService,
    },
    entities::Persona,
    error::AppRes,
};

pub struct UpdateUserUseCase<T: UserRepository> {
    user_service: UserService<T>,
}

impl<T: UserRepository> UpdateUserUseCase<T> {
    pub fn new(user_repo: T) -> Self {
        let user_service = UserService::new(user_repo);
        UpdateUserUseCase { user_service }
    }

    pub async fn update(&self, persona: Persona) -> AppRes<Persona> {
        self.user_service.update(persona).await
    }
}
