use std::{collections::HashMap, hash::Hash, iter::repeat};

use rand::{prelude::SliceRandom, Rng};
use serde::{Deserialize, Serialize};

use crate::model::{Hint, HintId, Player, PlayerId};

use super::model::{BoardState, PlayerKnowledges};

#[derive(Serialize, Deserialize, Clone)]
pub struct InitBoard {
    pub players: Vec<(PlayerId, InitPlayer)>,
    pub hints_num: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct InitPlayer {
    pub password: String,
    pub hints: Vec<String>,
}

pub fn init<R: Rng + Clone>(init: InitBoard, rng: &mut R) -> BoardState {
    let players_num = init.players.len();
    let mut players_id = Vec::with_capacity(players_num);
    let mut players_hints = Vec::with_capacity(players_num);
    let mut players_base = Vec::with_capacity(players_num);

    for (id, player) in init.players.into_iter() {
        players_id.push(id.clone());
        players_hints.push((
            id.clone(),
            player.hints.into_iter().map(|text| Hint { text }).collect(),
        ));
        players_base.push((id, player.password));
    }

    let mut player_2_target = shuffle_shift(
        players_base
            .iter()
            .map(|(p, _)| (p.clone(), p.clone()))
            .collect(),
        rng,
    );
    let (hints, mut players_hints) = extract_dictionary(players_hints, HintId);
    let mut knowledges =
        hand_out_hints(players_hints.iter(), init.hints_num, &player_2_target, rng);
    BoardState {
        hints,
        players: players_base
            .into_iter()
            .map(|(id, password)| {
                (
                    id.clone(),
                    Player {
                        password,
                        hints: players_hints.remove(&id).expect("TODO"),
                        target: player_2_target.remove(&id).expect("TODO"),
                        knowledges: knowledges.remove(&id).expect("TODO"),
                    },
                )
            })
            .collect(),
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

fn hand_out_hints<
    'a,
    R: Rng + Clone,
    Iter: Iterator<Item = (&'a PlayerId, &'a Vec<HintId>)> + Clone,
>(
    players: Iter,
    hints_num: usize,
    player_2_target: &PlayerToTarget,
    rng: &mut R,
) -> HashMap<PlayerId, PlayerKnowledges> {
    let converted = players
        .clone()
        .map(|(player, hints)| {
            let mut hints: Vec<_> = hints.iter().map(|hint| (player, hint)).collect();
            hints.shuffle(rng);
            hints
        })
        .collect();
    let separeted = cross_2d_vec(&converted, hints_num);
    let (first, others) = separeted.split_first().expect("TODO");
    let others: Vec<_> = others
        .iter()
        .map(|set| shuffle_shift(set.clone(), &mut rng.clone()))
        .collect();
    players
        .into_iter()
        .map(|(player, _)| {
            let target = player_2_target.get(player).expect("TODO");
            let (_, target_hint) = first
                .iter()
                .find(|(player, _)| player == &target)
                .expect("TODO");
            let others = others
                .iter()
                .map(|hints| hints.get(target).expect("TODO").clone().clone())
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

fn shuffle_shift<K: Eq + Hash, V, R: Rng>(mut vec: Vec<(K, V)>, rng: &mut R) -> HashMap<K, V> {
    vec.shuffle(rng);
    let (keys, mut values): (Vec<K>, Vec<V>) = vec.into_iter().unzip();
    let first = values.remove(0);
    values.push(first);
    keys.into_iter().zip(values.into_iter()).collect()
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

    use mytil::{testing::Counter, validate_no_duplicate};
    use rand::{thread_rng, Rng};

    use super::{cross_2d_vec, extract_dictionary, shuffle_shift};

    #[test]
    fn trial_iterator_lazy_evaluation() {
        let cnt = Counter::new();
        let iter = [1].iter().map(|e| {
            cnt.count();
            e
        });
        assert_eq!(cnt, 0);
        iter.count();
        assert_eq!(cnt, 1);
    }

    #[test]
    fn test_cross_2d_vec() {
        assert_eq!(
            cross_2d_vec(&vec![vec![1, 2, 3], vec![4, 5, 6]], 3),
            vec![vec![1, 4], vec![2, 5], vec![3, 6]]
        )
    }

    #[test]
    fn test_shuffle_shift() {
        fn assertion<R: Rng + Clone>(players: Vec<usize>, mut rng: R) -> bool {
            let result = shuffle_shift(players.iter().map(|n| (n, n)).collect(), &mut rng);
            result.iter().all(|(player, target)| {
                player != target && players.contains(player) && players.contains(target)
            }) && result.len() == players.len()
                && validate_no_duplicate(result.values())
        }
        for _ in 0..1000 {
            assert!(assertion(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], thread_rng()));
        }
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

    use std::collections::{HashMap, HashSet};

    use crate::model::{HintId, PlayerId};

    use super::{hand_out_hints, init, InitBoard, InitPlayer};
    use mytil::validate_no_duplicate;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_init() {
        fn assertion<R: Rng + Clone>(rng: &mut R) {
            let state = init(
                InitBoard {
                    players: vec![
                        (
                            PlayerId(0),
                            InitPlayer {
                                password: "123".to_owned(),
                                hints: vec!["A".to_owned(), "B".to_owned(), "C".to_owned()],
                            },
                        ),
                        (
                            PlayerId(1),
                            InitPlayer {
                                password: "456".to_owned(),
                                hints: vec!["D".to_owned(), "E".to_owned(), "F".to_owned()],
                            },
                        ),
                        (
                            PlayerId(2),
                            InitPlayer {
                                password: "789".to_owned(),
                                hints: vec!["G".to_owned(), "H".to_owned(), "I".to_owned()],
                            },
                        ),
                    ],
                    hints_num: 3,
                },
                rng,
            );
            // ヒントはもれなく辞書に格納されているか
            assert_eq!(
                state
                    .hints
                    .values()
                    .map(|e| e.text.as_str())
                    .collect::<HashSet<&str>>(),
                ["A", "B", "C", "D", "E", "F", "G", "H", "I"].into()
            );
            // プレイヤーが自分の指定した名前、合言葉を持っているか
            assert_eq!(
                state
                    .players
                    .iter()
                    .map(|(id, p)| (id.0, p.password.as_str()))
                    .collect::<HashSet<_>>(),
                [(0, "123"), (1, "456"), (2, "789")].into()
            );
            // プレイヤーが自分の指定したヒントを持っているか
            assert_eq!(
                state
                    .players
                    .iter()
                    .map(|(id, p)| (
                        id.0,
                        p.hints
                            .iter()
                            .map(|hint| state.hints.get(hint).unwrap().text.as_str())
                            .collect::<HashSet<_>>()
                    ))
                    .collect::<HashMap<_, _>>(),
                [
                    (0, ["A", "B", "C"].into()),
                    (1, ["D", "E", "F"].into()),
                    (2, ["G", "H", "I"].into())
                ]
                .into()
            );
            // ターゲットに重複がないか
            assert!(validate_no_duplicate(
                state.players.values().map(|p| &p.target)
            ));
            // 配られたヒントに重複がないか
            assert!(validate_no_duplicate(state.players.values().flat_map(
                |p| p.knowledges.others.iter().chain([&p.knowledges.target])
            )));
            for (id, player) in state.players.iter() {
                assert!(state.players.contains_key(&player.target));
                assert_ne!(id, &player.target);
                let target_id = &player.target;
                let target_hint = &player.knowledges.target;
                let target = state.players.get(target_id).unwrap();
                assert!(target.hints.contains(target_hint));
                assert!(!player
                    .knowledges
                    .others
                    .iter()
                    .any(|hint| target.hints.contains(hint)))
            }
        }
        // FIXME: 冪等性が確保できてない
        let mut rng = thread_rng();
        for _ in 0..1000 {
            assertion(&mut rng);
        }
    }

    #[test]
    fn test_handout() {
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let result = hand_out_hints(
                [
                    (&PlayerId(0), &vec![HintId(0), HintId(1), HintId(2)]),
                    (&PlayerId(1), &vec![HintId(3), HintId(4), HintId(5)]),
                    (&PlayerId(2), &vec![HintId(6), HintId(7), HintId(8)]),
                    (&PlayerId(3), &vec![HintId(9), HintId(10), HintId(11)]),
                    (&PlayerId(4), &vec![HintId(12), HintId(13), HintId(14)]),
                ]
                .into_iter(),
                3,
                &[
                    (PlayerId(0), PlayerId(1)),
                    (PlayerId(1), PlayerId(2)),
                    (PlayerId(2), PlayerId(3)),
                    (PlayerId(3), PlayerId(4)),
                    (PlayerId(4), PlayerId(0)),
                ]
                .into(),
                &mut rng,
            );
            let iter = result
                .values()
                .flat_map(|k| k.others.iter().chain([&k.target]));
            assert!(validate_no_duplicate(iter.clone()), "{:?}", result);
        }
    }
}
