#[cfg(test)]
mod tests {
    use reference_parser::{parse_reference, BibleReferenceQuery, BibleReferenceRange, Book};

    #[test]
    fn simple_range() {
        assert_eq!(
            parse_reference("Col. 1:29-2:2"),
            vec!(BibleReferenceRange {
                start: BibleReferenceQuery {
                    book: Some(Book::Colossians),
                    chapter: Some(1),
                    verse: Some(29)
                },
                end: Some(BibleReferenceQuery {
                    book: Some(Book::Colossians),
                    chapter: Some(2),
                    verse: Some(2)
                }),
                bracketed: false
            })
        );

        assert_eq!(
            parse_reference("Neh. 20:1-4"),
            vec!(BibleReferenceRange {
                start: BibleReferenceQuery {
                    book: Some(Book::Nehemiah),
                    chapter: Some(20),
                    verse: Some(1)
                },
                end: Some(BibleReferenceQuery {
                    book: Some(Book::Nehemiah),
                    chapter: Some(20),
                    verse: Some(4)
                }),
                bracketed: false
            })
        );
    }

    #[test]
    fn comma_and_misspelling() {
        assert_eq!(
            parse_reference("1 Tin 4:1-3, 4-6"),
            vec!(
                BibleReferenceRange {
                    start: BibleReferenceQuery {
                        book: Some(Book::FirstTimothy),
                        chapter: Some(4),
                        verse: Some(1)
                    },
                    end: Some(BibleReferenceQuery {
                        book: Some(Book::FirstTimothy),
                        chapter: Some(4),
                        verse: Some(3)
                    }),
                    bracketed: false
                },
                BibleReferenceRange {
                    start: BibleReferenceQuery {
                        book: Some(Book::FirstTimothy),
                        chapter: Some(4),
                        verse: Some(4)
                    },
                    end: Some(BibleReferenceQuery {
                        book: Some(Book::FirstTimothy),
                        chapter: Some(4),
                        verse: Some(6)
                    }),
                    bracketed: false
                }
            )
        );
    }

    #[test]
    fn two_books() {
        assert_eq!(
            parse_reference("1 Tin 4:1-3; Col. 3:1"),
            vec!(
                BibleReferenceRange {
                    start: BibleReferenceQuery {
                        book: Some(Book::FirstTimothy),
                        chapter: Some(4),
                        verse: Some(1)
                    },
                    end: Some(BibleReferenceQuery {
                        book: Some(Book::FirstTimothy),
                        chapter: Some(4),
                        verse: Some(3)
                    }),
                    bracketed: false
                },
                BibleReferenceRange {
                    start: BibleReferenceQuery {
                        book: Some(Book::Colossians),
                        chapter: Some(3),
                        verse: Some(1)
                    },
                    end: None,
                    bracketed: false
                }
            )
        );
    }

    #[test]
    fn misspelled_simple_range() {
        assert_eq!(
            parse_reference("1 Tin 4:1-3"),
            vec!(BibleReferenceRange {
                start: BibleReferenceQuery {
                    book: Some(Book::FirstTimothy),
                    chapter: Some(4),
                    verse: Some(1)
                },
                end: Some(BibleReferenceQuery {
                    book: Some(Book::FirstTimothy),
                    chapter: Some(4),
                    verse: Some(3)
                }),
                bracketed: false
            })
        );
    }

    #[test]
    fn single_chapter() {
        assert_eq!(
            parse_reference("1 Tin"),
            vec!(BibleReferenceRange {
                start: BibleReferenceQuery {
                    book: Some(Book::FirstTimothy),
                    chapter: None,
                    verse: None
                },
                end: None,
                bracketed: false
            })
        );
    }

    #[test]
    fn book_only() {
        assert_eq!(
            parse_reference("Colossians"),
            vec!(BibleReferenceRange {
                start: BibleReferenceQuery {
                    book: Some(Book::Colossians),
                    chapter: None,
                    verse: None
                },
                end: None,
                bracketed: false
            })
        );
    }

    #[test]
    fn single_verse() {
        assert_eq!(
            parse_reference("Colossians 1:1"),
            vec!(BibleReferenceRange {
                start: BibleReferenceQuery {
                    book: Some(Book::Colossians),
                    chapter: Some(1),
                    verse: Some(1)
                },
                end: None,
                bracketed: false
            })
        );
    }

