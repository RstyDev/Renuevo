use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use surrealdb::sql::Thing;
#[cfg(feature = "ssr")]
use crate::backend::infrastructure::db::LibroDB;
use crate::backend::infrastructure::db::PrestamoLibroDB;
use crate::entities::objetos::iglesia::Iglesia;
use crate::entities::Persona;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Libro {
    id: Option<String>,
    titulo: String,
    autor: String,
    isbn: String,
    editorial: String,
    pub_year: u16,
    edicion: u16,
    paginas: u16,
    prestamo: PrestamoLibro
}

impl Libro {
    pub fn new(
        id: Option<String>,
        titulo: String,
        autor: String,
        isbn: String,
        editorial: String,
        pub_year: u16,
        edicion: u16,
        paginas: u16,
        prestamo: PrestamoLibro
    ) -> Self {
        Self {
            id,
            titulo,
            autor,
            isbn,
            editorial,
            pub_year,
            edicion,
            paginas,
            prestamo
        }
    }
    pub fn id(&self) -> Option<&str> {
        self.id.as_ref().map(|s| s.as_str())
    }
    pub fn set_id(&mut self, id: Option<String>) {
        self.id = id;
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



    pub fn set_titulo(&mut self, titulo: String) {
        self.titulo = titulo;
    }

    pub fn set_autor(&mut self, autor: String) {
        self.autor = autor;
    }

    pub fn set_isbn(&mut self, isbn: String) {
        self.isbn = isbn;
    }

    pub fn set_editorial(&mut self, editorial: String) {
        self.editorial = editorial;
    }

    pub fn set_pub_year(&mut self, pub_year: u16) {
        self.pub_year = pub_year;
    }

    pub fn set_edicion(&mut self, edicion: u16) {
        self.edicion = edicion;
    }

    pub fn set_paginas(&mut self, paginas: u16) {
        self.paginas = paginas;
    }

    #[cfg(feature = "ssr")]
    pub fn from_db(libro: LibroDB) -> Self {
        Self{
            id: libro.id().as_ref().map(|s| s.id.to_string()),
            titulo: libro.titulo().to_owned(),
            autor: libro.autor().to_owned(),
            isbn: libro.isbn().to_owned(),
            editorial: libro.editorial().to_owned(),
            pub_year: libro.pub_year(),
            edicion: libro.edicion(),
            paginas: libro.paginas(),
            prestamo: PrestamoLibro::from_db(libro.prestamo().to_owned())
        }
    }
    #[cfg(feature = "ssr")]
    pub fn to_db(self) -> LibroDB {
        LibroDB::new(self.id.map(|id|Thing::from(("libros", id.as_str()))),self.titulo,self.autor,self.isbn,self.editorial,self.pub_year,self.edicion,self.paginas,self.prestamo.to_db())
    }
}


#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub enum PrestamoLibro {
    #[default]
    None,
    Usuario{
        id: String,
        dias: u16,
        fecha: NaiveDate,
    }
}

impl PrestamoLibro {
    #[cfg(feature = "ssr")]
    pub fn to_db(self) -> PrestamoLibroDB {
        match self{
            PrestamoLibro::None => {PrestamoLibroDB::None}
            PrestamoLibro::Usuario { fecha, id, dias } => {PrestamoLibroDB::Usuario {fecha,dias, id: Thing::from(("personas", id.as_str()))}}
        }
    }
    #[cfg(feature = "ssr")]
    pub fn from_db(prestamo: PrestamoLibroDB) -> Self {
        match prestamo{
            PrestamoLibroDB::None => PrestamoLibro::None,
            PrestamoLibroDB::Usuario { fecha,id, dias } => {PrestamoLibro::Usuario {fecha,dias,id:id.id.to_string()}}
        }
    }
}