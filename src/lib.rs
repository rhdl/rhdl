use lalrpop_util::lalrpop_mod;

#[macro_use]
mod ast;

// lalrpop_mod!(pub rhdl);

// #[cfg(test)]
// mod tests {
//     use super::{ast::*, rhdl::*};
//     use pretty_assertions::assert_eq;

//     #[test]
//     fn int_parser() {
//         macro_rules! parse {
//             ($($($input: expr),+ => $expected: expr),+) => {
//                 $(
//                     $(
//                         assert_eq!(IntLitParser::new().parse($input), Ok(Lit::Int {
//                             val: rug::Integer::from($expected),
//                             suffix: None,
//                             raw: $input.to_string(),
//                             span: Span(0, $input.len())
//                         }));
//                     )+
//                 )+
//             };
//         }
//         parse!(
//             "123", "1_2_3" => 123,
//             "0b101", "0b1_01" => 0b101,
//             "0o72" => 0o72,
//             "0d1_2_3" => 123,
//             "0xB105F00D", "0xB105_F00D" => 0xB105F00Du32,
//             "36#HELLO_THERE" => 1767707662651898u64
//         );
//         assert!(IntLitParser::new().parse("?").is_err());
//     }

//     #[test]
//     fn float_parser() {
//         macro_rules! parse {
//             ($($($input: expr),+ => ($prec: expr, $expected: expr)),+) => {
//                 $(
//                     $(
//                         let input = $input;
//                         let res = FloatParser::new().parse(&input);
//                         let expected = Ok(Lit::Float {
//                             val: rug::Float::with_val($prec, $expected),
//                             suffix: None,
//                             raw: input.to_string(),
//                             span: Span(0, $input.len()),
//                         });
//                         assert_eq!(res, expected);
//                     )+
//                 )+
//             };
//         }
//         parse!(
//             format!("{}f64", std::f64::consts::E) => (64, std::f64::consts::E),
//             format!("{}f32", std::f32::consts::PI) => (64, std::f32::consts::PI),
//             "0f16" => (16, 0),
//             "0f32" => (32, 0),
//             "0f64" => (64, 0),
//             "0f128" => (128, 0),
//             "0f256" => (256, 0)
//         );
//         assert!(FloatParser::new().parse("?").is_err());
//     }

//     #[test]
//     fn expr_parser_parses_all_ops() {
//         macro_rules! parse {
//             ($($input: expr),+) => {
//                 $(
//                     assert_eq!(ExprParser::new().parse(&$input).map(|output| format!("{}", output)), Ok($input.to_string()));
//                 )+
//             };
//         }
//         for op in UnOp::variants().iter().map(ToString::to_string) {
//             parse!(
//                 format!("{}a", op),
//                 format!("{}0", op),
//                 format!("{}{{ 0 }}", op)
//             );
//         }
//         for op in BinOp::variants().iter().map(ToString::to_string) {
//             parse!(
//                 format!("a {} 0", op),
//                 format!("a {} b", op),
//                 format!("a {} {{ b }}", op)
//             );
//         }
//         for op in AssOp::variants().iter().map(ToString::to_string) {
//             parse!(
//                 format!("{{ a {} 0; }}", op),
//                 format!("{{ a {} b; }}", op),
//                 format!("{{ a {} {{ b }}; }}", op)
//             );
//         }
//     }

//     #[test]
//     fn expr_parser() {
//         macro_rules! parse {
//             ($($input: expr),+) => {
//                 $(
//                     assert_eq!(ExprParser::new().parse($input).map(|output| format!("{}", output)), Ok($input.to_string()));
//                 )+
//             };
//         }
//         parse!(
//             "{ }",
//             "a",
//             "4",
//             "{ a }",
//             "[4; 5]",
//             "0..=9",
//             "point.x",
//             "call()",
//             "x.call()",
//             "x[0]",
//             "[0, 1, 2, 3, 4, 5]",
//             "(0, 1, 2, 3, 4, 4.5)",
//             "x as y",
//             "if a >= 4 { }",
//             "if a >= 4 { } else { }",
//             "if a >= 4 { } else if a < 0 { } else if a > 0 { } else { }",
//             "match x { 0 => { }, 1 => { y }, _ if x != 2 => { }, 2 => { } }",
//             "{ return a; }",
//             "Struct { x, y, z }",
//             "Struct { x: a, y: b, z: c }",
//             "Struct { x, .. z }",
//             "Struct { .. z }",
//             "for x in 0..16 { }"
//         );
//     }

//     #[test]
//     fn pat_parser() {
//         macro_rules! parse {
//             ($($input: expr),+) => {
//                 $(
//                     assert_eq!(PatParser::new().parse($input).map(|output| format!("{}", output)), Ok($input.to_string()));
//                 )+
//             };
//         }
//         parse!(
//             "0",
//             "a",
//             "module::Enum",
//             "_",
//             "0..10",
//             "0..=10",
//             "0..super::RAM_SIZE",
//             "Struct { a, b, c, .. }",
//             "Struct { a: x, b: y, c: z, .. }",
//             "Struct { a, b, c, d, e, f, g, h }",
//             "Struct { .. }",
//             "StructTup { 0: first, 1: second, 2: third }",
//             "Tup(a, b, c, d, .., z)",
//             "Tup(x, y, z)",
//             "(left, right)",
//             "(zero, .., hundred)",
//             "[1, _, _]"
//         );
//     }

//     #[test]
//     fn file_parser() {
//         macro_rules! parse {
//             ($($input: expr),+) => {
//                 $(
//                     assert_eq!(FileParser::new().parse($input).map(|output| format!("{}", output)), Ok($input.to_string()));
//                 )+
//             };
//         }
//         parse!(
//             r#"use super::X;
// use crate::Y;
// use crate::{ first::{ self, Type }, second::Type as AnotherType };
// pub const ROM_SIZE: uint = 64 * 1024 * 1024;
// mod in_another_file;
// mod in_this_file { }
// fn x(x: X) { }
// type AliasForX = X;
// pub(super) struct NamedWrapper { x: X }
// pub(crate) struct UnnamedWrapper(X);
// pub(in super::super) enum Z { A(X), B(Y), C(u12) }
// enum GrayU2 { Zero = 0b00, One = 0b01, Two = 0b11, Three = 0b10 }
// enum States { Uninitialized, Ready, Busy, Error }
// pub(self) bag AudioFrequency { 32_000, 41_000, 48_000 }
// ring AudioBitWidth = 16..=24;
// fn everything(parenthesized: (Type), typePath: Type, tuple1: (), tuple2: (u9, i9), array: [u4; 32], slice: [u8], infer: _, function: fn(u8), function_ret: fn(u10) -> u16) { }
// "#
//         );
//     }
// }
