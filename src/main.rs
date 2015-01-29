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
  let quote_style: QuoteStyle = if args.len() > 1 {
    let arg0 = &args[1];
    let mut arg0iter = arg0.chars();
    if arg0iter.next() == Some('-') {
      arg_given = true;
      match arg0iter.next() {
        Some('n') => QuoteStyle::None,
        Some('a') => QuoteStyle::All,
        Some('s') => QuoteStyle::WithSpace,
        _         => panic!("Invalid argument given"),
      }
    } else {
      QuoteStyle::WithSpace
    }
  } else {
    QuoteStyle::WithSpace
  };

  let mut shuffle_list = args[if arg_given {2} else {1}..].to_vec();

  let mut rng = thread_rng();
  rng.shuffle(shuffle_list.as_mut_slice());

  for item in shuffle_list.iter() {
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
