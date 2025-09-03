use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use crate::backend::infrastructure::db::IglesiaDB;
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Iglesia {
    id: Option<String>,
    nombre: String,
    denominacion: String,
    presbiterio: String
}

impl Iglesia {
    pub fn new(id: Option<String>, nombre: String, denominacion: String, presbiterio: String) -> Iglesia {
        Self {
            id,
            presbiterio,
            denominacion,
            nombre
        }
    }

    pub fn nombre(&self) -> &str {
        &self.nombre
    }

    pub fn denominacion(&self) -> &str {
        &self.denominacion
    }

    pub fn presbiterio(&self) -> &str {
        &self.presbiterio
    }
    #[cfg(feature = "ssr")]
    pub fn from_db(iglesia: IglesiaDB) -> Self {
        Self{
            id: iglesia.id.map(|t|t.id.to_string()),
            presbiterio: iglesia.presbiterio,
            denominacion: iglesia.denominacion,
            nombre: iglesia.nombre
        }
    }
    #[cfg(feature = "ssr")]
    pub fn to_db(self) -> IglesiaDB {
        IglesiaDB::new(self.id,self.nombre,self.denominacion,self.presbiterio)
    }

    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }
}