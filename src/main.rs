#![allow(unused)]
/// applies the given operator to the two operands.
fn apply_operator(op: char, a: i64, b: i64) -> i64 {
    match op {
        'a' => a + b,
        'b' => a - b,
        'c' => a * b,
        'd' => a / b,
        _ => panic!("Invalid operator"),
    }
}

/// evaluates the expression starting at the given index.
/// special characters: e - start of a new expression, f - end of an expression.
/// these characters explicitly denote precedence by grouping parts of an expression that should be evaluated first
fn evaluate_expression(input: &str, start: &mut usize) -> i64 {
    let mut num = 0;
    let mut result = 0;
    let mut op = 'a';

    while *start < input.len() {
        let c = input.chars().nth(*start);
        if let Some(ch) = c {
            *start += 1;

            match ch {
                '0'..='9' => {
                    num = num * 10 + (ch as i64 - '0' as i64);
                }
                'a' | 'b' | 'c' | 'd' => {
                    result = apply_operator(op, result, num);
                    op = ch;
                    num = 0;
                }
                'e' => {
                    num = evaluate_expression(input, start);
                }
                'f' => break,
                _ => unreachable!(),
            }
        } else {
            break;
        }
    }

    apply_operator(op, result, num)
}

/// evaluates the given input string, from left to right.
fn evaluate(input: &str) -> i64 {
    let mut start = 0;
    evaluate_expression(input, &mut start)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(evaluate("3a2c4"), 20);
        assert_eq!(evaluate("32a2d2"), 17);
        assert_eq!(evaluate("500a10b66c32"), 14208);
        assert_eq!(evaluate("3ae4c66fb32"), 235);
        assert_eq!(evaluate("3c4d2aee2a4c41fc4f"), 990);
    }
}
