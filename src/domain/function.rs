use std::{collections::HashMap, hash::Hash, iter::repeat};

use rand::{prelude::SliceRandom, Rng};

use crate::domain::model::{Hint, HintId, Player, PlayerId};

use super::model::{BoardState, PlayerKnowledges};

pub struct InitCommand {
    players: Vec<InitPlayer>,
}

pub struct InitPlayer {
    name: String,
    password: String,
    hints: Vec<String>,
}

pub fn init<R: Rng + Clone>(init: InitCommand,rng: &mut R) -> BoardState {
    let players: HashMap<PlayerId,InitPlayer> = init.players.into_iter().enumerate().map(|(index,player)| (PlayerId(index),player)).collect();
    let ids : Vec<_> = players.into_keys().collect();
    let player_to_target = shuffle_shift(&ids, Clone::clone, rng);
    fn init_player(player: InitPlayer, target: PlayerId, hints: &HashMap<HintId, Hint>) -> Player {
        Player {
            name: player.name,
            target,
            password: player.password,
            knowledges: todo!(),
        }
    }
    todo!()
}

fn hand_out_hints<R: Rng + Clone>(
    players: &Vec<(PlayerId, Vec<HintId>)>,
    hints_num: usize,
    player_2_target: &PlayerToTarget,
    rng: &mut R,
) -> HashMap<PlayerId, PlayerKnowledges> {
    let converted = players
        .iter()
        .map(|(player, hints)| {
            let mut hints: Vec<_> = hints.iter().map(|hint| (player, hint)).collect();
            hints.shuffle(rng);
            hints
        })
        .collect();
    let mut separeted = cross_2d_vec(&converted, hints_num);
    let first = separeted.remove(0);
    let others = separeted.into_iter().map(|set| shuffle_shift(&set,|(p,_)| p.clone(), &mut rng.clone()));
    players
        .iter()
        .map(|(player, _)| {
            let target = player_2_target.get(player).unwrap();
            let (_,target_hint) = first.iter().find(|(player,_)| *player == target).expect("TODO");
            let others = others.clone().map(|hints| hints.get(target).expect("TODO").1.clone()).collect();
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

fn shuffle_shift<T : Clone, K: Eq + Hash,F: Fn(&T) -> K, R: Rng>(vec: &Vec<T>, to_key:F, rng: &mut R) -> HashMap<K, T> {
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
mod test {
    use rand::rngs::mock::StepRng;

    use super::{cross_2d_vec, shuffle_shift};

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
}
