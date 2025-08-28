use crate::{
    backend::{
        domain::repositories::UserRepository,
        infrastructure::db::{establish_connection, DBPool, PersonaDB},
    },
    entities::Persona,
    error::{AppError, AppRes},
};
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
            sexo: $hermano.sexo,
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

//LET $user = (SELECT * FROM ONLY $id WHERE password = crypto::sha512($pass));
//RETURN IF array::len($user) > 0 THEN $user ELSE NONE END;
    async fn is_id_pass_correct(&self, id: &str, password: &str) -> AppRes<bool> {
        // println!("user repo 91: {}",password);
        // let thing = format!("personas:{}",id);

        match self.pool.query("RETURN crypto::sha512($pass)").bind(("pass",password.to_owned())).await {
            Ok(mut res) => {
                let pass = res.take::<Option<String>>(0).map_err(|e| AppError::DBErr(97, e.to_string()))?.unwrap();
                match self.get_by_id_with_password(id).await {
                    Ok(option) => {
                        match option {
                            Some(user) => {
                                if pass.eq(user.password().unwrap()) {
                                    Ok(true)
                                } else {
                                    Ok(false)
                                }
                            }
                            None => Ok(false),
                        }
                    }
                    Err(e) => Err(AppError::DBErr(102, e.to_string())),
                }
                // println!("{:#?}", pass);
                //
                // Ok(None)
            }
            Err(e) => Err(AppError::DBErr(108, e.to_string())),
        }

        // match self.pool.query(r#"
        //     LET $user = (SELECT * FROM ONLY type::thing($id) WHERE password = crypto::sha512($pass));
        //     RETURN $user;
        //     RETURN IF $user THEN $user ELSE NONE END;
        // "#).bind(("id",format!("personas:{}",id.to_owned()))).bind(("pass",password.to_owned())).await {
        //     Ok(mut response) => {
        //         println!("user repo 126 {:#?}", response);
        //         let res = response.take::<Option<PersonaDB>>(0).map_err(|e| AppError::DBErr(99, e.to_string()));
        //         println!("user repo 96 {:#?}",res);
        //         res.map(|option|option.map(|p|Persona::from_db(p.to_owned())))
        //     },
        //     Err(e) => {
        //         println!("user repo 100 {:#?}",e);
        //         Err(AppError::DBErr(100, e.to_string()))
        //     }
        // }
    }

    async fn get_by_id_with_password(&self, id: &str) -> AppRes<Option<Persona>> {
        match self
            .pool
            .select::<Option<PersonaDB>>(("personas", id))
            .await
        {
            Ok(Some(hermano)) => Ok(Some(Persona::from_db_complete(hermano))),
            Ok(None) => Ok(None),
            Err(e) => Err(AppError::DBErr(86, e.to_string())),
        }
    }

    async fn update(&self, persona: Persona) -> AppRes<Persona> {
        match persona.id() {
            None => Err(AppError::DBErr(92, "User sent without ID".to_string())),
            Some(id) => {
                match self
                    .pool
                    .upsert::<Option<PersonaDB>>(("personas", id.clone()))
                    .content(persona.to_db().unwrap())
                    .await
                {
                    Ok(a) => Ok(Persona::from_db(a.unwrap())),
                    Err(e) => Err(AppError::DBErr(134, e.to_string())),
                }
            }
        }
    }

    async fn update_password(&self, id: &str, password: &str) -> AppRes<()> {
        println!("user repo 155 {}", id);
        match self.pool.query(r#"
                   UPDATE type::thing($id) SET password = crypto::sha512($pass);
               "#).bind(("id",format!("personas:{}",id.to_owned()))).bind(("pass",password.to_owned())).await {
            Ok(a) => {
                println!("user repo 167 {:#?}", a);
                Ok(())
            },
            Err(e) => {
                println!("user repo 173: {}", e);
                Err(AppError::DBErr(134, e.to_string()))
            },
        }
        // match self
        //     .pool
        //     .upsert::<Option<PersonaDB>>(("personas", id.clone()))
        //     .content(persona.to_db().unwrap())
        //     .await
        // {
        // }

    }
}
