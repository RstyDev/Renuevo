use crate::frontend::{
    components::header::Header,
    pages::main_page::MainPage,
    structs::{Auth, Tabs},
};
use sycamore::prelude::*;

#[component]
pub fn App() -> View {
    let logged = create_signal(Auth::NotLogged);
    let error_message = create_signal(String::new());
    let tab = create_signal(Tabs::Inicio);
    let persona = create_signal(None);
    // create_memo(move ||{
    //     console_log!("Tab: {:#?}",tab.get_clone())
    // });
    create_memo(move || match logged.get_clone() {
        Auth::NotLogged => {
            tab.set(Tabs::Inicio);
            persona.set(None);
        }
        Auth::Logged(_) => {}
    });
    view! {
        article(id="main"){
            Header(auth = logged.clone(),tabs = tab.clone(), hermano = persona.clone())
            MainPage(auth = logged.clone(), tab = tab, resource = persona, error_message = error_message)
        }
    }
}
