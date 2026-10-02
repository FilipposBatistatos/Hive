use std::collections::HashSet;
use std::time:: { Instant, Duration };

use crate::board::Board;
use crate::types::*;
use crate::game::{ apply_move };
use crate::rules::{ legal_moves, legal_placements, opponent, is_occupied, neighbors };

// ====================== Evaluation ======================
pub fn all_legal_placements(state: &GameState, player: Player) -> Vec<Move> {
    let positions = legal_placements(&state.board, player);
    let pieces = &state.unplaced[&player];

    positions
        .iter()
        .flat_map(|&pos| {
            pieces.keys().map(move |&kind| Move::Place { kind, at: pos })
        })
        .collect()
}

pub fn all_legal_movement(state: &GameState, player: Player) -> Vec<Move> {
    // Go over all our pieces on the board
    state.board.stacks
        .iter()
        .filter(|(_, stack)| stack.last().map(|piece| piece.owner) == Some(player))
        .flat_map(|(pos, _)| legal_moves(pos, state, player))
        .collect()
}

pub fn all_legal_moves_for(state: &GameState, player: Player) -> Vec<Move> {
    let moves: Vec<Move> = all_legal_movement(state, player)
        .into_iter()
        .chain(all_legal_placements(state, player))
        .collect();
    
    if moves.is_empty() {
        vec![Move::Pass]
    } else {
        moves
    }
}

pub fn all_legal_moves(state: &GameState) -> Vec<Move> {
    all_legal_moves_for(state, state.turn)
}

fn placement_count(state: &GameState, player: Player) -> usize {
    legal_placements(&state.board, player).len() * state.unplaced[&player].len()
}

fn movement_count(state: &GameState, player: Player) -> usize {
    state.board.stacks.iter()
        .filter(|(_,stack)| stack.last().map(|piece| piece.owner) == Some(player))
        .map(|(pos,_)| legal_moves(pos, state, player).len())
        .sum()
}

// State evaluation heuristics
fn mobility(state: &GameState, player: Player) -> i32 {
    (placement_count(state, player) + movement_count(state, player)) as i32
}

fn queen_position(state: &GameState, player: Player) -> Option<Position> {
    state.board.stacks.iter()
        .find(|(_, stack)| stack.iter().any(|p| p.kind == PieceKind::Bee && p.owner == player))
        .map(|(&pos,_)| pos)
}

fn queen_danger(state: &GameState, player: Player) -> i32 {
    match queen_position(state, player) {
        None => 0,
        Some(pos) => neighbors(pos).iter().filter(|&&n| is_occupied(n, &state.board)).count() as i32,
    }
}

fn ordered_score(state: &GameState, mv: &Move) -> i32 {
    let destination = match mv {
        Move::Place {at, ..} => *at,
        Move::Move {to, ..} => *to,
        Move::Pass => return 0,
    };
    match queen_position(state, opponent(state.turn)) {
        Some(opp_queen) if neighbors(opp_queen).contains(&destination) => 1,
        _ => 0,
    }
}

fn ordered_moves(state: &GameState, perspective: Player, maximising: bool) -> Vec<Move> {
    let mut moves = all_legal_moves(state);
    moves.sort_by_key(|mv| std::cmp::Reverse(ordered_score(state, mv)));
    moves
}

pub fn evaluate(state: &GameState, player: Player) -> i32 {
    queen_danger(state, opponent(player)) - queen_danger(state, player)
}

// ============================= MINIMAX ===============================

const WIN_SCORE: i32 = 1_000_000;

pub fn minimax(state: &GameState, depth: u32, maximising: Player) -> i32 {
    match state.result {
        Some(GameResult::Win(p)) => {
            let score = WIN_SCORE + depth as i32;
            if p == maximising { score } else { -score }
        }
        Some(GameResult::Draw) => 0,
        None if depth == 0 => evaluate(state, maximising),
        None => {
            let scores = ordered_moves(state, maximising, state.turn == maximising)
                .into_iter()
                .map(|mv| minimax(&apply_move(state, mv), depth - 1, maximising));

            let best = if state.turn == maximising { scores.max() } else {scores.min() };
            best.expect("never and empty list")
        }
    }
}

