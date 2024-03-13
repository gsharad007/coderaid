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

    impl Type {
        /// Checks if the given direction is open.
        ///
        /// # Examples
        ///
        /// ```
        /// # use your_crate::Type;
        /// # let example = OPEN_NEG_X | OPEN_NEG_Y | OPEN_NEG_Z; // assuming Type has a new() method
        /// assert!(example.is_open(OPEN_NEG_X)); // assuming Type has a North variant
        /// assert!(example.is_open(OPEN_NEG_Y)); // assuming Type has a North variant
        /// assert!(example.is_open(OPEN_NEG_Z)); // assuming Type has a North variant
        /// assert!(!example.is_open(OPEN_POS_X)); // assuming Type has a South variant
        /// assert!(!example.is_open(OPEN_POS_Y)); // assuming Type has a South variant
        /// assert!(!example.is_open(OPEN_POS_X)); // assuming Type has a South variant
        /// ```
        pub fn is_open(self, direction: Self) -> bool {
            self & direction == direction
        }

        /// Checks if the given direction is open.
        ///
        /// # Examples
        ///
        /// ```
        /// # use your_crate::Type;
        /// # let example = OPEN_POS_X | OPEN_POS_Y | OPEN_POS_Z; // assuming Type has a new() method
        /// assert!(example.is_closed(OPEN_NEG_X)); // assuming Type has a North variant
        /// assert!(example.is_closed(OPEN_NEG_Y)); // assuming Type has a North variant
        /// assert!(example.is_closed(OPEN_NEG_Z)); // assuming Type has a North variant
        /// assert!(!example.is_closed(OPEN_POS_X)); // assuming Type has a South variant
        /// assert!(!example.is_closed(OPEN_POS_Y)); // assuming Type has a South variant
        /// assert!(!example.is_closed(OPEN_POS_X)); // assuming Type has a South variant
        /// ```
        pub fn is_closed(self, direction: Self) -> bool {
            !self.is_open(direction)
        }
    }
}

#[derive(Resource, Debug)]
pub struct Cells {
    pub array: Vec<Vec<Vec<cell::Type>>>,
    pub size: IVec3,
}

impl Cells {
    fn new(array: Vec<Vec<Vec<cell::Type>>>, size: IVec3) -> Self {
        Self { array, size }
    }

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_possible_wrap)]
    pub fn from_string(map_string: &str) -> Self {
        let cells = map_string
            .split("\n\n")
            .map(|level| {
                level
                    .lines()
                    .filter(|line| !line.is_empty())
                    .map(|line| line.chars().map(cell_char_to_cell_type).collect_vec())
                    .collect_vec()
            })
            .collect_vec();

        let x = map_max_or_default(&cells, |sub_vec| map_max_or_default(sub_vec, Vec::len));
        let y = map_max_or_default(&cells, Vec::len);
        let z = cells.len();

        assert!(
            x < i32::MAX as usize && y < i32::MAX as usize && z < i32::MAX as usize,
            "Map Cells Too big {x:?} {y:?} {z:?}"
        );
        Self::new(cells, IVec3::new(x as i32, y as i32, z as i32))
    }

    #[allow(clippy::cast_sign_loss)]
    pub fn get(&self, coords: IVec3) -> Option<cell::Type> {
        if coords.x >= 0
            && coords.x < self.size.x
            && coords.y >= 0
            && coords.y < self.size.y
            && coords.z >= 0
            && coords.z < self.size.z
            && (coords.z as usize) < self.array.len()
            && (coords.y as usize) < self.array.get(coords.z as usize)?.len()
            && (coords.x as usize) < self.array.get(coords.z as usize)?.get(coords.y as usize)?.len()
        {
            Some(self.array[coords.z as usize][coords.y as usize][coords.x as usize])
        } else {
            None
        }
    }
}

fn map_max_or_default<T, F>(vec: &[Vec<T>], f: F) -> usize
where
    F: Fn(&Vec<T>) -> usize,
{
    vec.iter().map(f).max().unwrap_or_default()
}

