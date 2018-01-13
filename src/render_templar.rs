use templar;

struct TemplarDirectiveHandler {}

#[derive(Debug)]
struct DirectiveError {
    pub file: String,
    pub directive: String,
    pub reason: String
}

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
                    file: "".to_string(),
                    directive: command.to_string(),
                    reason: format!("unrecognized directive: {}", command).to_string(),
                })
            }
        }
    }
}

pub fn render_template(template_path: &str) -> String {
    let mut template = ::std::fs::File::open(template_path).expect("template file to open");
    let mut bytebuffer = Vec::new();
    template.read_to_end(&mut bytebuffer).expect("template file to read");
    let s = String::from_utf8(bytebuffer).unwrap();

    match templar::parse::parse(&s) {
        Ok(nodes) => {
            let mut directive_handler = TemplarDirectiveHandler{};
            let empty_context = templar::TemplateContext::empty();
            let mut rendered_buffer = Vec::new();
            let render_result = templar::output::write_out(nodes.as_slice(), &empty_context, &mut rendered_buffer, 0, 2, &mut directive_handler);

            match render_result {
                Ok(_) => String::from_utf8(rendered_buffer).unwrap(),
                Err(e) => format!("{:?}", e),
            }
        },
        Err(e) => format!("{:?}", e),
    }
}