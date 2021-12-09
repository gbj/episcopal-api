use calendar::{Calendar, Date, LiturgicalDay, LiturgicalDayId, Weekday, BCP1979_CALENDAR};
use document::{DocumentComponent, DocumentMsg};
//use fetch_reading::BibleReadingFromAPI;
use liturgy::*;
use log::trace;
use sauron::prelude::*;
use sauron::{node, Application, Cmd, Node};
use serde::{Deserialize, Serialize};

//mod biblical_citation;
mod document;
mod fetch_reading;

#[derive(Debug)]
pub enum Msg {
    SetDocument(Document),
    ChildMsg(DocumentMsg),
    SetContent(Vec<usize>, Content),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Viewer {
    pub document: Document,
    //pub calendar: &'static Calendar,
}

impl Viewer {
    pub fn new() -> Self {
        Self {
            document: Document::new(),
            //calendar: &BCP1979_CALENDAR,
        }
    }

    pub fn to_html(&self) -> String {
        let view = self.view();
        let mut buffer = String::new();
        view.render(&mut buffer).expect("failed to render document");
        buffer
    }
}

impl Default for Viewer {
    fn default() -> Self {
        Self::new()
    }
}
impl From<Document> for Viewer {
    fn from(document: Document) -> Self {
        Self {
            document,
            //calendar: &BCP1979_CALENDAR,
        }
    }
}

impl From<(Document, &'static Calendar)> for Viewer {
    fn from((document, calendar): (Document, &'static Calendar)) -> Self {
        Self {
            document,
            //calendar,
        }
    }
}

impl Application<Msg> for Viewer {
    fn init(&mut self) -> Cmd<Self, Msg> {
        Cmd::none()
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg>
    where
        Self: Sized + 'static,
    {
        let cmd = match msg {
            Msg::ChildMsg(msg) => {
                trace!("message from child component: {:#?}", msg);
                if let DocumentMsg::LoadCitation(path, citation) = msg {
                    let doc = self.document.at_path_mut(path.clone());
                    if let Ok(doc) = doc {
                        doc.content = Content::Text(liturgy::Text::from("..."));
                    }
                    Some(self.fetch_biblical_reading(path, &citation))
                } else {
                    None
                }
            }
            Msg::SetContent(path, content) => {
                if let Ok(doc) = self.document.at_path_mut(path) {
                    doc.content = content;
                };
                None
            }
            Msg::SetDocument(document) => {
                self.document = document;
                None
            }
        };
        cmd.unwrap_or_else(Cmd::none)
    }

    fn view(&self) -> Node<Msg> {
        let component = DocumentComponent {
            document: self.document.clone(),
            top_level: true,
            path: vec![],
        };
        node! { <main>{component.view().map_msg(Msg::ChildMsg)}</main> }
    }
}

/// Creates a `Viewer` from the JSON in `serialized_state` and replaces the DOM Element found in `query_selector`
/// with that `Viewer`.
#[wasm_bindgen]
pub fn initialize_from_json(query_selector: String, serialized_state: String) {
    console_log::init_with_level(log::Level::Trace).unwrap();

    let app = if let Ok(doc) = serde_json::from_str::<Document>(&serialized_state) {
        Viewer::from(doc)
    } else {
        Viewer::default()
    };

    /* If there's a window (i.e., if this is running in the browser)
     * then mount the app by swapping out the <main> tag */
    match web_sys::window() {
        Some(window) => {
            let el = window
                .document()
                .expect("document not found")
                .query_selector(&query_selector)
                .expect("no element found matching the query")
                .expect("third level crash oops");

            Program::replace_mount(app, &el);
        }
        None => {
            Program::mount_to_body(app);
        }
    }
}
