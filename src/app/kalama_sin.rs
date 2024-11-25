use crate::app::*;

const TRANSCRIPTS: [(&'static str, &'static str); 29] = [
  ("https://docs.google.com/document/d/e/2PACX-1vTYQAKHmNWVFqnKfr9Z7Zen09agJQUJiQLfMZyTvJ_-0OU9juZ1FNNKgsAvFCRjnPkanc1ud61nI_2X/pub", "#1: nanpa open tan jan Juli"),
  ("https://docs.google.com/document/d/e/2PACX-1vSE37sqz6LfD4F0KgIAh9OQXr5zc9yQJVu8Fxfr3gm89fjMyvk7HCkkPUI6GTb-vf99_p91WURXjWv9/pub", "#2: pilin ku tan jan Tepo"),
  ("https://docs.google.com/document/d/e/2PACX-1vTPBeABxXHIWTk_i-4csAasUVFdKaAXGUcMi_R0ETo7zs4hW17AoZDA9JmRfJUr1fEiW_aovAGmrpsQ/pub", "#3: ilo sitelen tan jan Lipamanka"),
  ("https://docs.google.com/document/d/1zPMpb1-m-ickJakj0933cl3pnApegLPpToGlaJxdxVk/preview", "#4: musi pi kala ko tan jan Itan, jan nanpa luka tu tan jan Tepo"),
  ("https://docs.google.com/document/d/e/2PACX-1vTXq3OcM8u1_476zrIekPUhZ-biXWhY53_QQZSXp_ADesIdf2Go8PRgIeVvIOOBu5JBG7m9H_jMxDxO/pub", "#5: tenpo mun monsuta tan jan Teni"),
  ("https://lipukule.org/post/2021/03/14/o-lukin-ala-e-monsi/", "#6: o lukin ala e monsi tan jan Juli"),
  ("https://docs.google.com/document/d/e/2PACX-1vShYto392jps6POf-s8mztAlGCGlBk23L61FS4D-p4yGCBQmGgVI1_r5P1gZEbv5Pvyt7vEiH5mxGYa/pub", "#7: ma pi lipu Tun tan jan Tepo"),
  ("https://docs.google.com/document/d/e/2PACX-1vQW8Gls6Nds6irxYIiEbuXGf9ea_R_wNU20IdPrCzsK9K0bEEzGbOOyTO08yGSWYjObV-Py8_hGeM7U/pub", "#8: nasin lipu pona tan jan Lakuse"),
  ("https://docs.google.com/document/d/1AZFTm30kJyBjsd31UlOrz8ivMlSmULtMlig8x8Pn9qs/preview", "#9: o toki e ijo pi toki pona ala! tan jTepo tan jLakuse tan jItan"),
  ("https://docs.google.com/document/d/17ZDbcq_kKxXUL9jXA9JMJIEpCQrt4uuYfivxt6vqj-c/preview", "#10: tu lukin lon tan jan Lakuse"),
  ("https://joelthomastr.github.io/tokipona/toki-pi-kon-pona_si", "#11: pana sona pi nasin toki kepeken sitelen tawa, tan jan Telakoman"),
  ("https://docs.google.com/document/d/e/2PACX-1vQfC5lL405CmVLTc9VLoxH5GDGzZMHuGOIHCxIhrFqzmBmtgzBvpuksLXH5W66vgg/pub", "#12: ike li ken ike ala, tan monsuta pi soweli mun"),
  ("https://docs.google.com/document/d/e/2PACX-1vSXG4XS1fH-0GpJJvd79CXfvPXCXi5_Fb-2Grm1cqa0RDoJS54GY6DZvSOFWUpFB3Cn4gUhz0k2qpfL/pub", "#13: pu Tosi tan jan Juli"),
  ("https://docs.google.com/spreadsheets/d/e/2PACX-1vQtasUq60JG-ISBsO1hlEFv5JszjeI57wEyCNEGhnjDq8AeyzKE-tx1qdwWtuMT3FBlyzNcGPvkBntD/pubhtml", "#14: tenpo Santa li kama, tan jan Itan tan jan Tepo"),
  ("https://docs.google.com/document/d/e/2PACX-1vRGVMKDyMgI18rdv5gCHwu9b7pwDuy8Jth4fdzE30CQg-a-iQX3bp4vkfCPFH3LW9pS4-hh3uI5kf9-/pub", "#15: musi pi toki pona tan jan Sema"),
  ("https://docs.google.com/document/d/1AKLB6ddvDsr2SYZ-5W-mf7d48rUrmbrvEpM4cEuGB8s/preview", "#16: toki Wijosa tan jan Tesa tan jan Tepo"),
  ("https://docs.google.com/document/d/16k38wjGkXUfVYK2Q4fpcyzbf0k_rTQ6oei4IJ9-Xob4/preview", "#17: nasin pi kama sona tan soweli nata tan jan Tepo"),
  ("https://docs.google.com/document/d/11ZXrWwJ1vedw40sga1T98HfNLaUhA375s5Ffx8rAETg/preview", "#18: jan mun en nasin waso, tan jan Lakuse tan jan Tepo tan jan Itan"),
  ("https://docs.google.com/document/d/1nkuIu7QfuDHe_JkBp_cZFOaIBJTSdEz5UGvSl7PcC1c/preview", "#19: nasin ISO, tan jan Pensa tan jan Tepo"),
  ("https://docs.google.com/document/d/1a_zfXHqrSiRb8j5cR4RKfi1ZTx9ksoKyqQmN1OeqBXs/preview", "#20: ma li supa, tan jan Tepo tan jan Lakuse"),
  ("https://docs.google.com/document/d/1GftbtvxikDQJKmtmB_CxXItjxTgmg717FeecBS9Qd8M/preview", "#21: nasin Puta, tan kala Salan tan jan Lakuse tan jan Tepo"),
  ("https://docs.google.com/document/d/1l36PUgRxwDSWyuKGFBBKyjw20Bi_y11DGxPp74VI-iM/preview", "#22: poki nasa, tan jan Tepo tan jan Lakuse tan kala Salan"),
  ("https://docs.google.com/document/d/1eYRLrf4-w2_1VuY9Dc8kPsjPgrTHOr7uv3GHA9bGxEE/preview", "#23: jan monsuta loje, tan jan Kekan San tan jan Pensa"),
  ("https://docs.google.com/document/d/1dXmde4rhkUqtGcVrK1d4iC15Yiz2jjmblfuLGt-W0CU/preview", "#24: ijo sin, tan jan Lakuse"),
  ("https://docs.google.com/document/d/1vKsPFBHWWOTt-eQ0VNkC007ubsvpG_A9xDS7rz7PmpU/preview", "#25: suno pi toki pona, tan jan Lakuse tan jan Tepo tan palisa jelo Natan tan jan Kepe"),
  ("https://lipumonsuta.neocities.org/mun-monsuta/o-moku-pona", "#26: o moku pona! tan jan Simiman"),
  ("/pini_ala", "#27: sona pi toki luka, tan jan Lakuse tan jan Tepo (pini ala!)"),
  ("/pini_ala", "#28: ma tomo Win, tan jan Ke Tami tan kulupu pi ma Win (pini ala!)"),
  ("/pini_ala", "#29: pakala li lon telo sijelo loje, tan jan Luke tan jan Kiwisin (pini ala!)"),
];

