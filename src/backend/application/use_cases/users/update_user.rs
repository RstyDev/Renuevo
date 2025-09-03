use crate::{
    backend::domain::{repositories::UserRepository, services::UserService},
    entities::Persona,
    error::AppRes,
};
use crate::entities::Libro;

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
    pub async fn add_book(&self, id: &str, libro: Libro) -> AppRes<()> {
        self.user_service.add_book(id,libro).await
    }
    pub async fn is_id_pass_correct(&self, id: &str, password: &str) -> AppRes<bool> {
        self.user_service.is_id_pass_correct(id,password).await
    }
    pub async fn update_password(&self, id: &str, password: &str) -> AppRes<()> {
        self.user_service.update_password(id, password).await
    }
}
