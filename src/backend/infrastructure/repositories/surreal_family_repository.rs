use crate::backend::infrastructure::db::FamiliaDB;
use crate::backend::infrastructure::db::PersonaDB;
use crate::entities::{Familia, Sexo};
use crate::{
    backend::{
        domain::repositories::FamilyRepository,
        infrastructure::db::{establish_connection, DBPool},
    },
    error::{AppError, AppRes},
};
use std::sync::Arc;
use surrealdb::sql::Thing;

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
        let mut apellido = String::new();
        let mut id = String::new();
        if let Some(padre_local) = familia.padre() {
            match padre_local.id() {
                None => return Err(AppError::ValidationErr(32, String::from("Padre sin id"))),
                Some(local_id) => {
                    if let Some(hermano) = self
                        .pool
                        .select::<Option<PersonaDB>>(("personas", local_id))
                        .await
                        .map_err(|e| AppError::DBErr(38, e.to_string()))?
                    {
                        if hermano.sexo() == Sexo::Femenino {
                            return Err(AppError::ValidationErr(
                                35,
                                String::from("El padre debe ser masculino"),
                            ));
                        }
                        id = format!("familias:{}", local_id);
                        apellido = hermano.apellido().to_string();
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
                Some(local_id) => {
                    if let Some(hermana) = self
                        .pool
                        .select::<Option<PersonaDB>>(("personas", local_id))
                        .await
                        .map_err(|e| AppError::DBErr(38, e.to_string()))?
                    {
                        if hermana.sexo() == Sexo::Masculino {
                            return Err(AppError::ValidationErr(
                                44,
                                String::from("La madre debe ser femenino"),
                            ));
                        }
                        if familia.padre().is_none() {
                            id = format!("familias:{}", local_id);
                            apellido = hermana.apellido().to_string();
                        } else {
                            apellido = format!("{} {}", apellido, hermana.apellido());
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
        let familia = FamiliaDB::new(None, apellido, padre, madre, hijos);

        let res = self
            .pool
            .query(
                r#"
        upsert only type::thing($id) set
            apellido= $familia.apellido,
            madre= $familia.madre.id,
            padre= $familia.padre.id,
            hijos= $familia.hijos.map(|$hijo|{$hijo.id});
        update only $familia.madre.id set familia = type::thing($id);
        update only $familia.padre.id set familia = type::thing($id);
        update $familia.hijos set familia = type::thing($id);
        "#,
            )
            .bind(("id", id))
            .bind(("familia", familia))
            .await;
        match res {
            Ok(a) => {
                println!("{:#?}", a);

                Ok(())
            }
            Err(e) => Err(AppError::DBErr(135, e.to_string())),
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
        let mut res = self
            .pool
            .query(
                r#"
            SELECT * FROM familias FETCH padre, madre, hijos;
        "#,
            )
            .await
            .map_err(|e| AppError::DBErr(135, e.to_string()))?;
        let familias = res
            .take::<Vec<FamiliaDB>>(0)
            .map_err(|e| AppError::DBErr(136, e.to_string()))?;
        Ok(familias.into_iter().map(|f| Familia::from_db(f)).collect())
    }

    async fn get_by_id(&self, id: &str) -> AppRes<Option<Familia>> {
        let mut res = self
            .pool
            .query(
                r#"
            SELECT * FROM ONLY type::thing($familia) FETCH padre, madre, hijos;
        "#,
            )
            .bind(("familia", format!("familias:{}", id)))
            .await
            .map_err(|e| AppError::DBErr(143, e.to_string()))?;
        println!("{:#?}", res);
        let familia = res
            .take::<Option<FamiliaDB>>(0)
            .map_err(|e| AppError::DBErr(143, e.to_string()))?;
        Ok(familia.map(|f| Familia::from_db(f))) //TODO hay que arreglar algo en ese query
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
        let mut res = self
            .pool
            .query(
                r#"
            SELECT VALUE id FROM ONLY type::thing($familia);
        "#,
            )
            .bind(("familia", format!("familias:{}", familia.id().as_ref().unwrap())))
            .await
            .map_err(|e| AppError::DBErr(203, e.to_string()))?;
        if res
            .take::<Option<Thing>>(0)
            .map_err(|e| AppError::DBErr(206, e.to_string()))?
            .is_some()
        {
            self.save(&familia).await?;
        }
        Ok(())
    }
}
