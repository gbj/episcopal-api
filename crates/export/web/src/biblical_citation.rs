use liturgy::{BiblicalCitation, BiblicalCitationStatus, BiblicalReading, Document, Version};
use log::error;
use reference_parser::{BibleVerse, BibleVersePart, Book};
use sauron::html::text;
use sauron::prelude::NodeMapMsg;
use sauron::*;
use serde::Deserialize;

use crate::document::{DocumentComponent, DocumentMsg};
use crate::Msg;

pub struct BiblicalCitationComponent(BiblicalCitation);

#[derive(Debug)]
pub enum BiblicalCitationMsg {
    Load,
    Success(BiblicalReading),
    Error,
    DocumentComponentMsg(Box<DocumentMsg>),
}

#[derive(Deserialize)]
pub struct BibleReadingFromAPI {
    pub citation: String,
    pub label: String,
    pub version: Version,
    pub value: Vec<serde_json::value::Value>,
}

impl Component<BiblicalCitationMsg, Msg> for BiblicalCitationComponent {
    fn update(&mut self, msg: BiblicalCitationMsg) -> Effects<BiblicalCitationMsg, Msg> {
        match msg {
            BiblicalCitationMsg::Load => self.0.status = BiblicalCitationStatus::Loading,
            BiblicalCitationMsg::Success(reading) => {
                self.0.status = BiblicalCitationStatus::Success(reading)
            }
            BiblicalCitationMsg::Error => self.0.status = BiblicalCitationStatus::Error,
        }
    }

    fn view(&self) -> Node<BiblicalCitationMsg> {
        let header = node! {
            <h4 class="citation">{text(self.0)}</h4>
        };

        let main = match &self.0.status {
            BiblicalCitationStatus::Empty => node! {
                <article class="document biblical-citation">
                    <p>{text("Empty.")}</p> // i18n
                </article>
            },
            BiblicalCitationStatus::Loading => node! {
                <article class="document biblical-citation">
                    <p>{text("Loading...")}</p> // i18n
                </article>
            },

            BiblicalCitationStatus::Success(reading) => {
                DocumentComponent::from(Document::from(reading.clone()))
                    .view()
                    .map_msg(|msg| BiblicalCitationMsg::DocumentComponentMsg(Box::new(msg)))
            }

            BiblicalCitationStatus::Error(error) => {
                DocumentComponent::from(Document::from(error.clone()))
                    .view()
                    .map_msg(|msg| BiblicalCitationMsg::DocumentComponentMsg(Box::new(msg)))
            }
        };

        node! {
          <article class="document biblical-citation">
            <header>{header}</header>
            <main>{main}</main>
          </article>
        }
    }
}

impl BiblicalCitationComponent {
    pub fn new(document: BiblicalCitation) -> Self {
        Self(document)
    }
}
