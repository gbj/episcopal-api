use liturgy::{Reference, Source};
use perseus::{t, Html, RenderFnResultWithCause, Template};
use serde::{Deserialize, Serialize};
use sycamore::context::*;
use sycamore::{generic_node::GenericNode, prelude::*};

use crate::components::{menu_component, reference};

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("canticle-table")
        .template(canticle_table_page)
        .build_state_fn(get_build_props)
        .head(head)
}

#[perseus::head]
pub fn head<G: Html>() -> View<G> {
    let title = format!("{} - {}", t!("canticle_table"), t!("common_prayer"));
    view! {
        title { (title) }
        link(rel = "stylesheet", href = "/.perseus/static/canticle-table.css")
        link(rel = "stylesheet", href = "/.perseus/static/document.css")
    }
}

#[derive(Serialize, Deserialize)]
struct CanticleTablePageProps {
    locale: String,
}

#[derive(Clone)]
struct LocaleContext(String);

#[perseus::autoserde(build_state)]
pub async fn get_build_props(
    _path: String,
    locale: String,
) -> RenderFnResultWithCause<CanticleTablePageProps> {
    Ok(CanticleTablePageProps { locale })
}

#[perseus::template(CanticleTablePage)]
#[component(CanticleTablePage<G>)]
pub fn canticle_table_page(props: CanticleTablePageProps) -> View<G> {
    let locale = props.locale;

    view! {
      ContextProvider(ContextProviderProps {
          value: LocaleContext(locale.clone()),
          children: || view! {
              header {
                  (cloned!((locale) => menu_component(locale)))
                  p(class = "page-title") {
                      (t!("canticle_table"))
                  }
              }
              main {
                  h1 {
                  (t!("canticle_table"))
                  }
                  section {
                  h2 {
                      (t!("bcp_1979"))
                  }
                  (reference(Reference {
                      source: Source::BCP1979,
                      page: 144,
                  }))
                  h3 {
                      (t!("canticles_mp"))
                  }
                  table(class = "canticle-table") {
                      tr {
                      td { }
                      td {
                          em {
                          (t!("after_ot"))
                          }
                      }
                      td {
                          em {
                          (t!("after_nt"))
                          }
                      }
                      }
                      tr(class = "day") {
                      td(class = "day-name") {
                          (t!("sunday_abbrev"))
                      }
                      td {
                          (a_or_b(("4", "RiteI", "4."), ("16", "RiteII", "16. Benedictus Dominus")))
                      }
                      td {
                          (a_or_b(("7", "RiteI", "7."), ("21", "RiteII", "21. Te Deum laudamus")))
                      }
                      }
                      tr {
                      td {}
                      td {
                          em {
                          (t!("advent")) ": "
                          }
                          br { }
                          (canticle_link("11", "RiteII", "11. Surge, illuminare"))
                      }
                      td {
                          em {
                          (t!("advent_and_lent")) ": "
                          }
                          br { }
                          (a_or_b(("4", "RiteI", "4."), ("16", "RiteII", "16. Benedictus Dominus")))
                      }
                      }
                      tr {
                      td { }
                      td {
                          em {
                          (t!("lent")) ": "
                          }
                          br { }
                          (canticle_link("14", "RiteII", "14. Kyrie Pantokrator"))
                      }
                      td { }
                      }
                      tr {
                      td { }
                      td {
                          em {
                          (t!("easter")) ": "
                          }
                          br { }
                          (canticle_link("8", "RiteII", "8. Cantemus Domino"))
                      }
                      td { }
                      }
                      tr(class = "day") {
                      td(class = "day-name") {
                          (t!("monday_abbrev"))
                      }
                      td {
                          (canticle_link("9", "RiteII", "9. Ecce, Deus"))
                      }
                      td {
                          (canticle_link("19", "RiteII", "19. Magna et mirabilia"))
                      }
                      }
                      tr(class = "day") {
                      td(class = "day-name") {
                          (t!("tuesday_abbrev"))
                      }
                      td {
                          (a_or_b(("2", "RiteI", "2."), ("13", "RiteII", "13. Benedictus es")))
                      }
                      td {
                          (canticle_link("18", "RiteII", "18. Dignus es"))
                      }
                      }
                      tr(class = "day") {
                      td(class = "day-name") {
                          (t!("wednesday_abbrev"))
                      }
                      td {
                          (canticle_link("11", "RiteII", "11. Surge, illuminare"))
                      }
                      td {
                          (a_or_b(("4", "RiteI", "4."), ("16", "RiteII", "16. Benedictus Dominus")))
                      }
                      }
                      tr {
                      td {}
                      td {
                          em {
                          (t!("lent")) ": "
                          }
                          br { }
                          (canticle_link("14", "RiteII", "14. Kyrie Pantokrator"))
                      }
                      td {}
                      }
                      tr(class = "day") {
                      td(class = "day-name") {
                          (t!("thursday_abbrev"))
                      }
                      td {
                          (canticle_link("8", "RiteII", "8. Cantemus Domino"))
                      }
                      td {
                          (a_or_b(("6", "RiteI", "6."), ("20", "RiteII", "20. Gloria in excelsis")))
                      }
                      }
                      tr {
                      td { }
                      td { }
                      td {
                          em {
                          (t!("advent_and_lent")) ": "
                          }
                          br { }
                          (canticle_link("19", "RiteII", "19. Magna et mirabilia"))
                      }
                      }
                      tr(class = "day") {
                      td(class = "day-name") {
                          (t!("friday_abbrev"))
                      }
                      td {
                          (canticle_link("10", "RiteII", "10. Quaerite Dominums"))
                      }
                      td {
                          (canticle_link("18", "RiteII", "18. Dignus es"))
                      }
                      }
                      tr {
                      td {}
                      td {
                          em {
                          (t!("lent")) ": "
                          }
                          br { }
                          (canticle_link("14", "RiteII", "14. Kyrie Pantokrator"))
                      }
                      }
                      tr(class = "day") {
                      td(class = "day-name") {
                          (t!("saturday_abbrev"))
                      }
                      td {
                          (a_or_b(("1", "RiteI", "1."), ("12", "RiteII", "12. Benedicite")))
                      }
                      td {
                          (canticle_link("19", "RiteII", "19. Magna et mirabilia"))
                      }
                      }
                  }
                  p {
                      em(class = "rubric") {
                      (t!("on_feasts"))
                      }
                  }
                  table {
                      tr {
                      td { }
                      td {
                          (a_or_b(("4", "RiteI", "4."), ("16", "RiteII", "16. Benedictus Dominus")))
                      }
                      td {
                          (a_or_b(("7", "RiteI", "7."), ("21", "RiteII", "21. Te Deum laudamus")))
                      }
                      }
                  }
                  h3 {
                      (t!("canticles_ep"))
                  }
                  table(class = "canticle-table") {
                      tr {
                      td { }
                      td {
                          em {
                          (t!("after_ot"))
                          }
                      }
                      td {
                          em {
                          (t!("after_nt"))
                          }
                      }
                      }
                      tr(class = "day") {
                      td(class = "day-name") {
                          (t!("sunday_abbrev"))
                      }
                      td {
                          (a_or_b(("3", "RiteI", "3."), ("15", "RiteII", "15. Magnificat")))
                      }
                      td {
                          (a_or_b(("5", "RiteI", "5."), ("17", "RiteII", "17. Nunc dimittis")))
                      }
                      }
                      tr(class = "day") {
                      td(class = "day-name") {
                          (t!("monday_abbrev"))
                      }
                      td {
                          (canticle_link("8", "RiteII", "8. Cantemus, Domino"))
                      }
                      td {
                          (a_or_b(("5", "RiteI", "5."), ("17", "RiteII", "17. Nunc dimittis")))
                      }
                      }
                      tr {
                      td {}
                      td {
                          em {
                          (t!("lent")) ": "
                          }
                          br {}
                          (canticle_link("14", "RiteII", "14. Kyrie Pantrokrator"))
                      }
                      }
                      tr(class = "day") {
                      td(class = "day-name") {
                          (t!("tuesday_abbrev"))
                      }
                      td {
                          (canticle_link("10", "RiteII", "10. Quaerite Dominum"))
                      }
                      td {
                          (a_or_b(("3", "RiteI", "3."), ("15", "RiteII", "15. Nunc dimittis")))
                      }
                      }
                      tr(class = "day") {
                      td(class = "day-name") {
                          (t!("wednesday_abbrev"))
                      }
                      td {
                          (a_or_b(("1", "RiteI", "1."), ("12", "RiteII", "12. Benedicite")))
                      }
                      td {
                          (a_or_b(("5", "RiteI", "5."), ("17", "RiteII", "17. Nunc dimittis")))
                      }
                      }
                      tr(class = "day") {
                      td(class = "day-name") {
                          (t!("thursday_abbrev"))
                      }
                      td {
                          (canticle_link("11", "RiteII", "11. Surge, illuminare"))
                      }
                      td {
                          (a_or_b(("3", "RiteI", "3."), ("15", "RiteII", "15. Magnificat")))
                      }
                      }
                      tr(class = "day") {
                      td(class = "day-name") {
                          (t!("friday_abbrev"))
                      }
                      td {
                          (a_or_b(("2", "RiteI", "2."), ("13", "RiteII", "13. Benedictus es")))
                      }
                      td {
                          (a_or_b(("5", "RiteI", "5."), ("17", "RiteII", "17. Nunc dimittis")))
                      }
                      }
                      tr(class = "day") {
                      td(class = "day-name") {
                          (t!("saturday_abbrev"))
                      }
                      td {
                          (canticle_link("9", "RiteII", "9. Ecce, Deus"))
                      }
                      td {
                          (a_or_b(("3", "RiteI", "3."), ("15", "RiteII", "15. Magnificat")))
                      }
                      }
                  }
                  p {
                      em(class = "rubric") {
                      (t!("on_feasts"))
                      }
                  }
                  table {
                      tr {
                      td { }
                      td {
                          (a_or_b(("3", "RiteI", "3."), ("15", "RiteII", "15. Magnificat")))
                      }
                      td {
                          (a_or_b(("5", "RiteI", "5."), ("17", "RiteII", "17. Nunc dimittis")))
                      }
                      }
                  }
                  p {
                      em(class = "rubric") {
                      (t!("magnificat_note"))
                      }
                  }
                  }
                  section {
                  h2 {
                      (t!("eow_1"))
                  }
                  (reference(Reference {
                      source: Source::EOW1,
                      page: 44,
                  }))
                  details {
                      summary {
                      (t!("please_note"))
                      }
                      p {
                      (t!("eow_canticle_table_note"))
                      }
                  }
                  h3 {
                      (t!("canticles_mp"))
                  }
                  h4 {
                      (t!("supplemental_materials"))
                  }
                  table(class = "canticle-table") {
                      tr {
                      td { }
                      td {
                          em {
                          (t!("after_ot"))
                          }
                      }
                      td {
                          em {
                          (t!("after_nt"))
                          }
                      }
                      }
                      tr(class = "day") {
                      td(class = "day-name") {
                          (t!("sunday_abbrev"))
                      }
                      td {
                          (a_or_b(("e", "EOW", "E. A Song of Jerusalem Our Mother"), ("16", "EOW", "16. The Song of Zechariah")))
                      }
                      td {
                          (a_or_b(("k", "EOW", "K. A Song of Our Adoption"), ("21", "EOW", "21. We Praise You O Go")))
                      }
                      }
                      tr {
                      td {}
                      td {
                          em {
                          (t!("advent")) ": "
                          }
                          br { }
                          (canticle_link("d", "EOW", "D. A Song of the Wilderness"))
                      }
                      td {
                          em {
                          (t!("advent")) ": "
                          }
                          br { }
                          (canticle_link("p", "EOW", "P. A Song of the Spirit"))
                      }
                      }
                      tr {
                      td {}
                      td {
                          em {
                          (t!("christmas")) ": "
                          }
                          br { }
                          (a_or_b(("c", "EOW", "C. A Song of Hannah"), ("9", "RiteII", "9. The First Song of Isaiah")))
                      }
                      td {
                          em {
                          (t!("christmas")) ": "
                          }
                          br { }
                          (a_or_b(("n", "EOW", "N. A Song of God’s Love"), ("20", "RiteII", "20. Glory to God")))
                      }
                      }
                      tr {
                      td {}
                      td {
                          em {
                          (t!("lent")) ": "
                          }
                          br { }
                          (canticle_link("h", "EOW", "H. A Song of Hosea"))
                      }
                      td {
                          em {
                          (t!("lent")) ": "
                          }
                          br { }
                          (canticle_link("l", "EOW", "L. A Song of Christ’s Humility"))
                      }
                      }
                      tr {
                      td {}
                      td {
                          em {
                          (t!("easter")) ": "
                          }
                          br { }
                          (a_or_b(("a", "EOW", "A. A Song of Wisdom"), ("8", "RiteII", "8. The Song of Moses")))
                      }
                      td {
                          em {
                          (t!("easter")) ": "
                          }
                          br { }
                          (canticle_link("m", "EOW", "M. A Song of Faith"))
                      }
                      }
                      tr(class = "day") {
                      td(class = "day-name") {
                          (t!("monday_abbrev"))
                      }
                      td {
                          (a_or_b(("c", "EOW", "C. A Song of Hannah"), ("11", "RiteII", "11. The Third Song of Isaiah")))
                      }
                      td {
                          (a_or_b(("l", "EOW", "L. A Song of Christ’s Humility"), ("q", "EOW", "Q. A Song of Christ’s Goodness")))
                      }
                      }
                      tr(class = "day") {
                      td(class = "day-name") {
                          (t!("tuesday_abbrev"))
                      }
                      td {
                          (a_or_b(("b", "EOW", "B. A Song of Pilgrimage"), ("13", "EOW", "13. A Song of Praise")))
                      }
                      td {
                          (a_or_b(("m", "EOW", "M. A Song of Faith"), ("n", "EOW", "N. A Song of God’s Love")))
                      }
                      }
                      tr(class = "day") {
                      td(class = "day-name") {
                          (t!("wednesday_abbrev"))
                      }
                      td {
                          (a_or_b(("g", "EOW", "G. A Song of Ezekiel"), ("h", "EOW", "H. A Song of Hosea")))
                      }
                      td {
                          (a_or_b(("p", "EOW", "P. A Song of the Spirit"), ("s", "EOW", "S. A Song of Our True Nature")))
                      }
                      }
                      tr {
                      td { }
                      td {
                          em {
                          (t!("lent")) ": "
                          }
                          br { }
                          (a_or_b(("i", "EOW", "I. A Song of Jonah"), ("10", "RiteII", "10. The Second Song of Isaiah")))
                      }
                      td { }
                      }
                      tr(class = "day") {
                      td(class = "day-name") {
                          (t!("thursday_abbrev"))
                      }
                      td {
                          (a_or_b(("a", "EOW", "A. A Song of Wishdom"), ("j", "EOW", "J. A Song of Judith")))
                      }
                      td {
                          (a_or_b(("r", "EOW", "R. A Song of True Motherhood"), ("16", "RiteII", "16. A Song of Zechariah")))
                      }
                      }
                      tr(class = "day") {
                      td(class = "day-name") {
                          (t!("friday_abbrev"))
                      }
                      td {
                          (canticle_link("i", "EOW", "I. A Song of Jonah"))
                      }
                      td {
                          (canticle_link("18", "EOW", "18. Song to the Lamb"))
                      }
                      }
                      tr {
                      td {}
                      td {
                          em {
                          (t!("christmas")) ":* "
                          }
                          br { }
                          (canticle_link("j", "EOW", "J. A Song of Judith"))
                      }
                      td {
                          em {
                          (t!("christmas")) ":* "
                          }
                          br { }
                          (canticle_link("r", "EOW", "R. A Song of True Motherhood"))
                      }
                      }
                      tr {
                      td {}
                      td {
                          em {
                          (t!("lent")) ": "
                          }
                          br { }
                          (a_or_b(("f", "EOW", "F. A Song of Lamentation"), ("14", "RiteII", "14. A Song of Penitence")))
                      }
                      td {
                          em {
                          (t!("lent")) ": "
                          }
                          br { }
                          (canticle_link("s", "EOW", "S. A Song of Our True Nature"))
                      }
                      }
                      tr {
                      td {}
                      td {
                          em {
                          (t!("easter")) ":* "
                          }
                          br { }
                          (canticle_link("g", "EOW", "G. A Song of Ezekiel"))
                      }
                      td {
                          em {
                          (t!("easter")) ":* "
                          }
                          br { }
                          (canticle_link("k", "EOW", "K. A Song of Our Adoption"))
                      }
                      }
                      tr(class = "day") {
                      td(class = "day-name") {
                          (t!("saturday_abbrev"))
                      }
                      td {
                          (a_or_b(("12", "EOW", "12. A Song of Creation"), ("d", "EOW", "D. A Song of the Wilderness")))
                      }
                      td {
                          (a_or_b(("o", "EOW", "O. A Song of the Heavenly City"), ("19", "RiteII", "19. The Song of the Redeemed")))
                      }
                      }
                  }
                  p {
                      em(class = "rubric") {
                      (t!("on_feasts"))
                      }
                  }
                  table {
                      tr {
                      td { }
                      td {
                          (a_or_b(("16", "EOW", "16. A Song of Zechariah"), ("e", "EOW", "E. A Song of Jerusalem Our Mother")))
                      }
                      td {
                          (a_or_b(("21", "EOW", "21. We Praise You O GOd"), ("K", "EOW", "K. A Song of Our Adoption")))
                      }
                      }
                  }
                  p {
                      em(class = "rubric") {
                      (t!("canticles_appointed_for_christmas"))
                      }
                  }
                  h3 {
                      (t!("canticles_ep"))
                  }
                  table(class = "canticle-table") {
                      tr {
                      td { }
                      td {
                          em {
                          (t!("after_ot"))
                          }
                      }
                      td {
                          em {
                          (t!("after_nt"))
                          }
                      }
                      }
                      tr(class = "day") {
                      td(class = "day-name") {
                          (t!("sunday_abbrev"))
                      }
                      td {
                          (canticle_link("15", "EOW", "15. The Song of Mary"))
                      }
                      td {
                          (a_or_b(("17", "RiteII", "The Song of Simeon**"), ("m", "EOW", "M. A Song of Faith")))
                      }
                      }
                      tr(class = "day") {
                      td(class = "day-name") {
                          (t!("monday_abbrev"))
                      }
                      td {
                          (canticle_link("a", "EOW", "A. A Song of Wisdom"))
                      }
                      td {
                          (a_or_b(("n", "EOW", "N. A Song of God’s Love"), ("17", "RiteII", "The Song of Simeon")))
                      }
                      }
                      tr(class = "day") {
                      td(class = "day-name") {
                          (t!("tuesday_abbrev"))
                      }
                      td {
                          (canticle_link("d", "EOW", "D. A Song of the Wilderness"))
                      }
                      td {
                          (a_or_b(("15", "EOW", "15. The Song of Mary"), ("p", "EOW", "P. A Song of the Spirit")))
                      }
                      }
                      tr(class = "day") {
                      td(class = "day-name") {
                          (t!("wednesday_abbrev"))
                      }
                      td {
                          (canticle_link("c", "EOW", "C. The Song of Hannah"))
                      }
                      td {
                          (a_or_b(("l", "EOW", "L. A Song of Christ’s Humility"), ("17", "RiteII", "The Song of Simeon")))
                      }
                      }
                      tr(class = "day") {
                      td(class = "day-name") {
                          (t!("thursday_abbrev"))
                      }
                      td {
                          (canticle_link("j", "EOW", "J. A Song of Judith"))
                      }
                      td {
                          (a_or_b(("15", "EOW", "15. The Song of Mary"), ("s", "EOW", "S. A Song of Our True Nature")))
                      }
                      }
                      tr(class = "day") {
                      td(class = "day-name") {
                          (t!("friday_abbrev"))
                      }
                      td {
                          (canticle_link("g", "EOW", "G. A Song of Ezekiel"))
                      }
                      td {
                          (a_or_b(("q", "EOW", "Q. A Song of Christ’s Goodness"), ("17", "RiteII", "The Song of Simeon")))
                      }
                      }
                      tr(class = "day") {
                      td(class = "day-name") {
                          (t!("saturday_abbrev"))
                      }
                      td {
                          (canticle_link("b", "EOW", "B. A Song of Pilgrimage"))
                      }
                      td {
                          (a_or_b(("15", "EOW", "15. The Song of Mary"), ("r", "EOW", "R. A Song of True Motherhood")))
                      }
                      }
                  }
                  p {
                      em(class = "rubric") {
                      (t!("on_feasts"))
                      }
                  }
                  table {
                      tr {
                      td { }
                      td {
                          (canticle_link("15", "EOW", "15. The Song of Mary"))
                      }
                      td {
                          (a_or_b(("o", "EOW", "O. A Song of the Heavenly City**"), ("17", "RiteII", "The Song of Simeon**")))
                      }
                      }
                  }
                  p {
                      em(class = "rubric") {
                      (t!("magnificat_note_eow"))
                      }
                  }
                  }
              }
          }
      })
    }
}

fn a_or_b<G: GenericNode>(
    a: (&'static str, &'static str, &'static str),
    b: (&'static str, &'static str, &'static str),
) -> View<G> {
    let (a_number, a_version, a_label) = a;
    let (b_number, b_version, b_label) = b;
    view! {
      (canticle_link(a_number, a_version, a_label))
      " "
      (t!("or"))
      " "
      (canticle_link(b_number, b_version, b_label))
    }
}

fn canticle_link<G: GenericNode>(
    number: &'static str,
    version: &'static str,
    label: &'static str,
) -> View<G> {
    let locale = use_context::<LocaleContext>();

    view! {
      a(href = format!("{}/document/canticle/{}/{}", locale.0, number, version)) {
        (label)
      }
    }
}
