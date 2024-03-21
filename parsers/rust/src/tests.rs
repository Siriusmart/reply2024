use std::collections::HashMap;

use crate::{ReplyInput, ReplyOutput};

#[test]
fn carracing() {
    fn extractor(meta: Vec<i32>) -> (usize, HashMap<String, i32>) {
        (
            meta[1] as usize,
            HashMap::from([("track_length".to_string(), meta[0])]),
        )
    }

    let _input = ReplyInput::load(
        "../test-data/input-carracing-b975.txt".into(),
        Box::new(extractor),
    );
}

#[test]
fn slotmachine() {
    fn extractor(meta: Vec<i32>) -> (usize, HashMap<String, i32>) {
        (
            meta[0] as usize,
            HashMap::from([
                ("final_budget".to_string(), meta[1]),
                ("initial_budget".to_string(), meta[2]),
            ]),
        )
    }

    let _input = ReplyInput::load(
        "../test-data/input-slotmachine-4d99.txt".into(),
        Box::new(extractor),
    );
}

#[test]
fn flowers() {
    fn extractor(meta: Vec<i32>) -> (usize, HashMap<String, i32>) {
        (
            meta[3] as usize,
            HashMap::from([
                ("W".to_string(), meta[0]),
                ("H".to_string(), meta[1]),
                ("F".to_string(), meta[2]),
                ("G".to_string(), meta[3]),
            ]),
        )
    }

    let _input = ReplyInput::load(
        "../test-data/input-flowers-9c96.txt".into(),
        Box::new(extractor),
    );
}

#[test]
fn output() {
    ReplyOutput(vec![4, 12, 565, 1234]).save("../test-data/rs-output.txt".into());
}
