#![allow(unused)]
mod matrix;
mod matrix_ops;
use crate::matrix::{Matrix, MatrixIndex, MatrixSize};
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[test]
fn swizzle() {
	let matrix = Matrix::from_vec(vec![
		vec![3f32, 2f32, 1f32],
		vec![1f32, 8f32, 0f32],
		vec![-9f32, 2f32, 5f32],
	])
	.unwrap();
	let swizzle = matrix
		.swizzle(MatrixIndex::new(1, 1), MatrixSize::new(2, 2))
		.unwrap();
	assert_eq!(MatrixSize::new(2, 2), swizzle.get_size());
	assert_eq!(
		Matrix::from_vec(vec![vec![8f32, 0f32], vec![2f32, 5f32]]).unwrap(),
		swizzle
	);
}

#[test]
fn matrix_mul() {
	let matrix1 = Matrix::from_vec(vec![
		vec![3f32, 2f32, 1f32],
		vec![1f32, 8f32, 0f32],
		vec![-9f32, 2f32, 5f32],
	])
	.unwrap();
	let matrix2 = Matrix::from_vec(vec![
		vec![8f32, -1f32, -3f32],
		vec![0f32, -2f32, 3f32],
		vec![-9f32, 2f32, 5f32],
	])
	.unwrap();
	let result_desired = Matrix::from_vec(vec![
		vec![15f32, -5f32, 2f32],
		vec![8f32, -17f32, 21f32],
		vec![-117f32, 15f32, 58f32],
	])
	.unwrap();
	let result = &matrix1 * &matrix2;
	assert_eq!(result, result_desired);

	let matrix1 = Matrix::from_vec(vec![
		vec![3f32, 2f32, 1f32],
		vec![1f32, 8f32, 0f32],
		vec![-9f32, 2f32, 5f32],
		vec![15f32, -5f32, 2f32],
	])
	.unwrap();
	let matrix2 = Matrix::from_vec(vec![
		vec![8f32, -1f32],
		vec![0f32, -2f32],
		vec![-9f32, 2f32],
	])
	.unwrap();
	let result_desired = Matrix::from_vec(vec![
		vec![15f32, -5f32],
		vec![8f32, -17f32],
		vec![-117f32, 15f32],
		vec![102f32, -1f32],
	])
	.unwrap();
	let result = &matrix1 * &matrix2;
	assert_eq!(result, result_desired);
}
