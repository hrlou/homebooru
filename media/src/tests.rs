use std::{fs, path::Path};
use image::GenericImage;

use crate::video;

static TEST_VIDEO: &str = "./tests/test.mp4";
static TEST_IMAGE: &str = "./tests/test.jpg";
static OUTPUT_DIR: &str = "./tests/output";

#[test]
fn setup() {
    video::init().unwrap();

    let path = Path::new(TEST_VIDEO);
    assert_eq!(path.exists(), true, "Test video \'{:?}\' does not exist", path);
    let output = Path::new(OUTPUT_DIR);
    let r = match output.exists() {
        true => fs::remove_dir_all(output),
        false => fs::create_dir_all(output),
    };
    r.expect("failed to handle output directory");
}

#[test]
fn dump_frames() {
    let mut video = video::Video::new(TEST_VIDEO).expect("failed to get stream from test video");
    video.setup_stream(None).unwrap();

    let dump = &Path::new(OUTPUT_DIR).join("frame_dump");
    fs::create_dir_all(dump).expect("failed to create dump path");

    video.process_frames(|frame, index| {
        let img = video::frame::image(frame).unwrap();
        let path = dump.join(format!("frame_{:03}.jpg", index));
        img.save(path).expect("failed to save frame");
        true
    }).expect("failed to process frames");
}

#[test]
fn get_frame() {
    let mut video = video::Video::new(TEST_VIDEO).expect("failed to get stream from test video");
    let select: usize = 10;
    let mut img: video::RgbBuffer = video::RgbBuffer::default();

    video.setup_stream(None).unwrap();
    video.process_frames(|frame, index| {
        if index == select {
            img = video::frame::image(frame).unwrap();
            return false;
        }
        true
    }).unwrap();

    let path = &Path::new(OUTPUT_DIR).join("select_frame.png");
    img.save(path).expect("failed to save selected frame");
}