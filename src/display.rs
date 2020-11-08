use super::ast::token::Tok;

pub fn format(tokens: Vec<Tok>) -> String {
    let mut acc = String::new();

    for (i, token) in tokens.iter().enumerate() {
        acc += &token.to_string();
        if i + 1 != tokens.len() {
            acc += match spacing_hint(token, &tokens[..i], &tokens[i + 1..]) {
                SpacingHint::Any => " ",
                SpacingHint::None => "",
                SpacingHint::LineBreak => "\n",
            };
        }
    }
    acc
}

enum SpacingHint {
    Any,
    LineBreak,
    None,
}

fn spacing_hint<'a>(token: &'a Tok, before: &'a [Tok], after: &'a [Tok]) -> SpacingHint {
    use Tok::*;
    match token {
        As(_) | Break(_) | Const(_) | Continue(_) | Else(_) | Enum(_) | Extern(_) | For(_)
        | If(_) | Impl(_) | In(_) | Let(_) | Loop(_) | Match(_) | Mod(_) | Move(_) | Mut(_)
        | Ref(_) | Return(_) | UpperSelf(_) | Static(_) | Struct(_) | Trait(_) | TokenType(_)
        | Unsafe(_) | Use(_) | Where(_) | While(_) | Async(_) | Await(_) | Dyn(_) | Abstract(_)
        | Become(_) | TokenBox(_) | Do(_) | Final(_) | Macro(_) | Override(_) | Priv(_)
        | Typeof(_) | Unsized(_) | Virtual(_) | Yield(_) | Try(_) | Union(_) | Entity(_)
        | Bag(_) | Ring(_) | Arch(_) | When(_) | Out(_) | InOut(_) => SpacingHint::Any,

        Plus(_) | Star(_) | StarStar(_) | Slash(_) | Percent(_) | Caret(_) | And(_) | Or(_)
        | AndAnd(_) | OrOr(_) | Shl(_) | Shr(_) | PlusEq(_) | MinusEq(_) | StarEq(_)
        | StarStarEq(_) | SlashEq(_) | PercentEq(_) | CaretEq(_) | AndEq(_) | OrEq(_)
        | ShlEq(_) | ShrEq(_) | Eq(_) | EqEq(_) | Ne(_) | Gt(_) | Lt(_) | Ge(_) | Le(_) | At(_)
        | Comma(_) | Colon(_) | RArrow(_) | FatArrow(_) | Pound(_) | Dollar(_) | Question(_)
        | BracketClose(_) | ParenClose(_) | BraceOpen(_) => match after.first() {
            Some(Semi(_)) => SpacingHint::None,
            Some(Comma(_)) => SpacingHint::None,
            _ => SpacingHint::Any,
        },

        // TODO: semi is also valid in the context of a repeat expression
        Semi(_) => match after.first() {
            Some(Ident(_)) | Some(Lit(_)) | Some(ParenOpen(_)) | Some(BraceClose(_)) => {
                SpacingHint::Any
            }
            _ => SpacingHint::LineBreak,
        },

        BraceClose(_) => match after.first() {
            Some(Semi(_)) | Some(Comma(_)) => SpacingHint::None,
            Some(BraceClose(_)) => SpacingHint::Any,
            _ => SpacingHint::LineBreak,
        },

        Pub(_) | Fn(_) => match after.first() {
            Some(ParenOpen(_)) => SpacingHint::None,
            _ => SpacingHint::Any,
        },

        PathSep(_) | ParenOpen(_) | Dot(_) => SpacingHint::None,

        DotDot(_) | DotDotEq(_) => match before.last() {
            Some(BraceOpen(_)) | Some(ParenOpen(_)) => SpacingHint::Any,
            Some(Comma(_)) => match after.first() {
                Some(BraceClose(_)) | Some(Lit(_)) | Some(Ident(_)) => SpacingHint::Any,
                _ => SpacingHint::None,
            },
            _ => SpacingHint::None,
        },

        BracketOpen(_) => match after.first() {
            Some(Lit(_)) | Some(Ident(_)) => SpacingHint::None,
            _ => SpacingHint::Any,
        },

        Not(_) => SpacingHint::None,
        Minus(_) => match before.last() {
            Some(Ident(_)) | Some(Lit(_)) | Some(BraceClose(_)) | Some(ParenClose(_))
            | Some(BracketOpen(_)) => SpacingHint::Any,
            _ => SpacingHint::None,
        },

        Lit(_) | Ident(_) | Underscore(_) | Super(_) | LowerSelf(_) | Crate(_) => {
            match after.first() {
                Some(Gt(_)) => match after.first() {
                    Some(ParenOpen(_)) => SpacingHint::None,
                    _ => SpacingHint::Any,
                },
                Some(Colon(_))
                | Some(Semi(_))
                | Some(BracketClose(_))
                | Some(DotDot(_))
                | Some(DotDotEq(_))
                | Some(Dot(_))
                | Some(ParenOpen(_))
                | Some(ParenClose(_))
                | Some(BracketOpen(_))
                | Some(Comma(_))
                | Some(PathSep(_)) => SpacingHint::None,
                _ => SpacingHint::Any,
            }
        }
    }
}
