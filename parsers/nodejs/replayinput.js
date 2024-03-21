class ReplyInput {
    // file: String (Path)
    // meta_parser: Function(Number[]) -> [Number, Obj[String] = Number]
    constructor(file, meta_parser) {
        const fs = require("fs");
        let content = fs
            .readFileSync(file, {
                encoding: "utf8",
                flag: "r",
            })
            .split("\n")
            .reverse();

        let cases = parseInt(content.pop());

        let out = [];

        for (let case_no = 0; case_no < cases; case_no++) {
            if (content.length === 0)
                throw new Error(
                    "reached EOF while there are still remaining cases",
                );

            let [case_len, meta] = meta_parser(
                content
                    .pop()
                    .split(" ")
                    .map((s) => parseInt(s)),
            );

            let data = [];

            for (let case_len_no = 0; case_len_no < case_len; case_len_no++) {
                if (content.length === 0)
                    throw new Error(
                        "reached EOF while there are still remaining entries",
                    );
                data.push(content.pop().split(" "));
            }

            if (data.length !== case_len)
                throw new Error(
                    `expect ${case_len} entries, got ${data.length}`,
                );

            out.push(new ReplyCase(meta, data));
        }

        if (out.length !== cases)
            throw new Error(`expect ${cases} cases, got ${out.length}`);

        this.cases = out;
    }
}

class ReplyCase {
    // meta: Obj[String] = Number
    // data: String[][]
    constructor(meta, data) {
        this.meta = meta;
        this.data = data;
    }
}

class ReplyOutput {
    // data: Any[]
    constructor(data) {
        this.data = data;
    }

    // path: String
    save(path) {
        const fs = require("fs");
        let content = this.data
            .map((cell, index) => `Case #${index + 1}: ${cell}`)
            .join("\n");
        fs.writeFileSync(path, content);
    }
}

module.exports = { ReplyInput, ReplyOutput };
