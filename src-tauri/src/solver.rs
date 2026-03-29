use rand::seq::SliceRandom;
use rand::rngs::StdRng;
use rand::SeedableRng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

const MAX_NODES: usize = 250_000;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PuzzleState {
    pub vials: Vec<Vec<u8>>,
    pub n_volume: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PuzzleSettings {
    pub n_colors: usize,
    pub n_empty: usize,
    pub n_volume: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Move {
    pub src: usize,
    pub dst: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolverResult {
    pub status: String,
    pub nodes_generated: usize,
    pub optimal_moves: Option<usize>,
    pub moves: Vec<Move>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateCheck {
    pub status: String,
    pub moves_available: usize,
}

fn state_key(vials: &[Vec<u8>]) -> String {
    let mut out = String::new();
    for vial in vials {
        for c in vial {
            out.push_str(&c.to_string());
            out.push('.');
        }
        out.push('|');
    }
    out
}

fn vial_top(vial: &[u8]) -> Option<(u8, usize)> {
    let top_color = *vial.last()?;
    let mut run = 0usize;
    for c in vial.iter().rev() {
        if *c == top_color {
            run += 1;
        } else {
            break;
        }
    }
    Some((top_color, run))
}

fn is_solved(vials: &[Vec<u8>], n_volume: usize) -> bool {
    vials.iter().all(|vial| {
        if vial.is_empty() {
            return true;
        }
        if vial.len() != n_volume {
            return false;
        }
        vial.iter().all(|c| *c == vial[0])
    })
}

fn legal_moves(vials: &[Vec<u8>], n_volume: usize, single_mode: bool) -> Vec<Move> {
    let mut out = Vec::new();
    for src in 0..vials.len() {
        if vials[src].is_empty() {
            continue;
        }
        let (src_col, src_run) = match vial_top(&vials[src]) {
            Some(x) => x,
            None => continue,
        };

        for dst in 0..vials.len() {
            if src == dst {
                continue;
            }
            let dst_len = vials[dst].len();
            if dst_len >= n_volume {
                continue;
            }

            if dst_len == 0 {
                let src_has_single_color = src_run == vials[src].len();
                if src_has_single_color {
                    continue;
                }
                out.push(Move { src, dst });
                continue;
            }

            if let Some((dst_col, _)) = vial_top(&vials[dst]) {
                if dst_col == src_col {
                    let move_amount = if single_mode {
                        1
                    } else {
                        src_run.min(n_volume - dst_len)
                    };
                    if move_amount > 0 {
                        out.push(Move { src, dst });
                    }
                }
            }
        }
    }
    out
}

fn apply_move(vials: &[Vec<u8>], mv: &Move, n_volume: usize, single_mode: bool) -> Option<Vec<Vec<u8>>> {
    if mv.src >= vials.len() || mv.dst >= vials.len() || mv.src == mv.dst {
        return None;
    }

    let mut next = vials.to_vec();
    let (src_col, src_run) = vial_top(&next[mv.src])?;
    let dst_len = next[mv.dst].len();

    if dst_len >= n_volume {
        return None;
    }

    if dst_len > 0 {
        let (dst_col, _) = vial_top(&next[mv.dst])?;
        if dst_col != src_col {
            return None;
        }
    }

    if dst_len == 0 {
        let src_has_single_color = src_run == next[mv.src].len();
        if src_has_single_color {
            return None;
        }
    }

    let count = if single_mode {
        1
    } else {
        src_run.min(n_volume - dst_len)
    };

    for _ in 0..count {
        let c = next[mv.src].pop()?;
        next[mv.dst].push(c);
    }

    Some(next)
}

fn random_puzzle_with_rng<R: rand::Rng + ?Sized>(settings: PuzzleSettings, rng: &mut R) -> PuzzleState {
    let n_vials = settings.n_colors + settings.n_empty;
    let mut pool = Vec::with_capacity(settings.n_colors * settings.n_volume);

    for c in 1..=settings.n_colors as u8 {
        for _ in 0..settings.n_volume {
            pool.push(c);
        }
    }

    pool.shuffle(rng);

    let mut idx = 0usize;
    let mut vials = vec![Vec::with_capacity(settings.n_volume); n_vials];
    for vial in vials.iter_mut().take(settings.n_colors) {
        for _ in 0..settings.n_volume {
            vial.push(pool[idx]);
            idx += 1;
        }
    }

    PuzzleState {
        vials,
        n_volume: settings.n_volume,
    }
}

pub fn random_puzzle(settings: PuzzleSettings) -> PuzzleState {
    let mut rng = rand::rng();
    random_puzzle_with_rng(settings, &mut rng)
}

pub fn parse_seed_hex(seed_hex: &str) -> Result<u64, String> {
    let trimmed = seed_hex.trim();
    if trimmed.len() != 16 {
        return Err("Seed must be exactly 16 hex characters".to_string());
    }
    u64::from_str_radix(trimmed, 16).map_err(|_| "Seed must be valid hex".to_string())
}

pub fn random_puzzle_seeded(settings: PuzzleSettings, seed_hex: String) -> Result<PuzzleState, String> {
    let seed = parse_seed_hex(&seed_hex)?;
    let mut rng = StdRng::seed_from_u64(seed);
    Ok(random_puzzle_with_rng(settings, &mut rng))
}

pub fn check_state(state: PuzzleState, single_mode: bool) -> StateCheck {
    if is_solved(&state.vials, state.n_volume) {
        return StateCheck {
            status: "solved".to_string(),
            moves_available: 0,
        };
    }

    let moves = legal_moves(&state.vials, state.n_volume, single_mode).len();
    if moves == 0 {
        StateCheck {
            status: "stuck".to_string(),
            moves_available: 0,
        }
    } else {
        StateCheck {
            status: "inProgress".to_string(),
            moves_available: moves,
        }
    }
}

pub fn solve(state: PuzzleState, single_mode: bool) -> SolverResult {
    if is_solved(&state.vials, state.n_volume) {
        return SolverResult {
            status: "alreadySolved".to_string(),
            nodes_generated: 1,
            optimal_moves: Some(0),
            moves: Vec::new(),
        };
    }

    let start_key = state_key(&state.vials);
    let mut queue: VecDeque<Vec<Vec<u8>>> = VecDeque::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut parent: HashMap<String, (String, Move)> = HashMap::new();

    queue.push_back(state.vials.clone());
    visited.insert(start_key.clone());

    let mut nodes = 1usize;
    let mut solved_key: Option<String> = None;

    while let Some(cur) = queue.pop_front() {
        let cur_key = state_key(&cur);
        for mv in legal_moves(&cur, state.n_volume, single_mode) {
            let Some(next) = apply_move(&cur, &mv, state.n_volume, single_mode) else {
                continue;
            };
            let next_key = state_key(&next);
            if visited.contains(&next_key) {
                continue;
            }

            visited.insert(next_key.clone());
            parent.insert(next_key.clone(), (cur_key.clone(), mv));
            nodes += 1;

            if is_solved(&next, state.n_volume) {
                solved_key = Some(next_key);
                break;
            }

            if nodes >= MAX_NODES {
                return SolverResult {
                    status: "nodeLimitExceeded".to_string(),
                    nodes_generated: nodes,
                    optimal_moves: None,
                    moves: Vec::new(),
                };
            }

            queue.push_back(next);
        }

        if solved_key.is_some() {
            break;
        }
    }

    let Some(mut cur_key) = solved_key else {
        return SolverResult {
            status: "noSolutionFound".to_string(),
            nodes_generated: nodes,
            optimal_moves: None,
            moves: Vec::new(),
        };
    };

    let mut moves = Vec::new();
    while cur_key != start_key {
        let Some((prev, mv)) = parent.get(&cur_key) else {
            break;
        };
        moves.push(mv.clone());
        cur_key = prev.clone();
    }
    moves.reverse();

    SolverResult {
        status: "ok".to_string(),
        nodes_generated: nodes,
        optimal_moves: Some(moves.len()),
        moves,
    }
}
