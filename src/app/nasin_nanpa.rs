use crate::app::*;

#[component]
pub fn NasinNanpaPage() -> impl IntoView {
  let tp_hidden = create_rw_signal(false);
  let link_hidden = create_rw_signal(false);
  let nasin_nanpa_hidden = create_rw_signal(false);
  let webring_hidden = create_rw_signal(false);
  let loading_hidden = create_rw_signal(false);

  let footer_items = vec![
    ("lipu pi toki pona", tp_hidden),
    ("lon ilo Github", link_hidden),
    ("nasin nanpa", nasin_nanpa_hidden),
    ("sike pona", webring_hidden),
    ("o pona", loading_hidden),
  ];
  let z_idx = Some(create_rw_signal(1));

  view! {
    <LinkWindow     pos=WindowPos::Val((20, 20))   size=(255, 255) hidden=tp_hidden      z_idx=z_idx id="tp-link-win"    title="lipu pi toki pona".to_string() bg_img="/assets/itan.svg"      src="/tp" diag_tp=true/>
    <LinkWindow     pos=WindowPos::Val((310, 20))  size=(620, 255) hidden=link_hidden z_idx=z_idx id="nasin-nanpa-link-win" title="lon ilo GitHub".to_string() bg_img="/assets/nasin-nanpa-github-screenshot.png" src="https://github.com/ETBCOR/nasin-nanpa" external=true/>
    <NasinNanpaWindow pos=WindowPos::Val((20, 347))  size=(910, 255) hidden=nasin_nanpa_hidden z_idx=z_idx/>
    <WebringWindow  pos=WindowPos::Val((20, 674))  size=(720, 70)  hidden=webring_hidden   z_idx=z_idx webring=Webring::SikePona/>
    <LoadingWindow  pos=WindowPos::Val((775, 674)) size=(155, 70)  hidden=loading_hidden   z_idx=z_idx variant=LoadingWindowVariant::TP/>
    <Footer items=footer_items/>
    <GoatCounter path="/tp/nasin_nanpa"/>
  }
}