pub fn best_move(state: &GameState, depth: u32) -> Move {
    all_legal_moves(state)
        .into_iter()
        .map(|mv| {
            let score = minimax(&apply_move(state, mv.clone()), depth - 1, state.turn);
            (mv, score)
        })
        .max_by_key(|&(_, score)| score)
        .map(|(mv, _)| mv)
        .expect("all_legal_moves never returns an empty list")
}

// ============================== ALPHA BETA ==========================

fn best_move_inner(state: &GameState, depth: u32, deadline: Option<Instant>) -> Option<(Move, i32)> {
    all_legal_moves(state)
        .into_iter()
        .map(|mv| {
            let score = alpha_beta_inner(
                &apply_move(state, mv), depth - 1, i32::MIN, i32::MAX, state.turn, deadline
            )?;
            Some((mv,score))
        })
        .collect::<Option<Vec<_>>>()?
        .into_iter()
        .max_by_key(|&(_, score)| score)
}

fn best_move_alpha_beta(state: &GameState, depth: u32) -> Move {
    best_move_inner(state, depth, None)
        .expect("best_move_inner always returns a move with no deadline")
        .0
}

fn best_move_timed(state: &GameState, budget: Duration) -> Move {
    let deadline = Instant::now() + budget;
    let mut best = best_move_inner(state, 1, None)
        .expect("best_move_inner always returns a move with no deadline");
    for depth in 2..=20 {
        match best_move_inner(state, depth, Some(deadline)) {
            Some(result) => best = result,
            None => break,
        }
    }

    best.0
}

pub fn alpha_beta(state: &GameState, depth: u32, alpha: i32, beta: i32, maximising: Player) -> i32 {
    alpha_beta_inner(state, depth, alpha, beta, maximising, None)
        .expect("Alpha_beta with no deadline must always produce a score")
}

pub fn alpha_beta_timed(state: &GameState, depth: u32, alpha: i32, beta: i32, maximising: Player, deadline: Instant) -> Option<i32> {
    alpha_beta_inner(state, depth, alpha, beta, maximising, Some(deadline))
}

fn alpha_beta_inner(state: &GameState, depth: u32, alpha: i32, beta: i32, maximising: Player, deadline: Option<Instant>) -> Option<i32> {
    if deadline.is_some_and(|d| Instant::now() >= d) {
        return None;
    }
    match state.result {
        Some(GameResult::Win(p)) => {
            let score = WIN_SCORE + depth as i32;
            if p == maximising { Some(score) } else { Some(-score) }
        }
        Some(GameResult::Draw) => Some(0),
        None if depth == 0 => Some(evaluate(state, maximising)),
        None => {
            let moves = ordered_moves(state, maximising, state.turn == maximising);
            if state.turn == maximising {
                max_search(state, &moves, depth, alpha, beta, maximising, i32::MIN, deadline)
            } else {
                min_search(state, &moves, depth, alpha, beta, maximising, i32::MAX, deadline)
            }
        }
    }
}

fn max_search(state: &GameState, moves: &[Move], depth: u32, alpha: i32, beta: i32, perspective: Player, best: i32, deadline: Option<Instant>) -> Option<i32> {
    match moves {
        [] => Some(best),
        [mv, rest @ ..] => {
            let child_score = alpha_beta_inner(&apply_move(state, mv.clone()), depth - 1, alpha, beta, perspective, deadline)?;
            let new_best = best.max(child_score);
            let new_alpha = alpha.max(new_best);
            if new_alpha >= beta {
                Some(new_best)
            } else {
                max_search(state, rest, depth, new_alpha, beta, perspective, new_best, deadline)
            }
        }
    }
}

fn min_search(state: &GameState, moves: &[Move], depth: u32, alpha: i32, beta: i32, perspective: Player, best: i32, deadline: Option<Instant>) -> Option<i32> {
    match moves {
        [] => Some(best),
        [mv, rest @ ..] => {
            let child_score = alpha_beta_inner(&apply_move(state, mv.clone()), depth - 1, alpha, beta, perspective, deadline)?;
            let new_best = best.min(child_score);
            let new_beta = beta.min(new_best);
            if alpha >= new_beta {
                Some(new_best)
            } else {
                min_search(state, rest, depth, alpha, new_beta, perspective, new_best, deadline)
            }
        }
    }
}

#[cfg(test)]
mod engine_tests;