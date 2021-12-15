use std::{collections::HashMap, hash::Hash, iter::repeat};

use rand::{prelude::SliceRandom, Rng};
use serde::{Serialize, Deserialize};

use crate::model::{Hint, HintId, Player, PlayerId};

use super::model::{BoardState, PlayerKnowledges};

#[derive(Serialize,Deserialize,Clone)]
pub struct InitBoard {
    pub players: Vec<InitPlayer>,
    pub hints_num: usize,
}

#[derive(Serialize,Deserialize,Clone)]
pub struct InitPlayer {
    pub id: PlayerId,
    pub password: String,
    pub hints: Vec<String>,
}

pub fn init<R: Rng + Clone>(init: InitBoard, rng: &mut R) -> BoardState {
    let players_num = init.players.len();
    let mut players_id = Vec::with_capacity(players_num);
    let mut players_hints = Vec::with_capacity(players_num);
    let mut players_base = Vec::with_capacity(players_num);

    for player in init.players.into_iter() {
        players_id.push(player.id.clone());
        players_hints.push((
            player.id.clone(),
            player.hints.into_iter().map(|text| Hint { text }).collect(),
        ));
        players_base.push(( player.id, player.password));
    }

    let mut player_2_target = shuffle_shift(&players_id, Clone::clone, rng);
    let (hints, mut players_hints) = extract_dictionary(players_hints, HintId);
    let mut knowledges = hand_out_hints(|| players_hints.iter(), init.hints_num, &player_2_target, rng);
    BoardState {
        hints,
        players: players_base.into_iter().map(|(id,password)| {
            (
                id.clone(),
                Player {
                    password,
                    hints: players_hints.remove(&id).expect("TODO"),
                    target: player_2_target.remove(&id).expect("TODO"),
                    knowledges: knowledges.remove(&id).expect("TODO"),
                },
            )
        }).collect(),
    }
}

fn extract_dictionary<K: Eq + Hash, Item, Id: Clone + Eq + Hash, F: Fn(usize) -> Id>(
    inputs: Vec<(K, Vec<Item>)>,
    create_id: F,
) -> (HashMap<Id, Item>, HashMap<K, Vec<Id>>) {
    let mut dictionary = HashMap::with_capacity(inputs.iter().flat_map(|(_, vec)| vec).count());
    let mut lists = HashMap::with_capacity(inputs.len());
    for (key, items) in inputs.into_iter() {
        let mut ids = Vec::with_capacity(items.len());
        for hint in items.into_iter() {
            let id = create_id(dictionary.len());
            dictionary.insert(id.clone(), hint);
            ids.push(id);
        }
        lists.insert(key, ids);
    }
    (dictionary, lists)
}

fn hand_out_hints<'a,R: Rng + Clone,Iter: Iterator<Item=(&'a PlayerId, &'a Vec<HintId>)>,F: Fn() -> Iter>(
    players: F,
    hints_num: usize,
    player_2_target: &PlayerToTarget,
    rng: &mut R,
) -> HashMap<PlayerId, PlayerKnowledges> {
    let converted = players()
        .map(|(player, hints)| {
            let mut hints: Vec<_> = hints.iter().map(|hint| (player, hint)).collect();
            hints.shuffle(rng);
            hints
        })
        .collect();
    let mut separeted = cross_2d_vec(&converted, hints_num);
    let first = separeted.remove(0);
    let others = separeted
        .into_iter()
        .map(|set| shuffle_shift(&set, |(p, _)| p.clone(), &mut rng.clone()));
    players()
        .map(|(player, _)| {
            let target = player_2_target.get(player).expect("TODO");
            let (_, target_hint) = first
                .iter()
                .find(|(player, _)| player == &target)
                .expect("TODO");
            let others = others
                .clone()
                .map(|hints| hints.get(target).expect("TODO").1.clone())
                .collect();
            (
                player.clone(),
                PlayerKnowledges {
                    target: target_hint.clone().clone(),
                    others,
                },
            )
        })
        .collect()
}

type PlayerToTarget = HashMap<PlayerId, PlayerId>;

fn shuffle_shift<T: Clone, K: Eq + Hash, F: Fn(&T) -> K, R: Rng>(
    vec: &Vec<T>,
    to_key: F,
    rng: &mut R,
) -> HashMap<K, T> {
    let mut cloned = vec.clone();
    cloned.shuffle(rng);
    cloned
        .iter()
        .enumerate()
        .map(|(index, player)| {
            (
                to_key(player),
                (if index == 0 {
                    cloned.last()
                } else {
                    cloned.get(index - 1)
                })
                .expect("Never")
                .clone(),
            )
        })
        .collect()
}