    #[test]
    fn abbreviated_book_name() {
        assert_eq!(
            parse_reference("1 Cor. 13:1"),
            vec!(BibleReferenceRange {
                start: BibleReferenceQuery {
                    book: Some(Book::FirstCorinthians),
                    chapter: Some(13),
                    verse: Some(1)
                },
                end: None,
                bracketed: false
            })
        );
        assert_eq!(
            parse_reference("1 Thess 2:3"),
            vec!(BibleReferenceRange {
                start: BibleReferenceQuery {
                    book: Some(Book::FirstThessalonians),
                    chapter: Some(2),
                    verse: Some(3)
                },
                end: None,
                bracketed: false
            })
        );
        assert_eq!(
            parse_reference("1 Tim 2:3"),
            vec!(BibleReferenceRange {
                start: BibleReferenceQuery {
                    book: Some(Book::FirstTimothy),
                    chapter: Some(2),
                    verse: Some(3)
                },
                end: None,
                bracketed: false
            })
        );
        assert_eq!(
            parse_reference("Phlm 12"),
            vec!(BibleReferenceRange {
                start: BibleReferenceQuery {
                    book: Some(Book::Philemon),
                    chapter: None,
                    verse: Some(12),
                },
                end: None,
                bracketed: false
            })
        );
        assert_eq!(
            parse_reference("Heb 1"),
            vec!(BibleReferenceRange {
                start: BibleReferenceQuery {
                    book: Some(Book::Hebrews),
                    chapter: Some(1),
                    verse: None
                },
                end: None,
                bracketed: false
            })
        );
        assert_eq!(
            parse_reference("Phil 1"),
            vec!(BibleReferenceRange {
                start: BibleReferenceQuery {
                    book: Some(Book::Philippians),
                    chapter: Some(1),
                    verse: None
                },
                end: None,
                bracketed: false
            })
        );
        assert_eq!(
            parse_reference("Philip 1"),
            vec!(BibleReferenceRange {
                start: BibleReferenceQuery {
                    book: Some(Book::Philippians),
                    chapter: Some(1),
                    verse: None
                },
                end: None,
                bracketed: false
            })
        );
        assert_eq!(
            parse_reference("Rom 1"),
            vec!(BibleReferenceRange {
                start: BibleReferenceQuery {
                    book: Some(Book::Romans),
                    chapter: Some(1),
                    verse: None
                },
                end: None,
                bracketed: false
            })
        );
    }

    #[test]
    fn citation_with_comma() {
        assert_eq!(
            parse_reference("2 Samuel 7:4, 8-16"),
            vec![
                BibleReferenceRange {
                    bracketed: false,
                    start: BibleReferenceQuery {
                        book: Some(Book::SecondSamuel),
                        chapter: Some(7),
                        verse: Some(4)
                    },
                    end: None
                },
                BibleReferenceRange {
                    bracketed: false,
                    start: BibleReferenceQuery {
                        book: None,
                        chapter: None,
                        verse: Some(8)
                    },
                    end: Some(BibleReferenceQuery {
                        book: None,
                        chapter: None,
                        verse: Some(16)
                    })
                },
            ]
        );
    }

    #[test]
    fn multiple_ranges_starts_with_single_verse() {
        assert_eq!(
            parse_reference("Judith 9:1, 11-14"),
            vec![
                BibleReferenceRange {
                    bracketed: false,
                    start: BibleReferenceQuery {
                        book: Some(Book::Judith),
                        chapter: Some(9),
                        verse: Some(1)
                    },
                    end: None
                },
                BibleReferenceRange {
                    bracketed: false,
                    start: BibleReferenceQuery {
                        book: None,
                        chapter: None,
                        verse: Some(11)
                    },
                    end: Some(BibleReferenceQuery {
                        book: None,
                        chapter: None,
                        verse: Some(14)
                    })
                },
            ]
        );
    }

    #[test]
    fn across_chapters() {
        assert_eq!(
            parse_reference("Wisdom of Solomon 1:16-2:1"),
            vec![BibleReferenceRange {
                bracketed: false,
                start: BibleReferenceQuery {
                    book: Some(Book::Wisdom),
                    chapter: Some(1),
                    verse: Some(16)
                },
                end: Some(BibleReferenceQuery {
                    book: Some(Book::Wisdom),
                    chapter: Some(2),
                    verse: Some(1)
                })
            },]
        );
    }

