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
    Set,
    Delete,
    Create,
    And,
    Or,
    Param(String),
    By,
    ColumnName(String),
    None
}


impl KeyWord {

    pub(crate) fn split_to_keywords(input: &Ident, input_params: usize) -> Vec<KeyWord> {
        let re = Regex::new(r"(?i)(SelectBy|Set|DeleteBy|Create|And|Or|By)").unwrap();
        let mut param_amount = 0;
        let mut result = Vec::new();
        let mut last_index = 0;
        let mut column_amount = 0;
        let mut by_found = false;

        let input = input.to_string();

        for mat in re.find_iter(&input) {
            if mat.start() > last_index {
                let param = input[last_index..mat.start()].to_string();
                if result[0] == KeyWord::Set {
                    if by_found {
                        result.push(KeyWord::Param(param.clone()));
                        param_amount += 1;
                    } else {
                        result.push(KeyWord::ColumnName(param.clone()));
                        column_amount += 1;
                    }
                } else {
                    result.push(KeyWord::Param(param.clone()));
                    param_amount += 1;
                }
            }
            let keyword = match mat.as_str() {
                "SelectBy" => KeyWord::Select,
                "Set" => KeyWord::Set,
                "DeleteBy" => KeyWord::Delete,
                "Create" => KeyWord::Create,
                "And" => KeyWord::And,
                "Or" => KeyWord::Or,
                "By" => {
                    by_found = true;
                    KeyWord::By
                }
                _ => unreachable!(),
            };

            result.push(keyword);

            last_index = mat.end();
        }

        if last_index < input.len() {
            let param = input[last_index..].to_string();
            if result[0] == KeyWord::Set {
                if by_found {
                    result.push(KeyWord::Param(param.clone()));
                    param_amount += 1;
                } else {
                    result.push(KeyWord::ColumnName(param.clone()));
                    column_amount += 1;
                }
            } else {
                result.push(KeyWord::Param(param.clone()));
                param_amount += 1;
            }
        }

        Self::check_if_params_correct(param_amount, input_params, column_amount, &result);

        result
    }

    fn check_if_params_correct(param_amount: usize, input_params: usize, column_amount:usize, result: &Vec<KeyWord>) {
        if result[0] == KeyWord::Set && param_amount != column_amount {
            panic!("Amount of columns ({}) and amount of param ({}) should be equal", column_amount, param_amount)
        }

        if result[0] == KeyWord::Set && input_params != column_amount + param_amount {
            panic!("Amount of input params ({}) should be equal column amount ({}) + param amount ({}).", input_params, column_amount, param_amount)
        }

        if result[0] != KeyWord::Create  {
            if param_amount == 0 {
                println!("{:?}", result);
                panic!("No parameters specified");
            }
            if param_amount != input_params && result[0] != KeyWord::Set{
                panic!("Amount of input parameters is not equal to specified by method parameters");
            }

            if !matches!(result[result.len() - 1], KeyWord::Param(_)) {
                panic!("\"{:?}\" is illegal as the last part", result[result.len() - 1]);
            }
        }

        match result[0] {
            KeyWord::Select | KeyWord::Set | KeyWord::Delete | KeyWord::Create => {}
            _ => panic!("\"{:?}\" is illegal as the first part", result[0]),
        }

        let mut prev_keyword: &KeyWord = &KeyWord::None;

        for (index, keyword) in result.iter().enumerate() {
            if index > 0 {
                if prev_keyword == keyword {
                    panic!("It is illegal to have two same keywords in a row");
                }
                match keyword {
                    KeyWord::Select | KeyWord::Delete | KeyWord::Create | KeyWord::Set => {
                        panic!("It is illegal to have multiple actions");
                    }
                    KeyWord::Or | KeyWord::And => {
                        if !matches!(prev_keyword, KeyWord::Param(_) | KeyWord::ColumnName(_)) {
                            panic!("It is illegal to have 'And' or 'Or' not followed by a parameter");
                        }
                    }
                    KeyWord::Param(_) => {
                        if matches!(prev_keyword, KeyWord::Param(_)) {
                            panic!("It is illegal to have two same parameters in a row");
                        }
                    }
                    KeyWord::By => {
                        if matches!(prev_keyword, KeyWord::And) || matches!(prev_keyword, KeyWord::Or) {
                            panic!("It is illegal to have By after {:?}", prev_keyword)
                        }

                        if matches!(prev_keyword, KeyWord::Select | KeyWord::Create | KeyWord::Delete | KeyWord::Set) {
                            panic!("It is illegal to have By after {:?}", prev_keyword)
                        }
                    }
                    KeyWord::ColumnName(_) => {

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
            KeyWord::Set => write!(f, "update "),
            KeyWord::Delete => write!(f, "delete from "),
            KeyWord::Create => write!(f, "insert into "),
            KeyWord::And => write!(f, "and "),
            KeyWord::Or => write!(f, "or "),
            KeyWord::Param(param) => write!(f, " {} = ", param.to_ascii_lowercase()),
            KeyWord::By => write!(f, " where "),
            KeyWord::ColumnName(column_name) => write!(f, "{} = ", column_name.to_ascii_lowercase()),
            KeyWord::None => write!(f, "none "),
        }
    }
}

