use rand::prelude::*;

pub struct SessionProtector {
    pub hash: String
}

impl SessionProtector {
    pub fn next_session_key(&self, session_key:&String)->String {
        if self.hash == "".to_string(){panic!("Hash code is empty");}
        for idx in 0..self.hash.len() {
            if !self.hash.chars().nth(idx).unwrap().is_digit(10) {panic!("Hash code contains non-digit letter")}
        }
        let mut result = 0;
        for idx in 0..self.hash.len() {
            result = result + self.calc_hash(&session_key, self.hash.chars().nth(idx).unwrap().to_digit(10).unwrap() as i64).parse::<i64>().unwrap();
        }
        let result = if result.to_string().len() > 10 {result.to_string()[0..10].to_string()} else {result.to_string()};
        let result = format!("{}{}", "0".repeat(10), result);
        result[result.len()-10..result.len()].to_string()
    }

    fn calc_hash(&self, session_key:&String, val:i64)->String {
        let mut result = "".to_string();
        match val {
            1 => {
                result = format!("00{}", (session_key[0..5].parse::<i64>().unwrap() % 97).to_string()).to_string();
                return (&result[result.len()-2..result.len()]).to_string()
            },
            2 => {
                for i in 1..session_key.len() {
                    result = format!("{}{}", result, session_key.chars().nth(session_key.len()-i).unwrap()).to_string();
                }
                return format!("{}{}", result, session_key.chars().nth(0).unwrap())
            },
            3 => {
                return format!("{}{}", session_key[session_key.len()-5..session_key.len()].to_string(), session_key[0..5].to_string()).to_string()
            },
            4 => {
                let mut num  = 0;
                for i in 1..9 {
                    num += session_key.chars().nth(i).unwrap() as i64 + 41
                }
                return num.to_string()
            },
            5 => {
                let mut num = 0;
                for i in 0..session_key.len() {
                    let ch = session_key.chars().nth(i).unwrap() as i64 ^ 43 ;
                    let ch = if !(ch as u8 as char).is_digit(10) {ch.to_string()} else {(ch as u8 as char).to_string()};
                    num += ch.parse::<i64>().unwrap();
                }
                return num.to_string()
            },
            _ => return (session_key.parse::<i64>().unwrap() + val).to_string()
        }
    }
}

pub fn get_hash_str()->String {
    let mut result:String = "".to_string();
    for _ in 0..5 {
        let random: f64 = rand::thread_rng().gen();
        result = format!("{}{}", result, (6.0 * random + 1.0).round()).to_string();
    }
    result
}

pub fn get_session_key()->String {
    let mut result:String = "".to_string();
    for _ in 1..11 {
        let random: f64 = rand::thread_rng().gen();
        result = format!("{}{}", result, (9.0 * random + 1.0).round().to_string().chars().nth(0).unwrap()).to_string();
    }
    result
}
