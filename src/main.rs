use dotenv::dotenv;
use opencv::core::Vector;
use opencv::imgcodecs::imwrite;
use opencv::prelude::*;
use opencv::videoio;
use opencv::videoio::CAP_V4L2;
use std::env;

fn main() {
    dotenv().ok();
    let camera_index = env::var("CAMERA_INDEX").unwrap_or(String::from("0"));
    let camera_index =
        i32::from_str_radix(&camera_index, 10).expect("CAMERA_INDEX must be an integer");
    let mut cam = videoio::VideoCapture::new(camera_index, CAP_V4L2).unwrap();
    let mut frame = Mat::default();
    for _ in 0..10 {
        cam.read(&mut frame).unwrap();
    }
    let sf = Vector::<i32>::new();
    imwrite("./saved.jpg", &mut frame, &sf).unwrap();
}
