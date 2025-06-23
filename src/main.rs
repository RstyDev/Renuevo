pub mod entities;

#[cfg(feature = "ssr")]
pub mod backend;
pub mod error;
#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub mod frontend;

#[cfg(feature = "ssr")]
use crate::backend::infrastructure::web::run;
#[cfg(feature = "ssr")]
use chrono::TimeZone;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run().await
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
use frontend::app::App;

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(App);
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {}
