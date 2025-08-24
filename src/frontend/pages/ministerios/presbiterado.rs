use crate::frontend::{
    components::{
        add_user_form::AddUserForm, delete_user_form::DeleteUserForm, edit_user_form::EditUserForm,
        relate_users_form::RelateUsersForm,
    },
    lib::log,
    structs::Global,
};
use sycamore::prelude::*;
const NAME: &'static str = "Presbiterado";

#[component(inline_props)]
pub fn Presbiterado(global: Signal<Global>) -> View {
    let miembros = create_signal(None);
    let pr_state = create_signal(PRSelector::Add);
    let pr_selector = create_selector(move || pr_state.get());
    create_effect(move || log(NAME, 18, &pr_selector.get()));
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
            a(class=match pr_selector.get(){
                PRSelector::Relate => "selected",
                _=> "",
            },on:click=move|_|pr_state.set(PRSelector::Relate)){"Relacionar"}
        }
        (match pr_selector.get(){
            PRSelector::Add => view! {AddUserForm(global = global, miembros = miembros)},
            PRSelector::Remove => view! {DeleteUserForm(global = global, miembros = miembros)},
            PRSelector::Edit => view! {EditUserForm(global = global, miembros = miembros)},
            PRSelector::Relate => view!{RelateUsersForm(global = global, miembros = miembros)},
        })
    }
}
#[derive(Copy, Clone, Debug, PartialEq)]
enum PRSelector {
    Add,
    Edit,
    Remove,
    Relate,
}
