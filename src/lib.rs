use rand::{Rng, thread_rng};
use wasm_bindgen::prelude::*;

mod utils;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
	Dead = 0,
	Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
	width: u32,
	height: u32,
	cells: Vec<Cell>,
}

#[wasm_bindgen]
#[allow(clippy::must_use_candidate)]
impl Universe {
	fn get_index(&self, row: u32, column: u32) -> usize {
		(row * self.width + column) as usize
	}

	fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
		let mut count = 0;
		for delta_row in [self.height - 1, 0, 1].iter().copied() {
			for delta_column in [self.width - 1, 0, 1].iter().copied() {
				if delta_row == 0 && delta_column == 0 {
					continue;
				}

				let neighbor_row = (row + delta_row) % self.height;
				let neighbor_column = (column + delta_column) % self.width;
				let index = self.get_index(neighbor_row, neighbor_column);
				count += self.cells[index] as u8;
			}
		}
		count
	}

	pub fn tick(&mut self) {
		let mut next = self.cells.clone();

		for row in 0..self.height {
			for column in 0..self.width {
				let index = self.get_index(row, column);
				let cell = self.cells[index];
				let live_neighbors = self.live_neighbor_count(row, column);

				let next_cell = match (cell, live_neighbors) {
					// Rule 1: Any live cell with fewer than two live neighbors
					// dies, as if caused by underpopulation.
					(Cell::Alive, x) if x < 2 => Cell::Dead,
					// Rule 2: Any live cell with two or three live neighbors
					// lives on to the next generation.
					#[allow(clippy::match_same_arms)]
					(Cell::Alive, 2 | 3) => Cell::Alive,
					// Rule 3: Any live cell with more than three live
					// neighbors dies, as if by overpopulation.
					(Cell::Alive, x) if x > 3 => Cell::Dead,
					// Rule 4: Any dead cell with exactly three live neighbors
					// becomes a live cell, as if by reproduction.
					(Cell::Dead, 3) => Cell::Alive,
					// All other cells remain in the same state.
					(otherwise, _) => otherwise,
				};

				next[index] = next_cell;
			}
		}

		self.cells = next;
	}

	pub fn new() -> Universe {
		Universe::default()
	}

	pub fn random() -> Universe {
		let width = Universe::default().width;
		let height = Universe::default().height;

		let mut random = thread_rng();

		let cells = (0..width * height)
			.map(|_| {
				if random.gen_bool(1.0 / 2.0) {
					Cell::Alive
				} else {
					Cell::Dead
				}
			})
			.collect();

		Universe {
			width,
			height,
			cells,
		}
	}

	pub fn render_string(&self) -> String {
		self.to_string()
	}

	pub fn width(&self) -> u32 {
		self.width
	}

	pub fn height(&self) -> u32 {
		self.height
	}

	pub fn cells(&self) -> *const Cell {
		self.cells.as_ptr()
	}
}

impl Default for Universe {
	fn default() -> Self {
		let width = 160;
		let height = 90;

		let cells = (0..width * height)
			.map(|i| {
				if i % 2 == 0 || i % 7 == 0 {
					Cell::Alive
				} else {
					Cell::Dead
				}
			})
			.collect();

		Universe {
			width,
			height,
			cells,
		}
	}
}

impl std::fmt::Display for Universe {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		for line in self.cells.as_slice().chunks(self.width as usize) {
			for &cell in line {
				let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
				write!(f, "{symbol}")?;
			}
			writeln!(f)?;
		}

		Ok(())
	}
}