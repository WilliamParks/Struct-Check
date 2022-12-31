// Heavily borrowed from the gimli simple example

use object::{Object, ObjectSection};
use std::{
    borrow::{self, Cow},
    env, fs, collections::HashMap,
};

// use serde::Deserialize;

// #[derive(Debug, Deserialize)]
// struct CodeQL_Data {
//     first_name: String,
//     last_name: String,
//     age: u8,
//     phone_numbers: Vec<String>,
// }

fn get_dwarf_data(object: object::File, endian: gimli::RunTimeEndian) -> Result<HashMap<String, u64>, gimli::Error> {

    let mut res: HashMap<String, u64> = HashMap::new();

    // Load a section and return as `Cow<[u8]>`.
    let load_section = |id: gimli::SectionId| -> Result<borrow::Cow<[u8]>, gimli::Error> {
        match object.section_by_name(id.name()) {
            Some(ref section) => Ok(section
                .uncompressed_data()
                .unwrap_or(borrow::Cow::Borrowed(&[][..]))),
            None => Ok(borrow::Cow::Borrowed(&[][..])),
        }
    };

    // Load all of the sections.
    let dwarf_cow = gimli::Dwarf::load(&load_section)?;

    // Borrow a `Cow<[u8]>` to create an `EndianSlice`.
    let borrow_section: &dyn for<'a> Fn(
        &'a borrow::Cow<[u8]>,
    ) -> gimli::EndianSlice<'a, gimli::RunTimeEndian> =
        &|section| gimli::EndianSlice::new(&*section, endian);

    // Create `EndianSlice`s for all of the sections.
    let dwarf = dwarf_cow.borrow(&borrow_section);

    // Iterate over the compilation units.
    let mut iter = dwarf.units();
    while let Some(header) = iter.next()? {
        let unit = dwarf.unit(header)?;

        // Iterate over the Debugging Information Entries (DIEs) in the unit.
        let mut entries = unit.entries();
        while let Some((_, entry)) = entries.next_dfs()? {
            if entry.tag() != gimli::DW_TAG_class_type && entry.tag() != gimli::DW_TAG_structure_type {
                continue;
            }

            // Iterate over the attributes in the DIE.
            let mut attrs = entry.attrs();

            let mut size: Option<u64> = None;
            let mut name: Option<Cow<str>> = None;

            while let Some(attr) = attrs.next()? {
                match attr.name() {
                    gimli::DW_AT_name => {
                        if let gimli::AttributeValue::DebugStrRef(offset) = attr.value() {
                            if let Ok(s) = dwarf.debug_str.get_str(offset) {
                                name = Some(s.to_string_lossy());
                            }
                        } else {
                            eprintln!("Unable to get debug str ref")
                        }
                    }
                    gimli::DW_AT_byte_size => size = attr.udata_value(),
                    _ => continue,
                }
            }

            if let (Some(act_size), Some(act_name)) = (size, name) {
                // The replace is needed to standardize with CodeQL
                res.insert(act_name.into_owned().replace(" >", ">"), act_size);
            }
        }
    }

    Ok(res)
}

fn get_codeql_data(_input: String) -> Result<HashMap<String, u64>, gimli::Error> {
    let res: HashMap<String, u64> = HashMap::new();

    Ok(res)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: {} dwarf_filecodeql_json", args[0]);
        return;
    }

    let dwarf_path = &args[1];

    let file = fs::File::open(&dwarf_path).expect("Unable to open DWARF file");

    let mmap = unsafe { memmap2::Mmap::map(&file).unwrap() };
    let object = object::File::parse(&*mmap).unwrap();
    let endian = if object.is_little_endian() {
        gimli::RunTimeEndian::Little
    } else {
        gimli::RunTimeEndian::Big
    };
    let dwarf_res = get_dwarf_data(object, endian).unwrap();

    println!("Got {} dwarf entries", dwarf_res.len());

    for (key, value) in dwarf_res {
        println!("{} / {}", key, value);
    }
    let codeql_json_path = &args[2];

    let contents = fs::read_to_string(codeql_json_path).expect("Unable to open codeql json file");

    let _ = get_codeql_data(contents);



}