#[component]
fn NasinNanpaWindow(
  pos: WindowPos,
  size: (u32, u32),
  hidden: RwSignal<bool>,
  #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
) -> impl IntoView {
  let size = create_rw_signal(size);
  let active_tab = create_rw_signal("Font Versions");

  let content = WindowContent::Tabs((
    active_tab,
    vec![
      (
        "Font Versions",
        view! { <div style="padding: 5px">
          <p>"There are currently three parallel versions of nasin nanpa:"
            <ul>
              <li><span class="title">"nasin-nanpa-4.x.x.otf"</span>" - the \"main\" version; uses UCSUR and ligatures from latin characters"</li>
              <li><span class="title">"`nasin-nanpa-4.x.x-UCSUR.otf"</span>" - the \"UCSUR only\" version; doesn't have latin ligatures"</li>
              <li><span class="title">"`nasin-nanpa-4.x.x-Helvetica.otf"</span>" - the \"Discord\" version; makes UCSUR visible in vanilla Discord"</li>
            </ul>
          </p>
          <p>"You can find the "<b>"download links"</b>" to the latest version "<ExternalLink href="https://github.com/ETBCOR/nasin-nanpa/releases/latest" display="on the GitHub releases page" bold=true/>"."</p>
        </div> },
      ),
      (
        "Glyph Combos",
        view! { <div style="padding: 5px">
          <p>"The best way to combine glyphs in nasin nanpa is to put the "<span class="title">"ZERO WIDTH JOINER"</span>" character ("<span class="title">"&"</span>" with ligatures enabled) between them. If the first glyph has enough whitespace to contain another glyph, the scaling combination will be used. Otherwise, the stacking combination will be used."</p>
          <p>"Alternatively, one can use "<span class="title">"STACKING JOINER"</span>" or "<span class="title">"SCALING JOINER"</span>" ("<span class="title">"-"</span>" and "<span class="title">"+"</span>" respectively) to force a specific combination style."</p>
          <p>"Glyph combos can also be used inside cartouches and long glyphs."</p>
        </div> },
      ),
      (
        "Alternate Glyphs",
        view! { <div style="padding: 5px">
          <p>"The following alternate glyph forms can be accessed by adding "<span class="title">"VAR01"</span>" (or "<span class="title">"1"</span>" with ligatures enabled) directly after the base glyph:"
            <ul>
              <li><span class="sitelen-pona">"a|a1"</span><span class="title">"a"</span>" with two stems (can also be accessed with "<span class="title">"a a"</span>")"</li>
              <li><span class="sitelen-pona">"kala kala1"</span>" "<span class="title">"kala"</span>" with eyes"</li>
              <li><span class="sitelen-pona">"akesi akesi1"</span>" "<span class="title">"akesi"</span>" with three lines / six legs"</li>
              <li><span class="sitelen-pona">"meli meli1"</span>" "<span class="title">"meli"</span>" that's a circle with plus below"</li>
              <li><span class="sitelen-pona">"mije mije1"</span>" "<span class="title">"mije"</span>" that's a circle with arrow to upper right"</li>
              <li><span class="sitelen-pona">"mu mu1"</span>" "<span class="title">"mu"</span>" - sideways emitters"</li>
              <li><span class="sitelen-pona">"mute mute1"</span>" "<span class="title">"mute"</span>" - four "<span class="title">"luka"</span>"s"</li>
              <li><span class="sitelen-pona">"olin olin1"</span>" "<span class="title">"olin"</span>" - heart with emitters"</li>
              <li><span class="sitelen-pona">"pana pana1"</span>" "<span class="title">"pana"</span>" - just emitters"</li>
              <li><span class="sitelen-pona">"sewi sewi1"</span>" "<span class="title">"sewi"</span>" - secular sewi (matches other directional glyphs)"</li>
              <li><span class="sitelen-pona">"tenpo tenpo1"</span>" "<span class="title">"tenpo"</span>" - hourglass"</li>
              <li><span class="sitelen-pona">"uta uta1"</span>" "<span class="title">"uta"</span>" without dot"</li>
              <li><span class="sitelen-pona">"wile wile1"</span>" "<span class="title">"wile"</span>" - upside-down "<span class="title">"pilin"</span></li>
              <li><span class="sitelen-pona">"namako namako1"</span>" "<span class="title">"namako"</span>" - looks like crosshairs"</li>
              <li><span class="sitelen-pona">"lanpan lanpan1"</span>" "<span class="title">"lanpan"</span>" - upside-down "<span class="title">"pana"</span></li>
              <li><span class="sitelen-pona">"misikeke misikeke1"</span>" "<span class="title">"misikeke"</span>" - mortar and pestle"</li>
              <li><span class="sitelen-pona">"linluwi linluwi1"</span>" "<span class="title">"linluwi"</span>" - looks like "<span class="title">"len"</span></li>
            </ul>
          </p>

          <p>"There are a few other alternate glyph forms that can be accessed with the other variation selectors:"
            <ul>
              <li>"8 "<span class="title">"jaki"</span>"s (which are also used to randomize the base glyph wherever supported/enabled)"</li>
              <li>"8 "<span class="title">"ko"</span>"s (also used for the "<span class="title">"rand"</span>" feature like "<span class="title">"jaki"</span>")"</li>
              <li>"8 directional "<span class="title">"ni"</span>"s: 1 ←, 2 ↑, 3 →, 4 ↓, 5 ↖, 6 ↗, 7 ↘, 8 ↙ (can also be accessed with "<span class="title">"ni"</span>" + "<span class="title">"ZWJ"</span>" + [an arrow character / a sequence like "<span class="title">"v<"</span>", if ligatures are enabled])"</li>
              <li>""<span class="title">"a"</span>" with three stems (can also be accessed with "<span class="title">"a a a"</span>")"</li>
            </ul>
          </p>

          <p>"The following glyphs have a \"long glyph\" variation too (accessed by puting "<span class="title">"START OF LONG GLYPH"</span>" / "<span class="title">"("</span>" after it): "<span class="title">"a"</span>", "<span class="title">"alasa"</span>", "<span class="title">"anu"</span>", "<span class="title">"awen"</span>", "<span class="title">"kama"</span>", "<span class="title">"ken"</span>", "<span class="title">"kepeken"</span>", "<br/><span class="title">"la"</span>" (reversed; needs "<span class="title">"END OF REVERSE LONG GLYPH"</span>" / "<span class="title">"}"</span>" *before* it), "<span class="title">"lon"</span>", "<span class="title">"nanpa"</span>", "<span class="title">"open"</span>", "<span class="title">"pi"</span>", "<span class="title">"pini"</span>", "<span class="title">"sona"</span>", "<span class="title">"tawa"</span>", "<span class="title">"wile"</span>", "<span class="title">"wile"</span>" alt, "<span class="title">"n"</span>", and "<span class="title">"wa"</span>"."</p>
        </div> },
      ),
      (
        "Ligatures",
        view! { <div style="padding: 5px">
          <p>"Ligatures are a font feature that allow nasin nanpa (and many other sitelen pona fonts) to display strings of existing Unicode characters as sitelen pona glyphs. However, not every text rendering context (web browser, text editing program, etc.) supports this font feature by default, and some may not at all (so see the "<span class="title">"AHK Script Guide"</span>" tab)!"</p>

          <p>"This table describes both the ligatures in nasin nanpa and the AutoHotKey scripts:"</p>
          <table>
            <tr><th>"Codepoint"</th><th>"Latin Character(s)"</th><th>"Resulting Codepoint / Glyph"</th></tr>
            <tr><td>"U+F1900 -"<br/>"U+F1988"</td><td>"a, akesi ... wile, namako ... ku"</td><td>"A, AKESI ... WILE, NAMAKO ... KU"</td></tr>
            <tr><td>"U+3000"</td><td>"[two spaces] / zz"</td><td>"IDEOGRAPHIC SPACE"</td></tr>
            <tr><td>"U+F1990"</td><td>"["</td><td>"START OF CARTOUCHE"</td></tr>
            <tr><td>"U+F1991"</td><td>"]"</td><td>"END OF CARTOUCHE"</td></tr>
            <tr><td>"U+F1992"</td><td>"="</td><td>"COMBINING CARTOUCHE EXTENSION"</td></tr>
            <tr><td>"U+F1993"</td><td>"(none)"</td><td>"START OF LONG PI"</td></tr>
            <tr><td>"U+F1994"</td><td>"(none)"</td><td>"COMBINING LONG PI EXTENSION"</td></tr>
            <tr><td>"U+F1995"</td><td>"-"</td><td>"STACKING JOINER"</td></tr>
            <tr><td>"U+F1996"</td><td>"+"</td><td>"SCALING JOINER"</td></tr>
            <tr><td>"U+F1997"</td><td>"("</td><td>"START OF LONG GLYPH"</td></tr>
            <tr><td>"U+F1998"</td><td>")"</td><td>"END OF LONG GLYPH"</td></tr>
            <tr><td>"U+F1999"</td><td>"_"</td><td>"COMBINING LONG GLYPH EXTENSION"</td></tr>
            <tr><td>"U+F199A"</td><td>"{"</td><td>"START OF REVERSE LONG GLYPH"</td></tr>
            <tr><td>"U+F199B"</td><td>"}"</td><td>"END OF REVERSE LONG GLYPH"</td></tr>
            <tr><td>"U+F199C"</td><td>"."</td><td>"MIDDLE DOT"</td></tr>
            <tr><td>"U+F199D"</td><td>":"</td><td>"COLON"</td></tr>
            <tr><td>"U+FE00 -"<br/>"U+FE07"</td><td>"1 - 8"</td><td>"VARIATION SELECTOR 1 (VAR01) - VARIATION SELECTOR 8 (VAR08)"</td></tr>
            <tr><td>"U+200C"</td><td>"|"</td><td>"ZERO WIDTH NON JOINER (ZWNJ)"</td></tr>
            <tr><td>"U+200D"</td><td>"&"</td><td>"ZERO WIDTH JOINER (ZWJ)"</td></tr>
            <tr><td>(none)</td><td>"itan"</td><td>"jan Itan's personal glyph"</td></tr>
            <tr><td>(none)</td><td>"lepeka"</td><td>"jan Lepeka's personal glyph"</td></tr>
            <tr><td>(none)</td><td>"lipamanka"</td><td>"lipamanka's personal glyph"</td></tr>
          </table>
        </div> },
      ),
      (
        "AHK Scripts",
        view! { <div style="padding: 5px">
          <p>"See "<ExternalLink href="https://github.com/ETBCOR/nasin-nanpa/tree/main/ahk-script#readme" display="ahk-script/README.md"/>"."</p>
        </div> },
      ),
    ],
  ));

  view! { <Window
    base=WindowBase {
      id: "nasin-nanpa-win",
      title: "nasin sitelen tan anpa nanpa".to_string(),
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
