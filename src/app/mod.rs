use leptos::{ prelude::*, html::*, ev::* };
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
  components::{Route, Router, Routes},
  StaticSegment, WildcardSegment,
  hooks::use_navigate,
};
use leptos_use::{use_event_listener, use_event_listener_with_options, UseEventListenerOptions};
use rand::seq::SliceRandom;

pub mod home;
pub mod kalama_sin;
pub mod itan;
pub mod picks;
pub mod nasin_nanpa;
pub mod pakala;
pub mod portfolio;
pub mod tp;

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();

  view! {
    <Stylesheet id="leptos" href="/pkg/personal_site.css"/>

    <Title text="etbcor's website"/>

    // google fonts
    <Link rel="preconnect" href="https://fonts.googleapis.com"/>
    <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin=""/>
    <Link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=VT323&display=swap"/>
    <Link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Caveat:wght@700&display=swap"/>
    <Link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Josefin+Sans:ital,wght@0,600;0,700;1,600;1,700&display=swap"/>


    // main router
    <Router><main><Routes fallback=move || "Not found.">
      <Route path=StaticSegment("")
        view=home::HomePageWrap/>

      <Route path=StaticSegment("portfolio")
        view=portfolio::PortfolioPage/>

      <Route path=StaticSegment("itan")
        view=itan::ItanPage/>

        <Route path=StaticSegment("itan/wireless-nature")
          view=itan::WirelessNaturePage/>

        <Route path=StaticSegment("itan/wireless-nature-plugdin")
          view=itan::WirelessNaturePlugdinPage/>

      <Route path=StaticSegment("picks")
        view=picks::PicksPage/>

      <Route path=StaticSegment("tp")
        view=tp::TokiPonaPage/>

        <Route path=StaticSegment("tp/kalama_sin")
          view=kalama_sin::KalamaSinPage/>

        <Route path=StaticSegment("tp/nasin_nanpa")
          view=nasin_nanpa::NasinNanpaPage/>

      <Route path=StaticSegment("pakala")
        view=pakala::PakalaPage/>

      <Route path=WildcardSegment("any")
        view=NotFoundPage/>

    </Routes><Cyberpunk/></main></Router>
  }
}

#[component]
fn GoatCounter(path: &'static str) -> impl IntoView {
  let settings = format!("{{\"path\": \"{}\"}}", path);
  view! { <script
    data-goatcounter="https://etbcor.goatcounter.com/count"
    data-goatcounter-settings=settings
    async src="//gc.zgo.at/count.js">
  </script> }
}

