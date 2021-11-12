use crate::book_abbrevs::BOOKS;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Book {
    // OT books
    Genesis,
    Exodus,
    Leviticus,
    Numbers,
    Deuteronomy,
    Joshua,
    Judges,
    Ruth,
    FirstSamuel,
    SecondSamuel,
    FirstKings,
    SecondKings,
    FirstChronicles,
    SecondChronicles,
    Ezra,
    Nehemiah,
    Esther,
    Job,
    Psalms,
    Proverbs,
    Ecclesiastes,
    SongOfSolomon,
    Isaiah,
    Jeremiah,
    Lamentations,
    Ezekiel,
    Daniel,
    Hosea,
    Joel,
    Amos,
    Obadiah,
    Jonah,
    Micah,
    Nahum,
    Habakkuk,
    Zephaniah,
    Haggai,
    Zechariah,
    Malachi,
    // Apocrypha
    Tobit,
    Judith,
    Ester,
    Wisdom,
    Ecclesiasticus,
    Baruch,
    EpistleJeremiah,
    PrayerOfAzariah,
    Susanna,
    FirstMaccabees,
    SecondMaccabees,
    FirstEsdras,
    SecondEsdras,
    FourthEsdras,
    Psalm151,
    ThirdMaccabees,
    FourthMaccabees,
    Bel,
    // NT books
    Matthew,
    Mark,
    Luke,
    John,
    Acts,
    Romans,
    FirstCorinthians,
    SecondCorinthians,
    Galatians,
    Ephesians,
    Philippians,
    Colossians,
    FirstThessalonians,
    SecondThessalonians,
    FirstTimothy,
    SecondTimothy,
    Titus,
    Philemon,
    Hebrews,
    James,
    FirstPeter,
    SecondPeter,
    FirstJohn,
    SecondJohn,
    ThirdJohn,
    Jude,
    Revelation,
    None,
}

pub fn book_name_to_book(book_name: &str) -> Book {
    let ratings = BOOKS
        .iter()
        .map(|(abbrev, book)| (strsim::sorensen_dice(book_name, abbrev), book));

    let (_, closest_book) = ratings
        .max_by(|(rating_a, _), (rating_b, _)| {
            rating_a.partial_cmp(rating_b).unwrap_or(Ordering::Equal)
        })
        .unwrap_or((0.0, &Book::None));

    *closest_book
}

#[cfg(test)]
mod tests {
    use crate::{book_name_to_book, Book};

    #[test]
    fn exact_matches() {
        assert_eq!(book_name_to_book("1 Cor."), Book::FirstCorinthians);
        assert_eq!(book_name_to_book("Matt."), Book::Matthew);
        assert_eq!(book_name_to_book("John"), Book::John);
        assert_eq!(book_name_to_book("Neh."), Book::Nehemiah);
    }

    #[test]
    fn misspellings() {
        assert_eq!(book_name_to_book("1 Tin."), Book::FirstTimothy);
    }

    #[test]
    fn possible_ambiguities() {
        assert_eq!(book_name_to_book("Ecclus."), Book::Ecclesiasticus);
        assert_eq!(book_name_to_book("Eccl."), Book::Ecclesiastes);
        assert_eq!(book_name_to_book("1 Ch"), Book::FirstChronicles);
        assert_eq!(book_name_to_book("Phil"), Book::Philippians);
    }
}
