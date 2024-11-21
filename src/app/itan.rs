use std::time::Duration;

use crate::app::picks::PicksLinkWindow;
use crate::app::{
    ExternalLink, Footer, GoatCounter, LinkWindow, LoadingWindow, LoadingWindowVariant, Window, WindowContent, WindowPos
};
use leptos::*;

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
        <LoadingWindow   pos=WindowPos::Val((20, 20))  size=(225, 170) hidden=loading_hidden         z_idx=z_idx variant=LoadingWindowVariant::Default/>
        <PicksLinkWindow pos=WindowPos::Val((20, 262)) size=(225, 225) hidden=picks_hidden           z_idx=z_idx/> // music link window
        <LinkWindow      pos=WindowPos::Val((280, 20)) size=(467, 467) hidden=wireless_nature_hidden z_idx=z_idx id="" title="Wireless Nature".to_string() bg_img="/assets/wireless-nature.png" src="/itan/wireless-nature"/>
        <Footer items=footer_items/>
        <GoatCounter path="/itan"/>
    }
}

struct Info {
    title: &'static str,
    num_tracks: u32,
    running_length: Duration,
    bandcamp: &'static str,
    spotify: &'static str,
}

#[component]
fn MusicLinkPage(cover: &'static str, info: Info) -> impl IntoView {
    view! { <div style="width: 100%; max-height: 100%; display: inline-flex">
        <div style="width: 100%; max-height: 100%"><img style="padding: 0px; height: 100%; max-width: 100%" src=cover></img></div>
        <div style="height: fit-content; margin: 10px; border: 2px solid black; border-radius: 5px; padding: 10px">
            <ExternalLink href=info.bandcamp display="Bandcamp"/><br/><br/>
            <ExternalLink href=info.spotify display="Spotify"/><br/>
        </div>
    </div> }
}

#[component]
pub fn WirelessNaturePage() -> impl IntoView {
    let info = Info {
        title: "Wireless Nature",
        num_tracks: 6,
        running_length: Duration::new(19 * 60 + 34, 0),
        bandcamp: "https://ijotananpananpa.bandcamp.com/album/wireless-nature",
        spotify: "",
    };

    view! {
        <MusicLinkPage cover="/assets/wireless-nature.png" info=info/>
    }
}