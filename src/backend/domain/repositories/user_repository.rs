#![allow(async_fn_in_trait)]
use crate::{entities::Persona, error::AppRes};
use crate::entities::Libro;

pub trait UserRepository {
    async fn save(&self, user: &Persona) -> AppRes<()>;
    async fn delete(&self, id: &str) -> AppRes<()>;
    async fn get_all(&self) -> AppRes<Vec<Persona>>;
    async fn get_by_id(&self, id: &str) -> AppRes<Option<Persona>>;
    async fn is_id_pass_correct(&self, id: &str, password: &str) -> AppRes<bool>;
    async fn get_by_id_with_password(&self, id: &str) -> AppRes<Option<Persona>>;
    async fn update(&self, persona: Persona) -> AppRes<Persona>;

    async fn update_password(&self, id: &str, password: &str) -> AppRes<()>;
    async fn add_book(&self, id: &str, libro: Libro) -> AppRes<()>;

}