fn cross_2d_vec<T: Clone>(vec: &Vec<Vec<T>>, innner_len: usize) -> Vec<Vec<T>> {
    let init = repeat(Vec::with_capacity(vec.len()))
        .take(innner_len)
        .collect();
    vec.iter().fold(init, |mut acc, cur| {
        for (index, item) in cur.iter().enumerate() {
            acc.get_mut(index)
                .expect("innner_len is less than inner Vec len")
                .push(item.clone());
        }
        acc
    })
}

#[cfg(test)]
mod test_simple {

    use rand::rngs::mock::StepRng;

    use super::{cross_2d_vec, extract_dictionary, shuffle_shift};

    #[test]
    fn test_cross_2d_vec() {
        assert_eq!(
            cross_2d_vec(&vec![vec![1, 2, 3], vec![4, 5, 6]], 3),
            vec![vec![1, 4], vec![2, 5], vec![3, 6]]
        )
    }

    #[test]
    fn test_shuffle_shift() {
        fn assertion(players: Vec<usize>, mut rng: StepRng) -> bool {
            let result = shuffle_shift(&players, Clone::clone, &mut rng);
            result.iter().all(|(player, target)| {
                player != target && players.contains(player) && players.contains(target)
            }) && result.len() == players.len()
        }
        assert!(assertion(vec![0, 1, 2], StepRng::new(0, 1)),);
        assert!(assertion(vec![0, 1, 2, 3, 4], StepRng::new(0, 2)),);
    }

    #[test]
    fn test_extract_dictionary() {
        assert_eq!(
            extract_dictionary(
                vec![("A", vec!["a", "b", "c"]), ("B", vec!["d", "e", "f"])],
                usize::from
            ),
            (
                [(0, "a"), (1, "b"), (2, "c"), (3, "d"), (4, "e"), (5, "f")].into(),
                [("A", vec![0, 1, 2]), ("B", vec![3, 4, 5])].into()
            )
        )
    }
}

#[cfg(test)]
mod test {

    use std::collections::{HashSet, HashMap};

    use crate::model::PlayerId;

    use super::{init, InitBoard, InitPlayer};
    use mytil::validate_no_duplicate;
    use rand::{thread_rng,Rng};

    #[test]
    fn test_init() {
        fn assertion<R: Rng + Clone>(rng: &mut R) {
            let state = init(InitBoard 
                { 
                    players: vec![
                        InitPlayer {
                            id: PlayerId(0),
                            password: "123".to_owned(),
                            hints: vec!["A".to_owned(),"B".to_owned(),"C".to_owned()]
                        },
                        InitPlayer {
                            id: PlayerId(1),
                            password: "456".to_owned(),
                            hints: vec!["D".to_owned(),"E".to_owned(),"F".to_owned()]
                        },
                        InitPlayer {
                            id: PlayerId(2),
                            password: "789".to_owned(),
                            hints: vec!["G".to_owned(),"H".to_owned(),"I".to_owned()]
                        }
                    ], 
                    hints_num: 3 
                },
                rng
            );
            // ヒントはもれなく辞書に格納されているか
            assert_eq!(
                state.hints.values().map(|e| e.text.as_str()).collect::<HashSet<&str>>(),
                ["A","B","C","D","E","F","G","H","I"].into()
            );
            // プレイヤーが自分の指定した名前、合言葉を持っているか
            assert_eq!(
                state.players.iter().map(|(id,p)| (id.0,p.password.as_str())).collect::<HashSet<_>>(),
                [(0,"123"),(1,"456"),(2,"789")].into()
            );
            // プレイヤーが自分の指定したヒントを持っているか
            assert_eq!(
                state.players.iter().map(|(id,p)| (id.0,p.hints.iter().map(|hint| state.hints.get(hint).unwrap().text.as_str()).collect::<HashSet<_>>())).collect::<HashMap<_,_>>(),
                [(0,["A","B","C"].into()),(1,["D","E","F"].into()),(2,["G","H","I"].into())].into()
            );
            // ターゲットに重複がないか
            assert!(validate_no_duplicate(state.players.values().map(|p| &p.target)));
            for (id,player) in state.players.iter() {
                assert!(state.players.contains_key(&player.target));
                assert_ne!(id,&player.target);
                let target_id = &player.target;
                let target_hint = &player.knowledges.target;
                let target = state.players.get(target_id).unwrap();
                assert!(target.hints.contains(target_hint));
                assert!(!player.knowledges.others.iter().any(|hint| target.hints.contains(hint)))
            }
        }
        // FIXME: 冪等性が確保できてない
        let mut rng = thread_rng();
        for _ in 0..1000 {
            assertion(&mut rng);
        }
        
        
    }

    
}
