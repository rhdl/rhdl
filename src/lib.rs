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
                            type_hint: None,
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
    fn expr_parser() {
        macro_rules! parse {
            ($($($input: expr),+ => $expected: expr),+) => {
                $(
                    $(
                        assert_eq!(ExprParser::new().parse($input).map(|output| format!("{}", output)), Ok($expected.to_string()));
                    )+
                )+
            };
        }
        parse!(
            "a = b == c | d ^ e & f << g + h * i ** !j" => "(= a (== b (| c (^ d (& e (<< f (+ g (* h (** i (! j))))))))))",
            "a += !b ** c * d + e << f & g ^ h | i == j" => "(+= a (== (| (^ (& (<< (+ (* (** (! b) c) d) e) f) g) h) i) j))",
            "if a >= 4 { a = 4; b } else if a < 0 { a = 0; c } else { a +=1; d }" => "(if (>= a 4) ((= a 4); b) (else (if (< a 0) ((= a 0); c) (else (+= a 1); d))))"
        );
    }
}
