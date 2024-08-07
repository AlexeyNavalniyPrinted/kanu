use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fmt::Write;
use proc_macro2::{Ident, TokenStream};
use quote::{TokenStreamExt, ToTokens};
use regex::Regex;

#[derive(Debug, PartialEq)]
pub(crate) enum KeyWord {
    Select,
    Update,
    Delete,
    Create,
    And,
    Or,
    Param(String),
    None
}


impl KeyWord {
    pub(crate) fn split_to_keywords(input: &Ident, input_params: usize) -> Vec<KeyWord> {
        let re = Regex::new(r"(?i)(SelectBy|UpdateBy|DeleteBy|Create|And|Or)").unwrap();
        let mut param_amount = 0;
        let mut result = Vec::new();
        let mut last_index = 0;

        let input = input.clone().to_string();

        for mat in re.find_iter(&input) {
            if mat.start() > last_index {
                let param = input[last_index..mat.start()].to_string();
                result.push(KeyWord::Param(param.clone()));
                param_amount += 1;
            }
            result.push(match mat.as_str() {
                "SelectBy" => KeyWord::Select,
                "UpdateBy" => KeyWord::Update,
                "DeleteBy" => KeyWord::Delete,
                "Create" => KeyWord::Create,
                "And" => KeyWord::And,
                "Or" => KeyWord::Or,
                _ => unreachable!(),
            });
            last_index = mat.end();
        }

        if last_index < input.len() {
            let param = input[last_index..].to_string();
            result.push(KeyWord::Param(param.clone()));
            param_amount += 1;
        }

        Self::check_if_params_correct(param_amount, input_params, &result);

        result
    }

    fn check_if_params_correct(param_amount: usize, input_params: usize, result: &Vec<KeyWord>) {
        if param_amount == 0 {
            panic!("No parameters specified");
        }

        if param_amount != input_params {
            panic!("Amount of input parameters is not equal to specified by method parameters");
        }

        if !matches!(result[result.len() - 1], KeyWord::Param(_)) {
            panic!("\"{:?}\" is illegal as the last part", result[result.len() - 1]);
        }

        match result[0] {
            KeyWord::Select | KeyWord::Update | KeyWord::Delete | KeyWord::Create => {}
            _ => panic!("\"{:?}\" is illegal as the first part", result[0]),
        }

        let mut prev_keyword: &KeyWord = &KeyWord::None;

        for (index, keyword) in result.iter().enumerate() {
            if index > 0 {
                match keyword {
                    KeyWord::Select | KeyWord::Delete | KeyWord::Create | KeyWord::Update => {
                        panic!("It is illegal to have multiple actions");
                    }
                    KeyWord::Or | KeyWord::And => {
                        if prev_keyword == keyword {
                            panic!("It is illegal to have two same keywords in a row");
                        }
                        if !matches!(prev_keyword, KeyWord::Param(_)) {
                            panic!("It is illegal to have 'And' or 'Or' not followed by a parameter");
                        }
                    }
                    KeyWord::Param(_) => {
                        if matches!(prev_keyword, KeyWord::Param(_)) {
                            panic!("It is illegal to have two same parameters in a row");
                        }
                    }
                    KeyWord::None => {}
                }
            }
            prev_keyword = keyword;
        }
    }
}

impl Display for KeyWord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyWord::Select => write!(f, "select * from "),
            KeyWord::Update => write!(f, "update "),
            KeyWord::Delete => write!(f, "delete from "),
            KeyWord::Create => write!(f, "insert into "),
            KeyWord::And => write!(f, "and "),
            KeyWord::Or => write!(f, "or "),
            KeyWord::Param(param) => write!(f, "{} = ", param),
            KeyWord::None => write!(f, "none ")
        }
    }
}

