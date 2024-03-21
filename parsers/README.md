# Reply Input Parsers

Input parsers and output handlers for Reply Code Challenge.

What this is:

- A generic input parser - parse input to chunks of strings.
- Customisable behaviour, can be used for all sorts of inputs.
- Tested against the first 3 examples of Reply Code Challenge 2023.

What this is not:

- Parser specific to a question: you might have to further convert the parsed input in order for it to be useful.
- "Parse-anything" parser, for example it cannot parse question 4 and 5 in the 2023 examples, as they have different formats.

> For those files to be parsed, you might want to modify the source, or write your own parser.

## Usage

Check test cases for examples:

| Language   | Examples                                                                                                              |
| ---------- | --------------------------------------------------------------------------------------------------------------------- |
| Rust       | [`/rust/src/tests.rs`](https://github.com/Siriusmart/reply-parsers/blob/master/rust/src/tests.rs)                     |
| Java       | [`/java/src/main/java/App.java`](https://github.com/Siriusmart/reply-parsers/blob/master/java/src/main/java/App.java) |
| Javascript | [`/nodejs/tests.js`](https://github.com/Siriusmart/reply-parsers/blob/master/nodejs/tests.js)                         |

### Rust

```rs
// import the structs
use crate::{ReplyInput, ReplyOutput};

// to read from an input file
// first define the extractor function
// the extractor function extracts information from the case metadata string

// for example `/test-data/input-carracing-b957.txt` line 2:
//   "966 25" means track_length=996
//   and the 25 following lines will be from this case

// input: vector of numbers
// returns: (number of following rows, hashmap of other details)
fn extractor(meta: Vec<i32>) -> (usize, HashMap<String, i32>) {
    (
        meta[1] as usize,
        HashMap::from([("track_length".to_string(), meta[0])]),
    )
}

// now use `ReplyInput::load` to load the file in
// check code or use IDE linting to see how cases can be accessed.
let _input = ReplyInput::load(
    "../test-data/input-carracing-b975.txt".into(),
    Box::new(extractor),
);

// Outputting is a lot simpler,
// all you need is a vector of numbers
ReplyOutput(vec![4, 12, 565, 1234]).save("../test-data/rs-output.txt".into());
```

### Java

```java
// to read from an input file
// first define the extractor function
// the extractor function extracts information from the case metadata string

// for example `/test-data/input-carracing-b957.txt` line 2:
//   "966 25" means track_length=996
//   and the 25 following lines will be from this case

// input: arraylist of numbers
// returns: ExtractorOutput(number of following rows, hashmap of other details)
Function<ArrayList<Integer>, ExtractorOutput> extractor = meta -> {
    HashMap<String, Integer> parsed = new HashMap();
    parsed.put("track_length", meta.get(0));
    return new ExtractorOutput(meta.get(1), parsed);
};

// now use `ReplyInput` to load the file in
// check code or use IDE linting to see how cases can be accessed.
ReplyInput _input = new ReplyInput("../test-data/input-carracing-b975.txt", extractor);

// Outputting is a lot simpler,
// all you need is an array of numbers
ReplyOutput output = new ReplyOutput(new Object[] {1, 23, 456, 567});
output.save("../test-data/java-output.txt");
```

### Javascript

```js
// to read from an input file
// first define the extractor function
// the extractor function extracts information from the case metadata string

// for example `/test-data/input-carracing-b957.txt` line 2:
//   "966 25" means track_length=996
//   and the 25 following lines will be from this case

// input: array of numbers
// returns: [number of following rows, hashmap of other details]
function extractor(meta) {
    return [meta[1], { track_length: meta[0] }];
}

// now use `ReplyInput` to load the file in
// check code to see how cases can be accessed.
ReplyInput _input = new ReplyInput("../test-data/input-carracing-b975.txt", extractor);
let _input = new ReplyInput(
    "../test-data/input-carracing-b975.txt",
    extractor,
);

// Outputting is a lot simpler,
// all you need is an array of numbers
let output = new ReplyOutput([1, 25, 345, 45656, 15]);
output.save("../test-data/js-output.txt");
```