    #[test]
    fn across_chapters_multipart_citation() {
        assert_eq!(
            parse_reference("Wisdom of Solomon 1:16-2:1,12-22"),
            vec![
                BibleReferenceRange {
                    bracketed: false,
                    start: BibleReferenceQuery {
                        book: Some(Book::Wisdom),
                        chapter: Some(1),
                        verse: Some(16)
                    },
                    end: Some(BibleReferenceQuery {
                        book: Some(Book::Wisdom),
                        chapter: Some(2),
                        verse: Some(1)
                    })
                },
                BibleReferenceRange {
                    bracketed: false,
                    start: BibleReferenceQuery {
                        book: Some(Book::Wisdom),
                        chapter: Some(2),
                        verse: Some(12)
                    },
                    end: Some(BibleReferenceQuery {
                        book: Some(Book::Wisdom),
                        chapter: Some(2),
                        verse: Some(22)
                    })
                }
            ]
        );
    }

    #[test]
    fn starts_with_brackets() {
        assert_eq!(
            parse_reference("1 Cor. 13:[1-3]4-13"),
            vec![
                BibleReferenceRange {
                    bracketed: true,
                    start: BibleReferenceQuery {
                        book: Some(Book::FirstCorinthians),
                        chapter: Some(13),
                        verse: Some(1)
                    },
                    end: Some(BibleReferenceQuery {
                        book: Some(Book::FirstCorinthians),
                        chapter: Some(13),
                        verse: Some(3)
                    })
                },
                BibleReferenceRange {
                    bracketed: false,
                    start: BibleReferenceQuery {
                        book: None,
                        chapter: None,
                        verse: Some(4)
                    },
                    end: Some(BibleReferenceQuery {
                        book: None,
                        chapter: None,
                        verse: Some(13)
                    })
                }
            ]
        );

        assert_eq!(
            parse_reference("1 Cor. 13:(1-3)4-13"),
            vec![
                BibleReferenceRange {
                    bracketed: true,
                    start: BibleReferenceQuery {
                        book: Some(Book::FirstCorinthians),
                        chapter: Some(13),
                        verse: Some(1)
                    },
                    end: Some(BibleReferenceQuery {
                        book: Some(Book::FirstCorinthians),
                        chapter: Some(13),
                        verse: Some(3)
                    })
                },
                BibleReferenceRange {
                    bracketed: false,
                    start: BibleReferenceQuery {
                        book: None,
                        chapter: None,
                        verse: Some(4)
                    },
                    end: Some(BibleReferenceQuery {
                        book: None,
                        chapter: None,
                        verse: Some(13)
                    })
                }
            ]
        );
    }

    #[test]
    fn ends_with_brackets() {
        assert_eq!(
            parse_reference("Luke 2:1-14,(15-20)"),
            vec![
                BibleReferenceRange {
                    bracketed: false,
                    start: BibleReferenceQuery {
                        book: Some(Book::Luke),
                        chapter: Some(2),
                        verse: Some(1)
                    },
                    end: Some(BibleReferenceQuery {
                        book: Some(Book::Luke),
                        chapter: Some(2),
                        verse: Some(14)
                    })
                },
                BibleReferenceRange {
                    bracketed: true,
                    start: BibleReferenceQuery {
                        book: Some(Book::Luke),
                        chapter: Some(2),
                        verse: Some(15)
                    },
                    end: Some(BibleReferenceQuery {
                        book: Some(Book::Luke),
                        chapter: Some(2),
                        verse: Some(20)
                    })
                }
            ]
        );

        assert_eq!(
            parse_reference("Genesis 18:1-15; (21:1-7)"),
            vec![
                BibleReferenceRange {
                    bracketed: false,
                    start: BibleReferenceQuery {
                        book: Some(Book::Genesis),
                        chapter: Some(18),
                        verse: Some(1)
                    },
                    end: Some(BibleReferenceQuery {
                        book: Some(Book::Genesis),
                        chapter: Some(18),
                        verse: Some(15)
                    })
                },
                BibleReferenceRange {
                    bracketed: true,
                    start: BibleReferenceQuery {
                        book: Some(Book::Genesis),
                        chapter: Some(21),
                        verse: Some(1)
                    },
                    end: Some(BibleReferenceQuery {
                        book: Some(Book::Genesis),
                        chapter: Some(21),
                        verse: Some(7)
                    })
                }
            ]
        )
    }

