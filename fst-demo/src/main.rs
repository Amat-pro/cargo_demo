use std::fs::File;
use std::io;

use fst::{Automaton, IntoStreamer, Map, MapBuilder, Set, set, Streamer};
use fst::automaton::{Levenshtein, Str};
// use the memmap crate to make the file available as a &[u8] without necessarily reading it all into memory
// (the operating system will automatically handle that for you).
use memmap::Mmap;

/// [fst](https://docs.rs/fst/0.4.7/fst/)
fn main() {
    println!("Hello, fst!");

    let _ = fuzzy_query();

    let _ = stream_to_file_and_memory_it_for_search();

    let _ = search_multiples_set_efficiently();
}

// fuzzy query
fn fuzzy_query() -> Result<(), Box<dyn std::error::Error>> {

    // create sets in memory.
    let keys = vec!["fa", "fo", "fob", "focus", "foo", "food", "foul", "oo"];
    let set = Set::from_iter(keys)?;

    // fuzzy query
    let lev = Levenshtein::new("foo", 1)?;

    // Apply our fuzzy query to the set we built.
    let stream = set.search(lev).into_stream();

    let keys = stream.into_strs()?;

    println!("fuzzy query: {:?}", keys);

    assert_eq!(keys, vec!["fo", "fob", "foo", "food", "oo"]);
    Ok(())
}

// stream to a file and memory map it for searching
fn stream_to_file_and_memory_it_for_search() -> Result<(), Box<dyn std::error::Error>> {
    const FILE_NAME: &str = "map.fst";

    let writer = io::BufWriter::new(File::create(FILE_NAME)?);

    let mut build = MapBuilder::new(writer)?;
    build.insert("bruce", 1).unwrap();
    build.insert("clarence", 2).unwrap();
    build.insert("stevie", 3).unwrap();

    build.finish()?;

    // creates a memory map, which enables searching the map without loading all of it into memory
    let mmp = unsafe { Mmap::map(&File::open(FILE_NAME)?)? };
    let map = Map::new(mmp)?;

    // query for keys that are greater than or equal to clarence
    let stream = map.range().ge("clarence").into_stream();
    let kvs = stream.into_str_vec()?;

    println!("stream to file and memory it for search: {:?}", kvs);
    assert_eq!(kvs, vec![
        ("clarence".to_owned(), 2),
        ("stevie".to_owned(), 3),
    ]);

    Ok(())
}

// searching multiple sets efficiently
fn search_multiples_set_efficiently() -> Result<(), Box<dyn std::error::Error>> {
    let set1 = Set::from_iter(&["AC/DC", "Aerosmith"])?;
    let set2 = Set::from_iter(&["Bob Seger", "Bruce Springsteen"])?;
    let set3 = Set::from_iter(&["George Thorogood", "Golden Earring"])?;
    let set4 = Set::from_iter(&["Kansas"])?;
    let set5 = Set::from_iter(&["Metallica"])?;

    // create the matcher. We can reuse it to search all of the sets
    let matcher = Str::new("B")
            .starts_with()
            .union(Str::new("G").starts_with());

    let mut stream =
            set::OpBuilder::new()
                    .add(set1.search(&matcher))
                    .add(set2.search(&matcher))
                    .add(set3.search(&matcher))
                    .add(set4.search(&matcher))
                    .add(set5.search(&matcher))
                    .union();

    let mut keys = vec![];
    while let Some(key) = stream.next() {
        keys.push(String::from_utf8(key.to_vec())?);
    }

    println!("searching multiple sets efficiently: {:?}", keys);

    assert_eq!(keys, vec![
        "Bob Seger",
        "Bruce Springsteen",
        "George Thorogood",
        "Golden Earring",
    ]);

    Ok(())
}


























