use std::collections::HashSet;

use crate::board::Board;
use crate::types::*;
use crate::game::{ apply_move };
use crate::rules::{ legal_moves, legal_placements, opponent, is_occupied, neighbors };

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

pub fn evaluate(state: &GameState, player: Player) -> i32 {
    queen_danger(state, opponent(player)) - queen_danger(state, player)
}

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
            let scores = all_legal_moves(state)
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

pub fn best_move_pruned(state: &GameState, depth: u32) -> Move {
    all_legal_moves(state)
        .into_iter()
        .map(|mv| {
            let score = alpha_beta(&apply_move(state, mv.clone()), depth - 1, i32::MIN, i32::MAX, state.turn);
            (mv, score)
        })
        .max_by_key(|&(_, score)| score)
        .map(|(mv, _)| mv)
        .expect("all_legal_moves never return an empty list")
}

pub fn alpha_beta(state: &GameState, depth: u32, alpha: i32, beta: i32, maximising: Player) -> i32 {
    match state.result {
        Some(GameResult::Win(p)) => {
            let score = WIN_SCORE + depth as i32;
            if p == maximising { score } else { -score }
        }
        Some(GameResult::Draw) => 0,
        None if depth == 0 => evaluate(state, maximising),
        None => {
            let moves = all_legal_moves(state);
            if state.turn == maximising {
                max_search(state, &moves, depth, alpha, beta, maximising, i32::MIN)
            } else {
                min_search(state, &moves, depth, alpha, beta, maximising, i32::MAX)
            }
        }
    }
}

fn max_search(state: &GameState, moves: &[Move], depth: u32, alpha: i32, beta: i32, perspective: Player, best: i32) -> i32 {
    match moves {
        [] => best,
        [mv, rest @ ..] => {
            let child_score = alpha_beta(&apply_move(state, mv.clone()), depth - 1, alpha, beta, perspective);
            let new_best = best.max(child_score);
            let new_alpha = alpha.max(new_best);
            if new_alpha >= beta {
                new_best
            } else {
                max_search(state, rest, depth, new_alpha, beta, perspective, new_best)
            }
        }
    }
}

fn min_search(state: &GameState, moves: &[Move], depth: u32, alpha: i32, beta: i32, perspective: Player, best: i32) -> i32 {
    match moves {
        [] => best,
        [mv, rest @ ..] => {
            let child_score = alpha_beta(&apply_move(state, mv.clone()), depth - 1, alpha, beta, perspective);
            let new_best = best.min(child_score);
            let new_beta = beta.min(new_best);
            if alpha >= new_beta {
                new_best
            } else {
                min_search(state, rest, depth, alpha, new_beta, perspective, new_best)
            }
        }
    }
}

#[cfg(test)]
mod engine_tests;