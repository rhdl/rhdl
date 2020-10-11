#[macro_use]
extern crate lalrpop_util;

mod ast;

lalrpop_util::lalrpop_mod!(pub rhdl);

#[cfg(test)]
mod tests {
    use super::{ast::*, rhdl::*};

    #[test]
    fn int_parser() {
        macro_rules! parse {
            ($($($input: expr),+ => $expected: expr),+) => {
                $(
                    $(
                        assert_eq!(IntLitParser::new().parse($input), Ok(Lit::Int {
                            val: rug::Integer::from($expected),
                            suffix: None,
                        }));
                    )+
                )+
            };
        }
        parse!(
            "123", "1_2_3" => 123,
            "0b101", "0b1_01" => 0b101,
            "0o72" => 0o72,
            "0d1_2_3" => 123,
            "0xB105F00D", "0xB105_F00D" => 0xB105F00Du32,
            "36#HELLO_THERE" => 1767707662651898u64
        );
        assert!(IntLitParser::new().parse("?").is_err());
    }

    #[test]
    fn float_parser() {
        macro_rules! parse {
            ($($($input: expr),+ => ($prec: expr, $expected: expr)),+) => {
                $(
                    $(
                        let input = $input;
                        let res = FloatParser::new().parse(&input);
                        let expected = Ok(Lit::Float {
                            val: rug::Float::with_val($prec, $expected),
                            suffix: None,
                        });
                        assert_eq!(res, expected);
                    )+
                )+
            };
        }
        parse!(
            format!("{}f64", std::f64::consts::E) => (64, std::f64::consts::E),
            format!("{}f32", std::f32::consts::PI) => (64, std::f32::consts::PI),
            "0f16" => (16, 0),
            "0f32" => (32, 0),
            "0f64" => (64, 0),
            "0f128" => (128, 0),
            "0f256" => (256, 0)
        );
        assert!(FloatParser::new().parse("?").is_err());
    }

    #[test]
    fn expr_parser_parses_all_ops() {
        macro_rules! parse {
            ($($input: expr => $expected: expr),+) => {
                $(
                    assert_eq!(ExprParser::new().parse(&$input).map(|output| format!("{}", output)), Ok($expected.to_string()));
                )+
            };
        }
        for op in UnOp::variants().iter().map(ToString::to_string) {
            parse!(
                format!("{}a", op) => format!("{}a", op),
                format!("{}0", op) => format!("{}0", op),
                format!("{}{{0}}", op) => format!("{}{{ 0 }}", op)
            );
        }
        for op in BinOp::variants().iter().map(ToString::to_string) {
            parse!(
                format!("a {} 0", op) => format!("a {} 0", op),
                format!("a {} b", op) => format!("a {} b", op),
                format!("a {} {{ b }}", op) => format!("a {} {{ b }}", op)
            );
        }
        for op in AssOp::variants().iter().map(ToString::to_string) {
            parse!(
                format!("{{ a {} 0; }}", op) => format!("{{ a {} 0; }}", op),
                format!("{{ a {} b; }}", op) => format!("{{ a {} b; }}", op),
                format!("{{ a {} {{ b }}; }}", op) => format!("{{ a {} {{ b }}; }}", op)
            );
        }
    }

    #[test]
    fn expr_parser() {
        macro_rules! parse {
            ($($input: expr => $expected: expr),+) => {
                $(
                    assert_eq!(ExprParser::new().parse($input).map(|output| format!("{}", output)), Ok($expected.to_string()));
                )+
            };
        }
        parse!(
            "{}" => "{ }",
            "a" => "a",
            "4" => "4",
            "{ a }" => "{ a }",
            "[4; 5 ]" => "[4; 5]",
            "0..=9" => "0..=9",
            "point.x" => "point.x",
            "call()" => "call()",
            "x.call()" => "x.call()",
            "x[0]" => "x[0]",
            "[0,1,2,3,4,5]" => "[0, 1, 2, 3, 4, 5]",
            "(0,1,2,3,4,4.5)" => "(0, 1, 2, 3, 4, 4.5)",
            "x as y" => "x as y",
            "if a >= 4 {}" => "if a >= 4 { }",
            "if a >= 4 {} else {}" => "if a >= 4 { } else { }",
            "if a >= 4 {} else if a < 0 {} else if a > 0 {} else {}" => "if a >= 4 { } else if a < 0 { } else if a > 0 { } else { }",
            "match x { 0 => {}, 1 => { y }, _ if x != 2 => { }, 2 => { } }" => "match x { 0 => { }, 1 => { y }, _ if x != 2 => { }, 2 => { } }",
            "{ return a; }" => "{ return a; }",
            "Struct { x, y, z }" => "Struct { x, y, z }",
            "Struct { x: a, y: b, z: c }" => "Struct { x: a, y: b, z: c }",
            "Struct { x, .. z }" => "Struct { x, .. z }",
            "Struct { .. z }" => "Struct { .. z }"
        );
    }

    // #[test]
    // fn file_parser() {
    //     macro_rules! parse {
    //         ($($input: expr => $expected: expr),+) => {
    //             $(
    //                 assert_eq!(FileParser::new().parse($input).map(|output| format!("{}", output)), Ok($expected.to_string()));
    //             )+
    //         };
    //     }
    //     parse!(
    //         include_str!("../examples/tmds_channel.rhdl") => ""
    //     );
    // }
}