    #[test]
    fn complex_references_parse_correctly() {
        assert_eq!(
            parse_reference("Matthew 9:35-10:8,(9-23)"),
            vec![
                BibleReferenceRange {
                    bracketed: false,
                    start: BibleReferenceQuery {
                        book: Some(Book::Matthew),
                        chapter: Some(9),
                        verse: Some(35)
                    },
                    end: Some(BibleReferenceQuery {
                        book: Some(Book::Matthew),
                        chapter: Some(10),
                        verse: Some(8)
                    })
                },
                BibleReferenceRange {
                    bracketed: true,
                    start: BibleReferenceQuery {
                        book: Some(Book::Matthew),
                        chapter: Some(10),
                        verse: Some(9)
                    },
                    end: Some(BibleReferenceQuery {
                        book: Some(Book::Matthew),
                        chapter: Some(10),
                        verse: Some(23)
                    })
                }
            ]
        );

        assert_eq!(
            parse_reference("1 Samuel 8:4-11,(12-15),16-20; (11:14-15)"),
            vec![
                BibleReferenceRange {
                    bracketed: false,
                    start: BibleReferenceQuery {
                        book: Some(Book::FirstSamuel),
                        chapter: Some(8),
                        verse: Some(4)
                    },
                    end: Some(BibleReferenceQuery {
                        book: Some(Book::FirstSamuel),
                        chapter: Some(8),
                        verse: Some(11)
                    })
                },
                BibleReferenceRange {
                    bracketed: true,
                    start: BibleReferenceQuery {
                        book: Some(Book::FirstSamuel),
                        chapter: Some(8),
                        verse: Some(12)
                    },
                    end: Some(BibleReferenceQuery {
                        book: Some(Book::FirstSamuel),
                        chapter: Some(8),
                        verse: Some(15)
                    })
                },
                BibleReferenceRange {
                    bracketed: false,
                    start: BibleReferenceQuery {
                        book: Some(Book::FirstSamuel),
                        chapter: Some(8),
                        verse: Some(16)
                    },
                    end: Some(BibleReferenceQuery {
                        book: Some(Book::FirstSamuel),
                        chapter: Some(8),
                        verse: Some(20)
                    }),
                },
                BibleReferenceRange {
                    bracketed: true,
                    start: BibleReferenceQuery {
                        book: Some(Book::FirstSamuel),
                        chapter: Some(11),
                        verse: Some(14)
                    },
                    end: Some(BibleReferenceQuery {
                        book: Some(Book::FirstSamuel),
                        chapter: Some(11),
                        verse: Some(15)
                    })
                },
            ]
        );
    }

    #[test]
    fn strips_alphanumeric_codes_from_verse_numbers() {
        assert_eq!(
            parse_reference("1 Samuel 17:(1a,4-11,19-23),32-49"),
            vec![
                BibleReferenceRange {
                    bracketed: true,
                    start: BibleReferenceQuery {
                        book: Some(Book::FirstSamuel),
                        chapter: Some(17),
                        verse: Some(1)
                    },
                    end: Some(BibleReferenceQuery {
                        book: Some(Book::FirstSamuel),
                        chapter: Some(17),
                        verse: Some(1)
                    })
                },
                BibleReferenceRange {
                    bracketed: true,
                    start: BibleReferenceQuery {
                        book: None,
                        chapter: None,
                        verse: Some(4)
                    },
                    end: Some(BibleReferenceQuery {
                        book: None,
                        chapter: None,
                        verse: Some(11)
                    })
                },
                BibleReferenceRange {
                    bracketed: true,
                    start: BibleReferenceQuery {
                        book: None,
                        chapter: None,
                        verse: Some(19)
                    },
                    end: Some(BibleReferenceQuery {
                        book: None,
                        chapter: None,
                        verse: Some(23)
                    }),
                },
                BibleReferenceRange {
                    bracketed: false,
                    start: BibleReferenceQuery {
                        book: None,
                        chapter: None,
                        verse: Some(32)
                    },
                    end: Some(BibleReferenceQuery {
                        book: None,
                        chapter: None,
                        verse: Some(49)
                    })
                },
            ]
        );
    }

    #[test]
    fn books_with_only_one_chapter() {
        assert_eq!(
            parse_reference("Jude 1-16"),
            vec![BibleReferenceRange {
                start: BibleReferenceQuery {
                    book: Some(Book::Jude),
                    chapter: None,
                    verse: Some(1)
                },
                end: Some(BibleReferenceQuery {
                    book: Some(Book::Jude),
                    chapter: None,
                    verse: Some(16)
                }),
                bracketed: false
            }]
        );
    }
}
