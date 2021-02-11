use ndarray::prelude::*;
use ndarray::Array2;
use std::iter::FromIterator;

use opencv::{
    core::{no_array, KeyPoint, Vector},
    features2d::SIFT,
    imgcodecs::{imread, IMREAD_COLOR},
    prelude::*,
};

pub struct Image {
    mat: Mat,
}

impl Image {
    fn new(data: Mat) -> Self {
        Image { mat: data }
    }
}

impl From<Image> for Array2<i32> {
    fn from(image: Image) -> Array2<i32> {
        mat2d_to_array(image.mat)
    }
}

pub fn load_img(img_file: &str) -> Result<Image, opencv::Error> {
    Ok(Image::new(imread(img_file, IMREAD_COLOR)?))
}

fn mat2d_to_array(mat: Mat) -> Array<i32, Ix2> {
    let num_rows = mat.rows() as i32;
    let num_cols = mat.cols() as i32;
    let rows = (0..num_rows)
        .into_iter()
        .flat_map(|i| mat.at_row(i).unwrap().to_owned())
        .map(|n: f32| n as i32);
    Array::from_iter(rows)
        .into_shape((num_rows as usize, num_cols as usize))
        .unwrap()
}

pub fn get_descriptors(img: &Image) -> Array<i32, Ix2> {
    let mut sift = SIFT::create(5, 3, 0.04, 10.0, 1.6).unwrap();
    let mut mat_descriptors = Mat::default().unwrap();
    let mut kps: Vector<KeyPoint> = Vector::new();
    let mask = &no_array().unwrap();
    sift.detect_and_compute(&img.mat, mask, &mut kps, &mut mat_descriptors, false)
        .unwrap();

    mat2d_to_array(mat_descriptors)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let img = load_img("obama.jpg").unwrap();
        get_descriptors(&img);
    }
}
