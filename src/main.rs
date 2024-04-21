#![warn(clippy::pedantic, clippy::nursery, clippy::restriction)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::blanket_clippy_restriction_lints,
    clippy::implicit_return,
    clippy::missing_trait_methods,
    clippy::single_call_fn,
    clippy::min_ident_chars,
    clippy::default_numeric_fallback,
    clippy::shadow_unrelated,
    clippy::shadow_reuse,
    clippy::indexing_slicing,
    clippy::float_arithmetic,
    clippy::arithmetic_side_effects,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::self_named_module_files,
    clippy::question_mark_used
)]

use std::io::{self, Read as _};

mod chars;
mod evaluate;
mod operator;
mod parser;
mod sum_error;

use chars::Chars;
use evaluate::evaluate;
use parser::parse;

fn main() -> ! {
    loop {
        // Read the sum
        eprint!(">>> ");
        let mut chars = Chars::from(io::stdin().lock().bytes().map_while(Result::ok))
            .take_while(|&c| c != '\n')
            .filter(|c| !c.is_whitespace());

        // Parse the sum
        #[allow(clippy::significant_drop_in_scrutinee)]
        let (mut numbers, mut operators) = match parse(&mut chars) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("{e}");
                continue;
            }
        };

        // Calculate the result
        let result = evaluate(&mut numbers, &mut operators);

        // Print the result
        println!("{result}");
    }
}
