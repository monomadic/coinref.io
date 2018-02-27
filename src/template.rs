use templar;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    name: String,
    symbol: String,
    website: String,
    tags: Vec<String>,
}

struct TemplarDirectiveHandler {}

// #[derive(Debug)]
// struct DirectiveError {
//     pub file: String,
//     pub directive: String,
//     pub reason: String
// }

use ::error::CoinrefError as DirectiveError;

use std::io::{Write, Read};
impl templar::output::DirectiveHandler for TemplarDirectiveHandler {
    type DirectiveError = DirectiveError;

    // handle directives eg. template commands
    fn handle<W>(&mut self, _context:&templar::TemplateContext, command: &str, _children: &[templar::Node], _base_indent:usize, _indent_size: usize, _writer: &mut W) -> Result<(), DirectiveError> where W : Write {
        let parts : Vec<_> = command.split(" ").collect();
        // check the first word of the command
        match parts.first() {
            // Some(&"doctype") => {
            //     writer.write_all(b"<!DOCTYPE html>\n").map_err(|_| DirectiveError {
            //         file: "".to_string(),
            //         directive: command.to_string(),
            //         reason: "couldnt write doctype".to_string(),
            //     })
            // },
            // Some(&"print") => {
            //     writer.write_all(parts.last().unwrap().as_bytes()).map_err(|_| DirectiveError {
            //         file: "".to_string(),
            //         directive: command.to_string(),
            //         reason: "no such value".to_string(),
            //     })
            // },
            _ => {
                Err(DirectiveError {
                    error_type: ::error::CoinrefErrorType::ImportError,
                    message: format!("unrecognised directive: {}", command.to_string()),
                })
            }
        }
    }
}

pub fn parse(template_path: &str) -> Result<::models::Coin, ::error::CoinrefError> {
    let mut template = ::std::fs::File::open(template_path).expect("template file to open");
    let mut bytebuffer = Vec::new();
    template.read_to_end(&mut bytebuffer).expect("template file to read");
    let file = String::from_utf8(bytebuffer).unwrap();

    let s: Vec<&str> = file.split("::").collect();

    let metadata = extract_metadata(s.first().unwrap())?;

    let templar_document = s.last();

    // println!("{:?}", metadata);

    Ok(::models::Coin {
        id: 0,
        name: metadata.name,
        symbol: metadata.symbol,
        website: metadata.website,

        twitter:    None,
        reddit:     None,
        github:     None,
        telegram:   None,
        slack:      None,
        facebook:   None,

        market_cap: 0,
        page: "String".to_string(),
    })
}

use std::error::Error;

impl From<::toml::de::Error> for ::error::CoinrefError {
    fn from(error: ::toml::de::Error) -> Self {
        ::error::CoinrefError {
            error_type: ::error::CoinrefErrorType::ImportError,
            message: error.description().to_string(),
        }
    }
}

pub fn extract_metadata(toml_data: &str) -> Result<Config, ::error::CoinrefError> {
    let toml:Config = toml::from_str(toml_data)?;
    Ok(toml)
}

pub fn render(template: &str) -> Result<String, ::error::CoinrefError> {

    // let template_nodes = templar::parse::parse(template)?;
    let template = templar::parse::parse(template)?;

    let mut directive_handler = TemplarDirectiveHandler{};
    let empty_context = templar::TemplateContext::empty();
    let mut rendered_buffer = Vec::new();

    templar::output::write_out(template.as_slice(), &empty_context, &mut rendered_buffer, 0, 2, &mut directive_handler)?;
    let rendered_page = String::from_utf8(rendered_buffer).unwrap();

    Ok(rendered_page)

    // match templar::parse::parse(template) {
    //     Ok(nodes) => {
    //         let mut directive_handler = TemplarDirectiveHandler{};
    //         let empty_context = templar::TemplateContext::empty();
    //         let mut rendered_buffer = Vec::new();
    //         let render_result = templar::output::write_out(nodes.as_slice(), &empty_context, &mut rendered_buffer, 0, 2, &mut directive_handler)?;

    //         Ok(String::from_utf8(rendered_buffer).unwrap())
    //     },
    //     Err(e) => format!("{:?}", e),
    // }


    // let mut template = ::std::fs::File::open(template_path)?;

    // // let mut template = ::std::fs::File::open(template_path).expect("template file to open");
    // let mut bytebuffer = Vec::new();
    // template.read_to_end(&mut bytebuffer).expect("template file to read");
    // let file = String::from_utf8(bytebuffer).unwrap();

    // let s: Vec<&str> = file.split("::").collect();

    // match templar::parse::parse(&s.last().expect("templar document")) {
    //     Ok(nodes) => {
    //         let mut directive_handler = TemplarDirectiveHandler{};
    //         let empty_context = templar::TemplateContext::empty();
    //         let mut rendered_buffer = Vec::new();
    //         let render_result = templar::output::write_out(nodes.as_slice(), &empty_context, &mut rendered_buffer, 0, 2, &mut directive_handler);

    //         match render_result {
    //             Ok(_) => String::from_utf8(rendered_buffer).unwrap(),
    //             Err(e) => format!("{:?}", e),
    //         }
    //     },
    //     Err(e) => format!("{:?}", e),
    // }
}