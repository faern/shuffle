use std::os;
use std::rand::{thread_rng, Rng};

enum QuoteStyle {
  None,
  All,
  WithSpace,
}

fn main() {
  let args = os::args();
  if args.len() <= 1 {
    return;
  }

  let mut arg_given = false;
  let quote_style: QuoteStyle = match parse_quote_style(&args[]) {
    Some(q) => {
      arg_given = true;
      q
    },
    None => QuoteStyle::WithSpace, //Default quote style
  };

  let mut shuffle_list = args[if arg_given {2} else {1}..].to_vec();
  let mut rng = thread_rng();
  rng.shuffle(shuffle_list.as_mut_slice());

  print_args(&shuffle_list[], quote_style);
}

fn parse_quote_style(args: &[String]) -> Option<QuoteStyle> {
  if args.len() > 1 {
    let arg0 = &args[1];
    let mut arg0iter = arg0.chars();
    if arg0iter.next() == Some('-') {
      match arg0iter.next() {
        Some('n') => Some(QuoteStyle::None),
        Some('a') => Some(QuoteStyle::All),
        Some('s') => Some(QuoteStyle::WithSpace),
        _         => panic!("Invalid argument given"),
      }
    } else {
      None
    }
  } else {
    None
  }
}

fn print_args(args: &[String], quote_style: QuoteStyle) {
  for item in args.iter() {
    let quote = match quote_style {
      QuoteStyle::None => false,
      QuoteStyle::All => true,
      QuoteStyle::WithSpace => item.find(|&: c: char| c.is_whitespace()).is_some(),
    };
    match quote {
      true => print!("\"{}\" ", item),
      false => print!("{} ", item),
    };
  }
  println!("");
}
