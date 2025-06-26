#[cfg(feature = "ssr")]
use crate::backend::infrastructure::db::models::users::PersonaDB;
use crate::entities::{Bautismo, Servicio};
#[cfg(feature = "ssr")]
use crate::error::{AppError, AppRes};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Persona {
    id: Option<String>,
    password: Option<String>,
    nombre: String,
    apellido: String,
    sexo: Sexo,
    nacimiento: NaiveDate,
    estado_civil: EstadoCivil,
    estado: Estado,
    registrado: NaiveDate,
}

impl Persona {
    pub fn new(
        id: Option<String>,
        password: Option<String>,
        nombre: String,
        apellido: String,
        sexo: Sexo,
        nacimiento: NaiveDate,
        estado_civil: EstadoCivil,
        estado: Estado,
        registrado: NaiveDate,
    ) -> Self {
        Self {
            id,
            password,
            nombre,
            apellido,
            sexo,
            nacimiento,
            estado_civil,
            estado,
            registrado,
        }
    }
    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }
    pub fn nombre(&self) -> &str {
        &self.nombre
    }

    pub fn password(&self) -> Option<&String> {
        self.password.as_ref()
    }

    pub fn apellido(&self) -> &str {
        &self.apellido
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

    pub fn nacimiento(&self) -> NaiveDate {
        self.nacimiento
    }

    pub fn set_estado_civil(&mut self, estado_civil: EstadoCivil) {
        self.estado_civil = estado_civil;
    }
    pub fn sexo(&self) -> Sexo {
        self.sexo
    }
    pub fn set_estado(&mut self, estado: Estado) {
        self.estado = estado;
    }
    #[cfg(feature = "ssr")]
    pub fn to_db(self) -> AppRes<PersonaDB> {
        match self.password {
            None => Err(AppError::ValidationErr(
                59,
                "Se requiere password".to_string(),
            )),
            Some(p) => Ok(PersonaDB::new(
                self.id,
                p,
                self.nombre,
                self.apellido,
                self.sexo,
                self.nacimiento,
                self.estado_civil,
                self.estado,
                self.registrado,
            )),
        }
    }
    #[cfg(feature = "ssr")]
    pub fn from_db(persona: PersonaDB) -> Self {
        Self {
            id: persona.id().as_ref().map(|p| p.id.to_string()),
            password: None,
            nombre: persona.nombre().to_string(),
            apellido: persona.apellido().to_string(),
            sexo: persona.sexo(),
            nacimiento: persona.nacimiento(),
            estado_civil: persona.estado_civil().clone(),
            estado: persona.estado().clone(),
            registrado: persona.registrado(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub enum EstadoCivil {
    #[default]
    Soltero,
    Casado,
    Viudo,
}

impl ToString for EstadoCivil {
    fn to_string(&self) -> String {
        String::from(match self {
            EstadoCivil::Soltero => "Soltero",
            EstadoCivil::Casado => "Casado",
            EstadoCivil::Viudo => "Viudo",
        })
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum TipoPresbitero {
    Governante,
    Maestro,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub enum Estado {
    #[default]
    Visitante,
    Nuevo,
    Fundamentos {
        conversion: NaiveDate,
        bautismo: Option<Bautismo>,
    },
    PreMiembro {
        conversion: NaiveDate,
        bautismo: Option<Bautismo>,
    },
    Miembro {
        conversion: NaiveDate,
        bautismo: Bautismo,
        servicio: Vec<Servicio>,
    },
    Diacono {
        conversion: NaiveDate,
        bautismo: Bautismo,
        servicio: Vec<Servicio>,
    },
    Presbitero {
        tipo: TipoPresbitero,
        conversion: NaiveDate,
        bautismo: Bautismo,
        servicio: Vec<Servicio>,
    },
}

impl Estado {
    pub fn to_plain_string(&self) -> &str {
        match self {
            Estado::Visitante => "Visitante",
            Estado::Nuevo => "Nuevo",
            Estado::Fundamentos { .. } => "Fundamentos",
            Estado::PreMiembro { .. } => "Pre Miembro",
            Estado::Miembro { .. } => "Miembro",
            Estado::Diacono { .. } => "Diácono",
            Estado::Presbitero { .. } => "Presbítero",
        }
    }
}
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum Sexo {
    #[default]
    Masculino,
    Femenino,
}

impl ToString for Sexo {
    fn to_string(&self) -> String {
        String::from(match self {
            Sexo::Masculino => "Masculino",
            Sexo::Femenino => "Femenino",
        })
    }
}
