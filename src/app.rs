use dioxus::{
    desktop::{use_window, LogicalSize},
    prelude::*,
};

use crate::{
    evaluate::evaluate_postfix,
    expression::{infix_to_postfix, possible_next_expressions, ExpressionToken, Operator},
};

const CSS: Asset = asset!("/assets/main.css");
const PICO_CSS: Asset = asset!("/assets/pico.min.css");

#[component]
pub fn App() -> Element {
    let window = use_window();
    let mut expressions = use_signal::<Vec<ExpressionToken>>(|| vec![]);
    let mut error = use_signal::<Option<String>>(|| None);
    let possible_next_expression = use_memo(move || possible_next_expressions(&expressions.read()));
    let panel_value = use_memo(move || {
        expressions
            .read()
            .iter()
            .map(ExpressionToken::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    });

    use_effect(move || window.set_inner_size(LogicalSize::new(335, 420)));

    let clear_handler = move |_| expressions.clear();

    let open_paren_handler = move |_| {
        let is_openable = possible_next_expression
            .read()
            .iter()
            .find(|&ex| *ex == ExpressionToken::OpenParen)
            .is_some();
        if is_openable {
            expressions.push(ExpressionToken::OpenParen)
        }
    };

    let close_paren_handler = move |_| {
        let is_closable = possible_next_expression
            .read()
            .iter()
            .find(|&ex| *ex == ExpressionToken::CloseParen)
            .is_some();
        if is_closable {
            expressions.push(ExpressionToken::CloseParen);
        };
    };

    let make_number_handler = |n: char| {
        move |_| {
            let mut expressions = expressions.write();
            match expressions.last_mut() {
                // Extending current number
                Some(ExpressionToken::Operand(current)) => current.push(n),
                _ => {
                    let is_continuable = possible_next_expression
                        .read()
                        .iter()
                        .find(|&ex| *ex == ExpressionToken::Operand("".to_string()))
                        .is_some();
                    if is_continuable {
                        expressions.push(ExpressionToken::Operand(n.to_string()));
                    }
                }
            };
        }
    };

    let make_operator_handler = |op: Operator| {
        move |_| {
            let can_add_operator = possible_next_expression
                .read()
                .iter()
                .find(|&ex| *ex == ExpressionToken::Operator(Operator::Plus))
                .is_some();
            if can_add_operator {
                expressions.push(ExpressionToken::Operator(op));
            }
        }
    };

    let equal_handler = move |_| {
        let mut expressions = expressions.write();
        match infix_to_postfix(&expressions) {
            Ok(postfix) => match evaluate_postfix(&postfix) {
                Ok(x) => {
                    expressions.clear();
                    expressions.push(ExpressionToken::Operand(x.to_string()));
                }
                Err(e) => error.set(Some(e)),
            },
            Err(e) => error.set(Some(e)),
        }
    };

    rsx! {
        document::Stylesheet { href: PICO_CSS }
        document::Stylesheet { href: CSS }
        main {
            input {
                name: "text",
                class: "number-panel",
                r#type: "text",
                value: panel_value,
                readonly: true,
                "aria-invalid": if error().is_some() { "true" }
            }
            input { r#type: "button", value: "AC", onclick: clear_handler }
            input { r#type: "button", value: "(", onclick: open_paren_handler }
            input { r#type: "button", value: ")", onclick: close_paren_handler }
            input {
                r#type: "button",
                value: "/",
                onclick: make_operator_handler(Operator::Divide),
            }
            input {
                r#type: "button",
                class: "outline",
                value: "7",
                onclick: make_number_handler('7'),
            }
            input {
                r#type: "button",
                class: "outline",
                value: "8",
                onclick: make_number_handler('8'),
            }
            input {
                r#type: "button",
                class: "outline",
                value: "9",
                onclick: make_number_handler('9'),
            }
            input {
                r#type: "button",
                value: "*",
                onclick: make_operator_handler(Operator::Multiply),
            }
            input {
                r#type: "button",
                class: "outline",
                value: "4",
                onclick: make_number_handler('4'),
            }
            input {
                r#type: "button",
                class: "outline",
                value: "5",
                onclick: make_number_handler('5'),
            }
            input {
                r#type: "button",
                class: "outline",
                value: "6",
                onclick: make_number_handler('6'),
            }
            input {
                r#type: "button",
                value: "-",
                onclick: make_operator_handler(Operator::Minus),
            }
            input {
                r#type: "button",
                class: "outline",
                value: "1",
                onclick: make_number_handler('1'),
            }
            input {
                r#type: "button",
                class: "outline",
                value: "2",
                onclick: make_number_handler('2'),
            }
            input {
                r#type: "button",
                class: "outline",
                value: "3",
                onclick: make_number_handler('3'),
            }
            input {
                r#type: "button",
                value: "+",
                onclick: make_operator_handler(Operator::Plus),
            }
            input {
                r#type: "button",
                class: "outline zero",
                value: "0",
                onclick: make_number_handler('0'),
            }
            input {
                r#type: "button",
                class: "outline",
                value: ".",
                onclick: make_number_handler('.'),
            }
            input { r#type: "button", value: "=", onclick: equal_handler }
        }
    }
}
