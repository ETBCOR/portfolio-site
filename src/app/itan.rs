use crate::app::{ *, picks::PicksLinkWindow };
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
    <LoadingWindow
      pos=WindowPos::Val((20, 20))
      size=(225, 170)
      hidden=loading_hidden
      z_idx=z_idx
      variant=LoadingWindowVariant::Default
    />
    <PicksLinkWindow // music link window
      pos=WindowPos::Val((20, 262))
      size=(225, 225)
      hidden=picks_hidden
      z_idx=z_idx
    />
    <AlbumWindow
      id="wireless-nature-window"
      pos=WindowPos::Val((280, 20))
      hidden=wireless_nature_hidden
      title="Wireless Nature".to_string()
      img="/assets/wireless-nature.png"
      running_length="an amount of time"
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
  id: &'static str,
  title: String,
  pos: WindowPos,
  hidden: RwSignal<bool>,
  z_idx: Option<RwSignal<usize>>,
  img: &'static str,
  running_length: &'static str,
  bandcamp: &'static str,
  spotify: &'static str,
) -> impl IntoView {
  let content = WindowContent::Page(view! {
    <div style="heigh: 100%">
      <img
        src=img
        style="padding: 0px; height: 100%; max-width: 100%"
        draggable=false
        tabindex=0
      />
      <div style="text-align: center">
        <p>Running Length: {running_length}</p>
        <p><ExternalLink href=bandcamp display="Bandcamp"/></p>
        <p><ExternalLink href=spotify display="Spotify"/></p>
      </div>
    </div>
  });

  view! {
    <Window
      base=WindowBase {
        id,
        title,
        content,
        pos,
        size: (467, 567).into(),
        hidden,
      }
      extra=WindowExtra {
        z_idx,
        ..Default::default()
      }
    />
  }
}

