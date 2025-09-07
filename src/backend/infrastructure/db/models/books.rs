use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;


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
    prestamo: PrestamoLibroDB
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

    pub fn new(id: Option<Thing>, titulo: String, autor: String, isbn: String, editorial: String, pub_year: u16, edicion: u16, paginas: u16, prestamo: PrestamoLibroDB) -> Self {
        Self { id, titulo, autor, isbn, editorial, pub_year, edicion, paginas, prestamo }
    }

    pub fn prestamo(&self) -> &PrestamoLibroDB {
        &self.prestamo
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub enum PrestamoLibroDB {
    #[default]
    None,
    Usuario{
        id: Thing,
        dias: u16,
        fecha: NaiveDate,
    }
}