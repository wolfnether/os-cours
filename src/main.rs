use std::collections::BTreeMap;

use yew::prelude::*;

use crate::question::get_question;

mod question;

pub type Responses = UseStateHandle<BTreeMap<usize, bool>>;

#[function_component]
fn App() -> Html {
    let responses = use_state(BTreeMap::<usize, bool>::new);
    let questions = use_state(get_question);
    let index = use_state(|| 0);

    let state = use_state(|| false);

    let score = use_state(|| 0);

    if *index >= questions.len() {
        html!(<div>{"Vous avez repondu a toutes les questions."}<br/>{"Votre score est de "} {*score} {"/"} {questions.len()}</div>)
    } else if *state {
        let question = &questions[*index];
        html!(
            <div>
                {question.title()} <br/>

                if question.success(responses) {
                    { "Vous avez bien repondu." }
                } else {
                    { "Vous avez mal repondu." } <br/> {"Les bonnes reponses reponses etait :"} <br/>
                    {question.responses()}
                }
                <button onclick={
                    let state = state.clone();
                    let index = index.clone();
                    move|_| {state.set(false); index.set(*index+1)}}
                    >{"suivante"}</button>
            </div>
        )
    } else {
        let question = dyn_clone::clone_box(&*questions[*index]);
        html!(
            <div>
                {question.title()} <br/>
                {question.construct(responses.clone())} <br/>

                <button onclick={
                    let state = state.clone();
                    let score = score.clone();
                    move|_|{
                        state.set(true);
                        if question.success(responses.clone()) {
                            score.set(*score+1)
                        }
                    }}>{"repondre"}</button>
            </div>
        )
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
