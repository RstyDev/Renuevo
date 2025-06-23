use crate::backend::infrastructure::db::models::families::FamiliaDB;
use crate::entities::Familia;
use crate::{
    backend::{
        domain::repositories::family_repository::FamilyRepository,
        infrastructure::db::connection::{establish_connection, DBPool},
    },
    error::{AppError, AppRes},
};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct SurrealFamilyRepository {
    pool: DBPool,
}

impl SurrealFamilyRepository {
    pub async fn new(env_map: HashMap<String, String>) -> Self {
        Self {
            pool: establish_connection(env_map).await,
        }
    }
}

impl FamilyRepository for Arc<SurrealFamilyRepository> {
    async fn save(&self, familia: &Familia) -> AppRes<()> {
        // let res = self
        //     .pool
        //     .query(
        //         r#"
        // insert into familias {
        //     apellido: $hermano.apellido,
        //     padre: $hermano.nacimiento,
        //     madre: $hermano.estado_civil,
        //     hijos: $hermano.estado,
        //     registrado: $hermano.registrado,
        //
        // }
        // "#,
        //     )
        //     .bind(("hermano", user.clone()))
        //     .await;
        // match res {
        //     Ok(a) => {
        //         println!("{:#?}", a);
        //         Ok(())
        //     }
        //     Err(e) => Err(AppError::DBErr(41, e.to_string())),
        // }
        Ok(())
    }

    async fn delete(&self, id: &str) -> AppRes<()> {
        self.pool
            .delete::<Option<FamiliaDB>>(("familias", id))
            .await
            .map_err(|e| AppError::DBErr(68, e.to_string()))?;
        Ok(())
    }

    async fn get_all(&self) -> AppRes<Vec<Familia>> {
        match self.pool.select::<Vec<FamiliaDB>>("familias").await {
            Ok(res) => Ok(res
                .into_iter()
                .map(|persona| Familia::from_db(persona))
                .collect::<Vec<Familia>>()),
            Err(e) => Err(AppError::DBErr(75, e.to_string())),
        }
    }

    async fn get_by_id(&self, id: &str) -> AppRes<Option<Familia>> {
        match self
            .pool
            .select::<Option<FamiliaDB>>(("familias", id))
            .await
        {
            Ok(Some(hermano)) => Ok(Some(Familia::from_db(hermano))),
            Ok(None) => Ok(None),
            Err(e) => Err(AppError::DBErr(86, e.to_string())),
        }
    }

    async fn update(&self, familia: Familia) -> AppRes<()> {
        match familia.id() {
            None => Err(AppError::DBErr(92, "Family sent without ID".to_string())),
            Some(id) => {
                //let resource = Resource::new(("","").into());
                //match db.update::<Option<Hermano>>(resource.into()).content(user.into()).await {
                match self
                    .pool
                    .upsert::<Option<FamiliaDB>>(("familias", id.clone()))
                    .content(familia.to_db()?)
                    .await
                {
                    Ok(_) => Ok(()),
                    Err(e) => Err(AppError::DBErr(103, e.to_string())),
                }
            }
        }
    }
}
