use std::fmt::Debug;

impl PartialEq for Matrix {
	fn eq(&self, other: &Self) -> bool {
		if (*self).get_size() != (*other).get_size() {
			return false;
		}
		for (index_row, row) in self.element_array.iter().enumerate() {
			for (index_col, self_element) in row.iter().enumerate() {
				if (*self_element
					- other
						.get_element(MatrixIndex::new(index_row, index_col))
						.unwrap())
				.abs() > f32::EPSILON
				{
					return false;
				}
			}
		}
		true
	}
}
impl Debug for MatrixError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "MatrixError")
	}
}

impl Debug for MatrixSize {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Row: {}, Col: {}", self.row, self.col)
	}
}

impl Debug for Matrix {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "Size: [{:?}]", self.get_size());
		for row in self.element_array.iter() {
			for element in row.iter() {
				write!(f, "{}, ", element);
			}
			writeln!(f);
		}
		Ok(())
	}
}

pub type MatrixIndex = MatrixSize;
impl MatrixSize {
	pub fn new(row: usize, col: usize) -> MatrixSize {
		Self { row, col }
	}
	fn strictly_less_than(&self, other: &Self) -> bool {
		if self.row < other.row && self.col < other.col {
			return true;
		}
		false
	}
}

pub enum MatrixError {
	ZeroSizeMatrix,
	IncompatibleSizeMultiplication,
	IncompatibleSizeAddition,
	IncompatibleSizeSubtraction,
	IndexOutOfBound,
	NonRectangularElementVector,
	DeterminantOfNonSquareMatrix,
}

#[derive(PartialEq, Clone, Copy)]
pub struct MatrixSize {
	pub row: usize,
	pub col: usize,
}

#[derive(Clone)]
pub(crate) struct Matrix {
	element_array: Vec<Vec<f32>>,
}

impl Matrix {
	pub fn new(size: MatrixSize) -> Result<Self, MatrixError> {
		if size.row == 0 && size.col == 0 {
			return Err(MatrixError::ZeroSizeMatrix);
		}
		Ok(Self {
			element_array: vec![vec![0f32; size.col]; size.row],
		})
	}

	pub fn from_vec(element_array: Vec<Vec<f32>>) -> Result<Self, MatrixError> {
		if element_array.is_empty() || element_array[0].is_empty() {
			return Err(MatrixError::ZeroSizeMatrix);
		}
		let row_size = element_array[0].len();
		for row in element_array.iter() {
			if row.len() != row_size {
				return Err(MatrixError::NonRectangularElementVector);
			}
		}
		Ok(Self { element_array })
	}

	pub fn identity_matrix(size: usize) -> Result<Self, MatrixError> {
		let mut matrix = Matrix::new(MatrixSize::new(size, size))?;
		for index in 0..size {
			matrix.element_array[index][index] = 1f32;
		}
		Ok(matrix)
	}

	pub fn as_vec(&self) -> Vec<Vec<f32>> {
		self.element_array.clone()
	}

	pub fn set_vec(&mut self, element_array: &Vec<Vec<f32>>) {
		self.element_array = element_array.clone();
	}

	pub fn get_size(&self) -> MatrixSize {
		MatrixSize::new(self.element_array.len(), self.element_array[0].len())
	}

	pub fn get_row(&self, index_row: usize) -> Result<Self, MatrixError> {
		if self.get_size().row < index_row {
			return Err(MatrixError::IndexOutOfBound);
		}
		let mut result = Matrix::new(MatrixSize::new(1, self.get_size().col))?;
		for row in result.element_array.iter_mut() {
			for (index_col, result_element) in row.iter_mut().enumerate() {
				*result_element = self.element_array[index_row][index_col];
			}
		}
		Ok(result)
	}

	pub fn get_col(&self, index_col: usize) -> Result<Self, MatrixError> {
		if self.get_size().col < index_col {
			return Err(MatrixError::IndexOutOfBound);
		}
		let mut result = Matrix::new(MatrixSize::new(self.get_size().col, 1))?;
		for (index_row, row) in result.element_array.iter_mut().enumerate() {
			for result_element in row.iter_mut() {
				*result_element = self.element_array[index_row][index_col];
			}
		}
		Ok(result)
	}
	pub fn swizzle(&self, start_index: MatrixIndex, size: MatrixSize) -> Result<Self, MatrixError> {
		let self_size = self.get_size();
		let full_size = start_index + size;
		if self_size.row < full_size.row && self_size.col < full_size.col {
			return Err(MatrixError::IndexOutOfBound);
		}
		let mut result = Matrix::new(size)?;
		for (index_row, row) in result.element_array.iter_mut().enumerate() {
			for (index_col, result_element) in row.iter_mut().enumerate() {
				*result_element =
					self.element_array[start_index.row + index_row][start_index.col + index_col]
			}
		}
		Ok(result)
	}

	pub fn set_element(&mut self, index: MatrixIndex, element: f32) -> Result<(), MatrixError> {
		if index.strictly_less_than(&self.get_size()) {
			self.element_array[index.row][index.col] = element;
			return Ok(());
		}
		Err(MatrixError::IndexOutOfBound)
	}

	pub fn get_element(&self, index: MatrixIndex) -> Result<f32, MatrixError> {
		if index.strictly_less_than(&self.get_size()) {
			return Ok(self.element_array[index.row][index.col]);
		}
		Err(MatrixError::IndexOutOfBound)
	}

	pub fn matrix_mul(&self, other: &Matrix) -> Result<Self, MatrixError> {
		let (self_size, other_size) = (self.get_size(), other.get_size());
		if self_size.col != other_size.row {
			return Err(MatrixError::IncompatibleSizeMultiplication);
		}
		let common_size = self_size.col | other_size.row;
		let mut result = Matrix::new(MatrixSize::new(self_size.row, other_size.col))?;
		for (index_row, row) in result.element_array.iter_mut().enumerate() {
			for (index_col, result_element) in row.iter_mut().enumerate() {
				for index_common in 0..common_size {
					*result_element += self.element_array[index_row][index_common]
						* other.element_array[index_common][index_col];
				}
			}
		}
		Ok(result)
	}

	pub fn transpose(&self) -> Self {
		let self_size = self.get_size();
		let self_size = MatrixSize::new(self_size.col, self_size.row);
		let mut result = Matrix::new(self_size).expect("[ERROR] Failed to transpose");
		for (index_row, row) in result.element_array.iter_mut().enumerate() {
			for (index_col, result_element) in row.iter_mut().enumerate() {
				*result_element = self.element_array[index_row][index_col];
			}
		}
		result
	}
}