#[component]
pub fn KalamaSinPage() -> impl IntoView {
  let tp_hidden = create_rw_signal(false);
  let link_win_hidden = create_rw_signal(false);
  let kalama_sin_hidden = create_rw_signal(false);
  let webring_hidden = create_rw_signal(false);
  let file_hidden = create_rw_signal(true);
  let loading_hidden = create_rw_signal(false);

  let footer_items = vec![
    ("lipu pi toki pona", tp_hidden),
    ("lon ilo RedCircle", link_win_hidden),
    ("lipu pi kalama sin", kalama_sin_hidden),
    ("sike pona", webring_hidden),
    ("\"Inspiration\"", loading_hidden),
  ];
  let (file_src, set_file_src) = create_signal(None);
  let z_idx = Some(create_rw_signal(1));

  view! {
    <LinkWindow
      pos=WindowPos::Val((20, 20))
      size=(255, 255)
      hidden=tp_hidden
      z_idx=z_idx
      id="tp-link-win"
      title="lipu pi toki pona".to_string()
      bg_img="/assets/itan.svg"
      src="/tp"
      diag_tp=true
    />
    <LinkWindow
      pos=WindowPos::Val((20, 347))
      size=(255, 255)
      hidden=link_win_hidden
      z_idx=z_idx
      id="kalama-sin-link-win"
      title="lon ilo RedCircle".to_string()
      bg_img="/assets/kalama-sin.webp"
      src="https://redcircle.com/shows/kalama-sin"
      external=true
    />
    <KalamaSinWindow
      pos=WindowPos::Val((310, 20))
      size=(440, 582)
      hidden=kalama_sin_hidden
      z_idx=z_idx
      file_win_src=set_file_src
      links=TRANSCRIPTS.to_vec()
    />
    <FileWindow
      pos=WindowPos::Val((782, 20))
      size=(700, 744)
      hidden=file_hidden
      z_idx=z_idx
      src=file_src/>
    <WebringWindow
      pos=WindowPos::Val((20, 674))
      size=(430, 70)
      hidden=webring_hidden
      z_idx=z_idx
      webring=Webring::SikePona
    />
    <LoadingWindow
      pos=WindowPos::Val((480, 674))
      size=(270, 70)
      hidden=loading_hidden
      z_idx=z_idx
      variant=LoadingWindowVariant::TP
    />
    <Footer items=footer_items/>
    <GoatCounter path="/tp/kalama_sin"/>
  }
}

#[component]
fn KalamaSinWindow(
  pos: WindowPos,
  size: (u32, u32),
  hidden: RwSignal<bool>,
  #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
  file_win_src: WriteSignal<Option<&'static str>>,
  links: Vec<(&'static str, &'static str)>,
) -> impl IntoView {
  let size = create_rw_signal(size);
  let fws = file_win_src;

  let linksList = view! { <ul> {
    links.iter().map(|(s, d)| {
      view! { <li>
        <FileLink src={s} display={d} file_win_src=fws/>
      </li> }
    }).collect_view()
  } </ul> };

  let content = WindowContent::Page(view! { <div style="padding: 5px" tabindex=0>
    <p>
      "mi pali e sitelen anpa lon "
      <ExternalLink
        href="https://redcircle.com/shows/kalama-sin"
        display="kalama sin"
      />
      ". pali mi li pona "
      <ExternalLink
        href="https://www.youtube.com/playlist?list=PLjOmpMyMxd8Qs2mAXcLk817tQy_AQj09u"
        display="lon ilo Jutu"
      />
      " (o kepeken nena \"CC\"). sina ken lukin e lipu ale tan kalama sin lon ni kin (mi pali e lipu ni pi ale ala):"
    </p>
    { linksList }
  </div> });

  view! { <Window
    base=WindowBase {
      id: "kalama-sin-win",
      title: "lipu pi kalama sin".to_string(),
      content,
      pos,
      size,
      hidden,
    }
    extra=WindowExtra {
      z_idx,
      scroll: true,
      ..Default::default()
    }
  /> }
}

