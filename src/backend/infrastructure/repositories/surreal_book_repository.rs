use crate::{
    backend::{
        domain::repositories::BookRepository,
        infrastructure::db::{establish_connection, DBPool},
    },
    error::{AppError, AppRes},
};
use std::sync::Arc;
use crate::backend::infrastructure::db::{LibroDB, UbicacionDB};
use crate::entities::{Libro, Ubicacion};

#[derive(Clone)]
pub struct SurrealBookRepository {
    pool: DBPool,
}

impl SurrealBookRepository {
    pub async fn new() -> Self {
        Self {
            pool: establish_connection().await,
        }
    }
}

impl BookRepository for Arc<SurrealBookRepository> {
    /*
        id: Option<Thing>,
    titulo: String,
    autor: String,
    isbn: String,
    editorial: String,
    pub_year: u16,
    edicion: u8,
    paginas: u16,
    ubicacion: Option<Thing>,
    */
    async fn save(&self, book: Libro) -> AppRes<()> {
        println!("Saving Book");
        let ubicacion = book.ubicacion().clone();
        let res = self.pool.query(r#"
        insert into libros {
            titulo: $libro.titulo,
            autor: $libro.autor,
            isbn: $libro.isbn,
            editorial: $libro.editorial,
            pub_year: $libro.pub_year,
            edicion: $libro.edicion,
            paginas: $libro.paginas,
            ubicacion: $ubicacion
        }
        "#).bind(("libro",book)).bind(("ubicacion",match ubicacion{
            Ubicacion::None => {UbicacionDB::None}
            Ubicacion::Usuario(u) => UbicacionDB::Usuario(u.to_db_no_pass()),
            Ubicacion::Iglesia(i) => UbicacionDB::Iglesia(i.to_db())
        })).await;
        match res {
            Ok(a) => {
                println!("{:#?}", a);
                Ok(())
            }
            Err(e) => Err(AppError::DBErr(55, e.to_string())),
        }
    }

    async fn delete(&self, id: &str) -> AppRes<()> {
        self.pool
            .delete::<Option<LibroDB>>(("libros", id))
            .await
            .map_err(|e| AppError::DBErr(68, e.to_string()))?;
        Ok(())
    }

    async fn get_all(&self) -> AppRes<Vec<Libro>> {
        match self.pool.query(r#"SELECT * FROM libros;"#).await.unwrap().take::<Vec<LibroDB>>(0) {
            Ok(res) => {
                let mut libros = vec![];
                for libro in res {
                    libros.push(Libro::from_db(libro));
                }
                Ok(libros)
            },
            Err(e) => Err(AppError::DBErr(87, e.to_string())),
        }
    }

    async fn get_by_id(&self, id: &str) -> AppRes<Option<Libro>> {
        match self.pool.select::<Option<LibroDB>>(("libros",id)).await {
            Ok(res) => {
                // let mut libros = vec![];
                match res {
                    None => Ok(None),
                    Some(libro) => {
                        Ok(Some(Libro::from_db(libro)))
                    }
                }
            },
            Err(e) => Err(AppError::DBErr(96, e.to_string())),
        }
    }

    async fn get_by_user(&self, user: &str) -> AppRes<Vec<Libro>> {
        match self.pool.query(r#"
            select * from libros where user = $user
        "#).bind(format!("personas:{}",user)).await {
            Ok(mut res) => {
                let res = res.take::<Vec<LibroDB>>(0).map_err(|e| AppError::DBErr(105, e.to_string()))?;
                let mut libros = vec![];
                for libro in res {
                    libros.push(Libro::from_db(libro));
                }
                Ok(libros)
            }
            Err(e) => Err(AppError::DBErr(108,e.to_string()))
        }
    }

    async fn update(&self, libro: Libro) -> AppRes<Libro> {
        match libro.id() {
            None => Err(AppError::DBErr(92, "Book sent without ID".to_string())),
            Some(id) => {
                match self
                    .pool
                    .upsert::<Option<LibroDB>>(("libros", id))
                    .content(libro.to_db())
                    .await
                {
                    Ok(Some(libro)) => {
                        Ok(Libro::from_db(libro))
                    },
                    Ok(None) => Err(AppError::NotFound(138)),
                    Err(e) => Err(AppError::DBErr(134, e.to_string())),
                }
            }
        }
    }
}
