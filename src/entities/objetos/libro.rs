use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use surrealdb::sql::Thing;
#[cfg(feature = "ssr")]
use crate::backend::infrastructure::db::{LibroDB, UbicacionDB};
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
    ubicacion: Ubicacion,
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
        ubicacion: Ubicacion,
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
            ubicacion,
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

    pub fn ubicacion(&self) -> &Ubicacion {
        &self.ubicacion
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

    pub fn set_ubicacion(&mut self, ubicacion: Ubicacion) {
        self.ubicacion = ubicacion;
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
            ubicacion: Ubicacion::from_db(libro.ubicacion().to_owned()),
        }
    }
    #[cfg(feature = "ssr")]
    pub fn to_db(self) -> LibroDB {
        LibroDB::new(self.id.map(|id|Thing::from(("libros", id.as_str()))),self.titulo,self.autor,self.isbn,self.editorial,self.pub_year,self.edicion,self.paginas,self.ubicacion.to_db())
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub enum Ubicacion {
    #[default]
    None,
    Usuario(Persona),
    Iglesia(Iglesia)
}

#[cfg(feature = "ssr")]
impl Ubicacion {
    pub fn to_db(self) -> UbicacionDB {
        match self {
            Ubicacion::None => UbicacionDB::None,
            Ubicacion::Usuario(u) => UbicacionDB::Usuario(u.to_db_no_pass()),
            Ubicacion::Iglesia(i) => UbicacionDB::Iglesia(i.to_db())
        }
    }

    pub fn from_db(ubicacion: UbicacionDB) -> Self{
        match ubicacion {
            UbicacionDB::None => Ubicacion::None,
            UbicacionDB::Iglesia(i) => Ubicacion::Iglesia(Iglesia::from_db(i)),
            UbicacionDB::Usuario(u) => Ubicacion::Usuario(Persona::from_db(u)),
        }
    }
}