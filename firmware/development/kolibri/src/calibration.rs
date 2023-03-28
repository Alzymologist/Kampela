#[cfg(not(feature = "std"))]
use alloc::{format, string::String, vec::Vec};

#[cfg(feature = "std")]
use std::{format, string::String, vec::Vec};

use embedded_graphics_core::geometry::Point;
use nalgebra::{linalg::SVD, Affine2, Const, OMatrix, Point2, RowVector1, RowVector3, RowVector6};

use crate::uistate::MeasuredData;

#[derive(Debug)]
pub struct MeasuredAffine(Affine2<f32>);

pub const SET_LEN: usize = 30;

impl MeasuredAffine {
    pub fn new() -> Self {
        let affine_matrix = OMatrix::from_rows(&[
            RowVector3::<f32>::new(1.0, 0.0, 0.0),
            RowVector3::<f32>::new(0.0, 1.0, 0.0),
            RowVector3::<f32>::new(0.0, 0.0, 1.0),
        ]);
        Self(Affine2::from_matrix_unchecked(affine_matrix))
    }
    pub fn from_data(point_data_set: &[MeasuredData; SET_LEN]) -> Result<Self, &'static str> {
        let touch_data_matrix = touch_data_matrix(point_data_set);
        let svd = SVD::new_unordered(touch_data_matrix, true, true);
        let display_data_vector = display_data_vector(point_data_set);
        let solved_matrix = svd.solve(&display_data_vector, 0.0)?;
        let affine_matrix = OMatrix::from_rows(&[
            RowVector3::<f32>::new(
                solved_matrix[(0, 0)],
                solved_matrix[(1, 0)],
                solved_matrix[(2, 0)],
            ),
            RowVector3::<f32>::new(
                solved_matrix[(3, 0)],
                solved_matrix[(4, 0)],
                solved_matrix[(5, 0)],
            ),
            RowVector3::<f32>::new(0.0, 0.0, 1.0),
        ]);
        Ok(Self(Affine2::from_matrix_unchecked(affine_matrix)))
    }
    pub fn transform(&self, touch: &Point) -> Point {
        let touch_as_point2 = Point2::new(touch.x as f32, touch.y as f32);
        let display_as_point2 = self.0.transform_point(&touch_as_point2);
        Point {
            x: display_as_point2.coords[0] as i32,
            y: display_as_point2.coords[1] as i32,
        }
    }
    pub fn show(&self) -> String {
        format!("{:0>1.4}\t{:0>1.4}\t{:0>3.4}\n{:0>1.4}\t{:0>1.4}\t{:0>3.4}\n{:0>1.4}\t{:0>1.4}\t{:0>3.4}\n", self.0[(0,0)], self.0[(0,1)], self.0[(0,2)], self.0[(1,0)], self.0[(1,1)], self.0[(1,2)], self.0[(2,0)], self.0[(2,1)], self.0[(2,2)])
    }
}

impl Default for MeasuredAffine {
    fn default() -> Self {
        Self::new()
    }
}

pub fn touch_data_matrix(
    point_data_set: &[MeasuredData; SET_LEN],
) -> OMatrix<f32, Const<{ 2 * SET_LEN }>, Const<6>> {
    let mut rows: Vec<RowVector6<f32>> = Vec::with_capacity(2 * SET_LEN);
    for point_data in point_data_set.iter() {
        let x = point_data.touch_point.x as f32;
        let y = point_data.touch_point.y as f32;
        rows.push(RowVector6::<f32>::new(x, y, 1.0, 0.0, 0.0, 0.0));
        rows.push(RowVector6::<f32>::new(0.0, 0.0, 0.0, x, y, 1.0));
    }
    OMatrix::from_rows(&rows)
}

pub fn display_data_vector(
    point_data_set: &[MeasuredData; SET_LEN],
) -> OMatrix<f32, Const<{ 2 * SET_LEN }>, Const<1>> {
    let mut rows: Vec<RowVector1<f32>> = Vec::with_capacity(2 * SET_LEN);
    for point_data in point_data_set.iter() {
        let x = point_data.display_point.x as f32;
        let y = point_data.display_point.y as f32;
        rows.push(RowVector1::<f32>::new(x));
        rows.push(RowVector1::<f32>::new(y));
    }
    OMatrix::from_rows(&rows)
}
