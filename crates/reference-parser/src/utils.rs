use crate::{book_name_to_book, BibleReferenceQuery, BibleReferenceRange, Book};
use regex::{Match, Regex};

pub fn parse_reference(reference: &str) -> Vec<BibleReferenceRange> {
    let mut list: Vec<BibleReferenceRange> = Vec::new();
    let mut prev: Option<BibleReferenceRange> = None;
    let mut bracket_opened = false;

    let possible_bracket_delimiters = ["", ",", ";", "[", "]", "(", ")"]
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>();

    // basic case -- add a range for each of the pieces of the citation
    for part in split_str_and_keep_delimiters(reference, &[',', ';', '[', ']', '(', ')'][..]) {
        let trimmed = part.trim();
        // if it's only a delimiter, open or close bracket if necessary, but otherwise do nothing
        if possible_bracket_delimiters.contains(&trimmed.to_string()) {
            if trimmed == "[" || trimmed == "(" {
                bracket_opened = true;
            } else if trimmed == "]" || trimmed == ")" {
                bracket_opened = false;
            }
        } else {
            let current = parse_single_reference(&part, prev, bracket_opened);
            list.push(current);
            prev = Some(current);
        }
    }

    // handle citations like 1 Cor. 13:[1-3]4-13
    let starts_with_bracket = list
        .get(0)
        .map(|range| range.start)
        .and_then(|range| range.verse)
        .is_none()
        && list.get(1).map(|range| range.bracketed).unwrap_or(false);

    // TODO this is a disaster
    if starts_with_bracket {
        let start_book = match (
            list.get(1).and_then(|range| range.start.book),
            list.get(0).and_then(|range| range.start.book),
        ) {
            (Some(book), Some(_)) => Some(book),
            (None, Some(book)) => Some(book),
            (Some(book), None) => Some(book),
            (None, None) => None,
        };

        let start_chapter = match (
            list.get(1).and_then(|range| range.start.chapter),
            list.get(0).and_then(|range| range.start.chapter),
        ) {
            (Some(c), Some(_)) => Some(c),
            (None, Some(c)) => Some(c),
            (Some(c), None) => Some(c),
            (None, None) => None,
        };

        let start_verse = match (
            list.get(1).and_then(|range| range.start.verse),
            list.get(0).and_then(|range| range.start.verse),
        ) {
            (Some(v), Some(_)) => Some(v),
            (None, Some(v)) => Some(v),
            (Some(v), None) => Some(v),
            (None, None) => None,
        };

        let end_book = list
            .get(1)
            .and_then(|range| range.end.map(|query| query.book))
            .unwrap_or_else(|| {
                list.get(0)
                    .and_then(|range| range.end.and_then(|query| query.book))
            })
            .or(start_book);

        let end_chapter = list
            .get(1)
            .and_then(|range| range.end.map(|query| query.chapter))
            .unwrap_or_else(|| {
                list.get(0)
                    .and_then(|range| range.end.and_then(|query| query.chapter))
            })
            .or(start_chapter);

        let end_verse = list
            .get(1)
            .and_then(|range| range.end.map(|query| query.verse))
            .unwrap_or_else(|| {
                list.get(0)
                    .and_then(|range| range.end.and_then(|query| query.verse))
            })
            .or(start_verse);

        list.remove(0);
        list[0] = BibleReferenceRange {
            start: BibleReferenceQuery {
                book: start_book,
                chapter: start_chapter,
                verse: start_verse,
            },
            end: Some(BibleReferenceQuery {
                book: end_book,
                chapter: end_chapter,
                verse: end_verse,
            }),
            bracketed: true,
        };
    }
    // return
    list
}

fn split_str_and_keep_delimiters(text: &str, delimiters: &[char]) -> Vec<String> {
    let mut result = Vec::new();
    let mut last = 0;
    for (index, matched) in text.match_indices(delimiters) {
        if last != index {
            result.push(text[last..index].to_string());
        }
        result.push(matched.to_string());
        last = index + matched.len();
    }
    if last < text.len() {
        result.push(text[last..].to_string());
    }
    result
}

