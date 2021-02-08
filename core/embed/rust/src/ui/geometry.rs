use core::{
    convert::{TryFrom, TryInto},
    ops::{Add, Sub},
};

use crate::error::Error;
use crate::micropython::{map::Map, qstr::Qstr};
use crate::trezorhal::display;

/// Relative offset in 2D space.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Offset {
    pub x: i32,
    pub y: i32,
}

impl Offset {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn abs(self) -> Self {
        Self::new(self.x.abs(), self.y.abs())
    }
}

/// A point in 2D space defined by the the `x` and `y` coordinate.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn midpoint(self, rhs: Self) -> Self {
        Self::new((self.x + rhs.x) / 2, (self.y + rhs.y) / 2)
    }
}

impl Add<Offset> for Point {
    type Output = Point;

    fn add(self, rhs: Offset) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub<Offset> for Point {
    type Output = Point;

    fn sub(self, rhs: Offset) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Sub<Point> for Point {
    type Output = Offset;

    fn sub(self, rhs: Point) -> Self::Output {
        Offset::new(self.x - rhs.x, self.y - rhs.y)
    }
}

/// A rectangle in 2D space defined by the top-left point `x0`,`y0` and the
/// bottom-right point `x1`,`y1`.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Rect {
    pub x0: i32,
    pub y0: i32,
    pub x1: i32,
    pub y1: i32,
}

impl Rect {
    pub fn new(p0: Point, p1: Point) -> Self {
        Self {
            x0: p0.x,
            y0: p0.y,
            x1: p1.x,
            y1: p1.y,
        }
    }

    pub fn screen() -> Self {
        Self {
            x0: 0,
            y0: 0,
            x1: display::width(),
            y1: display::height(),
        }
    }

    pub fn width(&self) -> i32 {
        self.x1 - self.x0
    }

    pub fn height(&self) -> i32 {
        self.y1 - self.y0
    }

    pub fn top_left(&self) -> Point {
        Point::new(self.x0, self.y0)
    }

    pub fn top_right(&self) -> Point {
        Point::new(self.x1, self.y0)
    }

    pub fn bottom_left(&self) -> Point {
        Point::new(self.x0, self.y1)
    }

    pub fn bottom_right(&self) -> Point {
        Point::new(self.x1, self.y1)
    }

    pub fn midpoint(&self) -> Point {
        self.top_left().midpoint(self.bottom_right())
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.x0 && point.x < self.x1 && point.y >= self.y0 && point.y < self.y1
    }

    pub fn inset(&self, uniform: i32) -> Self {
        Self {
            x0: self.x0 + uniform,
            y0: self.y0 + uniform,
            x1: self.x1 - uniform,
            y1: self.y1 - uniform,
        }
    }
}

impl TryFrom<&Map> for Rect {
    type Error = Error;

    fn try_from(map: &Map) -> Result<Self, Self::Error> {
        Ok(Self {
            x0: map.get_qstr(Qstr::MP_QSTR_x0)?.try_into()?,
            y0: map.get_qstr(Qstr::MP_QSTR_y0)?.try_into()?,
            x1: map.get_qstr(Qstr::MP_QSTR_x1)?.try_into()?,
            y1: map.get_qstr(Qstr::MP_QSTR_y1)?.try_into()?,
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Align {
    Left,
    Right,
    Center,
}

pub struct Grid {
    /// Number of rows (cells on the y-axis) in the grid.
    pub rows: usize,
    /// Number of columns (cells on the x-axis) in the grid.
    pub cols: usize,
    /// Padding between cells.
    pub spacing: i32,
    /// Total area covered by this grid.
    pub area: Rect,
}

impl Grid {
    pub fn new(area: Rect, rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            spacing: 0,
            area,
        }
    }

    pub fn screen(rows: usize, cols: usize) -> Self {
        Self::new(Rect::screen(), rows, cols)
    }

    pub fn row_col(&self, row: usize, col: usize) -> Rect {
        let cell_width = self.area.width() / self.cols as i32;
        let cell_height = self.area.height() / self.rows as i32;
        let x = col as i32 * cell_width;
        let y = row as i32 * cell_width;
        Rect {
            x0: self.area.x0 + x,
            y0: self.area.y0 + y,
            x1: self.area.x0 + x + (cell_width - self.spacing),
            y1: self.area.y0 + y + (cell_height - self.spacing),
        }
    }

    pub fn cell(&self, index: usize) -> Rect {
        self.row_col(index / self.rows, index % self.cols)
    }
}