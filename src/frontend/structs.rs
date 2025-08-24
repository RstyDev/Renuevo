use std::fmt::{Debug, Display, Formatter};
use crate::entities::{LoginResult, Ministerio};

#[derive(Clone, Debug, PartialEq)]
pub struct Global {
    pub auth: Auth
}



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
    QuienesSomos,
    Donar,
    Miembros,
    Ministerio(Ministerio),
    Login,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct NotificationProps{
    pub text: String,
    pub notification_type: NotificationType,
}
#[derive(Clone, Debug, PartialEq, Default)]
pub enum NotificationType{
    Error,
    Warning,
    Success,
    #[default]
    None,
}

impl Display for NotificationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", match self{
            NotificationType::None => "None",
            NotificationType::Error => "Error",
            NotificationType::Warning => "Warning",
            NotificationType::Success => "Success",
        }))
    }
}