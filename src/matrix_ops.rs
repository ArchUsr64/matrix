use crate::matrix::*;
use std::ops::{Add, Div, Mul, Neg, Sub};

impl Neg for Matrix {
	type Output = Matrix;
	fn neg(self) -> Self {
		self.scalar_mul(-1f32).unwrap()
	}
}

impl Add<Self> for &Matrix {
	type Output = Matrix;
	fn add(self, rhs: Self) -> Matrix {
		self.matrix_add(rhs).unwrap()
	}
}

impl Add<f32> for &Matrix {
	type Output = Matrix;
	fn add(self, rhs: f32) -> Matrix {
		self.scalar_add(rhs).unwrap()
	}
}

impl Sub<Self> for &Matrix {
	type Output = Matrix;
	fn sub(self, rhs: Self) -> Matrix {
		self.matrix_sub(rhs).unwrap()
	}
}

impl Sub<f32> for &Matrix {
	type Output = Matrix;
	fn sub(self, rhs: f32) -> Matrix {
		self.scalar_sub(rhs).unwrap()
	}
}

impl Mul<Self> for &Matrix {
	type Output = Matrix;
	fn mul(self, rhs: Self) -> Matrix {
		self.matrix_mul(rhs).unwrap()
	}
}

impl Mul<f32> for &Matrix {
	type Output = Matrix;
	fn mul(self, rhs: f32) -> Matrix {
		self.scalar_mul(rhs).unwrap()
	}
}

impl Div<f32> for &Matrix {
	type Output = Matrix;
	fn div(self, rhs: f32) -> Matrix {
		self.scalar_div(rhs).unwrap()
	}
}

impl Add<Self> for MatrixSize {
	type Output = Self;
	fn add(self, rhs: Self) -> Self {
		Self {
			row: self.row + rhs.row,
			col: self.col + rhs.col,
		}
	}
}
impl Matrix {
	fn scalar_div(&self, other: f32) -> Result<Self, MatrixError> {
		let mut result = Matrix::new(self.get_size())?;
		for index_row in 0..result.get_size().row {
			for index_col in 0..result.get_size().col {
				result.set_element(
					MatrixSize::new(index_row, index_col),
					self.get_element(MatrixSize::new(index_row, index_col))? / other,
				)?
			}
		}
		Ok(result)
	}

	fn scalar_mul(&self, other: f32) -> Result<Self, MatrixError> {
		let mut result = Matrix::new(self.get_size())?;
		for index_row in 0..result.get_size().row {
			for index_col in 0..result.get_size().col {
				result.set_element(
					MatrixSize::new(index_row, index_col),
					self.get_element(MatrixSize::new(index_row, index_col))? * other,
				)?
			}
		}
		Ok(result)
	}

	fn scalar_add(&self, other: f32) -> Result<Self, MatrixError> {
		let mut result = Matrix::new(self.get_size())?;
		for index_row in 0..result.get_size().row {
			for index_col in 0..result.get_size().col {
				result.set_element(
					MatrixSize::new(index_row, index_col),
					self.get_element(MatrixSize::new(index_row, index_col))? + other,
				)?
			}
		}
		Ok(result)
	}

	fn scalar_sub(&self, other: f32) -> Result<Self, MatrixError> {
		let mut result = Matrix::new(self.get_size())?;
		for index_row in 0..result.get_size().row {
			for index_col in 0..result.get_size().col {
				result.set_element(
					MatrixSize::new(index_row, index_col),
					self.get_element(MatrixSize::new(index_row, index_col))? - other,
				)?
			}
		}
		Ok(result)
	}

	fn matrix_sub(&self, other: &Matrix) -> Result<Self, MatrixError> {
		if self.get_size() != other.get_size() {
			return Err(MatrixError::IncompatibleSizeAddition);
		}
		let mut result = Matrix::new(self.get_size())?;
		for index_row in 0..result.get_size().row {
			for index_col in 0..result.get_size().col {
				result.set_element(
					MatrixSize::new(index_row, index_col),
					self.get_element(MatrixSize::new(index_row, index_col))?
						- other.get_element(MatrixSize::new(index_row, index_col))?,
				)?
			}
		}
		Ok(result)
	}

	fn matrix_add(&self, other: &Matrix) -> Result<Self, MatrixError> {
		if self.get_size() != other.get_size() {
			return Err(MatrixError::IncompatibleSizeAddition);
		}
		let mut result = Matrix::new(self.get_size())?;
		for index_row in 0..result.get_size().row {
			for index_col in 0..result.get_size().col {
				result.set_element(
					MatrixSize::new(index_row, index_col),
					self.get_element(MatrixSize::new(index_row, index_col))?
						+ other.get_element(MatrixSize::new(index_row, index_col))?,
				)?
			}
		}
		Ok(result)
	}
}
