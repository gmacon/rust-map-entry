use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut files = HashMap::new();

    for (path, line) in [
        ("a.txt", "Hello"),
        ("b.txt", "Hello"),
        ("b.txt", "This is file B"),
        ("a.txt", "This is file A"),
        ("c.txt", "This is file C"),
    ] {
        let file = match files.entry(path.to_owned()) {
            // Entry::Occupied(mut o) => o.get_mut(), // WRONG!
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(File::create(path)?),
        };
        writeln!(file, "{line}")?;
    }

    Ok(())
}
