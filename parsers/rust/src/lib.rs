use std::{collections::HashMap, fmt::Display, fs, io::Write, path::PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplyInput {
    pub cases: Vec<ReplyCase>,
}

impl ReplyInput {
    pub fn load(
        file: PathBuf,
        meta_parser: Box<dyn Fn(Vec<i32>) -> (usize, HashMap<String, i32>)>,
    ) -> Self {
        let content = fs::read_to_string(file).expect("could not read file");
        let mut content = content.split('\n');

        let cases: usize = content
            .next()
            .expect("there are 0 lines in the file")
            .parse()
            .expect("first line is not contain number");

        let mut out = Vec::with_capacity(cases);

        for _ in 0..cases {
            let (case_len, meta) = meta_parser(
                content
                    .next()
                    .expect("reached EOF while there are still remaining cases")
                    .split_whitespace()
                    .map(|s| {
                        s.parse()
                            .expect(format!("could not parse {s} to number").as_str())
                    })
                    .collect(),
            );
            let mut data = Vec::with_capacity(case_len);

            (0..case_len).for_each(|_| {
                data.push(
                    content
                        .next()
                        .expect("reacher EOF while there are still remaining data entries")
                        .split_whitespace()
                        .map(str::to_string)
                        .collect::<Vec<_>>(),
                )
            });

            if data.len() != case_len {
                panic!("expect {case_len} entries, got {}", data.len())
            }

            out.push(ReplyCase::new(meta, data));
        }

        if out.len() != cases {
            panic!("expect {cases} cases, got {}", out.len())
        }

        ReplyInput { cases: out }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplyCase {
    pub meta: HashMap<String, i32>,
    pub data: Vec<Vec<String>>,
}

impl ReplyCase {
    pub fn new(meta: HashMap<String, i32>, data: Vec<Vec<String>>) -> Self {
        Self { meta, data }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplyOutput<T: Display>(pub Vec<T>);

impl<T: Display> ReplyOutput<T> {
    pub fn save(&self, path: PathBuf) {
        let content = self
            .0
            .iter()
            .enumerate()
            .map(|(num, ans)| format!("Case #{}: {ans}", num + 1))
            .collect::<Vec<_>>()
            .join("\n");

        let mut file = fs::OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(path)
            .expect("could not open file");

        file.write_all(content.as_bytes())
            .expect("could not write to file");
    }
}

#[cfg(test)]
mod tests;
