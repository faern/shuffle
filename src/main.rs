extern crate core;
extern crate argparse;

use std::os;
use argparse::{ArgumentParser, Collect, Store};
use core::str::FromStr;
use std::rand::{thread_rng, Rng};

enum QuoteStyle {
  None,
  All,
  WithSpaces,
}

impl FromStr for QuoteStyle {
  fn from_str(s: &str) -> Option<Self> {
    match s {
      "none" => Some(QuoteStyle::None),
      "all" => Some(QuoteStyle::All),
      "spaces" => Some(QuoteStyle::WithSpaces),
      _ => None
    }
  }
}

fn main() {
  let mut quote_style: QuoteStyle = QuoteStyle::WithSpaces;
  let mut values: Vec<String> = vec![];
  {
    let mut arg_parser = ArgumentParser::new();
    arg_parser.set_description("Shuffle input arguments and print them out");

    arg_parser.refer(&mut quote_style).add_option(&["-q", "--quote-style"],
      Box::new(Store::<QuoteStyle>),
      "Set what arguments to quote in the output. possible values: all, none, spaces");

    arg_parser.refer(&mut values).add_argument("values", Box::new(Collect::<String>), "List of values to shuffle");
    match arg_parser.parse_args() {
      Ok(()) => {},
      Err(x) => {
        os::set_exit_status(x);
        return;
      }
    }
  }

  let mut rng = thread_rng();
  rng.shuffle(values.as_mut_slice());

  print_args(&values[], quote_style);
}

fn print_args(args: &[String], quote_style: QuoteStyle) {
  for item in args.iter() {
    let quote = match quote_style {
      QuoteStyle::None => false,
      QuoteStyle::All => true,
      QuoteStyle::WithSpaces => item.find(|&: c: char| c.is_whitespace()).is_some(),
    };
    match quote {
      true => print!("\"{}\" ", item),
      false => print!("{} ", item),
    };
  }
  println!("");
}
