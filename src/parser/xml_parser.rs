use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

pub fn parse_xml(filename: &str) -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let file = BufReader::new(file);
    let parser = EventReader::new(file);

    let mut tags = HashSet::new();
    let mut inside_data = false;

    for e in parser {
        match e? {
            XmlEvent::StartElement { name, .. } => {
                let prefix = name.prefix.unwrap_or("No namespace".to_string());
                println!("Start element: {}, prefix: {}", name.local_name, prefix);
                if name.local_name == "Data" && prefix == "v8de" {
                    inside_data = true;
                } else if inside_data {
                    tags.insert(name.local_name);
                }
            }
            XmlEvent::EndElement { name } => {
                println!("End element: {}", name.local_name);
                if name.local_name == "Data" {
                    inside_data = false;
                }
            }
            _ => {}
        }
    }

    Ok(tags)
}
