use liturgy::{Reference, Source};
use perseus::{t, Html, Template};
use sycamore::{
    generic_node::GenericNode,
    prelude::{component, view, View},
};

use crate::components::reference;

#[perseus::template(CanticleTablePage)]
#[component(CanticleTablePage<G>)]
pub fn canticle_table_page() -> View<G> {
    view! {
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
                (a_or_b(("4", "4."), ("16", "16. Benedictus Dominus")))
              }
              td {
                (a_or_b(("7", "7."), ("21", "21. Te Deum laudamus")))
              }
            }
            tr {
              td {}
              td {
                em {
                  (t!("advent")) ": "
                }
                br { }
                (canticle_link("11", "11. Surge, illuminare"))
              }
              td {
                em {
                  (t!("advent_and_lent")) ": "
                }
                br { }
                (a_or_b(("4", "4."), ("16", "16. Benedictus Dominus")))
              }
            }
            tr {
              td { }
              td {
                em {
                  (t!("lent")) ": "
                }
                br { }
                (canticle_link("14", "14. Kyrie Pantokrator"))
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
                (canticle_link("8", "8. Cantemus Domino"))
              }
              td { }
            }
            tr(class = "day") {
              td(class = "day-name") {
                (t!("monday_abbrev"))
              }
              td {
                (canticle_link("9", "9. Ecce, Deus"))
              }
              td {
                (canticle_link("19", "19. Magna et mirabilia"))
              }
            }
            tr(class = "day") {
              td(class = "day-name") {
                (t!("tuesday_abbrev"))
              }
              td {
                (a_or_b(("2", "2."), ("13", "13. Benedictus es")))
              }
              td {
                (canticle_link("18", "18. Dignus es"))
              }
            }
            tr(class = "day") {
              td(class = "day-name") {
                (t!("wednesday_abbrev"))
              }
              td {
                (canticle_link("11", "11. Surge, illuminare"))
              }
              td {
                (a_or_b(("4", "4."), ("16", "16. Benedictus Dominus")))
              }
            }
            tr {
              td {}
              td {
                em {
                  (t!("lent")) ": "
                }
                br { }
                (canticle_link("14", "14. Kyrie Pantokrator"))
              }
              td {}
            }
            tr(class = "day") {
              td(class = "day-name") {
                (t!("thursday_abbrev"))
              }
              td {
                (canticle_link("8", "8. Cantemus Domino"))
              }
              td {
                (a_or_b(("6", "6."), ("20", "20. Gloria in excelsis")))
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
                (canticle_link("19", "19. Magna et mirabilia"))
              }
            }
            tr(class = "day") {
              td(class = "day-name") {
                (t!("friday_abbrev"))
              }
              td {
                (canticle_link("10", "10. Quaerite Dominums"))
              }
              td {
                (canticle_link("18", "18. Dignus es"))
              }
            }
            tr {
              td {}
              td {
                em {
                  (t!("lent")) ": "
                }
                br { }
                (canticle_link("14", "14. Kyrie Pantokrator"))
              }
            }
            tr(class = "day") {
              td(class = "day-name") {
                (t!("saturday_abbrev"))
              }
              td {
                (a_or_b(("1", "1."), ("12", "12. Benedicite")))
              }
              td {
                (canticle_link("19", "19. Magna et mirabilia"))
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
                (a_or_b(("4", "4."), ("16", "16. Benedictus Dominus")))
              }
              td {
                (a_or_b(("7", "7."), ("21", "21. Te Deum laudamus")))
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
                (a_or_b(("3", "3."), ("15", "15. Magnificat")))
              }
              td {
                (a_or_b(("5", "5."), ("17", "17. Nunc dimittis")))
              }
            }
            tr(class = "day") {
              td(class = "day-name") {
                (t!("monday_abbrev"))
              }
              td {
                (canticle_link("8", "8. Cantemus, Domino"))
              }
              td {
                (a_or_b(("5", "5."), ("17", "17. Nunc dimittis")))
              }
            }
            tr {
              td {}
              td {
                em {
                  (t!("lent")) ": "
                }
                br {}
                (canticle_link("14", "14. Kyrie Pantrokrator"))
              }
            }
            tr(class = "day") {
              td(class = "day-name") {
                (t!("tuesday_abbrev"))
              }
              td {
                (canticle_link("10", "10. Quaerite Dominum"))
              }
              td {
                (a_or_b(("3", "3."), ("15", "15. Nunc dimittis")))
              }
            }
            tr(class = "day") {
              td(class = "day-name") {
                (t!("wednesday_abbrev"))
              }
              td {
                (a_or_b(("1", "1."), ("12", "12. Benedicite")))
              }
              td {
                (a_or_b(("5", "5."), ("17", "17. Nunc dimittis")))
              }
            }
            tr(class = "day") {
              td(class = "day-name") {
                (t!("thursday_abbrev"))
              }
              td {
                (canticle_link("11", "11. Surge, illuminare"))
              }
              td {
                (a_or_b(("3", "3."), ("15", "15. Magnificat")))
              }
            }
            tr(class = "day") {
              td(class = "day-name") {
                (t!("friday_abbrev"))
              }
              td {
                (a_or_b(("2", "2."), ("13", "13. Benedictus es")))
              }
              td {
                (a_or_b(("5", "5."), ("17", "17. Nunc dimittis")))
              }
            }
            tr(class = "day") {
              td(class = "day-name") {
                (t!("saturday_abbrev"))
              }
              td {
                (canticle_link("9", "9. Ecce, Deus"))
              }
              td {
                (a_or_b(("3", "3."), ("15", "15. Magnificat")))
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
                (a_or_b(("3", "3."), ("15", "15. Magnificat")))
              }
              td {
                (a_or_b(("5", "5."), ("17", "17. Nunc dimittis")))
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
                (a_or_b(("e", "E. A Song of Jerusalem Our Mother"), ("16-EOW", "16. The Song of Zechariah")))
              }
              td {
                (a_or_b(("k", "K. A Song of Our Adoption"), ("21-EOW", "21. We Praise You O Go")))
              }
            }
            tr {
              td {}
              td {
                em {
                  (t!("advent")) ": "
                }
                br { }
                (canticle_link("d", "D. A Song of the Wilderness"))
              }
              td {
                em {
                  (t!("advent")) ": "
                }
                br { }
                (canticle_link("p", "P. A Song of the Spirit"))
              }
            }
            tr {
              td {}
              td {
                em {
                  (t!("christmas")) ": "
                }
                br { }
                (a_or_b(("c", "C. A Song of Hannah"), ("9", "9. The First Song of Isaiah")))
              }
              td {
                em {
                  (t!("christmas")) ": "
                }
                br { }
                (a_or_b(("n", "N. A Song of God’s Love"), ("20", "20. Glory to God")))
              }
            }
            tr {
              td {}
              td {
                em {
                  (t!("lent")) ": "
                }
                br { }
                (canticle_link("h", "H. A Song of Hosea"))
              }
              td {
                em {
                  (t!("lent")) ": "
                }
                br { }
                (canticle_link("l", "L. A Song of Christ’s Humility"))
              }
            }
            tr {
              td {}
              td {
                em {
                  (t!("easter")) ": "
                }
                br { }
                (a_or_b(("a", "A. A Song of Wisdom"), ("8", "8. The Song of Moses")))
              }
              td {
                em {
                  (t!("easter")) ": "
                }
                br { }
                (canticle_link("m", "M. A Song of Faith"))
              }
            }
            // START HERE
            tr(class = "day") {
              td(class = "day-name") {
                (t!("monday_abbrev"))
              }
              td {
                (a_or_b(("c", "C. A Song of Hannah"), ("11", "11. The Third Song of Isaiah")))
              }
              td {
                (a_or_b(("l", "L. A Song of Christ’s Humility"), ("q", "Q. A Song of Christ’s Goodness")))
              }
            }
            tr(class = "day") {
              td(class = "day-name") {
                (t!("tuesday_abbrev"))
              }
              td {
                (a_or_b(("b", "B. A Song of Pilgrimage"), ("13-eow", "13. A Song of Praise")))
              }
              td {
                (a_or_b(("m", "M. A Song of Faith"), ("n", "N. A Song of God’s Love")))
              }
            }
            tr(class = "day") {
              td(class = "day-name") {
                (t!("wednesday_abbrev"))
              }
              td {
                (a_or_b(("g", "G. A Song of Ezekiel"), ("h", "H. A Song of Hosea")))
              }
              td {
                (a_or_b(("p", "P. A Song of the Spirit"), ("s", "S. A Song of Our True Nature")))
              }
            }
            tr {
              td { }
              td {
                em {
                  (t!("lent")) ": "
                }
                br { }
                (a_or_b(("i", "I. A Song of Jonah"), ("10", "10. The Second Song of Isaiah")))
              }
              td { }
            }
            tr(class = "day") {
              td(class = "day-name") {
                (t!("thursday_abbrev"))
              }
              td {
                (a_or_b(("a", "A. A Song of Wishdom"), ("j", "J. A Song of Judith")))
              }
              td {
                (a_or_b(("r", "R. A Song of True Motherhood"), ("16", "16. A Song of Zechariah")))
              }
            }
            tr(class = "day") {
              td(class = "day-name") {
                (t!("friday_abbrev"))
              }
              td {
                (canticle_link("i", "I. A Song of Jonah"))
              }
              td {
                (canticle_link("18-eow", "18. Song to the Lamb"))
              }
            }
            tr {
              td {}
              td {
                em {
                  (t!("christmas")) ":* "
                }
                br { }
                (canticle_link("j", "J. A Song of Judith"))
              }
              td {
                em {
                  (t!("christmas")) ":* "
                }
                br { }
                (canticle_link("r", "R. A Song of True Motherhood"))
              }
            }
            tr {
              td {}
              td {
                em {
                  (t!("lent")) ": "
                }
                br { }
                (a_or_b(("f", "F. A Song of Lamentation"), ("14", "14. A Song of Penitence")))
              }
              td {
                em {
                  (t!("lent")) ": "
                }
                br { }
                (canticle_link("s", "S. A Song of Our True Nature"))
              }
            }
            tr {
              td {}
              td {
                em {
                  (t!("easter")) ":* "
                }
                br { }
                (canticle_link("g", "G. A Song of Ezekiel"))
              }
              td {
                em {
                  (t!("easter")) ":* "
                }
                br { }
                (canticle_link("k", "K. A Song of Our Adoption"))
              }
            }
            tr(class = "day") {
              td(class = "day-name") {
                (t!("saturday_abbrev"))
              }
              td {
                (a_or_b(("12-eow", "12. A Song of Creation"), ("d", "D. A Song of the Wilderness")))
              }
              td {
                (a_or_b(("o", "O. A Song of the Heavenly City"), ("19", "19. The Song of the Redeemed")))
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
                (a_or_b(("16-eow", "16. A Song of Zechariah"), ("e", "E. A Song of Jerusalem Our Mother")))
              }
              td {
                (a_or_b(("21-eow", "21. We Praise You O GOd"), ("K", "K. A Song of Our Adoption")))
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
                (canticle_link("15-eow", "15. The Song of Mary"))
              }
              td {
                (a_or_b(("17", "The Song of Simeon**"), ("m", "M. A Song of Faith")))
              }
            }
            tr(class = "day") {
              td(class = "day-name") {
                (t!("monday_abbrev"))
              }
              td {
                (canticle_link("a", "A. A Song of Wisdom"))
              }
              td {
                (a_or_b(("n", "N. A Song of God’s Love"), ("17", "The Song of Simeon")))
              }
            }
            tr(class = "day") {
              td(class = "day-name") {
                (t!("tuesday_abbrev"))
              }
              td {
                (canticle_link("d", "D. A Song of the Wilderness"))
              }
              td {
                (a_or_b(("15-eow", "15. The Song of Mary"), ("p", "P. A Song of the Spirit")))
              }
            }
            tr(class = "day") {
              td(class = "day-name") {
                (t!("wednesday_abbrev"))
              }
              td {
                (canticle_link("c", "C. The Song of Hannah"))
              }
              td {
                (a_or_b(("l", "L. A Song of Christ’s Humility"), ("17", "The Song of Simeon")))
              }
            }
            tr(class = "day") {
              td(class = "day-name") {
                (t!("thursday_abbrev"))
              }
              td {
                (canticle_link("j", "J. A Song of Judith"))
              }
              td {
                (a_or_b(("15-eow", "15. The Song of Mary"), ("s", "S. A Song of Our True Nature")))
              }
            }
            tr(class = "day") {
              td(class = "day-name") {
                (t!("friday_abbrev"))
              }
              td {
                (canticle_link("g", "G. A Song of Ezekiel"))
              }
              td {
                (a_or_b(("q", "Q. A Song of Christ’s Goodness"), ("17", "The Song of Simeon")))
              }
            }
            tr(class = "day") {
              td(class = "day-name") {
                (t!("saturday_abbrev"))
              }
              td {
                (canticle_link("b", "B. A Song of Pilgrimage"))
              }
              td {
                (a_or_b(("15-eow", "15. The Song of Mary"), ("r", "R. A Song of True Motherhood")))
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
                (canticle_link("15-eow", "15. The Song of Mary"))
              }
              td {
                (a_or_b(("o", "O. A Song of the Heavenly City**"), ("17", "The Song of Simeon**")))
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
}

fn a_or_b<G: GenericNode>(
    a: (&'static str, &'static str),
    b: (&'static str, &'static str),
) -> View<G> {
    let (a_number, a_label) = a;
    let (b_number, b_label) = b;
    view! {
      (canticle_link(a_number, a_label))
      " "
      (t!("or"))
      " "
      (canticle_link(b_number, b_label))
    }
}

fn canticle_link<G: GenericNode>(number: &'static str, label: &'static str) -> View<G> {
    view! {
      a(href = format!("/document/canticle/{}", number)) {
        (label)
      }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("canticle-table")
        .template(canticle_table_page)
        .head(head)
}

#[perseus::head]
pub fn head<G: Html>() -> View<G> {
    view! {
        title { "Table of Suggested Canticles – Common Prayer" }
        link(rel = "stylesheet", href = "/.perseus/static/canticle-table.css")
        link(rel = "stylesheet", href = "/.perseus/static/document.css")
    }
}
