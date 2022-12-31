// Heavily borrowed from the gimli simple example

use object::{Object, ObjectSection};
use std::{
    borrow::{self, Cow},
    env, fs,
};

fn dump_file(object: &object::File, endian: gimli::RunTimeEndian) -> Result<(), gimli::Error> {
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
            if entry.tag() != gimli::DW_TAG_class_type {
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
                println!("{} {}", act_name, act_size);
            }
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} executable", args[0]);
        return;
    }

    let path = &args[1];

    println!("Starting dump of {}", path);

    let file = fs::File::open(&path).unwrap();

    let mmap = unsafe { memmap2::Mmap::map(&file).unwrap() };
    let object = object::File::parse(&*mmap).unwrap();
    let endian = if object.is_little_endian() {
        gimli::RunTimeEndian::Little
    } else {
        gimli::RunTimeEndian::Big
    };
    dump_file(&object, endian).unwrap();
}
