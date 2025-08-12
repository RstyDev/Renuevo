use crate::backend::infrastructure::db::models::families::FamiliaDB;
use crate::backend::infrastructure::db::models::users::PersonaDB;
use crate::entities::{Familia, Sexo};
use crate::{
    backend::{
        domain::repositories::family_repository::FamilyRepository,
        infrastructure::db::connection::{establish_connection, DBPool},
    },
    error::{AppError, AppRes},
};
use std::sync::Arc;
use surrealdb::kvs::KeyEncode;

#[derive(Clone)]
pub struct SurrealFamilyRepository {
    pool: DBPool,
}

impl SurrealFamilyRepository {
    pub async fn new() -> Self {
        Self {
            pool: establish_connection().await,
        }
    }
}

impl FamilyRepository for Arc<SurrealFamilyRepository> {
    async fn save(&self, familia: &Familia) -> AppRes<()> {
        let mut padre = None;
        if let Some(padre_local) = familia.padre() {
            match padre_local.id() {
                None => return Err(AppError::ValidationErr(32, String::from("Padre sin id"))),
                Some(id) => {
                    if let Some(hermano) = self
                        .pool
                        .select::<Option<PersonaDB>>(("personas", id))
                        .await
                        .map_err(|e| AppError::DBErr(38, e.to_string()))?
                    {
                        if hermano.sexo() == Sexo::Femenino {
                            return Err(AppError::ValidationErr(
                                35,
                                String::from("El padre debe ser masculino"),
                            ));
                        }
                        padre = Some(hermano)
                    }
                }
            }
        }

        let mut madre = None;
        if let Some(madre_local) = familia.madre() {
            match madre_local.id() {
                None => {
                    return Err(AppError::ValidationErr(
                        41,
                        "Madre necesita tener id".to_string(),
                    ))
                }
                Some(id) => {
                    if let Some(hermana) = self
                        .pool
                        .select::<Option<PersonaDB>>(("personas", id))
                        .await
                        .map_err(|e| AppError::DBErr(38, e.to_string()))?
                    {
                        if hermana.sexo() == Sexo::Masculino {
                            return Err(AppError::ValidationErr(
                                44,
                                String::from("La madre debe ser femenino"),
                            ));
                        }
                        madre = Some(hermana)
                    }
                }
            }
        }
        let mut hijos = vec![];
        for hijo in familia.hijos() {
            match hijo.id() {
                None => {
                    return Err(AppError::ValidationErr(
                        74,
                        format!("Hijo sin id: {}", hijo.nombre()),
                    ))
                }
                Some(id) => {
                    if let Some(hermano) = self
                        .pool
                        .select::<Option<PersonaDB>>(("personas", id))
                        .await
                        .map_err(|e| AppError::DBErr(38, e.to_string()))?
                    {
                        hijos.push(hermano);
                    }
                }
            }
        }
        let familia = FamiliaDB::new(
            None,
            match &padre {
                None => match &madre {
                    None => return Err(AppError::DBErr(63, String::from("No parent"))),
                    Some(madre) => madre.apellido().to_string(),
                },
                Some(padre) => padre.apellido().to_string(),
            },
            padre,
            madre,
            hijos,
        );


        let res = self
            .pool
            .query(
                r#"
        insert into familias {
            apellido: $familia.apellido,
            madre: $familia.madre.id,
            padre: $familia.padre.id,
            hijos: $familia.hijos.map(|$hijo|{$hijo.id}),
        }
        "#,
            )
            .bind(("familia", familia))
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
            .delete::<Option<FamiliaDB>>(("familias", id))
            .await
            .map_err(|e| AppError::DBErr(68, e.to_string()))?;
        Ok(())
    }

    async fn get_all(&self) -> AppRes<Vec<Familia>> {
        let mut res = self.pool.query(r#"
            SELECT * FROM familias FETCH padre, madre, hijos;
        "#).await.map_err(|e| AppError::DBErr(135, e.to_string()))?;
        let familias = res.take::<Vec<FamiliaDB>>(0).map_err(|e| AppError::DBErr(136, e.to_string()))?;
        Ok(familias.into_iter().map(|f|Familia::from_db(f)).collect())
    }

    async fn get_by_id(&self, id: &str) -> AppRes<Option<Familia>> {
        let mut res = self.pool.query(r#"
            SELECT * FROM ONLY type::thing($familia) FETCH padre, madre, hijos;
        "#).bind(("familia",format!("familias:{}",id))).await.map_err(|e| AppError::DBErr(143, e.to_string()))?;
        println!("{:#?}",res);
        let familia = res.take::<Option<FamiliaDB>>(0).map_err(|e| AppError::DBErr(143, e.to_string()))?;
        Ok(familia.map(|f|Familia::from_db(f))) //TODO hay que arreglar algo en ese query
        // match self
        //     .pool
        //     .select::<Option<FamiliaDB>>(("familias", id))
        //     .await
        // {
        //     Ok(Some(hermano)) => Ok(
        //         None
        //         // Some(get_familia_from_db(self.pool.clone(), hermano).await?)
        //     ),
        //     Ok(None) => Ok(None),
        //     Err(e) => Err(AppError::DBErr(86, e.to_string())),
        // }
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

// async fn get_familia_from_db(pool: DBPool, familia: FamiliaDB) -> AppRes<Familia> {
// //     pool.query(r#"
// //         SELECT
// // "#)
//     let mut padre = None;
//     if let Some(padre_db) = familia.padre() {
//         match pool
//             .select::<Option<PersonaDB>>((padre_db.tb.as_str(), padre_db.id.to_string()))
//             .await
//         {
//             Ok(padre_db) => padre = padre_db,
//             Err(e) => return Err(AppError::DBErr(94, e.to_string())),
//         }
//     }
//     let mut madre = None;
//     if let Some(madre_db) = familia.madre() {
//         match pool
//             .select::<Option<PersonaDB>>((madre_db.tb.as_str(), madre_db.id.to_string()))
//             .await
//         {
//             Ok(madre_db) => madre = madre_db,
//             Err(e) => return Err(AppError::DBErr(94, e.to_string())),
//         }
//     }
//     let mut hijos = vec![];
//     for hijo_db in familia.hijos() {
//         match pool
//             .select::<Option<PersonaDB>>((hijo_db.tb.as_str(), hijo_db.id.to_string()))
//             .await
//         {
//             Ok(Some(hijo_db)) => hijos.push(hijo_db),
//             Ok(None) => {}
//             Err(e) => return Err(AppError::DBErr(94, e.to_string())),
//         }
//     }
//     Ok(Familia::from_db(familia, padre, madre, hijos))
// }
