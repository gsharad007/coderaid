use bevy::prelude::*;

use itertools::Itertools;

pub mod cell {
    use derive_more::{BitAnd, BitOr};

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, BitOr, BitAnd, PartialOrd, Ord)]
    pub struct Type(u8);
    // pub type Type = u8;

    pub const EMPTY: Type = Type(0b0);
    pub const OPEN_TOP: Type = Type(0b1);
    pub const OPEN_BOTTOM: Type = Type(0b10);
    pub const OPEN_LEFT: Type = Type(0b100);
    pub const OPEN_RIGHT: Type = Type(0b1000);
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
        '╨' => cell::OPEN_TOP,
        '╥' => cell::OPEN_BOTTOM,
        '╞' => cell::OPEN_RIGHT,
        '╡' => cell::OPEN_LEFT,
        '║' => cell::OPEN_TOP | cell::OPEN_BOTTOM,
        '═' => cell::OPEN_LEFT | cell::OPEN_RIGHT,
        '╝' => cell::OPEN_TOP | cell::OPEN_LEFT,
        '╚' => cell::OPEN_TOP | cell::OPEN_RIGHT,
        '╗' => cell::OPEN_BOTTOM | cell::OPEN_LEFT,
        '╔' => cell::OPEN_BOTTOM | cell::OPEN_RIGHT,
        '╠' => cell::OPEN_TOP | cell::OPEN_BOTTOM | cell::OPEN_RIGHT,
        '╣' => cell::OPEN_TOP | cell::OPEN_BOTTOM | cell::OPEN_LEFT,
        '╩' => cell::OPEN_TOP | cell::OPEN_LEFT | cell::OPEN_RIGHT,
        '╦' => cell::OPEN_BOTTOM | cell::OPEN_LEFT | cell::OPEN_RIGHT,
        '╬' => cell::OPEN_TOP | cell::OPEN_LEFT | cell::OPEN_RIGHT | cell::OPEN_BOTTOM,
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
                cell::OPEN_RIGHT,
                cell::OPEN_LEFT | cell::OPEN_RIGHT,
                cell::OPEN_LEFT | cell::OPEN_RIGHT | cell::OPEN_BOTTOM,
                cell::OPEN_LEFT | cell::OPEN_BOTTOM
            ]
        );

        // ╞═╬╣
        assert_eq!(
            cells.array[1],
            [
                cell::OPEN_RIGHT,
                cell::OPEN_LEFT | cell::OPEN_RIGHT,
                cell::OPEN_LEFT | cell::OPEN_RIGHT | cell::OPEN_BOTTOM | cell::OPEN_TOP,
                cell::OPEN_LEFT | cell::OPEN_BOTTOM | cell::OPEN_TOP
            ]
        );

        // ██║║
        assert_eq!(
            cells.array[2],
            [
                cell::EMPTY,
                cell::EMPTY,
                cell::OPEN_TOP | cell::OPEN_BOTTOM,
                cell::OPEN_TOP | cell::OPEN_BOTTOM
            ]
        );

        // ╞═╩╝
        assert_eq!(
            cells.array[3],
            [
                cell::OPEN_RIGHT,
                cell::OPEN_LEFT | cell::OPEN_RIGHT,
                cell::OPEN_LEFT | cell::OPEN_RIGHT | cell::OPEN_TOP,
                cell::OPEN_LEFT | cell::OPEN_TOP
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
                cell::OPEN_RIGHT,
                cell::OPEN_RIGHT | cell::OPEN_LEFT,
                cell::OPEN_RIGHT | cell::OPEN_LEFT | cell::OPEN_BOTTOM,
                cell::OPEN_LEFT | cell::OPEN_BOTTOM,
                cell::OPEN_RIGHT | cell::OPEN_BOTTOM,
                cell::OPEN_RIGHT | cell::OPEN_LEFT | cell::OPEN_BOTTOM,
                cell::OPEN_RIGHT | cell::OPEN_LEFT | cell::OPEN_TOP,
                cell::OPEN_LEFT
            ]
        );

        // ╞═╬╣╠╬╦╡
        assert_eq!(
            cells.array[1],
            [
                cell::OPEN_RIGHT,
                cell::OPEN_RIGHT | cell::OPEN_LEFT,
                cell::OPEN_RIGHT | cell::OPEN_LEFT | cell::OPEN_TOP | cell::OPEN_BOTTOM,
                cell::OPEN_LEFT | cell::OPEN_TOP | cell::OPEN_BOTTOM,
                cell::OPEN_RIGHT | cell::OPEN_TOP | cell::OPEN_BOTTOM,
                cell::OPEN_RIGHT | cell::OPEN_LEFT | cell::OPEN_TOP | cell::OPEN_BOTTOM,
                cell::OPEN_RIGHT | cell::OPEN_LEFT | cell::OPEN_BOTTOM,
                cell::OPEN_LEFT
            ]
        );

        // ██║║╠╣║█
        assert_eq!(
            cells.array[2],
            [
                cell::EMPTY,
                cell::EMPTY,
                cell::OPEN_TOP | cell::OPEN_BOTTOM,
                cell::OPEN_TOP | cell::OPEN_BOTTOM,
                cell::OPEN_TOP | cell::OPEN_BOTTOM | cell::OPEN_RIGHT,
                cell::OPEN_TOP | cell::OPEN_BOTTOM | cell::OPEN_LEFT,
                cell::OPEN_TOP | cell::OPEN_BOTTOM,
                cell::EMPTY
            ]
        );

        // ╞═╩╝╠╣║█
        assert_eq!(
            cells.array[3],
            [
                cell::OPEN_RIGHT,
                cell::OPEN_RIGHT | cell::OPEN_LEFT,
                cell::OPEN_RIGHT | cell::OPEN_LEFT | cell::OPEN_TOP,
                cell::OPEN_LEFT | cell::OPEN_TOP,
                cell::OPEN_TOP | cell::OPEN_BOTTOM | cell::OPEN_RIGHT,
                cell::OPEN_TOP | cell::OPEN_BOTTOM | cell::OPEN_LEFT,
                cell::OPEN_TOP | cell::OPEN_BOTTOM,
                cell::EMPTY
            ]
        );

        // ╔╗╔╗╚╝║█
        assert_eq!(
            cells.array[4],
            [
                cell::OPEN_BOTTOM | cell::OPEN_RIGHT,
                cell::OPEN_BOTTOM | cell::OPEN_LEFT,
                cell::OPEN_BOTTOM | cell::OPEN_RIGHT,
                cell::OPEN_BOTTOM | cell::OPEN_LEFT,
                cell::OPEN_TOP | cell::OPEN_RIGHT,
                cell::OPEN_TOP | cell::OPEN_LEFT,
                cell::OPEN_TOP | cell::OPEN_BOTTOM,
                cell::EMPTY
            ]
        );

        // ╝╚╝╚╗╔╝█
        assert_eq!(
            cells.array[5],
            [
                cell::OPEN_TOP | cell::OPEN_LEFT,
                cell::OPEN_TOP | cell::OPEN_RIGHT,
                cell::OPEN_TOP | cell::OPEN_LEFT,
                cell::OPEN_TOP | cell::OPEN_RIGHT,
                cell::OPEN_BOTTOM | cell::OPEN_LEFT,
                cell::OPEN_BOTTOM | cell::OPEN_RIGHT,
                cell::OPEN_TOP | cell::OPEN_LEFT,
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
                cell::OPEN_TOP | cell::OPEN_RIGHT,
                cell::OPEN_TOP | cell::OPEN_LEFT,
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
                cell::OPEN_BOTTOM | cell::OPEN_RIGHT,
                cell::OPEN_BOTTOM | cell::OPEN_LEFT,
                cell::EMPTY,
                cell::EMPTY,
            ]
        );
    }
}
