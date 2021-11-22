use liturgy::*;
use sauron::html::text;
use sauron::prelude::*;
use sauron::{node, Application, Cmd, Node, Program};

pub enum Msg {}
pub struct DocumentView {
    document: Document,
}

impl DocumentView {
    pub fn new() -> Self {
        Self {
            document: Document::new(),
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
        Self { document }
    }
}

impl Application<Msg> for DocumentView {
    fn update(&mut self, _msg: Msg) -> Cmd<Self, Msg>
    where
        Self: Sized + 'static,
    {
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
            <article class="document">
                {label.unwrap_or_else(|| text(""))}
                {source.unwrap_or_else(|| text(""))}
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

    // Content Types

    fn choice(&self, choice: &Choice) -> Node<Msg> {
        node! {
            <article class="document empty"></article>
        }
    }

    fn collect_of_the_day(&self) -> Node<Msg> {
        node! {
            <article class="document empty">{text("TODO")}</article>
        }
    }

    fn date(&self) -> Node<Msg> {
        node! {
            <article class="document empty">{text("TODO")}</article>
        }
    }

    fn day(&self) -> Node<Msg> {
        node! {
            <article class="document empty">{text("TODO")}</article>
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
            Heading::Date => self.date(),
            Heading::Day => self.day(),
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
                    <header>
                        <h3 class="local-name">{text(section.local_name)}</h3>
                        <em class="latin-name">{text(section.latin_name)}</em>
                        {self.reference(&section.reference)}
                    </header>
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
            <article class="document empty">{text("TODO")}</article>
        }
    }

    fn rubric(&self, rubric: &Rubric) -> Node<Msg> {
        node! {
            <article class="document empty">{text("TODO")}</article>
        }
    }

    fn sentence(&self, sentence: &Sentence) -> Node<Msg> {
        node! {
            <article class="document empty">{text("TODO")}</article>
        }
    }

    fn series(&self, series: &Series) -> Node<Msg> {
        node! {
            <section class="series">{
                for doc in series.iter() {
                    DocumentView::from(doc.clone()).view()
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
