// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::panic;
use std::error::Error;

use slint::{SharedString, ToSharedString};

slint::include_modules!();

#[derive(Debug)]
enum Operators {
    Number(f64),
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
}

/// Parses a string mathematical expression and converts it into a vector of `Operators`.
///
/// # Arguments
///
/// * `expr` - A string slice representing the mathematical expression to parse.
///
/// # Returns
///
/// * `Vec<Operators>` - A vector containing numbers and operators in the order they appear in the input.
fn operations(expr: &str) -> Vec<Operators> {
    let mut operators: Vec<Operators> = Vec::with_capacity(7);

    let chars: Vec<char> = expr.chars().collect();

    let mut i: usize = 0;
    while i < chars.len() {
        match chars[i] {
            '+' => {
                operators.push(Operators::Plus);
                i += 1;
            }
            '-' => {
                operators.push(Operators::Minus);
                i += 1;
            }
            '*' => {
                operators.push(Operators::Mul);
                i += 1;
            }
            '/' => {
                operators.push(Operators::Div);
                i += 1;
            }
            '%' => {
                operators.push(Operators::Mod);
                i += 1;
            }
            // '.' => {
            //     operators.push(Operators::Dot);
            //     i += 1;
            // }
            n if n.is_ascii_digit() || n == '.' => {
                let start = i;
                // let mut dot_present: bool = false;
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    // if chars[i] == '.' && dot_present {
                    i += 1;
                    // } else if chars[i] == '.' && !dot_present {
                    // dot_present = true;
                    // i += 1;
                    // }
                }

                let num: f64 = match expr[start..i].parse() {
                    Ok(val) => val,
                    Err(_) => panic!("Failed to parse the expression"),
                };

                operators.push(Operators::Number(num));
            }
            _ => {
                panic!("Invalid Character");
            }
        }
    }

    operators
}


/// Converts unary minus signs in the operator vector to negative numbers.
///
/// For example, `[-, Number(5.0)]` becomes `[Number(-5.0)]`.
///
/// # Arguments
///
/// * `ops` - A vector of parsed `Operators`, which may contain unary minus operators.
///
/// # Returns
///
/// * `Vec<Operators>` - A cleaned vector where unary minus signs are folded into negative `Number` values.
fn handle_unary_minus(mut ops: Vec<Operators>) -> Vec<Operators> {
    use Operators::*;

    let mut i = 0;
    while i < ops.len() {
        if let Minus = ops[i] {
            let is_unary = i == 0 || matches!(ops.get(i - 1), Some(Plus | Minus | Mul | Div | Mod));
            if is_unary {
                if let Some(Number(n)) = ops.get(i + 1) {
                    let neg = Number(-n);
                    ops.splice(i..=i + 1, [neg]);
                    continue; // stay at same i in case of multiple unary minus
                } else {
                    panic!("Invalid unary minus syntax");
                }
            }
        }
        i += 1;
    }

    ops
}

/// Evaluates a mathematical expression represented as a vector of `Operators`.
///
/// This function respects operator precedence:
/// 1. High precedence: `*`, `/`, `%`
/// 2. Low precedence: `+`, `-`
///
/// Unary minus is handled automatically before evaluation.
///
/// # Arguments
///
/// * `operators` - A vector of `Operators` parsed from a string expression.
///
/// # Returns
///
/// * `f64` - The result of the evaluated expression.
fn evaluate_expression(operators: Vec<Operators>) -> f64 {
    use Operators::*;
    let mut operators: Vec<Operators> = handle_unary_minus(operators);
    // Phase 1: High precedence: Div, Mul, Mod
    let mut i = 0;
    while i < operators.len() {
        match operators[i] {
            Mul | Div | Mod => {
                if let (Number(lhs), Number(rhs)) = (&operators[i - 1], &operators[i + 1]) {
                    let result = match operators[i] {
                        Mul => lhs * rhs,
                        Div => lhs / rhs,
                        Mod => lhs % rhs,
                        _ => unreachable!(),
                    };
                    operators.splice(i - 1..=i + 1, [Number(result)]);
                    i = 0; //restart pass
                } else {
                    panic!("Invalid syntax");
                }
            }
            _ => i += 1,
        }
    }

    //Phase 2: Low Precedance: Plus, Sub
    i = 0;
    while i < operators.len() {
        match operators[i] {
            Plus | Minus => {
                if let (Number(lhs), Number(rhs)) = (&operators[i - 1], &operators[i + 1]) {
                    let result = match operators[i] {
                        Plus => lhs + rhs,
                        Minus => lhs - rhs,
                        _ => unreachable!(),
                    };
                    operators.splice(i - 1..=i + 1, [Number(result)]);
                    i = 0; //restart pass
                } else {
                    panic!("Invalid syntax");
                }
            }
            _ => i += 1,
        }
    }

    if let Some(Number(result)) = operators.first() {
        *result
    } else {
        panic!("Failed to evaluate expression");
    }
}


