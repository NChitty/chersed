use std::{fmt::Display, str::FromStr};

type Bitboard = u64;

const FILES: [u64; 8] = [
    0x8080808080808080,
    0x4040404040404040,
    0x2020202020202020,
    0x1010101010101010,
    0x0808080808080808,
    0x0404040404040404,
    0x0202020202020202,
    0x0101010101010101,
];

const RANKS: [u64; 8] = [
    0x00000000000000FF,
    0x000000000000FF00,
    0x0000000000FF0000,
    0x00000000FF000000,
    0x000000FF00000000,
    0x0000FF0000000000,
    0x00FF000000000000,
    0xFF00000000000000,
];

const RANK_MATRIX: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];

fn square_number_from_str(str: &str) -> Option<u8> {
    Some(
        str.chars().nth(0)?.to_digit(16)? as u8 - 10 * 8 + str.chars().nth(1)?.to_digit(10)? as u8
            - 1,
    )
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Color {
    Black,
    White,
}

impl Color {
    fn index(&self) -> usize {
        match self {
            Black => 1,
            White => 0,
        }
    }
}

use Color::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Piece {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

impl Piece {
    fn index(&self) -> usize {
        match self {
            Pawn(color) => 0 + color.index(),
            Knight(color) => 2 + color.index(),
            Bishop(color) => 4 + color.index(),
            Rook(color) => 6 + color.index(),
            Queen(color) => 8 + color.index(),
            King(color) => 10 + color.index(),
        }
    }

    pub fn fen(&self) -> String {
        match self {
            Pawn(color) => {
                if matches!(color, White) {
                    "P".to_string()
                } else {
                    "p".to_string()
                }
            }
            Knight(color) => {
                if matches!(color, White) {
                    "N".to_string()
                } else {
                    "n".to_string()
                }
            }
            Bishop(color) => {
                if matches!(color, White) {
                    "B".to_string()
                } else {
                    "b".to_string()
                }
            }
            Rook(color) => {
                if matches!(color, White) {
                    "R".to_string()
                } else {
                    "r".to_string()
                }
            }
            Queen(color) => {
                if matches!(color, White) {
                    "Q".to_string()
                } else {
                    "q".to_string()
                }
            }
            King(color) => {
                if matches!(color, White) {
                    "K".to_string()
                } else {
                    "k".to_string()
                }
            }
        }
    }

    fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Pawn(White)),
            1 => Some(Pawn(Black)),
            2 => Some(Knight(White)),
            3 => Some(Knight(Black)),
            4 => Some(Bishop(White)),
            5 => Some(Bishop(Black)),
            6 => Some(Rook(White)),
            7 => Some(Rook(Black)),
            8 => Some(Queen(White)),
            9 => Some(Queen(Black)),
            10 => Some(King(White)),
            11 => Some(King(Black)),
            _ => None,
        }
    }

    fn from_char(char: char) -> Option<Self> {
        match char {
            'P' => Some(Pawn(White)),
            'p' => Some(Pawn(Black)),
            'N' => Some(Knight(White)),
            'n' => Some(Knight(Black)),
            'B' => Some(Bishop(White)),
            'b' => Some(Bishop(Black)),
            'R' => Some(Rook(White)),
            'r' => Some(Rook(Black)),
            'Q' => Some(Queen(White)),
            'q' => Some(Queen(Black)),
            'K' => Some(King(White)),
            'k' => Some(King(Black)),
            _ => None,
        }
    }
}

use Piece::*;

#[derive(Debug, PartialEq)]
struct GameState {
    bitboards: [Bitboard; 12],
    active_color: Color,
    castling_rights: [bool; 4],
    en_passant_target: Option<u8>,
    half_move_clock: u8,
    full_move_number: u8,
}

