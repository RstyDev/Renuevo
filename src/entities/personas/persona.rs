#[cfg(feature = "ssr")]
use crate::backend::infrastructure::db::PersonaDB;
use crate::entities::{Bautismo, Servicio};
// #[cfg(feature = "ssr")]
use crate::error::{AppError, AppRes};
use chrono::{Months, NaiveDate};
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

    pub fn is_possible_son_of(&self, other: &Persona) -> bool {
        self.nacimiento
            > other
                .nacimiento
                .checked_add_months(Months::new(12 * 18))
                .unwrap()
    }

    #[cfg(feature = "ssr")]
    pub fn to_db_no_pass(self) -> PersonaDB {
        PersonaDB::new(
            self.id,
            String::new(),
            self.nombre,
            self.apellido,
            self.sexo,
            self.nacimiento,
            self.estado_civil,
            self.estado,
            self.registrado,
            None,
        )
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
                None,
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

    #[cfg(feature = "ssr")]
    pub fn from_db_complete(persona: PersonaDB) -> Self {
        Self {
            id: persona.id().as_ref().map(|p| p.id.to_string()),
            password: Some(persona.password().to_string()),
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
    Divorciado,
}

impl ToString for EstadoCivil {
    fn to_string(&self) -> String {
        String::from(match self {
            EstadoCivil::Soltero => "Soltero",
            EstadoCivil::Casado => "Casado",
            EstadoCivil::Viudo => "Viudo",
            EstadoCivil::Divorciado => "Divorciado",
        })
    }
}

impl EstadoCivil {
    pub fn from_string(estado: String) -> AppRes<EstadoCivil> {
        match estado.as_str() {
            "Soltero" => Ok(EstadoCivil::Soltero),
            "Casado" => Ok(EstadoCivil::Casado),
            "Viudo" => Ok(EstadoCivil::Viudo),
            "Divorciado" => Ok(EstadoCivil::Divorciado),
            a => Err(AppError::UnknownState(147, a.to_string())),
        }
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
    pub fn to_plain_string(&self) -> String {
        String::from(match self {
            Estado::Visitante => "Visitante",
            Estado::Nuevo => "Nuevo",
            Estado::Fundamentos { .. } => "Fundamentos",
            Estado::PreMiembro { .. } => "PreMiembro",
            Estado::Miembro { .. } => "Miembro",
            Estado::Diacono { .. } => "Diacono",
            Estado::Presbitero { .. } => "Presbitero",
        })
    }
    pub fn get_bautismo(&self) -> Option<&Bautismo> {
        match self {
            Estado::Fundamentos { bautismo, .. } | Estado::PreMiembro { bautismo, .. } => {
                bautismo.as_ref()
            }
            Estado::Miembro { bautismo, .. }
            | Estado::Diacono { bautismo, .. }
            | Estado::Presbitero { bautismo, .. } => Some(bautismo),
            _ => return None,
        }
    }

    pub fn get_tipo_presbitero(&self) -> Option<&TipoPresbitero> {
        match self {
            Estado::Presbitero { tipo, .. } => Some(tipo),
            _ => None,
        }
    }
    pub fn get_conversion(&self) -> Option<&NaiveDate> {
        match self {
            Estado::Fundamentos { conversion, .. }
            | Estado::PreMiembro { conversion, .. }
            | Estado::Miembro { conversion, .. }
            | Estado::Diacono { conversion, .. }
            | Estado::Presbitero { conversion, .. } => Some(conversion),
            _ => None,
        }
    }
    pub fn get_servicio(&self) -> Option<&Vec<Servicio>> {
        match self {
            Estado::Miembro { servicio, .. }
            | Estado::Diacono { servicio, .. }
            | Estado::Presbitero { servicio, .. } => Some(servicio),
            _ => None,
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

impl ToString for TipoPresbitero {
    fn to_string(&self) -> String {
        match self {
            Self::Governante => String::from("Governante"),
            Self::Maestro => String::from("Maestro"),
        }
    }
}
