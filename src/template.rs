use templar;
use toml;

#[derive(Debug, Deserialize)]
pub struct CoinTemplate {
    pub name:       String,
    pub symbol:     String,
    pub website:    String,
    pub twitter:    Option<String>,
    pub reddit:     Option<String>,
    pub github:     Option<String>,
    pub telegram:   Option<String>,
    pub slack:      Option<String>,
    pub facebook:   Option<String>,
    pub youtube:    Option<String>,
    pub tags:       Vec<String>,
    pub skip:       Option<bool>,
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
        match parts.first() {
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

pub fn read(template_path: &str) -> Result<CoinTemplate, ::error::CoinrefError> {
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

    Ok(extract_metadata(s.first().expect("metadata to read")).map_err(|e|
        CoinrefError{ error_type: CoinrefErrorType::ImportError, message: e.message })?)
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

    let metadata = extract_metadata(s.first().expect("metadata to read")).map_err(|e|
        CoinrefError{ error_type: CoinrefErrorType::ImportError, message: e.message })?;

    let templar_document:String = s.last().unwrap().to_string();
    let html = render(&templar_document)?;

    Ok(::models::NewCoin {
        name: metadata.name,
        symbol: metadata.symbol,
        website: metadata.website,

        twitter:    metadata.twitter,
        reddit:     metadata.reddit,
        github:     metadata.github,
        telegram:   metadata.telegram,
        slack:      metadata.slack,
        facebook:   metadata.facebook,
        youtube:    metadata.youtube,

        market_cap_usd: None,
        market_cap_rank: None,

        circulating_supply: None,
        price_in_btc: None,
        price_in_usd: None,

        growth_potential: None,

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

pub fn extract_metadata(toml_data: &str) -> Result<CoinTemplate, ::error::CoinrefError> {
    let toml:CoinTemplate = toml::from_str(toml_data).map_err(|e|
        CoinrefError{ error_type: CoinrefErrorType::ImportError, message: format!("TOML error: {}", e) })?;
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