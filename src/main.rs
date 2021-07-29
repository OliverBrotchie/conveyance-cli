use std::fs;
use std::str;
use std::path::PathBuf;
use std::io::Cursor;
use std::io::prelude::*;
use quick_xml::Writer;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesEnd, BytesStart};
use zip::write::{FileOptions, ZipWriter};
use serde_json::{Map,Value};
use structopt::StructOpt;

/// Arguments
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Args {
    /// Word files
    #[structopt(short, long, parse(from_os_str))]
    file: PathBuf,
    /// Json File
    #[structopt(short, long, parse(from_os_str))]
    json: Vec<PathBuf>,
    /// Output Word File
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>>{

    let args = Args::from_args();
    let mut archive = zip::ZipArchive::new(fs::File::open(&args.file)?)?;
    let json = merge_json(args.json)?;
    
    let output = std::fs::File::create(args.output).unwrap();
    let options = FileOptions::default();
    let mut zip = ZipWriter::new(output);
    let mut buf = Vec::new();

    for i in 0..archive.len(){
        
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        // Create Directories
        if (&*file.name()).ends_with('/') {
            zip.add_directory(outpath.into_os_string().into_string().unwrap(), Default::default())?;
        } else {

            // // Make certain that directory exists
            // if let Some(p) = outpath.parent() {
            //     if !p.exists() {
            //         fs::create_dir_all(&p).unwrap();
            //     }
            // }

            file.read_to_end(&mut buf)?;
            if file.name() == "word/document.xml" {
                interpolate(&mut buf, &json)?;
            }

            zip.start_file(outpath.into_os_string().into_string().unwrap(), options)?;
            zip.write_all(&buf)?;
        }
    }

    zip.finish()?;
    Ok(())
}
/// Interpollate json data into word file
fn interpolate(buf:&mut Vec<u8>,json:&Map<String,Value>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    
    let mut reader = Reader::from_str(str::from_utf8(&buf)?);
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    let mut buf = Vec::new();
    
    
    loop {
        match reader.read_event(&mut buf) {
            // Ok(Event::Start(ref e)) if e.name() == b"this_tag" => {

            //     // crates a new element ... alternatively we could reuse `e` by calling
            //     // `e.into_owned()`
            //     let mut elem = BytesStart::owned(b"my_elem".to_vec(), "my_elem".len());

            //     // collect existing attributes
            //     elem.extend_attributes(e.attributes().map(|attr| attr.unwrap()));

            //     // copy existing attributes, adds a new my-key="some value" attribute
            //     elem.push_attribute(("my-key", "some value"));

            //     writer.write_event(Event::Start(elem))?
            // },
            Ok(Event::Start(ref e)) if e.name() == b"w:fldChar" => {

                println!("Found it!");
                writer.write_event(Event::Start(BytesStart::borrowed(e,e.name().len())))?
            }
            Ok(Event::End(ref e)) if e.name() == b"w:fldChar" => {
                writer.write_event(Event::End(BytesEnd::borrowed(e.name())))?
            },
            Ok(Event::Eof) => break,
            Ok(e) => writer.write_event(e)?,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
        }
        buf.clear();
    }
    
    Ok(writer.into_inner().into_inner())
}

fn merge_json(paths:Vec<PathBuf>)->Result<Map<String,Value>, Box<dyn std::error::Error>>{
    
    let mut json = Map::new();

    for p in paths {
        json.extend(
            match serde_json::from_str::<Value>(&fs::read_to_string(p)?)?.as_object() {
                Some(object) => object.to_owned(),
                None => continue
            }
        );
    }

    Ok(json)
}