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

use crate::position::Position;

/// Counts legal move paths of a given length.
///
/// Shorter paths (due to mate or stalemate) are not counted.
/// Computing perft numbers is useful for comparing, testing and
/// debugging move generation correctness and performance.
///
/// The method used here is simply recursively enumerating the entire tree of
/// legal moves. While this is fine for testing there is much
/// faster specialized software.
///
/// Warning: Computing perft numbers can take a long time, even at moderate
/// depths. The simple recursive algorithm can also overflow the stack at
/// high depths, but this will only come into consideration in the rare case
/// that high depths are feasible at all.
///
/// # Examples
///
/// ```
/// use shakmaty::{Chess, perft};
///
/// let pos = Chess::default();
/// assert_eq!(perft(&pos, 1), 20);
/// assert_eq!(perft(&pos, 2), 400);
/// assert_eq!(perft(&pos, 3), 8902);
/// ```
pub fn perft<P: Position + Clone>(pos: &P, depth: u32) -> u64 {
    if depth < 1 {
        1
    } else {
        let moves = pos.legal_moves();

        if depth == 1 {
            moves.len() as u64
        } else {
            moves
                .iter()
                .map(|m| {
                    let mut child = pos.clone();
                    child.play_unchecked(m);
                    perft(&child, depth - 1)
                })
                .sum()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::position::Chess;

    #[test]
    fn test_perft() {
        let pos = Chess::default();
        assert_eq!(perft(&pos, 0), 1);
        assert_eq!(perft(&pos, 1), 20);
    }
}
