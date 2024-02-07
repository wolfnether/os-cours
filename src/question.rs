use dyn_clone::DynClone;
use rand::seq::SliceRandom;
use yew::{html, Html};

use crate::Responses;

pub fn get_question() -> Vec<Box<dyn Question>> {
    vec![
        Qcm::new_qcm(
            "Parmi les propositions suivantes, lesquelles sont exactes ?",
            vec![
                Candidate::new(
                    "L’epiphyse supérieure du fémur contient quatre éléments osseux : une tête, un col et deux tubérosités.",
                    true,
                ),
                Candidate::new(
                    "L’epiphyse supérieure du fémur contient deux trochanters : le grand trochanter et le petit trochanter.",
                    true,
                ),
                Candidate::new("L’epiphyse supérieure du fémur est en forme cubiqu", false),
            ],
        ),
        Qcm::new_qcm(
            "Où s’insère le ligament de la tête fémoral ?",
            vec![
                Candidate::new("Sur le grand trochanter.", false),
                Candidate::new("Sur la fovéa.", true),
                Candidate::new("Sur la partie supérieure de la tête fémoral.", false),
            ],
        ),
        Qcm::new_qcm(
            "Choisissez les bonnes réponses. La tête fémorale regarde en :",
            vec![
                Candidate::new("Haut", true),
                Candidate::new("Dehors", false),
                Candidate::new("Dedans", true),
                Candidate::new("Avant", true),
                Candidate::new("Arrière", false),
                Candidate::new("Bas", false),
            ],
        ),
        Qcm::new_qcm(
            "Quel muscle s’insère sur la face supérieure du grand trochanter ?",
            vec![
                Candidate::new("Le grand fessier", false),
                Candidate::new("Le moyen fessier", false),
                Candidate::new("Le petit fessier", false),
                Candidate::new("Le piriforme", true),
            ],
        ),
        Qcm::new_tf("La tête fémorale s’articule avec l’os coxal.", true),
        Qcm::new_tf("L’articulation coxo-fémorale est de type bicondylaire.", false),
        Qcm::new_tf(
            "Le muscles moyen fessier s’insère sur la face supérieure du grand trochanter.",
            false,
        ),
        Qcm::new_tf(
            "Sur le petit trochanter, nous retrouvons trois insertions : le psoas, l’iliaque et le ligament pubo-fémoral.",
            true,
        ),
        Qcm::new_tf(
            "La face antérieure du col fémoral est intra capsulaire et est dépourvu d’insertion.",
            true,
        ),
        Qcm::new_qcm(
            "Quels sont les muscles qui s’insèrent sur la ligne âpre ?",
            vec![
                Candidate::new("Biceps brachial", false),
                Candidate::new("Vaste médial", true),
                Candidate::new("Grand adducteur", true),
                Candidate::new("Long adducteur", true),
                Candidate::new("Vaste intermédiaire", false),
            ],
        ),
        Qcm::new_qcm(
            "Quels sont les muscles adducteurs qui s’insèrent sur la ligne âpre ?",
            vec![
                Candidate::new("Gracile", false),
                Candidate::new("Grand adducteur", true),
                Candidate::new("Pectiné", false),
                Candidate::new("Long adducteur", true),
                Candidate::new("Court adducteur", true),
            ],
        ),
        Qcm::new_qcm(
            "Lequel des chefs du quadriceps est bi-articulaire ?",
            vec![
                Candidate::new("Vaste latéra", false),
                Candidate::new("Droit fémoral", true),
                Candidate::new("Vaste intermédiaire", false),
                Candidate::new("Vaste médial", false),
            ],
        ),
        Qcm::new_tf("Le fémur est un os long et symétrique.", false),
        Qcm::new_tf("La trifurcation se situe au niveau inférieur de la ligne âpre.", false),
        Qcm::new_tf("Le fémur est l’os le plus long du corps humain.", true),
        Qcm::new_tf("La diaphyse fémorale est triangulaire à la coupe.", true),
        Qcm::new_tf("Le muscle pectiné s’insère sur la trifurcation.", true),
        Qcm::new_qcm(
            "Quelles parties distingue-t-on sur l’extrémité inférieure du fémur :",
            vec![
                Candidate::new("La patella", false),
                Candidate::new("Les condyles fémoraux", true),
                Candidate::new("La surface patellaire", true),
                Candidate::new("La zone de transition", true),
            ],
        ),
        Qcm::new_qcm(
            "Où se situe la zone de transition ?",
            vec![
                Candidate::new("Entre les deux condyles", false),
                Candidate::new("Entre la zone de bifurcation de la ligne âpre et les condyles", true),
                Candidate::new("Dans la fosse poplitée", false),
                Candidate::new("Au-dessus de la surface patellaire", false),
            ],
        ),
        Qcm::new_qcm(
            "Comment est définie la surface patellaire ou trochlée ?",
            vec![
                Candidate::new("Convexe avec une seule joue", false),
                Candidate::new("De type trochoïde avec une gorge horizontale", false),
                Candidate::new("De type ginglyme avec une gorge verticale", true),
                Candidate::new("Plate sans caractéristique particulière", false),
            ],
        ),
        Qcm::new_qcm(
            "Qu’est-ce qui s’insère sur la partie postérieure de l’épicondyle du condyle fémoral latéral :",
            vec![
                Candidate::new("Le chef latéral du gastrocnémien", true),
                Candidate::new("Le tendon du muscle poplité", false),
                Candidate::new("Le ligament collatéral fibulaire", true),
                Candidate::new("Le rétinaculum patellaire", true),
            ],
        ),
        Qcm::new_qcm(
            "Comment est définie la face supérieure du condyle fémoral latérale ?",
            vec![
                Candidate::new("Convexe transversalement avec un tubercule médian", false),
                Candidate::new(
                    "Concave d’avant en arrière et convexe transversalement avec un tubercule supra condylaire",
                    true,
                ),
                Candidate::new("Convexe d’avant en arrière avec un sillon condylaire", false),
                Candidate::new("Plate avec une lèvre inférieure", false),
            ],
        ),
        Qcm::new_qcm(
            "Quelle est la forme générale des surfaces articulaires des condyles fémoraux ?",
            vec![
                Candidate::new("Sphérique", false),
                Candidate::new("Plan convexe", false),
                Candidate::new(
                    "Segment de tore convexe d’avant en arrière et légèrement transversalement",
                    true,
                ),
                Candidate::new("Cylindrique", false),
            ],
        ),
        Qcm::new_qcm(
            "Quel ligament s’insère sur la face latérale du condyle fémoral médial ?",
            vec![
                Candidate::new("Ligament croisé antérieur", false),
                Candidate::new("Ligament croisé postérieur", true),
                Candidate::new("Ligament collatéral fibulaire", false),
                Candidate::new("Rétinaculum patellaire", false),
            ],
        ),
        Qcm::new_qcm(
            "Quelle structure limite la fosse intercondylaire en avant ?",
            vec![
                Candidate::new("La surface poplitée", false),
                Candidate::new("Le bord inférieur de la trochlée", true),
                Candidate::new("Le ligament croisé antérieur", false),
                Candidate::new("La face axiale des condyles", false),
            ],
        ),
        Qcm::new_qcm(
            "Qu’est-ce qui caractérise le condyle médial ?",
            vec![
                Candidate::new("Il est plus large et plus long que le condyle latéral", false),
                Candidate::new("Il est plus large et plus court que le condyle latéral", false),
                Candidate::new("Il est plus étroit et plus long que le condyle latéral", true),
                Candidate::new("Il ne donne pas insertion au muscle plantaire", true),
            ],
        ),
        Qcm::new_qcm(
            "Où se situe le tubercule de l’adducteur ?",
            vec![
                Candidate::new("Au-dessus de la face supérieure du condyle médial", true),
                Candidate::new(
                    "Dans la continuité de la branche médiale de la branche médiale de bifurcation de la ligne âpre",
                    true,
                ),
                Candidate::new("Sur la face latérale du condyle médial", false),
                Candidate::new("Dans la fosse poplitée", false),
            ],
        ),
    ]
}