fn parse_single_reference(
    reference: &str,
    previous: Option<BibleReferenceRange>,
    bracketed: bool,
) -> BibleReferenceRange {
    let mut range_pieces = reference.split('-');
    let first_half = range_pieces.next();
    let second_half = range_pieces.next();

    let start_partial_structure = match previous {
        Some(_) => true,
        None => false,
    };

    let start: BibleReferenceQuery = match first_half {
        Some(cite) => match query_from_re(
            &cite,
            Regex::new(r"([\d\s]*[\w\.]+[a-zA-Z\s]*)\s*(\d+)?:?(\d+)?").expect("Regex invalid."),
            start_partial_structure,
            None,
        ) {
            Some(query) => query,
            None => {
                return BibleReferenceRange {
                    start: BibleReferenceQuery {
                        book: None,
                        chapter: None,
                        verse: None,
                    },
                    end: None,
                    bracketed,
                }
            }
        },
        None => {
            return BibleReferenceRange {
                start: BibleReferenceQuery {
                    book: None,
                    chapter: None,
                    verse: None,
                },
                end: None,
                bracketed,
            }
        }
    };

    // fill out the start of the range with the details of the end of the previous range
    // e.g., 1 Tim. 4:1-3, 4-6 will fill out with 1 Tim., chapter 4
    let previous_end: Option<BibleReferenceQuery> = match previous {
        Some(range) => range.end,
        None => None,
    };
    let augmented_start = fill_out(Some(start), previous_end);

    let end = match second_half {
        Some(cite) => query_from_re(
            &cite,
            Regex::new(r"([\d\s]*[\w\.]+)\s*(\d+)?:?(\d+)?").expect("Regex invalid."),
            true,
            augmented_start,
        ),
        None => None,
    };

    BibleReferenceRange {
        start: match augmented_start {
            Some(augmented) => augmented,
            None => start,
        },
        end,
        bracketed,
    }
}

fn query_from_re(
    reference: &str,
    re: Regex,
    partial_structure: bool,
    template: Option<BibleReferenceQuery>,
) -> Option<BibleReferenceQuery> {
    let captures = match re.captures(reference.trim()) {
        Some(capture) => capture,
        None => return None,
    };
    let mut iter = captures.iter();
    let matches = (
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    );

    let query: Option<BibleReferenceQuery>;
    if partial_structure {
        // build query based on matches
        // if only one part of Regex matches, assume it's a verse; if two, it's a chapter-verse; if three, book-chapter-verse
        // this allows a match on e.g., [Matthew 1:4-]3 to read "3" as a verse, not as a book name like "3 John"
        query = match matches {
            (Some(_), Some(book_name), Some(chapter_str), Some(verse_str)) => {
                Some(BibleReferenceQuery {
                    book: Some(book_name_to_book(book_name.as_str())),
                    chapter: match_to_int(chapter_str),
                    verse: match_to_int(verse_str),
                })
            }
            (Some(_), Some(chapter_str), None, Some(verse_str)) => Some(BibleReferenceQuery {
                book: None,
                chapter: match_to_int(chapter_str),
                verse: match_to_int(verse_str),
            }),
            (Some(_), Some(chapter_str), Some(verse_str), None) => Some(BibleReferenceQuery {
                book: None,
                chapter: match_to_int(chapter_str),
                verse: match_to_int(verse_str),
            }),
            (Some(_), Some(verse_str), None, None) => Some(BibleReferenceQuery {
                book: None,
                chapter: None,
                verse: match_to_int(verse_str),
            }),
            _ => None,
        };
    } else {
        let book = match matches.1 {
            Some(book_name) => Some(book_name_to_book(book_name.as_str())),
            None => None,
        };
        let chapter = match matches.2 {
            Some(num) => match_to_int(num),
            None => None,
        };
        let verse = match matches.3 {
            Some(num) => match_to_int(num),
            None => None,
        };
        query = Some(BibleReferenceQuery {
            book,
            chapter,
            verse,
        });
    }

    fill_out(query, template)
}

fn fill_out(
    query: Option<BibleReferenceQuery>,
    template: Option<BibleReferenceQuery>,
) -> Option<BibleReferenceQuery> {
    let mut final_query: Option<BibleReferenceQuery> = query;

    // if template provided, fill out query as needed
    if let Some(tpl) = template {
        if let Some(mut q) = query {
            if let None = q.book {
                q.book = tpl.book;
            }
            if let None = q.chapter {
                q.chapter = tpl.chapter;
            }
            if let None = q.verse {
                q.verse = tpl.verse;
            }
            final_query = Some(q);
        }
    }

    final_query
}

fn match_to_int(input: Match) -> Option<u16> {
    let input_digits_only = input
        .as_str()
        .replace(|c| ['a', 'b', 'c', 'd', 'e', 'f', 'g'].contains(&c), "");
    match str::parse::<u16>(&input_digits_only) {
        Ok(val) => Some(val),
        Err(_) => None,
    }
}