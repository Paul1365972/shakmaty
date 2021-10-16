// This file is part of the shakmaty library.
// Copyright (C) 2017-2021 Niklas Fiekas <niklas.fiekas@backscattering.de>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

//! A library for chess move generation.
//!
//! # Examples
//!
//! Generate legal moves in the starting position:
//!
//! ```
//! use shakmaty::{Chess, Position};
//!
//! let pos = Chess::default();
//! let legals = pos.legal_moves();
//! assert_eq!(legals.len(), 20);
//! ```
//!
//! Play moves:
//!
//! ```
//! # use std::error::Error;
//! # use shakmaty::{Chess, Position};
//! use shakmaty::{Square, Move, Role};
//! #
//! # let pos = Chess::default();
//!
//! // 1. e4
//! let pos = pos.play(Move::Normal {
//!     role: Role::Pawn,
//!     from: Square::E2,
//!     to: Square::E4,
//!     capture: None,
//!     promotion: None,
//! })?;
//! #
//! # Ok::<_, Box<dyn Error>>(())
//! ```
//!
//! Detect game end conditions:
//!
//! ```
//! # use shakmaty::{Chess, Position};
//! # let pos = Chess::default();
//! assert!(!pos.is_checkmate());
//! assert!(!pos.is_stalemate());
//! assert!(!pos.is_insufficient_material());
//!
//! assert_eq!(pos.outcome(), None); // no winner yet
//! ```
//!
//! Also supports [FEN](fen), [SAN](san) and
//! [UCI](uci) formats for positions and moves.
//!
//! # Feature flags
//!
//! * `variant`: Enables `shakmaty::variant` module for all Lichess variants.
//! * `step`: Implements [`std::iter::Step`] for `Square`, `File`, and `Rank`.
//!   Requires nightly.

#![doc(html_root_url = "https://docs.rs/shakmaty/0.20.1")]
#![forbid(unsafe_op_in_unsafe_fn)]
#![warn(missing_debug_implementations)]
#![cfg_attr(feature = "step", feature(step_trait))]
#![cfg_attr(docs_rs, feature(doc_cfg))]

mod board;
mod color;
mod magics;
mod material;
mod movelist;
mod perft;
mod position;
mod setup;
mod square;
mod types;
mod util;

pub mod attacks;
pub mod bitboard;
pub mod fen;
pub mod san;
pub mod uci;
pub mod zobrist;

#[cfg(feature = "variant")]
#[cfg_attr(docs_rs, doc(cfg(feature = "variant")))]
pub mod variant;

pub use crate::{
    bitboard::Bitboard,
    board::{Board, Pieces},
    color::{ByColor, ByColorIter, Color, ParseColorError},
    material::{Material, MaterialSide, ParseMaterialError},
    movelist::MoveList,
    perft::perft,
    position::{Chess, FromSetup, Outcome, PlayError, Position, PositionError, PositionErrorKinds},
    setup::{Castles, Setup},
    square::{File, ParseSquareError, Rank, Square},
    types::{CastlingMode, CastlingSide, Move, Piece, RemainingChecks, Role},
};
