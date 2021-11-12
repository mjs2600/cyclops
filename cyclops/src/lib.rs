use ndarray::prelude::*;
use opencv::core::{no_array, KeyPoint, Vector};
use opencv::features2d::SIFT;
use opencv::imgcodecs::{imread, IMREAD_COLOR};
use opencv::prelude::*;
use std::error::Error;

pub fn get_descriptors(image_file: &str) -> Result<Array2<u8>, Box<dyn Error>> {
    let mut sift = SIFT::create(10, 3, 0.04, 10.0, 1.6)?;
    let image = imread(&image_file, IMREAD_COLOR)?;
    let mut keypoints: Vector<KeyPoint> = Vector::new();
    let mut descriptors: Mat = Mat::default();
    sift.detect_and_compute(&image, &no_array(), &mut keypoints, &mut descriptors, false)?;
    let descriptors_iter = (0..descriptors.rows())
        .flat_map(|row| descriptors.at_row(row).expect("bad row").to_owned())
        .map(|el: f32| el as u8);

    Ok(Array::from_iter(descriptors_iter)
        .into_shape((descriptors.rows() as usize, descriptors.cols() as usize))?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn obama_picture() -> String {
        let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        crate_dir
            .join("tests")
            .join("obama.jpg")
            .to_str()
            .unwrap()
            .to_owned()
    }
    #[test]
    fn it_works() {
        let descriptors = get_descriptors(&obama_picture()).unwrap();
        println!("{:?}", descriptors);
        assert!(descriptors.shape() == vec![10, 128]);
    }
}
