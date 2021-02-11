use cyclops;

fn main() {
    let img = cyclops::load_img("obama.jpg").unwrap();
    let descriptors = cyclops::get_descriptors(&img);
    println!("{:?}", descriptors)
}
