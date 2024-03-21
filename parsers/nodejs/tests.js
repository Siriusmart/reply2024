let { ReplyInput, ReplyOutput } = require("./replayinput");

// carracing
{
    function extractor(meta) {
        return [meta[1], { track_length: meta[0] }];
    }

    let _input = new ReplyInput(
        "../test-data/input-carracing-b975.txt",
        extractor,
    );
}

// slotmachine
{
    function extractor(meta) {
        return [meta[0], { final_budget: meta[1], initial_budget: meta[2] }];
    }

    let _input = new ReplyInput(
        "../test-data/input-slotmachine-4d99.txt",
        extractor,
    );
}

// flowers
{
    function extractor(meta) {
        return [meta[3], { W: meta[0], H: meta[1], F: meta[2], G: meta[3] }];
    }

    let _input = new ReplyInput(
        "../test-data/input-flowers-9c96.txt",
        extractor,
    );
}

// output
{
    let output = new ReplyOutput([1, 25, 345, 45656, 15]);
    output.save("../test-data/js-output.txt");
}
