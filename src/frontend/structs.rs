use crate::entities::{LoginResult, Ministerio};

#[derive(Clone, Debug, PartialEq)]
pub enum Auth {
    Logged(LoginResult),
    NotLogged,
}
impl Auth {
    pub fn unwrap(&self) -> &LoginResult {
        match self {
            Self::NotLogged => panic!("Not Logged"),
            Self::Logged(result) => result,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Tabs {
    Inicio,
    Nosotros,
    Involucrate,
    Donar,
    Miembros,
    Ministerio(Ministerio),
    Login,
}