#[allow(clippy::non_ascii_literal)]
fn cell_char_to_cell_type(cell_char: char) -> cell::Type {
    match cell_char {
        // '█' => cell::EMPTY,
        '╨' => cell::OPEN_NEG_Y,
        '╥' => cell::OPEN_POS_Y,
        '╞' => cell::OPEN_POS_X,
        '╡' => cell::OPEN_NEG_X,
        '║' => cell::OPEN_NEG_Y | cell::OPEN_POS_Y,
        '═' => cell::OPEN_NEG_X | cell::OPEN_POS_X,
        '╝' => cell::OPEN_NEG_Y | cell::OPEN_NEG_X,
        '╚' => cell::OPEN_NEG_Y | cell::OPEN_POS_X,
        '╗' => cell::OPEN_POS_Y | cell::OPEN_NEG_X,
        '╔' => cell::OPEN_POS_Y | cell::OPEN_POS_X,
        '╠' => cell::OPEN_NEG_Y | cell::OPEN_POS_Y | cell::OPEN_POS_X,
        '╣' => cell::OPEN_NEG_Y | cell::OPEN_POS_Y | cell::OPEN_NEG_X,
        '╩' => cell::OPEN_NEG_Y | cell::OPEN_NEG_X | cell::OPEN_POS_X,
        '╦' => cell::OPEN_POS_Y | cell::OPEN_NEG_X | cell::OPEN_POS_X,
        '╬' => cell::OPEN_NEG_Y | cell::OPEN_NEG_X | cell::OPEN_POS_X | cell::OPEN_POS_Y,
        _ => cell::EMPTY,
    }
}

#[cfg(test)]
mod test_map_load_string {
    use super::*;

    #[allow(clippy::indexing_slicing)]
    #[test]
    fn test_map_load_string_1x4x4() {
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
            cells.array[0][0],
            [
                cell::OPEN_POS_X,
                cell::OPEN_NEG_X | cell::OPEN_POS_X,
                cell::OPEN_NEG_X | cell::OPEN_POS_X | cell::OPEN_POS_Y,
                cell::OPEN_NEG_X | cell::OPEN_POS_Y
            ]
        );

        // ╞═╬╣
        assert_eq!(
            cells.array[0][1],
            [
                cell::OPEN_POS_X,
                cell::OPEN_NEG_X | cell::OPEN_POS_X,
                cell::OPEN_NEG_X | cell::OPEN_POS_X | cell::OPEN_POS_Y | cell::OPEN_NEG_Y,
                cell::OPEN_NEG_X | cell::OPEN_POS_Y | cell::OPEN_NEG_Y
            ]
        );

        // ██║║
        assert_eq!(
            cells.array[0][2],
            [
                cell::EMPTY,
                cell::EMPTY,
                cell::OPEN_NEG_Y | cell::OPEN_POS_Y,
                cell::OPEN_NEG_Y | cell::OPEN_POS_Y
            ]
        );

