#[allow(non_snake_case)]
pub mod app;
use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "hydrate")] {
  pub fn hydrate() {
    use app::*;
    use leptos::*;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(move || {
      view! { <App/> }
    });
  }
}
}
