use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Libro {
    titulo: String,
    autor: String,
    isbn: String,
    editorial: String,
    pub_year: u16,
    edicion: u8,
    paginas: u16,
    ubicacion: String,
}

impl Libro {
    pub fn new(
        titulo: String,
        autor: String,
        isbn: String,
        editorial: String,
        pub_year: u16,
        edicion: u8,
        paginas: u16,
        ubicacion: String,
    ) -> Self {
        Self {
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
}