impl Default for GameState {
    fn default() -> Self {
        let bitboards = [
            RANKS[1],
            RANKS[6],
            0x0000000000000042,
            0x4200000000000000,
            0x0000000000000024,
            0x2400000000000000,
            0x0000000000000081,
            0x8100000000000000,
            0x0000000000000010,
            0x1000000000000000,
            0x0000000000000008,
            0x0800000000000000,
        ];
        GameState {
            bitboards,
            active_color: White,
            castling_rights: [true; 4],
            en_passant_target: None,
            half_move_clock: 0,
            full_move_number: 1,
        }
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board_state = self.get_board_state();
        let mut position: Vec<String> = Vec::new();
        for rank in (0..8).rev() {
            let mut buffer: String = "".to_string();
            let mut blanks = 0;
            for val in board_state[rank].iter() {
                if let Some(piece) = val {
                    if blanks > 0 {
                        buffer.push_str(&blanks.to_string());
                    }
                    blanks = 0;
                    buffer.push_str(&piece.fen());
                } else {
                    blanks += 1;
                }
            }
            if blanks > 0 {
                buffer.push_str(&blanks.to_string());
            }
            position.push(buffer);
        }
        let position = position.join("/");
        write!(f, "{} ", position)?;
        match self.active_color {
            Black => write!(f, "b ")?,
            White => write!(f, "w ")?,
        };
        let mut buffer = "".to_string();
        for (i, can_castle) in self.castling_rights.iter().enumerate() {
            let mut char = "".to_string();
            if i % 2 == 0 && *can_castle {
                char = "K".to_string();
            } else if *can_castle {
                char = "Q".to_string();
            }
            if i >= 2 {
                char = char.to_lowercase();
            }
            buffer.push_str(&char);
        }
        if buffer == "" {
            buffer = "-".to_string();
        }
        write!(f, "{} ", buffer)?;
        if let Some(en_passant_target) = self.en_passant_target {
            let file = en_passant_target % 8 + 1;
            let rank = RANK_MATRIX[en_passant_target as usize / 8];
            write!(f, "{}{} ", rank, file)?;
        } else {
            write!(f, "- ")?;
        }
        write!(f, "{} {}", self.half_move_clock, self.full_move_number)?;
        Ok(())
    }
}

impl FromStr for GameState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits: Vec<&str> = s.split(" ").collect();
        if splits.len() < 6 {
            return Err("Not enough fields for ".to_string());
        }
        let mut bitboards = [0x0; 12];
        let board: &str = splits.get(0).ok_or("No board state")?;
        for (rank, file_val) in board.split("/").enumerate() {
            let mut file = 0;
            for piece in file_val.chars() {
                if let Some(piece) = Piece::from_char(piece) {
                    bitboards[piece.index()] |= 1 << ((7 - rank) * 8 + (7 - file));
                    file += 1;
                } else if piece.is_numeric() {
                    file += piece.to_string().parse::<usize>().map_err(|parse_int_error| format!("Could not parse character a number: {parse_int_error}"))?;
                }
            }
        }
        let active_color = if *splits.get(1).ok_or("No color")? == "w" {
            White
        } else {
            Black
        };
        let mut castling_rights = [false; 4];
        let castling_rights_str = splits.get(2).ok_or("No castling rights")?;
        if castling_rights_str.contains("K") {
            castling_rights[0] = true;
        }
        if castling_rights_str.contains("k") {
            castling_rights[2] = true;
        }
        if castling_rights_str.contains("Q") {
            castling_rights[1] = true;
        }
        if castling_rights_str.contains("q") {
            castling_rights[3] = true;
        }
        let en_passant_target_str = *splits.get(3).ok_or("No en passant target")?;
        let en_passant_target = if en_passant_target_str == "-" {
            None
        } else {
            square_number_from_str(en_passant_target_str)
        };
        Ok(GameState {
            bitboards,
            active_color,
            castling_rights,
            en_passant_target,
            half_move_clock: splits.get(4).ok_or("No half move clock")?.parse::<u8>().map_err(|parse_int_error| format!("Could not parse half move clock: {parse_int_error}"))?,
            full_move_number: splits.get(5).ok_or("No full move number")?.parse::<u8>().map_err(|parse_int_error| format!("Could not parse full move number: {parse_int_error}"))?,
        })
    }
}

impl GameState {
    pub fn get_bitboard(&self, piece: Piece) -> &Bitboard {
        &self.bitboards[piece.index()]
    }

    pub fn get_piece_at(&self, rank: usize, file: usize) -> Option<Piece> {
        let mask = RANKS[rank] & FILES[file];
        let mut piece = None;
        for (index, val) in self.bitboards.iter().enumerate() {
            if mask & val != 0 {
                piece = Piece::from_index(index);
                break;
            }
        }
        piece
    }

    pub fn get_board_state(&self) -> [[Option<Piece>; 8]; 8] {
        let mut board = [[None; 8]; 8];
        for rank in 0..8 {
            for file in 0..8 {
                board[rank][file] = self.get_piece_at(rank, file);
            }
        }
        board
    }
}

#[cfg(test)]
mod test {
    use crate::GameState;

    #[test]
    fn board_state() {
        let game_state = GameState::default();
        assert_eq!(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            format!("{game_state}")
        );
    }

    #[test]
    fn board_from_fen() {
        let game_state = GameState::default();
        let default_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let actual = default_fen.parse::<GameState>();
        assert!(!actual.is_err());
        assert_eq!(
            game_state,
            actual.unwrap()
        );
    }
}
