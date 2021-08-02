use std::fs;
use std::str;
use std::path::PathBuf;
use std::io::Cursor;
use std::io::prelude::*;
use quick_xml::Writer;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesText, BytesEnd, BytesStart};
use serde_json::{Map,Value};
use structopt::StructOpt;
use zip::write::{FileOptions, ZipWriter};

/// Arguments
#[derive(StructOpt, Debug)]
#[structopt(name = "Conveyancing CLI")]
struct Args {
    /// The path/s to the desired word file/s.
    #[structopt(short, long, parse(from_os_str))]
    file: Vec<PathBuf>,
    /// The path/s to one or more JSON files containing the variables.
    #[structopt(short, long, parse(from_os_str))]
    json: Vec<PathBuf>,
    /// The name/paths of the output word file/s.
    #[structopt(short, long, parse(from_os_str))]
    output: Vec<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    
    let args = Args::from_args();
    for i in 0..args.file.len() { 
        let mut archive = zip::ZipArchive::new(fs::File::open(&args.file[i])
            .expect("Error: Document was not found, please specify a valid path."))?;
        let json = merge_json(&args.json)?;
        
        let output = std::fs::File::create(&args.output[i]).expect("Error: Could not create file.");
        let options = FileOptions::default();
        let mut zip = ZipWriter::new(output);
        let mut buf = Vec::new();

        // Loop through files
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
                
                // Read the file into a buffer
                file.read_to_end(&mut buf)?;
                if file.name() == "word/document.xml" {
                    buf = interpolate_json(buf, &json)?;
                }

                // Write the buffer into the file
                zip.start_file(outpath.into_os_string().into_string().unwrap(), options)?;
                zip.write_all(&buf)?;
                buf.clear();
            }
        }

        zip.finish()?;
    }
    
    println!("Conveyance template has been filled out successfully! 🚀");
    Ok(())
}

/// Interpolate json data into word file.
fn interpolate_json(buf:Vec<u8>, json:&Map<String,Value>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    
    let mut reader = Reader::from_str(str::from_utf8(&buf)?);
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    let mut xml_buf = Vec::new();
    
    let mut found = false;
    
    // Loop over every tag in the XML document
    loop {
        if !found{
            // Continue to iterate until the start of a variable
            match reader.read_event(&mut xml_buf) {
                Ok(Event::Empty(ref e)) if e.name() == b"w:fldChar" && 
                    e.attributes().any(|a| a.unwrap().value.into_owned() == b"begin") 
                => {
                    found = true
                },
                Ok(Event::Eof) => break,
                Ok(e) => writer.write_event(e)?,
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            }
        } else {
            // When the start of a new variable is found, 
            // skip through and replace it with the desired json value.
            match reader.read_event(&mut xml_buf) {
                Ok(Event::Start(ref e)) if e.name() == b"w:t" => {
                    
                    let mut text_buf = Vec::new();
                    reader.read_text(e.name(),&mut text_buf)?;
                    let text = String::from_utf8(text_buf)?.replace("«", "").replace("»","").replace("/w:t","");
                    
                    // Test each json value
                    json.iter().for_each(|(key,value)| {
                        if text == key.trim() {
                            // Write in a text tag
                            writer.write_event(Event::Start(BytesStart::borrowed(e, e.name().len())))
                                .expect("Error whilst writing value");
                            writer.write_event(Event::Text(BytesText::from_plain_str(value.as_str().unwrap())))
                                .expect("Error: Incorrect Json, key was not a String");
                            writer.write_event(Event::End(BytesEnd::borrowed(b"w:t")))
                                .expect("Error: Could not close tag");
                        }
                    })
                },
                Ok(Event::Empty(ref e)) if e.name() == b"w:fldChar" && 
                    e.attributes().any(|a| a.unwrap().value.into_owned() == b"end") 
                => {
                    found = false
                },
                Ok(_) => (),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            }
        }
        xml_buf.clear();
    }
    
    Ok(writer.into_inner().into_inner())
}

/// Read in and merge all specified Json files.
fn merge_json(paths:&[PathBuf])->Result<Map<String,Value>, Box<dyn std::error::Error>> {

    let mut json = Map::new();
    for p in paths {
        json.extend(
            match serde_json::from_str::<Value>(&fs::read_to_string(p)
                .expect("Error: Json file was not found, please specify a valid path."))
            ?.as_object() {
                Some(object) => object.to_owned(),
                None => continue
            }
        );
    }

    Ok(json)
}