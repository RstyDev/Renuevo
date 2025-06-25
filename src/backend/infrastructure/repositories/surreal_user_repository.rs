use crate::{
    backend::{
        domain::repositories::user_repository::UserRepository,
        infrastructure::db::{
            connection::{establish_connection, DBPool},
            models::users::PersonaDB,
        },
    },
    entities::Persona,
    error::{AppError, AppRes},
};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct SurrealUserRepository {
    pool: DBPool,
}

impl SurrealUserRepository {
    pub async fn new() -> Self {
        Self {
            pool: establish_connection().await,
        }
    }
}

impl UserRepository for Arc<SurrealUserRepository> {


    async fn save(&self, user: &Persona) -> AppRes<()> {
        if user.password().unwrap().len() < 6 {
            return Err(AppError::ValidationErr(
                39,
                String::from("La password debe ser igual o mayor a 6 caracteres"),
            ));
        }
        let res = self
            .pool
            .query(
                r#"
        insert into personas {
            password: crypto::sha512($hermano.password),
            nombre: $hermano.nombre,
            apellido: $hermano.apellido,
            nacimiento: $hermano.nacimiento,
            estado_civil: $hermano.estado_civil,
            estado: $hermano.estado,
            registrado: $hermano.registrado,

        }
        "#,
            )
            .bind(("hermano", user.clone()))
            .await;
        match res {
            Ok(a) => {
                println!("{:#?}", a);
                Ok(())
            }
            Err(e) => Err(AppError::DBErr(41, e.to_string())),
        }
    }

    async fn delete(&self, id: &str) -> AppRes<()> {
        self.pool
            .delete::<Option<PersonaDB>>(("personas", id))
            .await
            .map_err(|e| AppError::DBErr(68, e.to_string()))?;
        Ok(())
    }

    async fn get_all(&self) -> AppRes<Vec<Persona>> {
        match self.pool.select::<Vec<PersonaDB>>("personas").await {
            Ok(res) => Ok(res
                .into_iter()
                .map(|persona| Persona::from_db(persona))
                .collect::<Vec<Persona>>()),
            Err(e) => Err(AppError::DBErr(75, e.to_string())),
        }
    }

    async fn get_by_id(&self, id: &str) -> AppRes<Option<Persona>> {
        match self
            .pool
            .select::<Option<PersonaDB>>(("personas", id))
            .await
        {
            Ok(Some(hermano)) => Ok(Some(Persona::from_db(hermano))),
            Ok(None) => Ok(None),
            Err(e) => Err(AppError::DBErr(86, e.to_string())),
        }
    }

    async fn update(&self, persona: Persona) -> AppRes<()> {
        match persona.id() {
            None => Err(AppError::DBErr(92, "User sent without ID".to_string())),
            Some(id) => {
                //let resource = Resource::new(("","").into());
                //match db.update::<Option<Hermano>>(resource.into()).content(user.into()).await {
                match self
                    .pool
                    .upsert::<Option<PersonaDB>>(("personas", id.clone()))
                    .content(persona.to_db())
                    .await
                {
                    Ok(_) => Ok(()),
                    Err(e) => Err(AppError::DBErr(103, e.to_string())),
                }
            }
        }
    }
}
