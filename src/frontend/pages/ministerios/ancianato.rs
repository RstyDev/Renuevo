use crate::frontend::{
    components::{
        add_user_form::AddUserForm, delete_user_form::DeleteUserForm, edit_user_form::EditUserForm,
    },
    structs::Auth,
};
use sycamore::prelude::*;

#[component(inline_props)]
pub fn Ancianato(auth: Signal<Auth>) -> View {
    let state = create_signal(String::from("Add"));
    let miembros = create_signal(None);
    let state_selector = create_selector(move || state.get_clone());
    view! {
        select(id="select_ancianato",bind:value=state){
            option(value = String::from("Add")){"Add"}
            option(value = String::from("Edit")){"Edit"}
            option(value = String::from("Remove")){"Remove"}
        }
        p(){"Ancianato"}
        (match state_selector.get_clone().as_str(){
            "None" => view!{},
            "Add" => view! {AddUserForm(auth = auth, miembros = miembros)},
            "Remove" => view! {DeleteUserForm(auth = auth, miembros = miembros)},
            "Edit" => view! {EditUserForm(auth = auth, miembros = miembros)},
            _=> view!{}
        })
    }
}
