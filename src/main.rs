#![feature(rustc_private)]

extern crate syntax;
use std::rc::Rc;
use syntax::source_map::{FilePathMapping, SourceMap};
use syntax::parse::ParseSess;
use syntax::errors::emitter::ColorConfig;
use syntax::errors::Handler;

fn main() {
    let text = "fn test() ->".to_string();  // produces Err result
    let text = "fn test(a".to_string();     // exits silently

    syntax::with_globals(syntax::source_map::edition::Edition::Edition2018, || {
        let source_map = Rc::new(SourceMap::new(FilePathMapping::empty()));
        let parse_session = ParseSess::with_span_handler(
            Handler::with_tty_emitter(ColorConfig::Auto, true, None, Some(source_map.clone())),
            source_map
        );
        let parser = syntax::parse::maybe_new_parser_from_source_str(
            &parse_session,
            syntax::source_map::FileName::Custom("stdin".to_owned()),
            text,
        );

        println!("{:?}", parser.unwrap().parse_crate_mod().map_err(|mut x| {x.cancel(); x}));
    })
}
