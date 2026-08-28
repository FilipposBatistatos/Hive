use super::*;
use expect_test::expect;

fn render_moves(moves: &[Move]) -> String {
    // Helper function to visualise pieces on the board
    let visualised = moves.iter().fold(Board::new(), |b, mov| match mov {
        Move::Move { to, .. } => b.place_piece(*to, Piece { kind: PieceKind::Ant, owner: Player::White }),
        Move::Place { at, .. } => b.place_piece(*at, Piece { kind: PieceKind::Ant, owner: Player::White }),
    });
    visualised.snapshot()
}

#[test]
fn correct_neighbor_positions() {
    let board = neighbors(Position { q: 0, r: 0 })
        .into_iter()
        .fold(Board::new(), |board, pos| {
            board.place_piece(pos, Piece { kind: PieceKind::Ant, owner: Player::White })
        });

    let output = board.snapshot();

    expect![[r#"
        . A A
         A . A
          A A ."#]].assert_eq(&output);
}

#[test]
fn on_hive_requires_shared_neighbor() {
    let occupied_positions = vec![
        Position {q: 1, r: 1},
        Position {q: 0, r: 2},
        Position {q: -1, r: 2},
        Position {q: -1, r: 1},
    ];
    
    let board = occupied_positions.iter().fold(Board::new(), |board, &pos| {
        board.place_piece(pos, Piece {kind: PieceKind::Ant, owner: Player::White })
    });
    let moves = bee_moves(&Position {q: 0, r: 0}, &board);
    let output = render_moves(&moves);

    expect![[r#"
        A .
         . A"#]].assert_eq(&output);
}

#[test]
fn correct_bee_moves() {
    let occupied_positions = vec![
        Position {q: 1, r: 0},
        Position {q: 1, r: 1},
        Position {q: 0, r: 2},
        Position {q: -1, r: 2},
        Position {q: -1, r: 1},
    ];
    
    let board = occupied_positions.iter().fold(Board::new(), |board, &pos| {
        board.place_piece(pos, Piece {kind: PieceKind::Ant, owner: Player::White })
    });
    let moves = bee_moves(&Position {q: 0, r: 0}, &board);
    let output = render_moves(&moves);

    expect![[r#"
        . . A
         A . ."#]].assert_eq(&output);
}

#[test]
fn correct_ant_moves() {
    let occupied_positions = vec![
        Position {q: 1, r: 0},
        Position {q: 1, r: 1},
        Position {q: 0, r: 2},
        Position {q: -1, r: 2},
        Position {q: -1, r: 1},
    ];
    
    let board = occupied_positions.iter().fold(Board::new(), |board, &pos| {
        board.place_piece(pos, Piece {kind: PieceKind::Ant, owner: Player::White })
    });
    let moves = ant_moves(&Position {q: 0, r: 0}, &board);
    let output = render_moves(&moves);

    expect![[r#"
        . . . A A
         . A A . A
          A . . . A
           A . . A .
            A A A . ."#]].assert_eq(&output);
}

#[test]
fn correct_spider_moves() {
    let occupied_positions = vec![
        Position {q: 1, r: 0},
        Position {q: 1, r: 1},
        Position {q: 0, r: 2},
        Position {q: -1, r: 2},
        Position {q: -1, r: 1},
    ];
    
    let board = occupied_positions.iter().fold(Board::new(), |board, &pos| {
        board.place_piece(pos, Piece {kind: PieceKind::Ant, owner: Player::White })
    });
    let moves = spider_moves(&Position {q: 0, r: 0}, &board);
    let output = render_moves(&moves);

    expect![[r#"
        . . . . A
         . . . . .
          A . . . ."#]].assert_eq(&output);
}

#[test]
fn correct_beetle_moves() {
    let occupied_positions = vec![
        Position {q: 0, r: 0},
        Position {q: 1, r: 0},
        Position {q: 1, r: 1},
        Position {q: 0, r: 2},
        Position {q: -1, r: 2},
        Position {q: -1, r: 1},
    ];
    
    let board = occupied_positions.iter().fold(Board::new(), |board, &pos| {
        board.place_piece(pos, Piece {kind: PieceKind::Ant, owner: Player::White })
    });
    let moves = beetle_moves(&Position {q: 0, r: 0}, &board);
    let output = render_moves(&moves);

    expect![[r#"
        . . A
         A . A
          A . ."#]].assert_eq(&output);
}

#[test]
fn complex_beetle_moves() {
    let occupied_positions = vec![
        // Double stacked pieces
        Position {q: 0, r: 0},
        Position {q: 0, r: 0},
        Position {q: 1, r: 0},
        Position {q: 1, r: 0},
        Position {q: -1, r: 1},
        Position {q: -1, r: 1},
        // Single stacked pieces
        Position {q: -1, r: 0},
        Position {q: 0, r: -1},
        Position {q: 1, r: -1}
    ];
    
    let board = occupied_positions.iter().fold(Board::new(), |board, &pos| {
        board.place_piece(pos, Piece {kind: PieceKind::Ant, owner: Player::White })
    });
    let moves = beetle_moves(&Position {q: 0, r: 0}, &board);
    let output = render_moves(&moves);

    expect![[r#"
        . A A
         A . A
          A . ."#]].assert_eq(&output);
}

#[test]
fn placing_only_with_no_enemy_neighbors() {
    let occupied_positions = vec![
        Position {q: 1, r: 0},
        Position {q: 1, r: 1},
        Position {q: 0, r: 2},
        Position {q: -1, r: 2},
        Position {q: -1, r: 1},
    ];
    
    let board = occupied_positions.iter().enumerate().fold(Board::new(), |board, (index, &pos)| {
        board.place_piece(pos, Piece {kind: PieceKind::Ant, owner: if index % 2 == 0 { Player::White } else { Player::Black} })
    });
    let placements = legal_placements(&board, Player::White)
        .into_iter()
        .map(|p| Move::Place {kind: PieceKind::Bee, at: p })
        .collect::<Vec<Move>>();    
    let output = render_moves(&placements);

    expect![[r#"
        . . . A A
         . A A . .
          A . . . .
           . . . . .
            . . A . ."#]].assert_eq(&output);
}

#[test]
fn legal_place_when_board_has_only_one_piece() {
    let board = Board::new().place_piece(Position {q: 0, r: 0}, Piece {kind: PieceKind::Bee, owner: Player::White});
    let placements = legal_placements(&board, Player::Black)
        .into_iter()
        .map(|p| Move::Place {kind: PieceKind::Bee, at: p })
        .collect::<Vec<Move>>();
    let output = render_moves(&placements);

    expect![[r#"
        . A A
         A . A
          A A ."#]].assert_eq(&output);
}

#[test]
fn legal_place_when_board_is_empty() {
    let board = Board::new();
    let placements = legal_placements(&board, Player::White)
        .into_iter()
        .map(|p| Move::Place {kind: PieceKind::Bee, at: p})
        .collect::<Vec<Move>>();
    let output = render_moves(&placements);

    expect!["A"].assert_eq(&output);
}

// Everything here really should be a prop test
#[test]
fn removing_a_bridge_piece_breaks_the_hive() {
    let board = Board::new()
        .place_piece(Position { q: 0, r: 0}, Piece { kind: PieceKind::Ant, owner: Player::White })
        .place_piece(Position { q: 1, r: 0}, Piece { kind: PieceKind::Ant, owner: Player::White })
        .place_piece(Position { q: 2, r: 0}, Piece { kind: PieceKind::Ant, owner: Player::White });
    
    assert!(!preserves_hive(&board, Position {q: 1, r: 0}));
    assert!(preserves_hive(&board, Position {q: 2, r: 0}));
}

#[test]
fn cannot_slide_through_pinched_gap() {
    let board = Board::new()
        .place_piece(Position { q: 1, r: 0}, Piece { kind: PieceKind::Ant, owner: Player::White })
        .place_piece(Position { q: -1, r: 1}, Piece { kind: PieceKind::Ant, owner: Player::White });

    assert!(!can_slide(&board, Position { q: 0, r: 0 }, Position { q: 0, r: 1 }, 1));
}

use proptest::prelude::*;
use proptest::test_runner::TestRunner;
use proptest::strategy::ValueTree;

fn arbitrary_board(steps: usize) -> impl Strategy<Value = Board> {
    prop::collection::vec(any::<usize>(), steps).prop_map(move |choices| {
        choices.into_iter().enumerate().fold(Board::new(), |board, (step, choice)| {
            let player = if step % 2 == 0 { Player::White } else { Player::Black };
            let candidates: Vec<Position> = legal_placements(&board, player).into_iter().collect();
            
            if candidates.is_empty() {
                return board; // Nowhere is legal to place, skip this step rather than panic
            }

            let pos = candidates[choice % candidates.len()];
            board.place_piece(pos, Piece { kind: PieceKind::Ant, owner: player })
        })
    })
}

fn arbitrary_player() -> impl Strategy<Value = Player> {
    prop_oneof![Just(Player::White), Just(Player::Black)]
}

#[test]
fn generated_boards_have_balanced_piece_counts() {
    let mut runner = TestRunner::default();
    let steps = 10;

    let tree = arbitrary_board(steps).new_tree(&mut runner).unwrap();
    let board = tree.current();

    let white_count = board.stacks
        .values()
        .flatten()
        .filter(|piece| piece.owner == Player::White)
        .count();

    let black_count = board.stacks
        .values()
        .flatten()
        .filter(|piece| piece.owner == Player::Black)
        .count();
    
    assert_eq!(white_count + black_count, steps, "Every step should place exactly one piece");
    assert_eq!(white_count, black_count, "Even step count should split evenly between players");
}

proptest! {
    #[test]
    fn legal_place_have_no_enemy_neighbors (board in arbitrary_board(8), player in arbitrary_player()) {
        // Iterate over different boards to ensure that they positions they produce 
        // Do not come in contact with any enemies
        let placements = legal_placements(&board, player);
        prop_assert!(
            placements.iter().all(|&p| !adjacent_to_opponent(&board, p, player))
        );
    }
}