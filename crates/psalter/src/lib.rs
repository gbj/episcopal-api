#[macro_use]
extern crate lazy_static;
pub mod bcp1979;

use liturgy::Psalm;
use reference_parser::BibleReference;

/// Defines a version or translation of the psalms, with a single entry per psalm
pub struct Psalter {
    psalms: Vec<(u8, &'static Psalm)>,
}

impl Psalter {
    pub fn psalm_by_number(&self, number: u8) -> Option<&Psalm> {
        self.psalms
            .iter()
            .find(|(s_number, _)| *s_number == number)
            .map(|(_, psalm)| *psalm)
    }
}
