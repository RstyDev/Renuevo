
/*


    async fn save(&self, book: Libro) -> AppRes<()>;
    async fn delete(&self, id: &str) -> AppRes<()>;
    async fn get_all(&self) -> AppRes<Vec<Libro>>;
    async fn get_by_id(&self, id: &str) -> AppRes<Option<Libro>>;
    async fn get_by_user(&self, user: &str) -> AppRes<Vec<Libro>>;
    async fn update(&self, persona: Libro) -> AppRes<Libro>;


*/
mod save;
mod delete;
mod get_all;
mod get_by_id;
mod update;

pub use save::*;
pub use delete::*;
pub use get_all::*;
pub use get_by_id::*;
pub use update::*;