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

use ::error::*;
use ::error::CoinrefError as DirectiveError;

use std::io::{Write, Read};
impl templar::output::DirectiveHandler for TemplarDirectiveHandler {
    type DirectiveError = DirectiveError;

    // handle directives eg. template commands
    fn handle<W>(&mut self, _context:&templar::TemplateContext, command: &str, _children: &[templar::Node], _base_indent:usize, _indent_size: usize, writer: &mut W) -> Result<(), DirectiveError> where W : Write {
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

            Some(&"youtube") => {
                if let Some(youtube_id) = parts.get(1) {
                    let html = format!("<img src='{}'/>", youtube_id);

                    writer.write_all(html.as_bytes()).map_err(|_| DirectiveError {
                        error_type: CoinrefErrorType::ViewError,
                        message: format!("templar error writing: {}", command),
                    })
                } else {
                    Err(DirectiveError {
                        error_type: CoinrefErrorType::ViewError,
                        message: format!("missing operand to youtube directive."),
                    })
                }
            },

            _ => {
                Err(DirectiveError {
                    error_type: ::error::CoinrefErrorType::ImportError,
                    message: format!("unrecognised directive: {}", command.to_string()),
                })
            }
        }
    }
}

pub fn parse(template_path: &str) -> Result<::models::NewCoin, ::error::CoinrefError> {
    let mut template = ::std::fs::File::open(template_path).expect("template file to open");
    let mut bytebuffer = Vec::new();

    template.read_to_end(&mut bytebuffer).expect("template file to read");

    let file = String::from_utf8(bytebuffer).unwrap();
    let s:Vec<&str> = file.split("::").collect();

    if s.len() != 2 {
        return Err(CoinrefError {
            error_type: CoinrefErrorType::ImportError,
            message: format!("No metadata found in toml file: {}", template_path),
        });
    }

    let metadata = extract_metadata(s.first().unwrap())?;
    let templar_document:String = s.last().unwrap().to_string();
    let html = render(&templar_document)?;

    Ok(::models::NewCoin {
        name: metadata.name,
        symbol: metadata.symbol,
        website: metadata.website,

        twitter:    None,
        reddit:     None,
        github:     None,
        telegram:   None,
        slack:      None,
        facebook:   None,

        market_cap_usd: None,
        market_cap_rank: None,
        page: html,
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

pub fn render_file(template_file: &str) -> Result<String, ::error::CoinrefError> {
    let coin = parse(template_file)?;
    Ok(coin.page)
}

pub fn render(template: &str) -> Result<String, ::error::CoinrefError> {
    // let template_nodes = templar::parse::parse(template)?;
    // println!("template: {:?}", template);
    let template = templar::parse::parse(template)?;

    let mut directive_handler = TemplarDirectiveHandler{};
    let empty_context = templar::TemplateContext::empty();
    let mut rendered_buffer = Vec::new();

    templar::output::write_out(template.as_slice(), &empty_context, &mut rendered_buffer, 0, 2, &mut directive_handler)?;
    let rendered_page = String::from_utf8(rendered_buffer).unwrap();

    Ok(rendered_page)
}