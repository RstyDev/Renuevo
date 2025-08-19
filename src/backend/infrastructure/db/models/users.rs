use crate::entities::{Estado, EstadoCivil, Sexo};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[cfg(feature = "ssr")]
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct PersonaDB {
    id: Option<Thing>,
    password: String,
    nombre: String,
    apellido: String,
    sexo: Sexo,
    nacimiento: NaiveDate,
    estado_civil: EstadoCivil,
    estado: Estado,
    registrado: NaiveDate,
    familia: Option<Thing>,
}

impl PersonaDB {
    pub fn new(
        id: Option<String>,
        password: String,
        nombre: String,
        apellido: String,
        sexo: Sexo,
        nacimiento: NaiveDate,
        estado_civil: EstadoCivil,
        estado: Estado,
        registrado: NaiveDate,
        familia: Option<Thing>,
    ) -> Self {
        Self {
            id: id.map(|s| Thing::from(("personas", s.as_str()))),
            password,
            nombre,
            apellido,
            sexo,
            nacimiento,
            estado_civil,
            estado,
            registrado,
            familia,
        }
    }

    pub fn id(&self) -> &Option<Thing> {
        &self.id
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn nombre(&self) -> &str {
        &self.nombre
    }

    pub fn apellido(&self) -> &str {
        &self.apellido
    }

    pub fn nacimiento(&self) -> NaiveDate {
        self.nacimiento
    }

    pub fn estado_civil(&self) -> &EstadoCivil {
        &self.estado_civil
    }

    pub fn estado(&self) -> &Estado {
        &self.estado
    }

    pub fn registrado(&self) -> NaiveDate {
        self.registrado
    }

    pub fn sexo(&self) -> Sexo {
        self.sexo
    }

    pub fn familia(&self) -> &Option<Thing> {
        &self.familia
    }
}
