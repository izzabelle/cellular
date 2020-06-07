use offbrand::{color::Color, Context};

pub struct World {
    pub cells: Vec<Cell>,
    pub width: usize,
    pub height: usize,
}

impl std::ops::Index<(usize, usize)> for World {
    type Output = Cell;

    fn index(&self, index: (usize, usize)) -> &Cell {
        &self.cells[index.1 * self.width + index.0]
    }
}

impl std::ops::IndexMut<(usize, usize)> for World {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Cell {
        &mut self.cells[index.1 * self.width + index.0]
    }
}

impl World {
    pub fn new(width: usize, height: usize) -> World {
        let cells: Vec<Cell> = vec![Cell::new(CellKind::Nothing); width * height];
        World { cells, width, height: height }
    }

    pub fn insert(&mut self, x: usize, y: usize, cell: Cell) {
        self[(x, y)] = cell;
    }

    pub fn update(&mut self) {
        for x in 0..(self.width - 1) {
            for y in 0..(self.height - 1) {
                let mut cell = self[(x, y)];

                if cell.modified {
                    continue;
                }

                match cell.kind {
                    CellKind::Nothing => {}
                    CellKind::Solid { color: _color } => match self[(x, y + 1)].kind {
                        CellKind::Nothing => {
                            cell.modified = true;
                            self[(x, y + 1)] = cell;
                            self[(x, y)] = Cell::new(CellKind::Nothing);
                        }
                        CellKind::Solid { color: _color } => {
                            if self[(x - 1, y + 1)].kind == CellKind::Nothing {
                                cell.modified = true;
                                self[(x - 1, y + 1)] = cell;
                                self[(x, y)] = Cell::new(CellKind::Nothing);
                            } else if self[(x + 1, y + 1)].kind == CellKind::Nothing {
                                cell.modified = true;
                                self[(x + 1, y + 1)] = cell;
                                self[(x, y)] = Cell::new(CellKind::Nothing);
                            }
                        }
                    },
                }
            }
        }

        self.cells.iter_mut().for_each(|cell| {
            if cell.modified {
                cell.modified = false;
            }
        });
    }

    pub fn render(&self, ctx: &mut Context, scale: usize) {
        for x in 0..self.width {
            for y in 0..self.height {
                let cell = self[(x, y)];

                match cell.kind {
                    CellKind::Nothing => {}
                    CellKind::Solid { color } => {
                        for i in 0..scale {
                            for j in 0..scale {
                                ctx.set_pixel(x * scale + i, y * scale + j, color);
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub kind: CellKind,
    pub modified: bool,
}

impl Cell {
    pub fn new(kind: CellKind) -> Cell {
        Cell { kind, modified: false }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellKind {
    Nothing,
    Solid { color: Color },
}
