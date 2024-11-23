# Some Advent Of Code solutions in Rust

## Input file

For reading the input file as a string, I was initially using the
solution suggested
in [this SO post](https://stackoverflow.com/questions/45882329/read-large-files-line-by-line-in-rust).

```rust
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("foo.txt").expect("Should have opened file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.expect("Should have gotten line"));
    }

    Ok(())
}
```

However, I switched to using the [`include_str!`](https://doc.rust-lang.org/std/macro.include_str.html)
macro, as suggested
[in this video](https://www.youtube.com/watch?v=fEQv-cqzbPghttps://www.youtube.com/watch?v=fEQv-cqzbPg).
See also [this Reddit post](https://www.reddit.com/r/adventofcode/comments/zpv8e6/rust_convenient_reading_of_input/).
Keep in mind that using `const` (as suggested in the Reddit post) is not
a good solution for a normal program because you would have to recompile
the program to evaluate a different input file.

## Rust's module system

[This article](https://www.sheshbabu.com/posts/rust-module-system/) has a great explanation.