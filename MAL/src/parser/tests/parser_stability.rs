use mal_arena::Arena;
use mal_parser::parse;

fn parses(src: &str) -> bool {
    let mut arena = Arena::new(128);
    parse(src, &mut arena).is_ok()
}

#[test]
fn direct_call_parses() {
    assert!(parses("f(x)"));
}

#[test]
fn multi_arg_call_parses() {
    assert!(parses("f(x, y + 1)"));
}

#[test]
fn additive_multiplicative_expression_parses() {
    assert!(parses("1 + 2 · 3"));
}

#[test]
fn subtraction_chain_parses() {
    assert!(parses("1 - 2 - 3"));
}

#[test]
fn missing_call_paren_fails_closed() {
    assert!(!parses("f(x"));
}

#[test]
#[ignore = "postfix calls such as (f)(x) are a future parser feature"]
fn parenthesized_callee_is_future_feature() {
    assert!(parses("(f)(x)"));
}
