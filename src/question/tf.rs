use gloo_console::log;
use yew::{html, html_nested};

use super::Question;
use super::Responses;

#[derive(Debug, Clone)]
pub struct Tf<'a> {
    title: &'a str,
    response: bool,
}

impl Question for Tf<'_> {
    fn title(&self) -> &str {
        self.title
    }

    fn construct(&self, responses_state: &Responses) -> yew::prelude::Html {
        let response = responses_state.tf();
        let responses_state = responses_state.clone();
        html_nested!(
            <>
                <input class="pure-radio" type="radio" name="response" checked={response == Some(true)} onclick={let responses_state = responses_state.clone();move |_| responses_state.set(crate::question::ResponsesEnum::Tf(Some(true))) }/> {" Vrai"}
                <br/>
                <input class="pure-radio" type="radio" name="response" checked={response == Some(false)} onclick={move |_| responses_state.set(crate::question::ResponsesEnum::Tf(Some(false))) }/> {" Faux"}
                <br/>
            </>
        )
    }

    fn success(&self, responses_state: &Responses) -> bool {
        responses_state
            .tf()
            .map(|response| response == self.response)
            .unwrap_or(false)
    }

    fn responses(&self) -> yew::prelude::Html {
        html!({
            if self.response {
                {
                    "Vrai"
                }
            } else {
                {
                    "Faux"
                }
            }
        })
    }

    fn setup(&self, responses_state: &Responses) {
        log!("tf setup called");
        responses_state.set(super::ResponsesEnum::Tf(None))
    }
}

impl<'a> Tf<'a> {
    pub fn new(title: &'a str, response: bool) -> Self {
        Self { title, response }
    }

    pub fn new_boxed(title: &'a str, response: bool) -> Box<Self> {
        Box::new(Self::new(title, response))
    }
}
