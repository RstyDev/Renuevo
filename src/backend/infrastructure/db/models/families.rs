use crate::backend::infrastructure::db::models::users::PersonaDB;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
/*
[
    {
        "id": "6n6f24mfgiljtif9m8or",
        "password": null,
        "nombre": "María José",
        "apellido": "Cortés Alarcón",
        "sexo": "Femenino",
        "nacimiento": "2025-07-03",
        "estado_civil": "Casado",
        "estado": {
            "Miembro": {
                "conversion": "2019-12-01",
                "bautismo": {
                    "fecha": "2024-09-01",
                    "profesion_de_fe": "2020-12-01",
                    "iglesia": "Iglesia Presbiteriana Renuevo"
                },
                "servicio": [
                    {
                        "leader": false,
                        "area": "Bienvenida"
                    }
                ]
            }
        },
        "registrado": "2025-07-03"
    },
    {
        "id": "81u686fr18nm1aijn1lv",
        "password": null,
        "nombre": "Rafael",
        "apellido": "De Lima",
        "sexo": "Masculino",
        "nacimiento": "2025-07-03",
        "estado_civil": "Casado",
        "estado": {
            "Presbitero": {
                "tipo": "Maestro",
                "conversion": "2025-07-03",
                "bautismo": {
                    "fecha": "2025-07-03",
                    "profesion_de_fe": "2025-07-03",
                    "iglesia": "Pentecostal de Misiones"
                },
                "servicio": [
                    {
                        "leader": true,
                        "area": "Palabra"
                    },
                    {
                        "leader": false,
                        "area": "Presbiterado"
                    }
                ]
            }
        },
        "registrado": "2025-07-03"
    },
    {
        "id": "f6zpfrk466xer47n1491",
        "password": null,
        "nombre": "Luciano",
        "apellido": "Suarez",
        "sexo": "Masculino",
        "nacimiento": "2025-07-03",
        "estado_civil": "Casado",
        "estado": "Nuevo",
        "registrado": "1970-01-01"
    },
    {
        "id": "pn52xuc0uob6k7yizcki",
        "password": null,
        "nombre": "Lucas",
        "apellido": "Igarzabal",
        "sexo": "Masculino",
        "nacimiento": "2025-07-03",
        "estado_civil": "Casado",
        "estado": {
            "Presbitero": {
                "tipo": "Governante",
                "conversion": "2025-07-03",
                "bautismo": {
                    "fecha": "2025-07-03",
                    "profesion_de_fe": "2025-07-03",
                    "iglesia": "Vida Sobrenatural"
                },
                "servicio": [
                    {
                        "leader": true,
                        "area": "Tesoro"
                    },
                    {
                        "leader": false,
                        "area": "Sonido"
                    }
                ]
            }
        },
        "registrado": "2025-07-03"
    }
]
*/
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct FamiliaDB {
    id: Option<Thing>,
    apellido: String,
    padre: Option<Thing>,
    madre: Option<Thing>,
    hijos: Vec<Thing>,
}
impl FamiliaDB {
    pub fn new(
        id: Option<String>,
        apellido: String,
        padre: Option<PersonaDB>,
        madre: Option<PersonaDB>,
        hijos: Vec<PersonaDB>,
    ) -> Self {
        Self {
            id: id.map(|s| Thing::from(("familias", s.as_str()))),
            apellido,
            padre: padre.map(|p| p.id().to_owned()).flatten(),
            madre: madre.map(|m| m.id().to_owned()).flatten(),
            hijos: hijos
                .into_iter()
                .filter_map(|h| h.id().to_owned())
                .collect(),
        }
    }
    pub fn id(&self) -> &Option<Thing> {
        &self.id
    }

    pub fn apellido(&self) -> &str {
        &self.apellido
    }

    pub fn padre(&self) -> &Option<Thing> {
        &self.padre
    }

    pub fn madre(&self) -> &Option<Thing> {
        &self.madre
    }

    pub fn hijos(&self) -> &Vec<Thing> {
        &self.hijos
    }
}
