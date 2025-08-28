use crate::entities::Persona;
use crate::frontend::{
    components::{AddUserForm, DeleteUserForm, EditUserForm, RelateUsersForm},
    lib::log,
    structs::Auth,
};
use sycamore::prelude::*;

const NAME: &'static str = "Presbiterado";

#[component(inline_props)]
pub fn Presbiterado(auth: Signal<Auth>, miembros: Signal<Vec<Persona>>) -> View {
    // let miembros = create_signal(None);
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
            PRSelector::Add => view! {AddUserForm(auth = auth, miembros = miembros)},
            PRSelector::Remove => view! {DeleteUserForm(auth = auth, miembros = miembros)},
            PRSelector::Edit => view! {EditUserForm(auth = auth, miembros = miembros)},
            PRSelector::Relate => view!{RelateUsersForm(auth = auth, miembros = miembros)},
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
