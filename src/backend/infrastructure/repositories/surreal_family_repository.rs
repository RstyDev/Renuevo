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

        let _: Vec<FamiliaDB> = self
            .pool
            .insert("familias")
            .content(familia)
            .await
            .map_err(|e| AppError::DBErr(28, e.to_string()))?;

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
            Ok(res) => {
                let mut familias = vec![];
                for familia in res {
                    familias.push(get_familia_from_db(self.pool.clone(), familia).await?)
                }
                Ok(familias)
            }
            Err(e) => Err(AppError::DBErr(75, e.to_string())),
        }
    }

    async fn get_by_id(&self, id: &str) -> AppRes<Option<Familia>> {
        match self
            .pool
            .select::<Option<FamiliaDB>>(("familias", id))
            .await
        {
            Ok(Some(hermano)) => Ok(Some(get_familia_from_db(self.pool.clone(), hermano).await?)),
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

async fn get_familia_from_db(pool: DBPool, familia: FamiliaDB) -> AppRes<Familia> {
    let mut padre = None;
    if let Some(padre_db) = familia.padre() {
        match pool
            .select::<Option<PersonaDB>>((padre_db.tb.as_str(), padre_db.id.to_string()))
            .await
        {
            Ok(padre_db) => padre = padre_db,
            Err(e) => return Err(AppError::DBErr(94, e.to_string())),
        }
    }
    let mut madre = None;
    if let Some(madre_db) = familia.madre() {
        match pool
            .select::<Option<PersonaDB>>((madre_db.tb.as_str(), madre_db.id.to_string()))
            .await
        {
            Ok(madre_db) => madre = madre_db,
            Err(e) => return Err(AppError::DBErr(94, e.to_string())),
        }
    }
    let mut hijos = vec![];
    for hijo_db in familia.hijos() {
        match pool
            .select::<Option<PersonaDB>>((hijo_db.tb.as_str(), hijo_db.id.to_string()))
            .await
        {
            Ok(Some(hijo_db)) => hijos.push(hijo_db),
            Ok(None) => {}
            Err(e) => return Err(AppError::DBErr(94, e.to_string())),
        }
    }
    Ok(Familia::from_db(familia, padre, madre, hijos))
}
