#[cfg(test)]
mod tests {
    use reference_parser::{parse_reference, BibleReferenceQuery, BibleReferenceRange, Book};

    #[test]
    fn parse_reference_test() {
        // TODO — subsequent verses/chapters within same book, separated by comma or semicolon
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
                    chapter: Some(12),
                    verse: None
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

        // TODO convert these from JS
        /*     expect(parseReference('2 Samuel 7:4, 8-16')).toEqual([
            { start: { book: Book.SecondSamuel, chapter: 7, verse: 4 }, end: null },
            { start: { book: null, chapter: null, verse: 8 }, end: { book: null, chapter: null, verse: 16 } },
          ]);
          expect(parseReference('Judith 9:1, 11-14')).toEqual([
            { start: { book: Book.Judith, chapter: 9, verse: 1 }, end: null },
            { start: { book: null, chapter: null, verse: 11 }, end: { book: null, chapter: null, verse: 14 } },
          ]);

          expect(parseReference('Wisdom of Solomon 1:16-2:1')).toEqual([
            {
              start: { book: Book.Wisdom, chapter: 1, verse: 16 },
              end: { book: Book.Wisdom, chapter: 2, verse: 1 },
            },
          ]);

          expect(parseReference('Wisdom of Solomon 1:16-2:1,12-22')).toEqual([
            {
              start: { book: Book.Wisdom, chapter: 1, verse: 16 },
              end: { book: Book.Wisdom, chapter: 2, verse: 1 },
            },
            {
              start: { book: Book.Wisdom, chapter: 2, verse: 12 },
              end: { book: Book.Wisdom, chapter: 2, verse: 22 },
            },
          ]);

          expect(parseReference('1 Cor. 13:[1-3]4-13')).toEqual([
            {
              start: { book: Book.FirstCorinthians, chapter: 13, verse: 1 },
              end: { book: Book.FirstCorinthians, chapter: 13, verse: 3 },
              bracketed: true,
            },
            {
              start: { book: null, chapter: null, verse: 4 },
              end: { book: null, chapter: null, verse: 13 },
            },
          ]);

          expect(parseReference('1 Cor. 13:(1-3)4-13')).toEqual([
            {
              start: { book: Book.FirstCorinthians, chapter: 13, verse: 1 },
              end: { book: Book.FirstCorinthians, chapter: 13, verse: 3 },
              bracketed: true,
            },
            {
              start: { book: null, chapter: null, verse: 4 },
              end: { book: null, chapter: null, verse: 13 },
            },
          ]);

          expect(parseReference('Luke 2:1-14,(15-20)')).toEqual([
            {
              start: { book: Book.Luke, chapter: 2, verse: 1 },
              end: { book: Book.Luke, chapter: 2, verse: 14 },
            },
            {
              start: { book: Book.Luke, chapter: 2, verse: 15 },
              end: { book: Book.Luke, chapter: 2, verse: 20 },
              bracketed: true,
            },
          ]);

          expect(parseReference('Genesis 18:1-15; (21:1-7)')).toEqual([
            {
              start: { book: Book.Genesis, chapter: 18, verse: 1 },
              end: { book: Book.Genesis, chapter: 18, verse: 15 },
            },
            {
              start: { book: Book.Genesis, chapter: 21, verse: 1 },
              end: { book: Book.Genesis, chapter: 21, verse: 7 },
              bracketed: true,
            },
          ]);

          expect(parseReference('Matthew 9:35-10:8,(9-23)')).toEqual([
            {
              start: { book: Book.Matthew, chapter: 9, verse: 35 },
              end: { book: Book.Matthew, chapter: 10, verse: 8 },
            },
            {
              start: { book: Book.Matthew, chapter: 10, verse: 9 },
              end: { book: Book.Matthew, chapter: 10, verse: 23 },
              bracketed: true,
            },
          ]);

          expect(parseReference('1 Samuel 8:4-11,(12-15),16-20; (11:14-15)')).toEqual([
            {
              start: { book: Book.FirstSamuel, chapter: 8, verse: 4 },
              end: { book: Book.FirstSamuel, chapter: 8, verse: 11 },
            },
            {
              start: { book: Book.FirstSamuel, chapter: 8, verse: 12 },
              end: { book: Book.FirstSamuel, chapter: 8, verse: 15 },
              bracketed: true,
            },
            {
              start: { book: Book.FirstSamuel, chapter: 8, verse: 16 },
              end: { book: Book.FirstSamuel, chapter: 8, verse: 20 },
            },
            {
              start: { book: Book.FirstSamuel, chapter: 11, verse: 14 },
              end: { book: Book.FirstSamuel, chapter: 11, verse: 15 },
              bracketed: true,
            },
          ]);
        });

        expect(parseReference('1 Samuel 17:(1a,4-11,19-23),32-49')).toEqual([
          {
            start: { book: Book.FirstSamuel, chapter: 17, verse: 1 },
            end: { book: Book.FirstSamuel, chapter: 17, verse: 1 },
            bracketed: true,
          },
          {
            start: { book: null, chapter: null, verse: 4 },
            end: { book: null, chapter: null, verse: 11 },
            bracketed: true,
          },
          {
            start: { book: null, chapter: null, verse: 19 },
            end: { book: null, chapter: null, verse: 23 },
            bracketed: true,
          },
          {
            start: { book: null, chapter: null, verse: 32 },
            end: { book: null, chapter: null, verse: 49 },
          },
        ]);

        expect(parseReference('Neh. 20:1-4')).toEqual([
          {
            start: { book: Book.Nehemiah, chapter: 20, verse: 1 },
            end: { book: Book.Nehemiah, chapter: 20, verse: 4 },
          },
        ]); */
    }
}
