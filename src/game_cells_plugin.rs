

use bevy::prelude::*;


use itertools::Itertools;

pub mod cell {
    // #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, BitOr)]
    // pub struct Type(u8);
    pub type Type = u8;

    pub const EMPTY: Type = 0b0;
    pub const OPEN_TOP: Type = 0b1;
    pub const OPEN_BOTTOM: Type = 0b10;
    pub const OPEN_LEFT: Type = 0b100;
    pub const OPEN_RIGHT: Type = 0b1000;
}

#[derive(Component, Debug)]
pub struct Cells {
    pub cells: Vec<Vec<cell::Type>>,
}

impl Cells {
    #[allow(clippy::non_ascii_literal)]
    pub fn from_string(map_string: &str) -> Self {
        let cells = map_string
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .map(|c| -> cell::Type {
                        match c {
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
                            '╬' => {
                                cell::OPEN_TOP
                                    | cell::OPEN_LEFT
                                    | cell::OPEN_RIGHT
                                    | cell::OPEN_BOTTOM
                            }
                            _ => cell::EMPTY,
                        }
                    })
                    .collect_vec()
            })
            .collect_vec();

        Self { cells }
    }
}

#[cfg(test)]
mod test_map_load_string {
    use super::*;

    #[test]
    fn test_map_load_string_4x4() {
        #[allow(clippy::non_ascii_literal)]
        let map_string = "
╞═╦╗
╞═╬╣
██║║
╞═╩╝
";

        let map = Cells::from_string(map_string);

        // println!("{}", map_string);
        // println!("{:?}", map.cells);

        assert_eq!(map.cells[0][0], cell::OPEN_RIGHT);
        assert_eq!(map.cells[0][1], cell::OPEN_LEFT | cell::OPEN_RIGHT);
        assert_eq!(
            map.cells[0][2],
            cell::OPEN_LEFT | cell::OPEN_RIGHT | cell::OPEN_BOTTOM
        );
        assert_eq!(map.cells[0][3], cell::OPEN_LEFT | cell::OPEN_BOTTOM);

        assert_eq!(map.cells[1][0], cell::OPEN_RIGHT);
        assert_eq!(map.cells[1][1], cell::OPEN_LEFT | cell::OPEN_RIGHT);
        assert_eq!(
            map.cells[1][2],
            cell::OPEN_LEFT | cell::OPEN_RIGHT | cell::OPEN_BOTTOM | cell::OPEN_TOP
        );
        assert_eq!(
            map.cells[1][3],
            cell::OPEN_LEFT | cell::OPEN_BOTTOM | cell::OPEN_TOP
        );

        assert_eq!(map.cells[2][0], cell::EMPTY);
        assert_eq!(map.cells[2][1], cell::EMPTY);
        assert_eq!(map.cells[2][2], cell::OPEN_TOP | cell::OPEN_BOTTOM);
        assert_eq!(map.cells[2][3], cell::OPEN_TOP | cell::OPEN_BOTTOM);

        assert_eq!(map.cells[3][0], cell::OPEN_RIGHT);
        assert_eq!(map.cells[3][1], cell::OPEN_LEFT | cell::OPEN_RIGHT);
        assert_eq!(
            map.cells[3][2],
            cell::OPEN_LEFT | cell::OPEN_RIGHT | cell::OPEN_TOP
        );
        assert_eq!(map.cells[3][3], cell::OPEN_LEFT | cell::OPEN_TOP);
    }

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

        let map = Cells::from_string(map_string);

        // println!("{}", map_string);
        // println!("{:?}", map.cells);

        // ╞═╦╗╔╦╩╡
        assert_eq!(
            map.cells[0],
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
            map.cells[1],
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
            map.cells[2],
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
            map.cells[3],
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
            map.cells[4],
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
            map.cells[5],
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
            map.cells[6],
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
            map.cells[7],
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
