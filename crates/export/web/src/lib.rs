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
}

impl DocumentView {
    pub fn new() -> Self {
        Self {
            document: Document::new(),
            calendar: &BCP1979_CALENDAR,
        }
    }

    pub fn to_html(&self) -> String {
        let view = self.view();
        let mut buffer = String::new();
        view.render(&mut buffer).expect("failed to render document");
        buffer
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
        }
    }
}

impl From<(Document, &'static Calendar)> for DocumentView {
    fn from((document, calendar): (Document, &'static Calendar)) -> Self {
        Self { document, calendar }
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
        let label: Option<Node<Msg>> = self.document.label.as_ref().map(|label| {
            node! {
                <h3>{text(label)}</h3>
            }
        });

        let source: Option<Node<Msg>> = self
            .document
            .source
            .map(|reference| self.reference(&reference));

        let content: Node<Msg> = match &self.document.content {
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
        };

        node! {
            <div class="document">
                {label.unwrap_or_else(|| text(""))}
                {source.unwrap_or_else(|| text(""))}
                {content}
            </div>
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

    fn choice(&self, choice: &Choice) -> Node<Msg> {
        node! {
            <section class="document choice">
                <nav>
                    <ul>
                        {for (doc, ii) in choice.options.iter().enumerate() {
                            node! {
                                <li>{text(choice.option_label(&doc, ii))}</li>
                            }
                        }}
                    </ul>
                </nav>
            {for doc in choice.options.iter() {
                {DocumentView::from(doc.clone()).view()}
            }}
            </section>
        }
    }

    fn collect_of_the_day(&self) -> Node<Msg> {
        node! {
            <article class="document empty">{text("TODO")}</article>
        }
    }

    fn date(&self, date: &Option<Date>) -> Node<Msg> {
        match date {
            Some(date) => node! {
                <h2 class="date">{text(date.to_localized_name(self.document.language))}</h2>
            },
            None => text(""),
        }
    }

    fn day(&self, day: &Option<LiturgicalDay>) -> Node<Msg> {
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
            <h2>
                {base_name.unwrap_or_else(|| text(""))}
                {text(proper.unwrap_or_default())}
            </h2>
        }
    }

    fn empty(&self) -> Node<Msg> {
        node! {
            <article class="document empty"></article>
        }
    }

    fn gloria_patri(&self, content: &GloriaPatri) -> Node<Msg> {
        let display_format = self.display_format_as_class(content.display_format);
        node! {
            <article class={format!("document gloria-patri {}", display_format)}>
                <p>{text(format!("{} {}", content.text.0, content.text.1))}</p>
                <p>{text(format!("{} {}", content.text.2, content.text.3))}</p>
            </article>
        }
    }

    fn heading(&self, heading: &Heading) -> Node<Msg> {
        match heading {
            Heading::Date(date) => self.date(date),
            Heading::Day(day) => self.day(day),
            Heading::Text(level, content) => match level {
                HeadingLevel::Heading1 => node! { <h1>{text(content)}</h1> },
                HeadingLevel::Heading2 => node! { <h2>{text(content)}</h2> },
                HeadingLevel::Heading3 => node! { <h3>{text(content)}</h3> },
                HeadingLevel::Heading4 => node! { <h4>{text(content)}</h4> },
                HeadingLevel::Heading5 => node! { <h5>{text(content)}</h5> },
            },
        }
    }

    fn preces(&self, preces: &Preces) -> Node<Msg> {
        node! {
            <article class="document preces">{
                for (label, prayer) in preces.iter() {
                    node! {
                        <p class="line">
                            <em class="label">{text(label)}</em>
                            <span class="text">{text(prayer)}</span>
                        </p>
                    }
                }
            }</article>
        }
    }

    fn psalm(&self, psalm: &Psalm) -> Node<Msg> {
        node! {
            <article class="document psalm">
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
            </article>
        }
    }

    fn psalm_citation(&self, psalm_citation: &PsalmCitation) -> Node<Msg> {
        node! {
            <article class="document psalm-citation"><h3>{text(psalm_citation)}</h3></article>
        }
    }

    fn responsive_prayer(&self, responsive_prayer: &ResponsivePrayer) -> Node<Msg> {
        node! {
            <article class="document responsive-prayer">
            {for line in responsive_prayer.iter() {
                node! {
                    <p>{text(line)}</p>
                }
            }}
            </article>
        }
    }

    fn rubric(&self, rubric: &Rubric) -> Node<Msg> {
        node! {
            <article class="document rubric">{text(rubric)}</article>
        }
    }

    fn sentence(&self, sentence: &Sentence) -> Node<Msg> {
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

        node! {
            <article class="document sentence">
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
            </article>
        }
    }

    fn series(&self, series: &Series) -> Node<Msg> {
        node! {
            <section class="series">{
                for doc in series.iter() {
                    DocumentView::from((doc.clone(), self.calendar)).view()
                }
            }</section>
        }
    }

    fn sub_liturgy(&self, sub_liturgy: &SubLiturgy) -> Node<Msg> {
        node! {
            <article class="document empty">{text("TODO")}</article>
        }
    }

    fn text(&self, content: &liturgy::Text) -> Node<Msg> {
        let paragraphs = content.text.split("\n\n");
        let display_format = self.display_format_as_class(content.display_format);
        node! {
            <article class={format!("document text {}", display_format)}>
                {for paragraph in paragraphs {
                    node! {
                        <p>{text(paragraph)}</p>
                    }
                }}
            </article>
        }
    }
}
