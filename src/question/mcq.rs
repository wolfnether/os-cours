use std::collections::BTreeMap;

use gloo_console::log;
use rand::prelude::SliceRandom;
use yew::{html, html_nested, Html};

use super::{Question, Responses};

#[derive(Debug, Clone)]
pub struct Candidate<'a> {
    title: &'a str,
    value: bool,
}

impl<'a> Candidate<'a> {
    pub fn new(title: &'a str, value: bool) -> Self {
        Self { title, value }
    }
}

#[derive(Debug, Clone)]
pub struct Mcq<'a> {
    title: &'a str,
    candidate: Vec<Candidate<'a>>,
}

impl<'a> Mcq<'a> {
    pub fn new(title: &'a str, mut candidate: Vec<Candidate<'a>>) -> Self {
        candidate.shuffle(&mut rand::thread_rng());
        Self { title, candidate }
    }

    pub fn new_boxed(title: &'a str, candidate: Vec<Candidate<'a>>) -> Box<Self> {
        Box::new(Self::new(title, candidate))
    }
}

impl Question for Mcq<'_> {
    fn title(&self) -> &str {
        self.title
    }

    fn construct(&self, responses_state: &Responses) -> Html {
        self.candidate
            .iter()
            .enumerate()
            .map(|(i, c)| {
                html_nested!(
                    <>
                    <input class="pure-checkbox" type="checkbox" onclick={
                        let responses_state = responses_state.clone();
                        move |_| {
                            let mut responses = responses_state.mcq();
                            if let Some(v) = responses_state.mcq().get(&i) {
                                responses.insert(i, !v);
                            } else {
                                responses.insert(i, true);
                            }
                            responses_state.set(crate::question::ResponsesEnum::Mcq(responses))
                        }
                    }/>
                    {" "} {c.title}<br/></>)
            })
            .collect::<Html>()
    }

    fn success(&self, responses_state: &Responses) -> bool {
        for (i, candidate) in self.candidate.iter().enumerate() {
            if let Some(r) = responses_state.mcq().get(&i) {
                if *r != candidate.value {
                    return false;
                }
            } else if candidate.value {
                return false;
            }
        }
        true
    }

    fn responses(&self) -> Html {
        html!({
            self.candidate
                .iter()
                .filter(|candidate| candidate.value)
                .map(|candidate| html!(<div>{candidate.title}</div>))
                .collect::<Html>()
        })
    }

    fn setup(&self, responses_state: &Responses) {
        log!("mcq setup called");
        responses_state.set(super::ResponsesEnum::Mcq(BTreeMap::new()))
    }
}
