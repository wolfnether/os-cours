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
        let question = dyn_clone::clone_box(&*questions[*index]);
        html!(
            <div>
                {question.title()} <br/>

                if question.success(responses) {
                    { "Vous avez bien répondu. Bravo !" }
                } else {
                    { "Votre réponse est incorrecte ou incomplète." } <br/> {"La réponse attendue :"} <br/>
                    {question.responses()}
                }
                <button onclick={
                    move|_| {
                        state.set(false);
                        index.set(*index+1);
                        let mut _questions = questions.iter().map(|q| dyn_clone::clone_box(&(**q))).collect::<Vec<_>>();
                        _questions.push(dyn_clone::clone_box(&*question));
                        questions.set(_questions);
                        }}
                    >{"Passer a la question suivante"}</button>
            </div>
        )
    } else {
        let question = dyn_clone::clone_box(&*questions[*index]);
        html!(
            <div>
                {question.title()} <br/>
                {question.construct(responses.clone())} <br/>

                <button onclick={
                    move|_|{
                        state.set(true);
                        if question.success(responses.clone()) {
                            score.set(*score+1)
                        }
                    }}>{"Passer a la correction"}</button>
            </div>
        )
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