pub enum WindowContent {
  Page(HtmlElement<Div, (), ()>),
  Tabs(
    (
      RwSignal<&'static str>,
      Vec<(&'static str, HtmlElement<Div, (), ()>)>,
    ),
  ),
}

#[derive(Copy, Clone)]
pub enum WindowPos {
  Val((i32, i32)),
  Sig(RwSignal<(i32, i32)>),
  OffsetSignal(RwSignal<(i32, i32)>),
}

pub struct WindowBase {
  id: &'static str,
  title: String,
  content: WindowContent,
  pos: WindowPos,
  size: RwSignal<(u32, u32)>,
  hidden: RwSignal<bool>,
}

pub struct WindowExtra {
  z_idx: Option<RwSignal<usize>>,
  expandable: bool,
  expanded: bool,
  min_button: Option<(RwSignal<bool>, RwSignal<(u32, u32)>)>,
  diag: bool,
  scroll: bool,
  rainbow: bool,
  diag_tp: bool,
}

impl Default for WindowExtra {
  fn default() -> Self {
    Self {
      z_idx: None,
      expandable: true,
      expanded: false,
      min_button: None,
      diag: false,
      scroll: false,
      rainbow: false,
      diag_tp: false,
    }
  }
}

#[component]
fn Window(
  base: WindowBase,
  #[prop(default = WindowExtra::default())]
  extra: WindowExtra,
) -> impl IntoView {
  let mut offset = false;
  let pos = match base.pos {
    WindowPos::Val(v) => RwSignal::new(v),
    WindowPos::Sig(s) => s,
    WindowPos::OffsetSignal(s) => {
      offset = true;
      s
    }
  };
  let dpos = RwSignal::new((0, 0));

  let expanded = RwSignal::new(extra.expanded);
  let this_z_idx = RwSignal::new(
    if base.id.eq("ad-win") || base.id.eq("john-win") || !extra.z_idx.is_some() {
      0
    } else {
      extra.z_idx.unwrap().get_untracked()
    },
  );

  let dragWin = move |e: MouseEvent| {
    if let Some(z_idx) = extra.z_idx {
      z_idx.update(|z| *z = *z + 1);
      this_z_idx.set(z_idx());
    }

    let (x, y) = pos.get_untracked();
    dpos.set((x - e.client_x(), y - e.client_y()));
    let drag_cleanup = use_event_listener(document(), mousemove, move |e| {
      if !expanded.get_untracked() {
        let (dx, dy) = dpos.get_untracked();
        pos.set((e.client_x() + dx, e.client_y() + dy))
      }
    });

    let once_opt = UseEventListenerOptions::default();
    once_opt.once(true);
    let _ = use_event_listener_with_options(
      document(),
      mouseup,
      move |_| {
        drag_cleanup();
      },
      once_opt,
    );
  };

  let get_title = move || {
    if base.title.starts_with("Loading")
    || base.title.starts_with("Obtain")
    || base.title.starts_with("o pona") {
      let split: Vec<_> = base.title.split_whitespace().collect();
      view! { <p class="title">
        { split[0].to_string() } " "
        <span style="font-family: 'Cedarville Cursive', cursive; font-size: 12pt; font-style: oblique">{
          { split[1].to_string() }
        }</span>
      </p> }
    } else {
      view! { <p class="title">{&base.title}</p> }
    }
  };

  let get_pos_size = move || {
    if !expanded() {
      format!(
        "left: {}px; top: {}px; width: {}px; height: {}px; z-index: {}",
        pos().0,
        pos().1 + if offset { 35 } else { 0 }, // add space for meta title
        (base.size)().0,
        (base.size)().1 + 39, // add space for title
        this_z_idx()
      )
    } else {
      "".to_string()
    }
  };
  let get_content_size = move || {
    if !expanded() {
      format!("height: {}px", (base.size)().1)
    } else {
      "".to_string()
    }
  };
  let get_tab_size = move || {
    if !expanded() {
      format!("height: {}px", (base.size)().1 - 34)
    } else {
      "".to_string()
    }
  };

  let get_content = match base.content {
    WindowContent::Page(content) => view! {
      <div
        class="win-content"
        style=get_content_size
        class:diag={extra.diag}
        class:diag-tp={extra.diag_tp}
        class:scroll={extra.scroll}
        class:rainbow={extra.rainbow}
      >
        { content }
      </div>
    },
    WindowContent::Tabs((active_tab, combined_vec)) => {
      let (titles, tabs): (Vec<_>, Vec<_>) = combined_vec
        .into_iter()
        .map(|(title, content)| {
          (
            view! {
              <div
                class="title"
                class:active=move || active_tab().eq(title)
                on:mousedown=move |_| active_tab.set(title)
                tabindex=0
                on:keydown=move |k| if k.key() == "Enter" { active_tab.set(title) }>
                { title }
              </div>
            },
            view! {
              <div
                class="tab-content"
                tabindex=0
                class:hidden=move || !active_tab().eq(title)>
                { content }
              </div>
            },
          )
        })
        .unzip();

      view! {
        <div class="win-content" style=get_content_size>
          <div class="tab-titlebar">{titles}</div>
          <div
            class="tab-outer"
            style=get_tab_size
            class:scroll={extra.scroll}
            class:diag={extra.diag}
            class:diag-tp={extra.diag_tp}
            class:rainbow={extra.rainbow}
          >{tabs}</div>
        </div>
      }
    }
  };

  view! {
    <div
      id=base.id
      class="win-outer"
      style=get_pos_size
      class:hidden=move || (base.hidden)()
      class:win-expanded=move || extra.expanded
    >
      <div
        class="win-titlebar"
        on:mousedown=dragWin
        tabindex=0
        on:keydown=move |k| {
          if let Some(z_idx) = extra.z_idx {
            z_idx.update(|z| *z = *z + 1);
            this_z_idx.set(z_idx());
          }
          if !expanded() {
            if match k.key().as_str() {
              "ArrowUp" => { pos.update(|(_, a)| *a = *a - 10); true }
              "ArrowDown" => { pos.update(|(_, a)| *a = *a + 10); true }
              "ArrowLeft" => { pos.update(|(a, _)| *a = *a - 10); true }
              "ArrowRight" => { pos.update(|(a, _)| *a = *a + 10); true }
              _ => false,
            } { k.prevent_default() }
          }
        }
      >
        { get_title }
        <div class="win-buttons">
          { match extra.min_button {
            Some((deeper, size)) => { Some(view! { <a
              class="win-min"
              title="minimize window"
              on:mousedown=move |_| {deeper.set(false); size.set((200, 437))}
              on:keydown=move |k| if k.key() == "Enter" {deeper.set(false); size.set((200, 437))}
              tabindex=0
            ></a> }) }
            None => { None } }
          }
          { if extra.expandable { Some(view! { <a
            class="win-expand"
            title="expand window"
            on:mousedown=move |_| expanded.update(|e| *e = !*e)
            on:keydown=move |k| if k.key() == "Enter" { expanded.update(|e| *e = !*e) }
            tabindex=0
          ></a> }) } else { None } }
          <a
            class="win-close"
            title="close window"
            on:mousedown=move |_| base.hidden.set(true)
            on:keydown=move |k| if k.key() == "Enter" { base.hidden.set(true) }
            tabindex=0
          ></a>
        </div>
      </div>
      { get_content }
    </div>
  }
}

#[component]
fn NotFoundPage() -> impl IntoView {
  #[cfg(feature = "ssr")]
  {
    let resp = expect_context::<leptos_actix::ResponseOptions>();
    resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
  }
  let loading = RwSignal::new(false);

  view! { <LoadingWindow
      pos=WindowPos::Val((20, 20))
      size=(500, 500)
      hidden=loading
      variant=LoadingWindowVariant::PageNotFound
  /> }
}

#[component]
fn Footer(
  items: Vec<(&'static str, RwSignal<bool>)>,
  #[prop(default = false)] nasa: bool,
) -> impl IntoView {
  view! {
    <div id="ale-li-pona"></div>
    <div id="nasa-a-a-a" class:hidden={!nasa}></div>
    <div style="height: 70px"></div> // spacer in narrow view
    <footer>
      {
        items
          .into_iter()
          .map(|(title, hidden)| view! {
            <div
              class="title win-minimized"
              on:mousedown=move |_| hidden.set(false)
              class:hidden=move || !hidden()
              tabindex=0
              on:keydown=move |k| if k.key() == "Enter" { hidden.set(false) }
              title="open window"
            >{title}</div>
          })
          .collect::<Vec<_>>()
      }
      <a class="title win-minimized favicon" href="/"></a>
    </footer>
  }
}

#[component]
fn Cyberpunk() -> impl IntoView {
  view! { <div id="cyberpunk">
    <video
      muted
      autoplay
      loop="true"
      poster="/assets/cyberpunk.png"
      on:contextmenu=move |e| e.prevent_default()>
      <source src="/assets/cyberpunk.webm" type="video/webm"/>
    </video>
  </div> }
}

#[rustfmt::skip]
const ABSTRACT_NOUNS: [&str; 95] = [
  "Joy", "Hope", "Love", "Peace", "Serenity", "Happiness", "Bliss", "Gratitude", "Contentment", "Harmony",
  "Beauty", "Abundance", "Faith", "Trust", "Wonder", "Inspiration", "Courage", "Freedom", "Unity",
  "Compassion", "Generosity", "Empathy", "Kindness", "Forgiveness", "Patience", "Respect", "Gentleness",
  "Humility", "Graciousness", "Acceptance", "Radiance", "Positivity", "Enthusiasm", "Laughter", "Elation",
  "Zeal", "Determination", "Confidence", "Belief", "Optimism", "Sincerity", "Hopefulness", "Foresight",
  "Integrity", "Authenticity", "Nobility", "Honesty", "Loyalty", "Resilience", "Appreciation", "Vitality",
  "Curiosity", "Imagination", "Wonderment", "Exploration", "Ingenuity", "Creativity", "Innovation",
  "Empowerment", "Success", "Satisfaction", "Fulfillment", "Excitement", "Thrill", "Delight",
  "Exhilaration", "Peacefulness", "Tranquility", "Stillness", "Clarity", "Serendipity", "Enlightenment",
  "Progress", "Growth", "Change", "Expansion", "Meaning", "Grace", "Blessing", "Brilliance", "Affection",
  "Warmth", "Caring", "Tenderness", "Nurturing", "Support", "Balance", "Moderation", "Simplicity",
  "Adaptability", "Flexibility", "Openness", "Belonging", "Ingenuity", "Mediation"
];

#[derive(PartialEq, Copy, Clone)]
enum LoadingWindowVariant {
  Default,
  HomePageLink,
  #[allow(dead_code)]
  PageComingSoon,
  PageNotFound,
  StackOverflow,
  TP,
}

#[component]
fn LoadingWindow(
  pos: WindowPos,
  size: (u32, u32),
  hidden: RwSignal<bool>,
  #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
  variant: LoadingWindowVariant,
) -> impl IntoView {
  let size = RwSignal::new(size);

  let mut rng = rand::thread_rng();
  let noun: &'static str = ABSTRACT_NOUNS.choose(&mut rng).unwrap();
  let title = {
    use LoadingWindowVariant::*;
    match variant {
      Default => format!("Loading {}", noun),
      HomePageLink => format!("Obtain {}", noun),
      PageComingSoon => "Page Coming Soon".to_string(),
      PageNotFound => "Page Not Found".to_string(),
      StackOverflow => "Uh-oh! The stack overflowed".to_string(),
      TP => "o pona".to_string(),
    }
  };

  let content = WindowContent::Page(view! {
    <div
      class="loading-img"
      class:wait={variant == LoadingWindowVariant::Default}
      on:mousedown=move |_| use_navigate()(if variant == LoadingWindowVariant::StackOverflow { "/pakala" } else { "/" }, Default::default(),)
      on:keydown=move |k| if k.key() == "Enter" { use_navigate()(if variant == LoadingWindowVariant::StackOverflow { "/pakala" } else { "/" }, Default::default(),) }
      tabindex=0
      title="ale li pona"
    ></div>
  });

  view! { <Window
    base=WindowBase {
      id: "loading-win",
      title,
      content,
      pos,
      size,
      hidden,
    }
    extra=WindowExtra {
      expandable: false,
      z_idx,
      rainbow: true,
      ..Default::default()
    }
  /> }
}

#[component]
fn AdWindow(
  pos: WindowPos,
  size: (u32, u32),
  hidden: RwSignal<bool>,
  #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
) -> impl IntoView {
  let size = RwSignal::new(size);
  let content = WindowContent::Page(view! { <div style="cursor: wait">
    <img src="/assets/ur-ad-here.png" draggable="false"/>
  </div> });

  view! { <Window
    base=WindowBase {
      id: "ad-win",
      title: "Advertisement".to_string(),
      content,
      pos,
      size,
      hidden,
    }
    extra=WindowExtra {
      expandable: false,
      z_idx,
      ..Default::default()
    }
  /> }
}

enum Webring {
  Bucket,
  SikePona,
}

#[component]
fn WebringWindow(
  pos: WindowPos,
  size: (u32, u32),
  hidden: RwSignal<bool>,
  #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
  webring: Webring,
) -> impl IntoView {
  let size = RwSignal::new(size);
  let id = match webring {
    Webring::Bucket => "bucket-webring-win",
    Webring::SikePona => "sike-pona-webring-win",
  };
  let title = match webring {
    Webring::Bucket => "Bucket Webring",
    Webring::SikePona => "sike pona",
  }
  .to_string();

  let content = WindowContent::Page(match webring {
    Webring::Bucket => view! { <div style="margin-left: 16px; margin-right: 16px">
      <iframe
        src="https://webring.bucketfish.me/embed.html?name=etbcor"
        id="bucket-webring"
        style="width: 100%; height: 63px; border: none"
      ></iframe>
    </div> },
    Webring::SikePona => {
      view! { <div id="sike-pona" style="margin-left: 16px; margin-right: 16px; height: 90%; color: #c8ace5">
        <link rel="stylesheet" href="https://sike.pona.la/embed.css"/>
        <span id="left">
          <a href="https://sike.pona.la/jan/jan%20Itan/prev.html" id="prev">"← prev"</a>
          </span>
        <span id="mid"><a href="https://sike.pona.la">
          <img class="tokipona" src="https://sike.pona.la/assets/tokipona.svg"></img>
          "sike pona"
          <img class="tokipona" src="https://sike.pona.la/assets/tokipona.svg"></img>
        </a></span>
        <span id="right">
        <a href="https://sike.pona.la/jan/jan%20Itan/next.html" id="next">"next →"</a>
        </span>
      </div> }
    }
  });

  view! { <Window
    base=WindowBase {
      id,
      title,
      content,
      pos,
      size,
      hidden,
    }
    extra=WindowExtra {
      expandable: false,
      z_idx,
      ..Default::default()
    }
  /> }
}

#[component]
fn JohnWindow(
  pos: WindowPos,
  size: (u32, u32),
  hidden: RwSignal<bool>,
  #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
) -> impl IntoView {
  let size = RwSignal::new(size);
  let content = WindowContent::Page(view! { <div class="rainbow">
     <iframe
      src="https://john.citrons.xyz/embed?ref=etbcor.com"
      style="max-height: 94px; width: 100%; aspect-ratio: 732 / 94; border:none"
    ></iframe>
  </div> });

  view! { <Window
    base=WindowBase {
      id: "john-win",
      title: "Johnvertisement".to_string(),
      content,
      pos,
      size,
      hidden,
    }
    extra=WindowExtra {
      expandable: false,
      z_idx,
      ..Default::default()
    }
  /> }
}

#[component]
fn LonelyWindow(
  pos: WindowPos,
  size: (u32, u32),
  hidden: RwSignal<bool>,
  #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
) -> impl IntoView {
  let size = RwSignal::new(size);
  let content = WindowContent::Page(view! { <div tabindex=0>
  </div> });

  view! { <Window
    base=WindowBase {
      id: "lonely-win",
      title: "A bit lonely...".to_string(),
      content,
      pos,
      size,
      hidden,
    }
    extra=WindowExtra {
      expandable: false,
      z_idx,
      ..Default::default()
    }
  /> }
}

#[component]
fn LinkWindow(
  pos: WindowPos,
  size: (u32, u32),
  hidden: RwSignal<bool>,
  id: &'static str,
  title: String,
  bg_img: &'static str,
  src: &'static str,
  #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
  #[prop(default = false)] diag: bool,
  #[prop(default = false)] diag_tp: bool,
  #[prop(default = false)] external: bool,
) -> impl IntoView {
  let size = RwSignal::new(size);
  let content = WindowContent::Page(if external {
    view! { <div style="cursor: alias; text-align: center; height: 100%">
      <a href=src target="_blank" style="height: 100%">
        <img
          src=bg_img
          style="padding: 0px; height: 100%; max-width: 100%"
          draggable=false
        />
      </a>
    </div> }
  } else {
    view! { <div style="cursor: pointer; text-align: center; height: 100%">
      <img
        src=bg_img
        style="padding: 0px; height: 100%; max-width: 100%"
        draggable=false
        on:mousedown=move |_| use_navigate()(src, Default::default())
        on:keydown=move |k| if k.key() == "Enter" { use_navigate()(src, Default::default()) }
        tabindex=0
      />
    </div> }
  });

  view! { <Window
    base=WindowBase {
      id,
      title,
      content,
      pos,
      size,
      hidden,
    }
    extra=WindowExtra {
      expandable: false,
      z_idx,
      rainbow: {!diag && !diag_tp},
      diag,
      diag_tp,
      ..Default::default()
    }
  /> }
}

#[component]
fn ExternalLink(
  href: &'static str,
  display: &'static str,
  #[prop(default = false)] title_style: bool,
  #[prop(default = false)] bold: bool,
) -> impl IntoView {
  if bold {
    view! {
      <a class="external-link" target="_blank" href=href class:title=title_style>
        <b>{display}</b>
        <span></span> // for the link-out image
      </a>
    }
  } else {
    view! {
      <a class="external-link" target="_blank" href=href class:title=title_style>
        {display}
        <span></span> // for the link-out image
      </a>
    }
  }
}

#[component]
fn FileWindow(
  pos: WindowPos,
  size: (u32, u32),
  hidden: RwSignal<bool>,
  #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
  src: ReadSignal<Option<&'static str>>,
) -> impl IntoView {
  let size = RwSignal::new(size);
  let content = WindowContent::Page(view! { <div style="width: 100%; height: 100%">
    <object
      data=move || { hidden.set(!src().is_some()); src().unwrap_or("") }
      allow="autoplay"
      style="width: 100%; height: 100%"
    ><p>
      "Failed to display PDF file here, but you can "<a href=move || { src().unwrap_or("") }>"download"</a>" it instead if you want."
    </p></object>
  </div> });

  view! { <Window
    base=WindowBase {
      id: "file-win",
      title: "File Viewer".to_string(),
      content,
      pos,
      size,
      hidden,
    }
    extra=WindowExtra {
      expanded: false,
      z_idx,
      ..Default::default()
    }
  /> }
}

#[component]
fn FileLink(
  src: &'static str,
  display: &'static str,
  file_win_src: WriteSignal<Option<&'static str>>,
) -> impl IntoView {
  view! { <a href="" on:mousedown=move |_| file_win_src.set(Some(src)) on:keydown=move |k| if k.key() == "Enter" { file_win_src.set(Some(src)) }>{display}</a> }
}
