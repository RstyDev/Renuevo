use crate::{entities::Persona, error::AppRes};

pub trait UserRepository {
    async fn find_by_email(&self, email: &str) -> Option<Persona>;

    async fn save(&self, user: &Persona) -> AppRes<()>;

    async fn delete(&self, id: &str) -> AppRes<()>;
    async fn get_all(&self) -> AppRes<Vec<Persona>>;
    async fn get_by_id(&self, id: &str) -> AppRes<Option<Persona>>;
    async fn update(&self, persona: Persona) -> AppRes<()>;
}
