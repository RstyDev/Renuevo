use crate::frontend::{
    components::{
        add_user_form::AddUserForm, delete_user_form::DeleteUserForm, edit_user_form::EditUserForm,
    },
    structs::Auth,
    lib::log
};
use sycamore::prelude::*;
const NAME: &'static str = "Presbiterado";

#[component(inline_props)]
pub fn Presbiterado(auth: Signal<Auth>) -> View {
    let miembros = create_signal(None);
    let pr_state = create_signal(PRSelector::Add);
    let pr_selector = create_selector(move || pr_state.get());
    create_effect(move || log(NAME,18,&pr_selector.get()));
    view! {
        section(id="selector_section"){
            a(class=match pr_selector.get(){
                PRSelector::Add => "selected",
                _=> "",
            },on:click=move|_|pr_state.set(PRSelector::Add)){"Agregar"}
            a(class=match pr_selector.get(){
                PRSelector::Edit => "selected",
                _=> "",
            },on:click=move|_|pr_state.set(PRSelector::Edit)){"Editar"}
            a(class=match pr_selector.get(){
                PRSelector::Remove => "selected",
                _=> "",
            },on:click=move|_|pr_state.set(PRSelector::Remove)){"Quitar"}
        }
        (match pr_selector.get(){
            PRSelector::Add => view! {AddUserForm(auth = auth, miembros = miembros)},
            PRSelector::Remove => view! {DeleteUserForm(auth = auth, miembros = miembros)},
            PRSelector::Edit => view! {EditUserForm(auth = auth, miembros = miembros)},
        })
    }
}
#[derive(Copy, Clone, Debug, PartialEq)]
enum PRSelector {
    Add,
    Edit,
    Remove,
}