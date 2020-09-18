// https://doc.rust-lang.org/stable/rustc/lints/listing/warn-by-default.html
#![deny(const_err)]
#![deny(non_camel_case_types)]
#![deny(non_shorthand_field_patterns)]
#![deny(non_snake_case)]
#![deny(non_upper_case_globals)]
#![deny(path_statements)]
#![deny(renamed_and_removed_lints)]
#![deny(unconditional_recursion)]
#![deny(unknown_lints)]
#![deny(unreachable_code)]
#![deny(unreachable_patterns)]
#![deny(unused_assignments)]
#![deny(unused_comparisons)]
#![deny(unused_mut)]
#![deny(unused_parens)]
#![deny(unused_variables)]
#![deny(while_true)]
#![deny(unused_imports)]

#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate derive_getters;

mod exec;
mod prelude;

fn main() {}
