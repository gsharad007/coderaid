use bevy::prelude::*;

use itertools::Itertools;

pub mod cell {
    use derive_more::{BitAnd, BitOr};

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, BitOr, BitAnd, PartialOrd, Ord)]
    pub struct Type(u8);
    // pub type Type = u8;

    pub const EMPTY: Type = Type(0b0);
    pub const OPEN_NEG_X: Type = Type(0b1 << 0);
    pub const OPEN_POS_X: Type = Type(0b1 << 1);
    pub const OPEN_NEG_Y: Type = Type(0b1 << 2);
    pub const OPEN_POS_Y: Type = Type(0b1 << 3);
    pub const OPEN_NEG_Z: Type = Type(0b1 << 4);
    pub const OPEN_POS_Z: Type = Type(0b1 << 5);
}

#[derive(Component, Debug)]
pub struct Cells {
    pub array: Vec<Vec<cell::Type>>,
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl Cells {
    fn new(array: Vec<Vec<cell::Type>>, x: u32, y: u32, z: u32) -> Self {
        Self { array, x, y, z }
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn from_string(map_string: &str) -> Self {
        let cells = map_string
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().map(cell_char_to_cell_type).collect_vec())
            .collect_vec();

        let x = cells.iter().map(Vec::len).max().unwrap_or_default();
        let z = cells.len();

        Self::new(cells, x as u32, 1, z as u32)
    }
}

#[allow(clippy::non_ascii_literal)]
fn cell_char_to_cell_type(cell_char: char) -> cell::Type {
    match cell_char {
        // '█' => cell::EMPTY,
        '╨' => cell::OPEN_NEG_Z,
        '╥' => cell::OPEN_POS_Z,
        '╞' => cell::OPEN_POS_X,
        '╡' => cell::OPEN_NEG_X,
        '║' => cell::OPEN_NEG_Z | cell::OPEN_POS_Z,
        '═' => cell::OPEN_NEG_X | cell::OPEN_POS_X,
        '╝' => cell::OPEN_NEG_Z | cell::OPEN_NEG_X,
        '╚' => cell::OPEN_NEG_Z | cell::OPEN_POS_X,
        '╗' => cell::OPEN_POS_Z | cell::OPEN_NEG_X,
        '╔' => cell::OPEN_POS_Z | cell::OPEN_POS_X,
        '╠' => cell::OPEN_NEG_Z | cell::OPEN_POS_Z | cell::OPEN_POS_X,
        '╣' => cell::OPEN_NEG_Z | cell::OPEN_POS_Z | cell::OPEN_NEG_X,
        '╩' => cell::OPEN_NEG_Z | cell::OPEN_NEG_X | cell::OPEN_POS_X,
        '╦' => cell::OPEN_POS_Z | cell::OPEN_NEG_X | cell::OPEN_POS_X,
        '╬' => cell::OPEN_NEG_Z | cell::OPEN_NEG_X | cell::OPEN_POS_X | cell::OPEN_POS_Z,
        _ => cell::EMPTY,
    }
}

#[cfg(test)]
mod test_map_load_string {
    use super::*;