pub trait Question: DynClone {
    fn title(&self) -> &str;
    fn construct(&self, responses_state: Responses) -> Html;
    fn success(&self, responses_state: Responses) -> bool;
    fn responses(&self) -> Html;
}

#[derive(Debug, Clone)]
pub struct Candidate<'a> {
    title: &'a str,
    value: bool,
}

impl<'a> Candidate<'a> {
    fn new(title: &'a str, value: bool) -> Self {
        Self { title, value }
    }
}

#[derive(Debug, Clone)]
pub struct Qcm<'a> {
    title: &'a str,
    candidate: Vec<Candidate<'a>>,
}

impl<'a> Qcm<'a> {
    fn new_qcm(title: &'a str, mut candidate: Vec<Candidate<'a>>) -> Box<Self> {
        candidate.shuffle(&mut rand::thread_rng());
        Box::new(Self { title, candidate })
    }

    fn new_tf(title: &'a str, value: bool) -> Box<Self> {
        Box::new(Self {
            title,
            candidate: vec![Candidate::new("Vrai", value), Candidate::new("Faux", !value)],
        })
    }
}

impl Question for Qcm<'_> {
    fn title(&self) -> &str {
        self.title
    }

    fn construct(&self, responses_state: Responses) -> Html {
        html!(
            <div>
            {self.candidate.iter().enumerate().map(|(i,c)|
                html!(
                    <div>
                    <input type="checkbox" onclick={
                        let responses_state = responses_state.clone();
                        move |_| {
                            let mut responses = (*responses_state).clone();
                            if let Some(v) = responses_state.get(&i) {
                                responses.insert(i, !v);
                            } else {
                                responses.insert(i, true);
                            }
                            responses_state.set(responses)
                        }
                    }/>
                    {" "} {c.title}</div>)).collect::<Html>()
                }
            </div>
        )
    }

    fn success(&self, responses_state: Responses) -> bool {
        for (i, candidate) in self.candidate.iter().enumerate() {
            if let Some(r) = responses_state.get(&i) {
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
}