        // ╞═╩╝
        assert_eq!(
            cells.array[0][3],
            [
                cell::OPEN_POS_X,
                cell::OPEN_NEG_X | cell::OPEN_POS_X,
                cell::OPEN_NEG_X | cell::OPEN_POS_X | cell::OPEN_NEG_Y,
                cell::OPEN_NEG_X | cell::OPEN_NEG_Y
            ]
        );
    }

    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::too_many_lines)]
    #[test]
    fn test_map_load_string_1x8x8() {
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
            cells.array[0][0],
            [
                cell::OPEN_POS_X,
                cell::OPEN_POS_X | cell::OPEN_NEG_X,
                cell::OPEN_POS_X | cell::OPEN_NEG_X | cell::OPEN_POS_Y,
                cell::OPEN_NEG_X | cell::OPEN_POS_Y,
                cell::OPEN_POS_X | cell::OPEN_POS_Y,
                cell::OPEN_POS_X | cell::OPEN_NEG_X | cell::OPEN_POS_Y,
                cell::OPEN_POS_X | cell::OPEN_NEG_X | cell::OPEN_NEG_Y,
                cell::OPEN_NEG_X
            ]
        );

        // ╞═╬╣╠╬╦╡
        assert_eq!(
            cells.array[0][1],
            [
                cell::OPEN_POS_X,
                cell::OPEN_POS_X | cell::OPEN_NEG_X,
                cell::OPEN_POS_X | cell::OPEN_NEG_X | cell::OPEN_NEG_Y | cell::OPEN_POS_Y,
                cell::OPEN_NEG_X | cell::OPEN_NEG_Y | cell::OPEN_POS_Y,
                cell::OPEN_POS_X | cell::OPEN_NEG_Y | cell::OPEN_POS_Y,
                cell::OPEN_POS_X | cell::OPEN_NEG_X | cell::OPEN_NEG_Y | cell::OPEN_POS_Y,
                cell::OPEN_POS_X | cell::OPEN_NEG_X | cell::OPEN_POS_Y,
                cell::OPEN_NEG_X
            ]
        );

        // ██║║╠╣║█
        assert_eq!(
            cells.array[0][2],
            [
                cell::EMPTY,
                cell::EMPTY,
                cell::OPEN_NEG_Y | cell::OPEN_POS_Y,
                cell::OPEN_NEG_Y | cell::OPEN_POS_Y,
                cell::OPEN_NEG_Y | cell::OPEN_POS_Y | cell::OPEN_POS_X,
                cell::OPEN_NEG_Y | cell::OPEN_POS_Y | cell::OPEN_NEG_X,
                cell::OPEN_NEG_Y | cell::OPEN_POS_Y,
                cell::EMPTY
            ]
        );

        // ╞═╩╝╠╣║█
        assert_eq!(
            cells.array[0][3],
            [
                cell::OPEN_POS_X,
                cell::OPEN_POS_X | cell::OPEN_NEG_X,
                cell::OPEN_POS_X | cell::OPEN_NEG_X | cell::OPEN_NEG_Y,
                cell::OPEN_NEG_X | cell::OPEN_NEG_Y,
                cell::OPEN_NEG_Y | cell::OPEN_POS_Y | cell::OPEN_POS_X,
                cell::OPEN_NEG_Y | cell::OPEN_POS_Y | cell::OPEN_NEG_X,
                cell::OPEN_NEG_Y | cell::OPEN_POS_Y,
                cell::EMPTY
            ]
        );

        // ╔╗╔╗╚╝║█
        assert_eq!(
            cells.array[0][4],
            [
                cell::OPEN_POS_Y | cell::OPEN_POS_X,
                cell::OPEN_POS_Y | cell::OPEN_NEG_X,
                cell::OPEN_POS_Y | cell::OPEN_POS_X,
                cell::OPEN_POS_Y | cell::OPEN_NEG_X,
                cell::OPEN_NEG_Y | cell::OPEN_POS_X,
                cell::OPEN_NEG_Y | cell::OPEN_NEG_X,
                cell::OPEN_NEG_Y | cell::OPEN_POS_Y,
                cell::EMPTY
            ]
        );

        // ╝╚╝╚╗╔╝█
        assert_eq!(
            cells.array[0][5],
            [
                cell::OPEN_NEG_Y | cell::OPEN_NEG_X,
                cell::OPEN_NEG_Y | cell::OPEN_POS_X,
                cell::OPEN_NEG_Y | cell::OPEN_NEG_X,
                cell::OPEN_NEG_Y | cell::OPEN_POS_X,
                cell::OPEN_POS_Y | cell::OPEN_NEG_X,
                cell::OPEN_POS_Y | cell::OPEN_POS_X,
                cell::OPEN_NEG_Y | cell::OPEN_NEG_X,
                cell::EMPTY
            ]
        );

        // ████╚╝██
        assert_eq!(
            cells.array[0][6],
            [
                cell::EMPTY,
                cell::EMPTY,
                cell::EMPTY,
                cell::EMPTY,
                cell::OPEN_NEG_Y | cell::OPEN_POS_X,
                cell::OPEN_NEG_Y | cell::OPEN_NEG_X,
                cell::EMPTY,
                cell::EMPTY,
            ]
        );

        // ████╔╗██
        assert_eq!(
            cells.array[0][7],
            [
                cell::EMPTY,
                cell::EMPTY,
                cell::EMPTY,
                cell::EMPTY,
                cell::OPEN_POS_Y | cell::OPEN_POS_X,
                cell::OPEN_POS_Y | cell::OPEN_NEG_X,
                cell::EMPTY,
                cell::EMPTY,
            ]
        );
    }
}
