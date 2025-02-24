# Tab Session Manager JSON Reader

This will be a simple app to read the export data JSON file
for the Tab Session Manager extension for Firefox and Chrome
and put it into formats that we can use more easily. Currently
it reads the JSON file and simplifies and sorts its data,
writing it back out to a new JSON file.

## Usage

The export JSON file has some objects that map `index -> key`,
where the number of indices varies from object to object,
and these are hard to specify when parsing using Serde.
To handle this situation, we have a Python scipt that restructures
the Python for easier parsing with Serde.

To apply the Python data reformat script, from project root:

```bash
./restructure_json.py --in-file "/home/sean/Backup/362 Sessions - 2025-02-23 15-47-40.json" --out-file scratch/input.json
```

Then to run the rust program on the output you can use:

```bash
cargo build && target/debug/tab_session_json --in-file scratch/input.json --out-file output.json
```

Note that the `--in-file` and `--out-file` arguments are both
required.

## To do next

Maybe just have the Rust program call the Python script. We
could add a `build.rs` script to check for a Python interpreter,
perhaps.

As a side note, it would be nice to have a JSON API like Json for
Modern C++ in Rust. It is able to flexibly handle JSON that is
structured to have objects with varying keys, as this has.
