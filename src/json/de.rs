use std::str::Chars;
use std::collections::HashMap;

use super::value::Value;
use super::value::Number;
use super::value::Num;

pub struct De<'a> {
    program: Chars<'a>,
    peek: Option<char>,
}

impl De<'_> {
    pub fn new(str: &String) -> De {
        De {
            program: str.chars(),
            peek: Some(' '),
        }
    }

    pub fn deserialization(&mut self) -> Result<Value, String> {
        &self.next();
        self.match_object()
    }

    pub fn next(&mut self) -> Option<char> {
        self.peek = self.program.next();
        self.peek
    }

    pub fn ignore_space(&mut self) {
        loop {
            match self.next() {
                Some(' ') => {
                    continue;
                }
                Some('\n') => {
                    continue;
                }
                _ => {
                    break;
                }
            }
        };
    }

    pub fn match_value(&mut self) -> Result<Value, String> {
        self.next();

        match self.peek {
            Some('{') => {
                self.match_object()
            }
            Some('[') => {
                self.match_array()
            }
            Some(c) => {
                if c == 't' {
                    let mut char_list: Vec<char> = Vec::new();

                    char_list.push('t');
                    loop {
                        self.next();
                        match self.peek {
                            Some(c) => {
                                char_list.push(c);
                            }
                            None => {
                                let accept: String = char_list.into_iter().collect();
                                break Err(String::from("Expect: null, accept: ".to_owned() + &accept))
                            }
                        }
                        if char_list.len() == 4 {
                            let current: String = char_list.iter().collect();
                            if current == String::from("true") {
                                break Ok(Value::Boolean(true))
                            } else {
                                let accept: String = char_list.into_iter().collect();
                                break Err(String::from("Expect: null, accept: ".to_owned() + &accept))
                            }
                        } else {
                            continue
                        }
                    }
                } else if c == 'f' {
                    let mut char_list: Vec<char> = Vec::new();

                    char_list.push('f');
                    loop {
                        self.next();
                        match self.peek {
                            Some(c) => {
                                char_list.push(c);
                            }
                            None => {
                                let accept: String = char_list.into_iter().collect();
                                break Err(String::from("Expect: null, accept: ".to_owned() + &accept))
                            }
                        }
                        if char_list.len() == 5 {
                            let current: String = char_list.iter().collect();
                            if current== String::from("false") {
                                break Ok(Value::Boolean(true))
                            } else {
                                let accept: String = char_list.into_iter().collect();
                                break Err(String::from("Expect: null, accept: ".to_owned() + &accept))
                            }
                        } else {
                            continue
                        }
                    }
                } else if c == 'n' {
                    let mut char_list: Vec<char> = Vec::new();

                    char_list.push('f');
                    loop {
                        self.next();
                        match self.peek {
                            Some(c) => {
                                char_list.push(c);
                            }
                            None => {
                                let accept: String = char_list.into_iter().collect();
                                break Err(String::from("Expect: null, accept: ".to_owned() + &accept))
                            }
                        }
                        if char_list.len() == 4 {
                            let current: String = char_list.iter().collect();
                            if current == String::from("null") {
                                break Ok(Value::Boolean(true))
                            } else {
                                let accept: String = char_list.into_iter().collect();
                                break Err(String::from("Expect: null, accept: ".to_owned() + &accept))
                            }
                        } else {
                            continue
                        }
                    }
                } else {
                    match self.match_number() {
                        Ok(num) => {
                            Ok(Value::Number(num))
                        }
                        Err(message) => {
                            match self.match_string() {
                                Ok(str) => {
                                    Ok(Value::String(str))
                                }
                                Err(message) => {
                                    Err(message)
                                }
                            }
                        }
                    }
                }
            }
            None => {
                Err(String::from("invalid string format"))
            }
        }
    }

    pub fn match_object(&mut self) -> Result<Value, String> {
        match self.peek {
            Some(c) => {
                if c == '{' {
                    self.ignore_space();
                    let map = self.match_members();
                    match map {
                        Ok(map) => {
                            match self.match_spec_char('}') {
                                Ok(_) => {
                                    Ok(Value::Object(map))
                                }
                                Err(message) => {
                                    Err(String::from(message))
                                }
                            }
                        }
                        Err(message) => {
                            Err(String::from(message))
                        }
                    }
                } else {
                    Err(String::from(""))
                }
            }
            None => {
                Err(String::from(""))
            }
        }
    }


    pub fn match_members(&mut self) -> Result<HashMap<String, Value>, String> {
        let mut map: HashMap<String, Value> = HashMap::new();
        loop {
            let member = self.match_member();
            match member {
                Ok((key, value)) => {
                    map.insert(key, value);
                    self.next();
                    match self.match_spec_char(',') {
                        Ok(_) => {
                            continue
                        }
                        Err(message) => {
                            break Ok(map)
                        }
                    }
                }
                Err(message) => {
                    match message {
                        Ok(code) => {
                            break Ok(map)
                        }
                        Err(message) => {
                            break Err(String::from(message))
                        }
                    }
                }
            }
        }
    }

    pub fn match_member(&mut self) -> Result<(String, Value), Result<u8, String>> {
        self.ignore_space();
        match self.match_spec_char('}') {
            Ok(_) => {
                Err(Ok(0))
            }
            Err(message) => {
                let key = self.match_string();
                match key {
                    Ok(key) => {
                        let colon = self.match_spec_char(':');
                        match colon {
                            Ok(_) => {
                                let value = self.match_element();
                                match value {
                                    Ok(value) => {
                                        Ok((key, value))
                                    }
                                    Err(message) => {
                                        Err(Err(String::from(message)))
                                    }
                                }
                            }
                            Err(message) => {
                                Err(Err(String::from(message)))
                            }
                        }
                    }
                    Err(message) => {
                        Err(Err(String::from(message)))
                    }
                }
            }
        }
    }

    pub fn match_array(&mut self) -> Result<Value, String> {
        match self.peek {
            Some(c) => {
                if c == '[' {
                    self.ignore_space();
                    let list = self.match_elements();
                    match list {
                        Ok(list) => {
                            match self.match_spec_char(']') {
                                Ok(_) => {
                                    Ok(Value::Array(list))
                                }
                                Err(message) => {
                                    Err(String::from(message))
                                }
                            }
                        }
                        Err(message) => {
                            Err(String::from(message))
                        }
                    }
                } else {
                    Err(String::from(""))
                }
            }
            None => {
                Err(String::from(""))
            }
        }
    }

    pub fn match_elements(&mut self) -> Result<Vec<Value>, String> {
        let mut list: Vec<Value> = Vec::new();
        loop {
            let element = self.match_element();
            match element {
                Ok(value) => {
                    list.push(value);
                    self.next();
                    match self.match_spec_char(',') {
                        Ok(_) => {
                            continue;
                        }
                        Err(message) => {
                            break Ok(list)
                        }
                    }
                }
                Err(message) => {
                    break Err(String::from(message))
                }
            }
        }
    }

    pub fn match_element(&mut self) -> Result<Value, String> {
        self.ignore_space();
        let value = self.match_value();
        self.ignore_space();

        value
    }

    pub fn match_string(&mut self) -> Result<String, String> {
        match self.peek {
            Some(c) => {
                let mut str_slice = Vec::new();

                loop {
                    self.next();
                    match self.match_letter() {
                        Ok(c) => {
                            str_slice.push(c)
                        }
                        Err(message) => {
                            break;
                        }
                    }
                }

                Ok(str_slice.into_iter().collect())
            }
            None => {
                Err(String::from(""))
            }
        }
    }

    pub fn match_number(&mut self) -> Result<Number, String> {
        match self.peek {
            Some(c) => {
                loop {
                    self.next();
                    match self.match_integer() {
                        Ok(int) => {
                            self.next();
                            match self.match_spec_char('.') {
                                Ok(_) => {
                                    match self.match_integer() {
                                        Ok(fraction) => {
                                            break Ok(
                                                Number {
                                                    num: Num::Float((int + (fraction / (fraction.to_string().len() as u32))) as f64)
                                                }
                                            )
                                        }
                                        Err(message) => {
                                            break Err(String::from(message) as String)
                                        }
                                    }
                                }
                                Err(message) => {
                                    match self.match_spec_char('e') {
                                        Ok(_) => {
                                            match self.match_integer() {
                                                Ok(e) => {
                                                    break Ok(
                                                        Number {
                                                            num: Num::PosInt((int * 10 ^ e) as u64)
                                                        }
                                                    )
                                                }
                                                Err(message) => {
                                                    break Err(String::from(message))
                                                }
                                            }
                                        }
                                        Err(ref message) => {
                                            break Err(String::from(message))
                                        }
                                    }
                                }
                            }
                        }
                        Err(ref message) => {
                            break Err(String::from(message))
                        }
                    }
                }
            }
            None => {
                Err(String::from(""))
            }
        }
    }

    pub fn match_integer(&mut self) -> Result<u32, String> {
        match self.peek {
            Some(c) => {
                let mut int: u32 = 0;
                let mut count: u16 = 0;

                loop {
                    self.next();
                    break match self.match_digit() {
                        Ok(c) => {
                            match c.to_digit(10) {
                                Some(n) => {
                                    int = (int * (10 ^ count) as u32) + n;
                                    continue
                                }
                                None => {
                                    break Ok(int)
                                }
                            }
                        }
                        Err(ref message) => {
                            break Err(message.to_string())
                        }
                    }
                }
            }
            None => {
                Err(String::from(""))
            }
        }
    }

    pub fn match_true(&mut self) -> Result<bool, String> {
        match self.match_spec_char('t') {
            Ok(c) => {
                let mut str = String::from("");
                let mut index = 0;
                loop {
                    index += 1;
                    match self.next() {
                        Some(c) => {
                            str.push(c);
                            if index >= 3 {
                                if "true" == str {
                                    break Ok(true)
                                } else {
                                    break Err(String::from("expected: true, found: ") + str.as_str())
                                }
                            }
                        }
                        None => {
                            break Err(String::from(""))
                        }
                    }
                }
            }
            Err(message) => {
                Err(String::from(message))
            }
        }
    }

    pub fn match_letter(&mut self) -> Result<char, String> {
        match self.peek {
            Some(c) => {
                if c.is_alphabetic() {
                    Ok(c)
                } else {
                    Err(String::from(""))
                }
            }
            None => {
                Err(String::from(""))
            }
        }
    }

    pub fn match_digit(&mut self) -> Result<char, String> {
        match self.peek {
            Some(c) => {
                if c.is_digit(10) {
                    Ok(c)
                } else {
                    Err(String::from(""))
                }
            }
            None => {
                Err(String::from(""))
            }
        }
    }

    pub fn match_spec_char(&mut self, ic: char) -> Result<char, String> {
        match self.peek {
            Some(c) => {
                if c == ic {
                    Ok(c)
                } else {
                    Err(String::from("expected: ".to_owned() + &ic.to_string() + ", found: " + &c.to_string()))
                }
            }
            None => {
                Err(String::from("current string parser ending."))
            }
        }
    }
}