    #[allow(clippy::indexing_slicing)]
    #[test]
    fn test_map_load_string_4x4() {
        #[allow(clippy::non_ascii_literal)]
        let map_string = "
╞═╦╗
╞═╬╣
██║║
╞═╩╝
";

        let cells = Cells::from_string(map_string);

        // println!("{}", map_string);
        // println!("{:?}", map.cells);

        // ╞═╦╗
        assert_eq!(
            cells.array[0],
            [
                cell::OPEN_POS_X,
                cell::OPEN_NEG_X | cell::OPEN_POS_X,
                cell::OPEN_NEG_X | cell::OPEN_POS_X | cell::OPEN_POS_Z,
                cell::OPEN_NEG_X | cell::OPEN_POS_Z
            ]
        );

        // ╞═╬╣
        assert_eq!(
            cells.array[1],
            [
                cell::OPEN_POS_X,
                cell::OPEN_NEG_X | cell::OPEN_POS_X,
                cell::OPEN_NEG_X | cell::OPEN_POS_X | cell::OPEN_POS_Z | cell::OPEN_NEG_Z,
                cell::OPEN_NEG_X | cell::OPEN_POS_Z | cell::OPEN_NEG_Z
            ]
        );

        // ██║║
        assert_eq!(
            cells.array[2],
            [
                cell::EMPTY,
                cell::EMPTY,
                cell::OPEN_NEG_Z | cell::OPEN_POS_Z,
                cell::OPEN_NEG_Z | cell::OPEN_POS_Z
            ]
        );

        // ╞═╩╝
        assert_eq!(
            cells.array[3],
            [
                cell::OPEN_POS_X,
                cell::OPEN_NEG_X | cell::OPEN_POS_X,
                cell::OPEN_NEG_X | cell::OPEN_POS_X | cell::OPEN_NEG_Z,
                cell::OPEN_NEG_X | cell::OPEN_NEG_Z
            ]
        );
    }

    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::too_many_lines)]
    #[test]
    fn test_map_load_string_8x8() {
        #[allow(clippy::non_ascii_literal)]
        let map_string = "
╞═╦╗╔╦╩╡
╞═╬╣╠╬╦╡
██║║╠╣║█
╞═╩╝╠╣║█
╔╗╔╗╚╝║█
╝╚╝╚╗╔╝█
████╚╝██
████╔╗██
";

        let cells = Cells::from_string(map_string);

        // println!("{}", map_string);
        // println!("{:?}", map.cells);

        // ╞═╦╗╔╦╩╡
        assert_eq!(
            cells.array[0],
            [
                cell::OPEN_POS_X,
                cell::OPEN_POS_X | cell::OPEN_NEG_X,
                cell::OPEN_POS_X | cell::OPEN_NEG_X | cell::OPEN_POS_Z,
                cell::OPEN_NEG_X | cell::OPEN_POS_Z,
                cell::OPEN_POS_X | cell::OPEN_POS_Z,
                cell::OPEN_POS_X | cell::OPEN_NEG_X | cell::OPEN_POS_Z,
                cell::OPEN_POS_X | cell::OPEN_NEG_X | cell::OPEN_NEG_Z,
                cell::OPEN_NEG_X
            ]
        );

        // ╞═╬╣╠╬╦╡
        assert_eq!(
            cells.array[1],
            [
                cell::OPEN_POS_X,
                cell::OPEN_POS_X | cell::OPEN_NEG_X,
                cell::OPEN_POS_X | cell::OPEN_NEG_X | cell::OPEN_NEG_Z | cell::OPEN_POS_Z,
                cell::OPEN_NEG_X | cell::OPEN_NEG_Z | cell::OPEN_POS_Z,
                cell::OPEN_POS_X | cell::OPEN_NEG_Z | cell::OPEN_POS_Z,
                cell::OPEN_POS_X | cell::OPEN_NEG_X | cell::OPEN_NEG_Z | cell::OPEN_POS_Z,
                cell::OPEN_POS_X | cell::OPEN_NEG_X | cell::OPEN_POS_Z,
                cell::OPEN_NEG_X
            ]
        );

        // ██║║╠╣║█
        assert_eq!(
            cells.array[2],
            [
                cell::EMPTY,
                cell::EMPTY,
                cell::OPEN_NEG_Z | cell::OPEN_POS_Z,
                cell::OPEN_NEG_Z | cell::OPEN_POS_Z,
                cell::OPEN_NEG_Z | cell::OPEN_POS_Z | cell::OPEN_POS_X,
                cell::OPEN_NEG_Z | cell::OPEN_POS_Z | cell::OPEN_NEG_X,
                cell::OPEN_NEG_Z | cell::OPEN_POS_Z,
                cell::EMPTY
            ]
        );

        // ╞═╩╝╠╣║█
        assert_eq!(
            cells.array[3],
            [
                cell::OPEN_POS_X,
                cell::OPEN_POS_X | cell::OPEN_NEG_X,
                cell::OPEN_POS_X | cell::OPEN_NEG_X | cell::OPEN_NEG_Z,
                cell::OPEN_NEG_X | cell::OPEN_NEG_Z,
                cell::OPEN_NEG_Z | cell::OPEN_POS_Z | cell::OPEN_POS_X,
                cell::OPEN_NEG_Z | cell::OPEN_POS_Z | cell::OPEN_NEG_X,
                cell::OPEN_NEG_Z | cell::OPEN_POS_Z,
                cell::EMPTY
            ]
        );

        // ╔╗╔╗╚╝║█
        assert_eq!(
            cells.array[4],
            [
                cell::OPEN_POS_Z | cell::OPEN_POS_X,
                cell::OPEN_POS_Z | cell::OPEN_NEG_X,
                cell::OPEN_POS_Z | cell::OPEN_POS_X,
                cell::OPEN_POS_Z | cell::OPEN_NEG_X,
                cell::OPEN_NEG_Z | cell::OPEN_POS_X,
                cell::OPEN_NEG_Z | cell::OPEN_NEG_X,
                cell::OPEN_NEG_Z | cell::OPEN_POS_Z,
                cell::EMPTY
            ]
        );

        // ╝╚╝╚╗╔╝█
        assert_eq!(
            cells.array[5],
            [
                cell::OPEN_NEG_Z | cell::OPEN_NEG_X,
                cell::OPEN_NEG_Z | cell::OPEN_POS_X,
                cell::OPEN_NEG_Z | cell::OPEN_NEG_X,
                cell::OPEN_NEG_Z | cell::OPEN_POS_X,
                cell::OPEN_POS_Z | cell::OPEN_NEG_X,
                cell::OPEN_POS_Z | cell::OPEN_POS_X,
                cell::OPEN_NEG_Z | cell::OPEN_NEG_X,
                cell::EMPTY
            ]
        );

        // ████╚╝██
        assert_eq!(
            cells.array[6],
            [
                cell::EMPTY,
                cell::EMPTY,
                cell::EMPTY,
                cell::EMPTY,
                cell::OPEN_NEG_Z | cell::OPEN_POS_X,
                cell::OPEN_NEG_Z | cell::OPEN_NEG_X,
                cell::EMPTY,
                cell::EMPTY,
            ]
        );

        // ████╔╗██
        assert_eq!(
            cells.array[7],
            [
                cell::EMPTY,
                cell::EMPTY,
                cell::EMPTY,
                cell::EMPTY,
                cell::OPEN_POS_Z | cell::OPEN_POS_X,
                cell::OPEN_POS_Z | cell::OPEN_NEG_X,
                cell::EMPTY,
                cell::EMPTY,
            ]
        );
    }
}
