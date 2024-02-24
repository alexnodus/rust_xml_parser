mod config;
mod parser;

use config::args::Config;
use std::fs;
use std::collections::HashSet;

// - Чтобы исправить проблему с "крякозябрами" в PowerShell или командной строке Windows, переключитесь на использование кодовой страницы UTF-8 выполнив следующую команду в терминале:
//  chcp 65001

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new();
    println!("Processing file: {}", config.filename);
    println!("Обрабатывается файл: {}", config.filename);


    println!("{}", &config.filename);
    let tags = parser::xml_parser::parse_xml(&config.filename)?;

    // Генерация HTML-документа с уникальными элементами
    // Generating an HTML document with unique elements
    let mut html_content = String::from("<html><body><ul>");
    for element in tags {
        html_content.push_str(&format!("<li>{}</li>", element));
    }
    html_content.push_str("</ul></body></html>");

    fs::write("unique_elements.html", html_content)?;
    println!("HTML файл с уникальными элементами был создан.");
    println!("An HTML file with unique elements has been created.");
    
    Ok(())
}