/// Main function boby for GUI Window
fn main() -> Result<(), Box<dyn Error>> {
    let match_opp: [char; 6] = ['+', '-', '*', '/', '.', '%'];

    let ui: AppWindow = AppWindow::new()?;

    ui.on_request_btn_1({
        let ui: AppWindow = ui.as_weak().unwrap();
        move || {
            let string: &str = "1";
            let screen = ui.get_screen();
            ui.set_screen(screen + string);
        }
    });

    ui.on_request_btn_2({
        let ui: AppWindow = ui.as_weak().unwrap();

        move || {
            let string: &str = "2";
            let screen = ui.get_screen();
            ui.set_screen(screen + string);
        }
    });
    ui.on_request_btn_3({
        let ui: AppWindow = ui.as_weak().unwrap();

        move || {
            let string: &str = "3";
            let screen = ui.get_screen();
            ui.set_screen(screen + string);
        }
    });
    ui.on_request_btn_4({
        let ui: AppWindow = ui.as_weak().unwrap();

        move || {
            let string: &str = "4";
            let screen = ui.get_screen();
            ui.set_screen(screen + string);
        }
    });
    ui.on_request_btn_5({
        let ui: AppWindow = ui.as_weak().unwrap();

        move || {
            let string: &str = "5";
            let screen = ui.get_screen();
            ui.set_screen(screen + string);
        }
    });
    ui.on_request_btn_6({
        let ui: AppWindow = ui.as_weak().unwrap();

        move || {
            let string: &str = "6";
            let screen = ui.get_screen();
            ui.set_screen(screen + string);
        }
    });
    ui.on_request_btn_7({
        let ui: AppWindow = ui.as_weak().unwrap();

        move || {
            let string: &str = "7";
            let screen = ui.get_screen();
            ui.set_screen(screen + string);
        }
    });
    ui.on_request_btn_8({
        let ui: AppWindow = ui.as_weak().unwrap();

        move || {
            let string: &str = "8";
            let screen = ui.get_screen();
            ui.set_screen(screen + string);
        }
    });
    ui.on_request_btn_9({
        let ui: AppWindow = ui.as_weak().unwrap();

        move || {
            let string: &str = "9";
            let screen = ui.get_screen();
            ui.set_screen(screen + string);
        }
    });
    ui.on_request_btn_0({
        let ui: AppWindow = ui.as_weak().unwrap();

        move || {
            let string: &str = "0";
            let screen = ui.get_screen();
            ui.set_screen(screen + string);
        }
    });
    ui.on_request_btn_plus({
        let ui: AppWindow = ui.as_weak().unwrap();

        move || {
            let screen = ui.get_screen();

            if !screen.is_empty() && !match_opp.contains(&screen.chars().last().unwrap()) {
                let string: &str = "+";
                ui.set_screen(screen + string);
            }
        }
    });
    ui.on_request_btn_minus({
        let ui: AppWindow = ui.as_weak().unwrap();

        let match_opp: [char; 5] = ['+', '*', '/', '.', '%'];

        move || {
            let screen = ui.get_screen();

            let target: char = '-';
            let string = "-";
            let last: Option<char> = screen.chars().last();
            let last_second: Option<char> = screen.chars().rev().nth(1);

            if screen.is_empty() {
                ui.set_screen(screen + string);
            } else if last.unwrap() != target && !match_opp.contains(&last.unwrap()) {
                ui.set_screen(screen + string);
            } else {
                if let Some(l) = last
                    && let Some(ls) = last_second
                {
                    if (l == target) && (ls != target) && !match_opp.contains(&last.unwrap()) {
                        ui.set_screen(screen + string);
                    }
                }
            }
        }
    });
    ui.on_request_btn_mod({
        let ui: AppWindow = ui.as_weak().unwrap();

        move || {
            let screen = ui.get_screen();

            if !screen.is_empty() && !match_opp.contains(&screen.chars().last().unwrap()) {
                let string: &str = "%";
                ui.set_screen(screen + string);
            }
        }
    });
    ui.on_request_btn_div({
        let ui: AppWindow = ui.as_weak().unwrap();

        move || {
            let screen = ui.get_screen();
            if !screen.is_empty() && !match_opp.contains(&screen.chars().last().unwrap()) {
                let string: &str = "/";
                ui.set_screen(screen + string);
            }
        }
    });
    ui.on_request_btn_cc({
        let ui: AppWindow = ui.as_weak().unwrap();

        move || {
                ui.set_screen(SharedString::new());
        }
    });

    ui.on_request_btn_mul({
        let ui: AppWindow = ui.as_weak().unwrap();

        move || {
            let screen = ui.get_screen();

            if !screen.is_empty() && !match_opp.contains(&screen.chars().last().unwrap()) {
                let string: &str = "*";
                ui.set_screen(screen + string);
            }
        }
    });
    ui.on_request_btn_equal({
        let ui: AppWindow = ui.as_weak().unwrap();
        move || {
            let screen = ui.get_screen();

            if !screen.is_empty() && !match_opp.contains(&screen.chars().last().unwrap()) {
                // println!("{}", screen.chars().last().unwrap());
                let enum_parsed: Vec<Operators> = operations(&(screen.to_string()));
                // println!("{:?}", &enum_parsed);
                let result: f64 = evaluate_expression(enum_parsed);
                ui.set_answer(result.to_shared_string());
                ui.set_screen(result.to_shared_string());
            }
        }
    });
    ui.on_request_btn_dot({
        let ui: AppWindow = ui.as_weak().unwrap();
        move || {
            let screen = ui.get_screen();

            if !screen.is_empty()
                && !match_opp.contains(&screen.chars().last().unwrap())
                && !screen.contains('.')
            {
                let string: &str = ".";
                ui.set_screen(screen + string);
            }
        }
    });

    ui.run()?;

    Ok(())
}
