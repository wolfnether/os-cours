use std::collections::BTreeMap;

use dyn_clone::DynClone;
use rand::seq::SliceRandom;
use yew::{Html, UseStateHandle};

use self::mcq::{Candidate, Mcq};
use self::tf::Tf;

mod mcq;
mod tf;

type Responses = UseStateHandle<ResponsesEnum>;

#[derive(Debug)]
pub enum ResponsesEnum {
    Mcq(BTreeMap<usize, bool>),
    Tf(Option<bool>),
}

impl Clone for ResponsesEnum {
    fn clone(&self) -> Self {
        match self {
            Self::Mcq(v) => Self::Mcq(v.clone()),
            Self::Tf(v) => Self::Tf(*v),
        }
    }
}

impl ResponsesEnum {
    pub fn mcq(&self) -> BTreeMap<usize, bool> {
        if let Self::Mcq(v) = self {
            v.clone()
        } else {
            panic!()
        }
    }

    pub fn tf(&self) -> Option<bool> {
        if let Self::Tf(v) = self {
            *v
        } else {
            None
        }
    }
}

pub fn get_question() -> Vec<Box<dyn Question>> {
    let mut questions: Vec<Box<dyn Question>> = vec![
        Mcq::new_boxed(
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
                Candidate::new("L’epiphyse supérieure du fémur est en forme cubique.", false),
            ],
        ),
        Mcq::new_boxed(
            "Où s’insère le ligament de la tête fémoral ?",
            vec![
                Candidate::new("Sur le grand trochanter.", false),
                Candidate::new("Sur la fovéa.", true),
                Candidate::new("Sur la partie supérieure de la tête fémoral.", false),
            ],
        ),
        Mcq::new_boxed(
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
        Mcq::new_boxed(
            "Quel muscle s’insère sur la face supérieure du grand trochanter ?",
            vec![
                Candidate::new("Le grand fessier", false),
                Candidate::new("Le moyen fessier", false),
                Candidate::new("Le petit fessier", false),
                Candidate::new("Le piriforme", true),
            ],
        ),
        Tf::new_boxed("La tête fémorale s’articule avec l’os coxal.", true),
        Tf::new_boxed("L’articulation coxo-fémorale est de type bicondylaire.", false),
        Tf::new_boxed(
            "Le muscles moyen fessier s’insère sur la face supérieure du grand trochanter.",
            false,
        ),
        Tf::new_boxed(
            "Sur le petit trochanter, nous retrouvons trois insertions : le psoas, l’iliaque et le ligament pubo-fémoral.",
            true,
        ),
        Tf::new_boxed(
            "La face antérieure du col fémoral est intra capsulaire et est dépourvu d’insertion.",
            true,
        ),
        Mcq::new_boxed(
            "Quels sont les muscles qui s’insèrent sur la ligne âpre ?",
            vec![
                Candidate::new("Biceps brachial", false),
                Candidate::new("Vaste médial", true),
                Candidate::new("Grand adducteur", true),
                Candidate::new("Long adducteur", true),
                Candidate::new("Vaste intermédiaire", false),
            ],
        ),
        Mcq::new_boxed(
            "Quels sont les muscles adducteurs qui s’insèrent sur la ligne âpre ?",
            vec![
                Candidate::new("Gracile", false),
                Candidate::new("Grand adducteur", true),
                Candidate::new("Pectiné", false),
                Candidate::new("Long adducteur", true),
                Candidate::new("Court adducteur", true),
            ],
        ),
        Mcq::new_boxed(
            "Lequel des chefs du quadriceps est bi-articulaire ?",
            vec![
                Candidate::new("Vaste latéra", false),
                Candidate::new("Droit fémoral", true),
                Candidate::new("Vaste intermédiaire", false),
                Candidate::new("Vaste médial", false),
            ],
        ),
        Tf::new_boxed("Le fémur est un os long et symétrique.", false),
        Tf::new_boxed("La trifurcation se situe au niveau inférieur de la ligne âpre.", false),
        Tf::new_boxed("Le fémur est l’os le plus long du corps humain.", true),
        Tf::new_boxed("La diaphyse fémorale est triangulaire à la coupe.", true),
        Tf::new_boxed("Le muscle pectiné s’insère sur la trifurcation.", true),
        Mcq::new_boxed(
            "Quelles parties distingue-t-on sur l’extrémité inférieure du fémur :",
            vec![
                Candidate::new("La patella", false),
                Candidate::new("Les condyles fémoraux", true),
                Candidate::new("La surface patellaire", true),
                Candidate::new("La zone de transition", true),
            ],
        ),
        Mcq::new_boxed(
            "Où se situe la zone de transition ?",
            vec![
                Candidate::new("Entre les deux condyles", false),
                Candidate::new("Entre la zone de bifurcation de la ligne âpre et les condyles", true),
                Candidate::new("Dans la fosse poplitée", false),
                Candidate::new("Au-dessus de la surface patellaire", false),
            ],
        ),
        Mcq::new_boxed(
            "Comment est définie la surface patellaire ou trochlée ?",
            vec![
                Candidate::new("Convexe avec une seule joue", false),
                Candidate::new("De type trochoïde avec une gorge horizontale", false),
                Candidate::new("De type ginglyme avec une gorge verticale", true),
                Candidate::new("Plate sans caractéristique particulière", false),
            ],
        ),
        Mcq::new_boxed(
            "Qu’est-ce qui s’insère sur la partie postérieure de l’épicondyle du condyle fémoral latéral ?",
            vec![
                Candidate::new("Le chef latéral du gastrocnémien", true),
                Candidate::new("Le tendon du muscle poplité", false),
                Candidate::new("Le ligament collatéral fibulaire", true),
                Candidate::new("Le rétinaculum patellaire", true),
            ],
        ),
        Mcq::new_boxed(
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
        Mcq::new_boxed(
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
        Mcq::new_boxed(
            "Quel ligament s’insère sur la face latérale du condyle fémoral médial ?",
            vec![
                Candidate::new("Ligament croisé antérieur", false),
                Candidate::new("Ligament croisé postérieur", true),
                Candidate::new("Ligament collatéral fibulaire", false),
                Candidate::new("Rétinaculum patellaire", false),
            ],
        ),
        Mcq::new_boxed(
            "Quelle structure limite la fosse intercondylaire en avant ?",
            vec![
                Candidate::new("La surface poplitée", false),
                Candidate::new("Le bord inférieur de la trochlée", true),
                Candidate::new("Le ligament croisé antérieur", false),
                Candidate::new("La face axiale des condyles", false),
            ],
        ),
        Mcq::new_boxed(
            "Qu’est-ce qui caractérise le condyle médial ?",
            vec![
                Candidate::new("Il est plus large et plus long que le condyle latéral", false),
                Candidate::new("Il est plus large et plus court que le condyle latéral", false),
                Candidate::new("Il est plus étroit et plus long que le condyle latéral", true),
                Candidate::new("Il ne donne pas insertion au muscle plantaire", true),
            ],
        ),
        Mcq::new_boxed(
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
    ];
    questions.shuffle(&mut rand::thread_rng());
    questions
}

pub trait Question: DynClone {
    fn title(&self) -> &str;
    fn construct(&self, responses_state: &Responses) -> Html;
    fn success(&self, responses_state: &Responses) -> bool;
    fn responses(&self) -> Html;
    fn setup(&self, responses_state: &Responses);
}
