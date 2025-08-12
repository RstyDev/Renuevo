use crate::{
    backend::domain::repositories::user_repository::UserRepository, entities::Persona,
    error::AppRes,
};

#[derive(Clone)]
pub struct UserService<T: UserRepository> {
    user_repo: T,
}

impl<T: UserRepository> UserService<T> {
    pub fn new(user_repo: T) -> Self {
        Self { user_repo }
    }
    pub async fn register_user(&self, user: Persona) -> AppRes<()> {
        self.user_repo.save(&user).await
    }

    pub async fn get_by_id(&self, id: &str) -> AppRes<Option<Persona>> {
        self.user_repo.get_by_id(id).await
    }
    pub async fn get_by_id_with_password(&self, id: &str) -> AppRes<Option<Persona>> {
        self.user_repo.get_by_id_with_password(id).await
    }
    pub async fn delete(&self, id: &str) -> AppRes<()> {
        self.user_repo.delete(id).await
    }
    pub async fn get_all(&self) -> AppRes<Vec<Persona>> {
        self.user_repo.get_all().await
    }
    pub async fn update(&self, persona: Persona) -> AppRes<Persona> {
        self.user_repo.update(persona).await
    }
}
