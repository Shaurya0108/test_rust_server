use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    InGame,
    GameOver,
}

#[derive(Component)]
pub struct Board {
    pub squares: [[Option<Piece>; 8]; 8],
}

#[derive(Component, Clone, Copy, Serialize, Deserialize)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: PieceColor,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum PieceColor {
    White,
    Black,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_systems(Startup, setup_board)
            .add_systems(Update, (
                handle_piece_selection,
                handle_piece_movement,
                check_game_state,
            ));
    }
}

fn setup_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Initialize the chess board
    let mut board = Board {
        squares: [[None; 8]; 8],
    };

    // Set up initial piece positions
    setup_initial_pieces(&mut board);

    commands.spawn(board);

    // Setup camera
    commands.spawn(Camera2dBundle::default());
}

fn setup_initial_pieces(board: &mut Board) {
    // Set up pawns
    for i in 0..8 {
        board.squares[1][i] = Some(Piece {
            piece_type: PieceType::Pawn,
            color: PieceColor::White,
        });
        board.squares[6][i] = Some(Piece {
            piece_type: PieceType::Pawn,
            color: PieceColor::Black,
        });
    }

    // Set up other pieces
    let piece_order = [
        PieceType::Rook,
        PieceType::Knight,
        PieceType::Bishop,
        PieceType::Queen,
        PieceType::King,
        PieceType::Bishop,
        PieceType::Knight,
        PieceType::Rook,
    ];

    for (i, &piece_type) in piece_order.iter().enumerate() {
        board.squares[0][i] = Some(Piece {
            piece_type,
            color: PieceColor::White,
        });
        board.squares[7][i] = Some(Piece {
            piece_type,
            color: PieceColor::Black,
        });
    }
}