use cv;
use cv::feature::akaze::Akaze;
use ndarray::prelude::*;
use std::error::Error;
use std::iter::FromIterator;

pub fn get_descriptors(image: &str) -> Result<Array2<u8>, Box<dyn Error>> {
    let akaze = Akaze::sparse();
    let (_, bit_descriptors) = akaze.extract_path(image)?;
    let height = bit_descriptors.len();
    let width = bit_descriptors[0].len();

    let descriptors_iter = bit_descriptors
        .iter()
        .flat_map(|a| a.into_iter().map(|i| i.to_owned()));
    let descriptors = Array::from_iter(descriptors_iter).into_shape((height, width))?;
    Ok(descriptors)
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
        assert!(descriptors.shape() == vec![332, 64]);
    }
}
