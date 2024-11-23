use std::time::Duration;

use crate::app::{
  ExternalLink, LoadingWindow, picks::PicksLinkWindow, LoadingWindowVariant, WindowPos, Window, Footer, GoatCounter
};
use leptos::*;

use super::WindowContent;

#[component]
pub fn ItanPage() -> impl IntoView {
  let picks_hidden = create_rw_signal(false);
  let loading_hidden = create_rw_signal(false);
  let wireless_nature_hidden = create_rw_signal(false);

  let footer_items = vec![
    ("\"Inspiration\"", loading_hidden),
    ("My Picks", picks_hidden),
    ("Wireless Nature", wireless_nature_hidden),
  ];
  let z_idx = Some(create_rw_signal(1));

  view! {
    <LoadingWindow   pos=WindowPos::Val((20, 20))  size=(225, 170) hidden=loading_hidden     z_idx=z_idx variant=LoadingWindowVariant::Default/>
    <PicksLinkWindow pos=WindowPos::Val((20, 262)) size=(225, 225) hidden=picks_hidden       z_idx=z_idx/> // music link window
    <AlbumWindow
      pos=WindowPos::Val((280, 20))
      hidden=wireless_nature_hidden
      id=""
      title="Wireless Nature".to_string()
      img="/assets/wireless-nature.png"
      running_length=Duration::new(10, 0)
      bandcamp=""
      spotify=""
      z_idx=z_idx
    />
    <Footer items=footer_items/>
    <GoatCounter path="/itan"/>
  }
}

#[component]
fn AlbumWindow(
  pos: WindowPos,
  hidden: RwSignal<bool>,
  id: &'static str,
  title: String,
  img: &'static str,
  running_length: Duration,
  bandcamp: &'static str,
  spotify: &'static str,
  z_idx: Option<RwSignal<usize>>,
) -> impl IntoView {
  let content = WindowContent::Page(view! {
    <div style="heigh: 100%">
      <img
        src=img
        style="padding: 0px; height: 100%; max-width: 100%"
        draggable=false
        tabindex=0
      />
      <ExternalLink href=bandcamp display="Bandcamp"/><br/><br/>
      <ExternalLink href=spotify display="Spotify"/><br/>
    </div>
  });

  view! {
    <Window id=id title=title content=content pos=pos size=(467, 467).into() hidden=hidden z_idx=z_idx/>
  }
}

