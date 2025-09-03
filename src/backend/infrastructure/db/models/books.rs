use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::backend::infrastructure::db::{IglesiaDB, PersonaDB};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct LibroDB {
    id: Option<Thing>,
    titulo: String,
    autor: String,
    isbn: String,
    editorial: String,
    pub_year: u16,
    edicion: u16,
    paginas: u16,
    ubicacion: UbicacionDB,
}

impl LibroDB {

    pub fn id(&self) -> &Option<Thing> {
        &self.id
    }

    pub fn titulo(&self) -> &str {
        &self.titulo
    }

    pub fn autor(&self) -> &str {
        &self.autor
    }

    pub fn isbn(&self) -> &str {
        &self.isbn
    }

    pub fn editorial(&self) -> &str {
        &self.editorial
    }

    pub fn pub_year(&self) -> u16 {
        self.pub_year
    }

    pub fn edicion(&self) -> u16 {
        self.edicion
    }

    pub fn paginas(&self) -> u16 {
        self.paginas
    }

    pub fn ubicacion(&self) -> &UbicacionDB {
        &self.ubicacion
    }

    pub fn new(id: Option<Thing>, titulo: String, autor: String, isbn: String, editorial: String, pub_year: u16, edicion: u16, paginas: u16, ubicacion: UbicacionDB) -> Self {
        Self { id, titulo, autor, isbn, editorial, pub_year, edicion, paginas, ubicacion }
    }
}
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub enum UbicacionDB{
    #[default]
    None,
    Iglesia(IglesiaDB),
    Usuario(PersonaDB)
}