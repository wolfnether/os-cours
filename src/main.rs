use yew::prelude::*;

use crate::question::get_question;

mod question;

#[function_component]
fn App() -> Html {
    let responses = use_state(|| question::ResponsesEnum::Tf(None));
    let questions = use_state(get_question);
    let index = use_state(|| 0);

    let state = use_state(|| false);

    let score = use_state(|| 0);

    let setup = use_state(|| false);

    if *index >= questions.len() {
        html!(<div class="content">{"Vous avez répondu a toutes les questions."}<br/>{"Votre score est de "} {*score} {"/"} {questions.len()}</div>)
    } else if *state {
        let question = dyn_clone::clone_box(&*questions[*index]);
        let success = question.success(&responses);
        html!(
            <div class="content">
                <h3>{question.title()}</h3>

                if success{
                    { "Tu a bien répondu. Bravo !" }
                } else {
                    { "Ta réponse est incorrecte ou incomplète." } <br/><br/> {"La réponse attendue :"} <br/>
                    {question.responses()}
                    <br/>
                    {"Nous reviendrons plus tard sur cette question."}
                }
                <br/><br/>
                <button class="pure-button" onclick={
                    move|_| {
                        state.set(false);
                        setup.set(false);
                        index.set(*index+1);
                        if success {
                            score.set(*score+1);
                        } else {
                            let mut _questions = questions.iter().map(|q| dyn_clone::clone_box(&(**q))).collect::<Vec<_>>();
                            _questions.push(dyn_clone::clone_box(&*question));
                            questions.set(_questions);
                        }
                    }
                }>{"Passons a la question suivante"}</button>
            </div>
        )
    } else {
        let question = dyn_clone::clone_box(&*questions[*index]);
        if !*setup {
            question.setup(&responses);
            setup.set(true)
        }
        html!(
            <div class="content">
                <h3>{*index + 1} {"/"} {questions.len()} {" - "} {question.title()}</h3>
                {question.construct(&responses)}

                <br/>

                <button class="pure-button" onclick={
                    move|_|{
                        state.set(true);
                        if question.success(&responses) {
                            score.set(*score+1)
                        }
                    }}>{"Verifions ta réponse"}</button>
            </div>
        )
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
