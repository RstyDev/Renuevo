use sycamore::prelude::*;

#[component]
pub fn Footer() -> View {
    view! {
        footer(){
            p(){"Organizaciones de las que formamos parte:"}
            section(id="organizations"){
                a(){
                    img(id="acts29", title="acts29", src="public/acts29.jpg"){}
                }
                a(){
                    img(id="ante_su_palabra", title="ante_su_palabra", src="public/ante_su_palabra.png"){}
                }
                a(){
                    img(id="world_reformed_fellowship", title="world_reformed_fellowship", src="public/world_reformed_fellowship.png"){}
                }
            }
            section(id="social_media"){
                a(id="instagram"){
                    i(class="fa fa-instagram"){}
                }
                a(id="facebook"){
                    i(class="fa fa-facebook"){}
                }
                a(id="email"){
                    i(class="fa fa-envelope"){}
                }
                a(id="whatsapp"){
                    i(class="fa fa-whatsapp"){}
                }
            }
            a(id="address"){"Calle 42 Nº 345 entre 1 y 2, La Plata, Buenos Aires, Argentina"}
            section(id="copyright_section"){
                span(id="copyright"){"Copyright © 2024"}
                "|"
                a(id="iglesia"){"Iglesia Presbiteriana Renuevo"}
                "|"
                span(id="derechos"){"Reservados todos los derechos."}
            }
        }
    }
}
