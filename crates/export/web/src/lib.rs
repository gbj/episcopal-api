use calendar::{Calendar, Date, LiturgicalDay, LiturgicalDayId, Weekday, BCP1979_CALENDAR};
use liturgy::*;
use sauron::html::text;
use sauron::prelude::*;
use sauron::{node, Application, Cmd, Node};

#[derive(Debug)]
pub enum Msg {
    SetDocument(Document),
}
pub struct DocumentView {
    pub document: Document,
    pub calendar: &'static Calendar,
    top_level: bool,
}

impl DocumentView {
    pub fn new() -> Self {
        Self {
            document: Document::new(),
            calendar: &BCP1979_CALENDAR,
            top_level: false,
        }
    }

    pub fn to_html(&self) -> String {
        let view = self.view();
        let mut buffer = String::new();
        view.render(&mut buffer).expect("failed to render document");
        buffer
    }

    pub fn mark_as_top_level(mut self) -> Self {
        self.top_level = true;
        self
    }
}

impl Default for DocumentView {
    fn default() -> Self {
        Self::new()
    }
}
impl From<Document> for DocumentView {
    fn from(document: Document) -> Self {
        Self {
            document,
            calendar: &BCP1979_CALENDAR,
            top_level: false,
        }
    }
}

impl From<(Document, &'static Calendar)> for DocumentView {
    fn from((document, calendar): (Document, &'static Calendar)) -> Self {
        Self {
            document,
            calendar,
            top_level: false,
        }
    }
}

impl Application<Msg> for DocumentView {
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg>
    where
        Self: Sized + 'static,
    {
        match msg {
            Msg::SetDocument(doc) => self.document = doc,
        }
        Cmd::none()
    }

    fn view(&self) -> Node<Msg> {
        let label: Option<Node<Msg>> = self.document.label.as_ref().and_then(|label| {
            // top-level label handled differently
            if self.top_level {
                None
            } else {
                Some(node! {
                    <h3 class="label">{text(label)}</h3>
                })
            }
        });

        let source: Option<Node<Msg>> = self
            .document
            .source
            .map(|reference| self.reference(&reference));

        let (header, content) = match &self.document.content {
            Content::BiblicalCitation(content) => self.biblical_citation(content),
            Content::BiblicalReading(content) => self.biblical_reading(content),
            Content::Canticle(content) => self.canticle(content),
            Content::Empty => self.empty(),
            Content::GloriaPatri(content) => self.gloria_patri(content),
            Content::Heading(content) => self.heading(content),
            Content::Preces(content) => self.preces(content),
            Content::Psalm(content) => self.psalm(content),
            Content::ResponsivePrayer(content) => self.responsive_prayer(content),
            Content::Rubric(content) => self.rubric(content),
            Content::Sentence(content) => self.sentence(content),
            Content::Text(content) => self.text(content),
            Content::Series(content) => self.series(content),
            Content::Parallel(content) => todo!(),
            Content::Choice(content) => self.choice(content),
            Content::CollectOfTheDay => self.collect_of_the_day(),
            Content::PsalmCitation(content) => self.psalm_citation(content),
            Content::SubLiturgy(content) => self.sub_liturgy(content),
            Content::LectionaryReading(content) => todo!(),
            Content::Antiphon(content) => self.antiphon(content),
            Content::Litany(content) => self.litany(content),
        };

        let header = match (label, source, header) {
            (None, None, None) => text(""),
            (label, source, Some(headers)) => {
                node! {
                    <header>
                        {label.unwrap_or_else(|| text(""))}
                        {source.unwrap_or_else(|| text(""))}
                        {for header in headers {
                            header
                        }}
                    </header>
                }
            }
            (label, source, None) => {
                node! {
                    <header>
                        {label.unwrap_or_else(|| text(""))}
                        {source.unwrap_or_else(|| text(""))}
                    </header>
                }
            }
        };

        node! {
            <article class="document">
                {header}
                {content}
            </article>
        }
    }
}

impl DocumentView {
    // Helpers
    fn display_format_as_class(&self, display_format: DisplayFormat) -> String {
        match display_format {
            DisplayFormat::Default => "default",
            DisplayFormat::Abbreviated => "abbreviated",
            DisplayFormat::Omit => "omit",
            DisplayFormat::Unison => "unison",
        }
        .to_string()
    }

    fn reference(&self, reference: &Reference) -> Node<Msg> {
        node! {
            <a class="reference" href={reference.as_url()}>
                {text(reference.to_string())}
            </a>
        }
    }

    fn i18n(&self, text: &str) -> String {
        self.document.language.i18n(text)
    }

    // Content Types

    fn biblical_citation(
        &self,
        biblical_citation: &BiblicalCitation,
    ) -> (Option<Vec<Node<Msg>>>, Node<Msg>) {
        (
            None,
            node! {
                <article class="document biblical-citation">
                    <h3>{text(biblical_citation)}</h3>
                </article>
            },
        )
    }

    fn biblical_reading(&self, reading: &BiblicalReading) -> (Option<Vec<Node<Msg>>>, Node<Msg>) {
        let intro = if let Some(intro) = &reading.intro {
            let doc = Document::from(intro.clone());
            DocumentView::from(doc).view()
        } else {
            text("")
        };

        let header = node! {
            <h3 class="citation">{text(&reading.citation)}</h3>
        };

        let main = node! {
            <main class="biblical-reading">
                {intro}
                {for (verse, verse_text) in &reading.text {
                    node! {
                        <span class="verse">
                            <sup class="verse-number">{text(verse.verse)}</sup>
                            {text(verse_text)}
                        </span>
                    }
                }}
            </main>
        };

        (Some(vec![header]), main)
    }

    fn canticle(&self, canticle: &Canticle) -> (Option<Vec<Node<Msg>>>, Node<Msg>) {
        let citation = if let Some(citation) = &canticle.citation {
            node! {
                <h3 class="citation">{text(citation)}</h3>
            }
        } else {
            text("")
        };

        let header = vec![
            node! { <h3 class="local-name">{text(format!("{}. {}", canticle.number, canticle.local_name))}</h3> },
            node! { <em class="latin-name">{text(&canticle.latin_name)}</em> },
            self.reference(&canticle.reference),
            citation,
        ];

        let main = node! {
            <main class="canticle">
            {for section in &canticle.sections {
                let header = if let Some(title) = &section.title {
                    node! {
                        <header>
                            <h4 class="canticle-section-title">{text(title)}</h4>
                        </header>
                    }
                } else {
                    text("")
                };

                node! {
                    <section>
                        {header}
                        <main>
                        {for verse in &section.verses {
                            node! {
                                <p class="verse">
                                    <span class="a">{text(&verse.a)}</span>
                                    <span class="b">{text(&verse.b)}</span>
                                </p>
                            }
                        }}
                        </main>
                    </section>
                }
            }}
            </main>
        };

        (Some(header), main)
    }

    fn choice(&self, choice: &Choice) -> (Option<Vec<Node<Msg>>>, Node<Msg>) {
        let header = node! {
            <nav class="choice-nav">
                <select class="choice-menu">
                    {for (ii, doc) in choice.options.iter().enumerate() {
                        node! {
                            <option value={ii}>{text(choice.option_label(doc, ii))}</option>
                        }
                    }}
                </select>
            </nav>
        };

        let main = node! {
            <section class="choice">
            {for doc in choice.options.iter() {
                {DocumentView::from(doc.clone()).view()}
            }}
            </section>
        };

        (Some(vec![header]), main)
    }

    fn collect_of_the_day(&self) -> (Option<Vec<Node<Msg>>>, Node<Msg>) {
        let main = node! {
            <article class="document empty">{text("TODO")}</article>
        };

        (None, main)
    }

    fn empty(&self) -> (Option<Vec<Node<Msg>>>, Node<Msg>) {
        let main = node! {
            <main class="empty"></main>
        };

        (None, main)
    }

    fn gloria_patri(&self, content: &GloriaPatri) -> (Option<Vec<Node<Msg>>>, Node<Msg>) {
        let display_format = self.display_format_as_class(content.display_format);
        let main = node! {
            <main class={format!("gloria-patri {}", display_format)}>
                <p>
                    <span class="a">{text(&content.text.0)}</span>
                    <span class="b">{text(&content.text.1)}</span>
                    <br/>
                    <span class="c">{text(&content.text.2)}</span>
                    <span class="d">{text(&content.text.3)}</span>
                </p>
                /* <p></p>
                <p>{text(format!("{}<wb>{}", content.text.0, content.text.1))}</p>
                <p>{text(format!("{}<wb>{}", content.text.2, content.text.3))}</p> */
            </main>
        };

        (None, main)
    }

    fn heading(&self, heading: &Heading) -> (Option<Vec<Node<Msg>>>, Node<Msg>) {
        let main = node! {
            <main class="heading">
                {match heading {
                    Heading::Date(date) => self.heading_date(date),
                    Heading::Day(day) => self.heading_day(day),
                    Heading::Text(level, content) => match level {
                        HeadingLevel::Heading1 => node! { <h1>{text(content)}</h1> },
                        HeadingLevel::Heading2 => node! { <h2>{text(content)}</h2> },
                        HeadingLevel::Heading3 => node! { <h3>{text(content)}</h3> },
                        HeadingLevel::Heading4 => node! { <h4>{text(content)}</h4> },
                        HeadingLevel::Heading5 => node! { <h5>{text(content)}</h5> },
                    },
                }}
            </main>
        };

        (None, main)
    }

    fn heading_date(&self, date: &Option<Date>) -> Node<Msg> {
        match date {
            Some(date) => node! {
                <h2 class="date">{text(date.to_localized_name(self.document.language))}</h2>
            },
            None => text(""),
        }
    }

    fn heading_day(&self, day: &Option<LiturgicalDay>) -> Node<Msg> {
        let observed = day.as_ref().map(|day| day.observed);
        let base_name = match observed {
            Some(LiturgicalDayId::Feast(feast)) => self
                .calendar
                .feast_name(feast, self.document.language)
                .map(text),
            Some(LiturgicalDayId::TransferredFeast(feast)) => self
                .calendar
                .feast_name(feast, self.document.language)
                .map(|name| {
                    node! {
                        <p>
                            {text(name)}
                            <br/>
                            // TODO i18n
                            {text(self.i18n("(Transferred)"))}
                        </p>
                    }
                }),
            _ => {
                if let Some(day) = day {
                    self.calendar
                        .week_name(day.week, self.document.language)
                        .map(|name| {
                            if day.weekday == Weekday::Sun {
                                node! {
                                    <p>
                                        {text(name)}
                                    </p>
                                }
                            } else {
                                node! {
                                    <p>
                                        {text(self.i18n(&day.weekday.to_string()))}
                                        {text(self.i18n("after"))}
                                        {text(name.replace("The", "the"))}
                                    </p>
                                }
                            }
                        })
                } else {
                    None
                }
            }
        };

        let proper = observed
            .and_then(|id| {
                if let LiturgicalDayId::ProperAndDay(proper, _) = id {
                    Some(proper)
                } else {
                    None
                }
            })
            .and_then(|proper| self.calendar.proper_name(proper, self.document.language))
            .map(|name| format!("({})", name));

        node! {
            <h2 class="day">
                {base_name.unwrap_or_else(|| text(""))}
                {text(proper.unwrap_or_default())}
            </h2>
        }
    }

    fn litany(&self, litany: &Litany) -> (Option<Vec<Node<Msg>>>, Node<Msg>) {
        let main = node! {
            <main class="litany">
                {for line in litany.iter() {
                    node! {
                        <p>
                            <span>{text(line)}</span>
                            <strong class="response">{text(&litany.response)}</strong>
                        </p>
                    }
                }}
            </main>
        };

        (None, main)
    }

    fn preces(&self, preces: &Preces) -> (Option<Vec<Node<Msg>>>, Node<Msg>) {
        let main = node! {
            <main class="preces">{
                for (label, prayer) in preces.iter() {
                    node! {
                        <p class="line">
                            <em class="label">{text(label)}</em>
                            <span class="text">{text(prayer)}</span>
                        </p>
                    }
                }
            }
            </main>
        };

        (None, main)
    }

    fn psalm(&self, psalm: &Psalm) -> (Option<Vec<Node<Msg>>>, Node<Msg>) {
        let main = node! {
            <main class="psalm">
            {for section in psalm.filtered_sections() {
                node! {
                    <section>
                        <header>
                            <h3 class="local-name">{text(section.local_name)}</h3>
                            <em class="latin-name">{text(section.latin_name)}</em>
                            {self.reference(&section.reference)}
                        </header>
                        <main>
                        {for verse in section.verses {
                            node! {
                                <p class="verse">
                                    <sup class="number">{text(verse.number)}</sup>
                                    <span class="a">{text(verse.a)}</span>
                                    <span class="b">{text(verse.b)}</span>
                                </p>
                            }
                        }}
                        </main>
                    </section>
                }
            }}
            </main>
        };

        (None, main)
    }

    fn psalm_citation(
        &self,
        psalm_citation: &PsalmCitation,
    ) -> (Option<Vec<Node<Msg>>>, Node<Msg>) {
        let main = node! {
            <main class="psalm-citation"><h3>{text(psalm_citation)}</h3></main>
        };

        (None, main)
    }

    fn responsive_prayer(
        &self,
        responsive_prayer: &ResponsivePrayer,
    ) -> (Option<Vec<Node<Msg>>>, Node<Msg>) {
        let main = node! {
            <main class="responsive-prayer">
            {for line in responsive_prayer.iter() {
                node! {
                    <p>{text(line)}</p>
                }
            }}
            </main>
        };

        (None, main)
    }

    fn rubric(&self, rubric: &Rubric) -> (Option<Vec<Node<Msg>>>, Node<Msg>) {
        let main = node! {
            <main class="rubric">{text(rubric)}</main>
        };

        (None, main)
    }

    fn sentence(&self, sentence: &Sentence) -> (Option<Vec<Node<Msg>>>, Node<Msg>) {
        let short_text_response = sentence
            .response
            .as_ref()
            .and_then(|doc| match &doc.content {
                Content::Text(text) => {
                    if text.text.len() <= 5 && text.response.is_none() {
                        Some(text)
                    } else {
                        None
                    }
                }
                _ => None,
            });

        let citation: Node<Msg> = match &sentence.citation {
            Some(citation) => node! { <span class="citation">{text(citation)}</span> },
            None => text(""),
        };

        let main = node! {
            <main class="sentence">
            {match (&sentence.response, short_text_response) {
                // No response
                (None, _) => node! {
                    <p>
                        {text(&sentence.text)}
                        {citation}
                    </p>
                },
                // With a short enough response to be shown inline
                (_, Some(response)) => node! {
                    <p>
                        {text(&sentence.text)}
                        <strong class="response">{text(response)}</strong>
                        {citation}
                    </p>
                },
                // With a longer response, which should be shown on its own level
                (Some(response), None) => node! {
                    <div>
                        <p>
                            {text(&sentence.text)}
                            {citation}
                        </p>
                        {DocumentView::from((*response.clone(), self.calendar)).view()}
                    </div>
                },
            }}
            </main>
        };

        (None, main)
    }

    fn series(&self, series: &Series) -> (Option<Vec<Node<Msg>>>, Node<Msg>) {
        let main = node! {
            <section class="series">{
                for doc in series.iter() {
                    DocumentView::from((doc.clone(), self.calendar)).view()
                }
            }</section>
        };
        (None, main)
    }

    fn sub_liturgy(&self, sub_liturgy: &SubLiturgy) -> (Option<Vec<Node<Msg>>>, Node<Msg>) {
        let main = node! {
            <main class="sub_liturgy">{text("TODO")}</main>
        };

        (None, main)
    }

    fn text(&self, content: &liturgy::Text) -> (Option<Vec<Node<Msg>>>, Node<Msg>) {
        let paragraphs = content.text.split("\n\n");
        let display_format = self.display_format_as_class(content.display_format);
        let main = node! {
            <main class={format!("text {}", display_format)}>
                {for paragraph in paragraphs {
                    node! {
                        <p>{text(paragraph)}</p>
                    }
                }}
            </main>
        };

        (None, main)
    }

    fn antiphon(&self, antiphon: &Antiphon) -> (Option<Vec<Node<Msg>>>, Node<Msg>) {
        let main = node! {
            <main class="antiphon">{text(antiphon)}</main>
        };

        (None, main)
    }
}
