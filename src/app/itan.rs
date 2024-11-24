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
      title="Wireless Nature".to_string()
      pos=WindowPos::Val((280, 20))
      hidden=wireless_nature_hidden
      z_idx=z_idx
      album=Album {
        cover: "/assets/wireless-nature.png",
        links: Some(AlbumLinks {
          bandcamp: "https://ijotananpananpa.bandcamp.com/album/wireless-nature",
          spotify: "https://open.spotify.com/album/1ttWxlDv1kxizGTJpDBCXL",
          youtube: "https://music.youtube.com/playlist?list=OLAK5uy_lSIIgpA8_vEFSw08M2fcuRp9veDcaEfdQ",
          amazon: "https://music.amazon.com/albums/B0DFL1NPH1",
          apple: "https://music.apple.com/us/album/wireless-nature-ep/1765424044",
        }),
      }
    />
    <AlbumWindow
      id="wireless-nature-plugdin-window"
      title="Wireless Nature (Plug'din)".to_string()
      pos=WindowPos::Val((749, 20))
      hidden=wireless_nature_hidden
      z_idx=z_idx
      album=Album {
        cover: "/assets/wireless-nature-plugdin.png",
        links: None,
      }
    />
    <Footer items=footer_items/>
    <GoatCounter path="/itan"/>
  }
}

struct AlbumLinks {
  bandcamp: &'static str,
  spotify: &'static str,
  youtube: &'static str,
  amazon: &'static str,
  apple: &'static str,
}

struct Album {
  cover: &'static str,
  links: Option<AlbumLinks>,
}

#[component]
fn AlbumWindow(
  id: &'static str,
  title: String,
  pos: WindowPos,
  hidden: RwSignal<bool>,
  z_idx: Option<RwSignal<usize>>,
  album: Album,
) -> impl IntoView {
  let image_link = if let Some(links) = &album.links { links.bandcamp } else { "https://ijotananpananpa.bandcamp.com/" };
  let content = WindowContent::Page(view! {
    <div style="text-align: center; height: 100%">
      <a href={image_link} target="_blank" style="height: 100%">
        <img
          src=album.cover
          style="padding: 0px; height-max: 100%; max-width: 100%"
          draggable=false
          tabindex=0
        />
      </a>
      { if let Some(links) = album.links { view! { <div>
        <ExternalLink href=links.bandcamp display="Bandcamp"/>" "
        <ExternalLink href=links.spotify display="Spotify"/>" "
        <ExternalLink href=links.youtube display="YouTube"/>" "
        <ExternalLink href=links.amazon display="Amazon"/>" "
        <ExternalLink href=links.apple display="Apple"/>" "
      </div> } } else { view! { <div>
        "Coming soon!"
      </div> } } }
      <div>
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
        size: (434, 467).into(),
        hidden,
      }
      extra=WindowExtra {
        z_idx,
        rainbow: true,
        ..Default::default()
      }
    />
  }
}

