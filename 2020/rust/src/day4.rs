use std::collections::HashMap;

use regex::Regex;

pub fn run_day_four() {
    let inputs = inputs();
    let passports = parse_all_passports(&inputs);

    let part_1 = passports.iter().filter(|p| p.has_fields()).count();
    println!("Day 4, Part 1: {}", part_1);

    let part_2 = passports.iter().filter(|p| p.is_valid()).count();
    println!("Day 4, Part 2: {}", part_2);
}

fn inputs() -> String {
    crate::util::read_file("inputs/day4.txt")
}

#[derive(Debug)]
struct Passport<'text> {
    fields: HashMap<PassportField, &'text str>,
}

impl Passport<'_> {
    pub fn has_fields(&self) -> bool {
        self.fields.contains_key(&PassportField::BirthYear)
            && self.fields.contains_key(&PassportField::IssueYear)
            && self.fields.contains_key(&PassportField::ExpirationYear)
            && self.fields.contains_key(&PassportField::Height)
            && self.fields.contains_key(&PassportField::HairColour)
            && self.fields.contains_key(&PassportField::EyeColor)
            && self.fields.contains_key(&PassportField::PassportId)
    }

    pub fn is_valid(&self) -> bool {
        self.has_fields()
            && valid_year(self.fields[&PassportField::BirthYear], 1920, 2002)
            && valid_year(self.fields[&PassportField::IssueYear], 2010, 2020)
            && valid_year(self.fields[&PassportField::ExpirationYear], 2020, 2030)
            && valid_height(self.fields[&PassportField::Height])
            && valid_hair_color(self.fields[&PassportField::HairColour])
            && valid_eye_color(self.fields[&PassportField::EyeColor])
            && valid_passport_id(self.fields[&PassportField::PassportId])
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum PassportField {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColour,
    EyeColor,
    PassportId,
    CountryId,
}

lazy_static! {
    static ref HEIGHT_PATTERN: Regex = Regex::new(r"^(\d{2,3})(in|cm)$").unwrap();
    static ref HAIR_PATTERN: Regex = Regex::new(r"^#([0-9a-f]{6})$").unwrap();
    static ref EYE_PATTERN: Regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
    static ref PASSPORT_PATTERN: Regex = Regex::new(r"^\d{9}$").unwrap();
}

fn valid_passport_id(text: &str) -> bool {
    let valid = || -> Option<u64> {
        let captures = PASSPORT_PATTERN.captures(text)?;
        Some(captures.get(0)?.as_str().parse::<u64>().ok()?)
    };
    valid().is_some()
}

fn valid_eye_color(text: &str) -> bool {
    EYE_PATTERN.is_match(text)
}

fn valid_hair_color(text: &str) -> bool {
    HAIR_PATTERN.is_match(text)
}

fn valid_height(text: &str) -> bool {
    let valid = || -> Option<u16> {
        let captures = HEIGHT_PATTERN.captures(text)?;
        if captures.get(2)?.as_str() == "cm" {
            let height = captures.get(1)?.as_str().parse::<u16>().ok()?;
            if height >= 150 && height <= 193 {
                Some(height)
            } else {
                None
            }
        } else {
            let height = captures.get(1)?.as_str().parse::<u16>().ok()?;
            if height >= 59 && height <= 76 {
                Some(height)
            } else {
                None
            }
        }
    };

    valid().is_some()
}

fn valid_year(text: &str, earliest: u16, latest: u16) -> bool {
    if text.len() != 4 {
        return false;
    }

    match text.parse::<u16>().ok() {
        Some(year) => year >= earliest && year <= latest,
        None => false,
    }
}

fn parse_field_type(key: &str) -> Option<PassportField> {
    Some(match key {
        "byr" => PassportField::BirthYear,
        "iyr" => PassportField::IssueYear,
        "eyr" => PassportField::ExpirationYear,
        "hgt" => PassportField::Height,
        "hcl" => PassportField::HairColour,
        "ecl" => PassportField::EyeColor,
        "pid" => PassportField::PassportId,
        "cid" => PassportField::CountryId,
        _ => panic!("Unexpected passport field"),
    })
}

fn parse_all_passports(text: &str) -> Vec<Passport> {
    let mut result = Vec::new();
    let mut current_passport: Option<Passport> = None;

    for line in text.lines() {
        let line = line.trim();
        if line.len() == 0 {
            if let Some(passport) = current_passport.take() {
                result.push(passport);
            }
            continue;
        }

        if let None = current_passport {
            current_passport = Some(Passport {
                fields: HashMap::new(),
            });
        }

        let passport = current_passport.as_mut().unwrap();

        for pair in line.split(' ') {
            let colon = pair.find(':').expect("Each pair should have a colon");
            let (key, value) = pair.split_at(colon);
            let value = &value[1..];
            let key = parse_field_type(key).expect("Should have a valid field type");
            passport.fields.insert(key, value);
        }
    }

    if let Some(passport) = current_passport.take() {
        result.push(passport);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let text = r"
            ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
            byr:1937 iyr:2017 cid:147 hgt:183cm

            iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
            hcl:#cfa07d byr:1929

            hcl:#ae17e1 iyr:2013
            eyr:2024
            ecl:brn pid:760753108 byr:1931
            hgt:179cm

            hcl:#cfa07d eyr:2025 pid:166559648
            iyr:2011 ecl:brn hgt:59in";

        let passports = parse_all_passports(text);

        assert_eq!(2, passports.iter().filter(|p| p.has_fields()).count());
    }

    #[test]
    fn example_2() {
        let text = r"
            eyr:1972 cid:100
            hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
            
            iyr:2019
            hcl:#602927 eyr:1967 hgt:170cm
            ecl:grn pid:012533040 byr:1946
            
            hcl:dab227 iyr:2012
            ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
            
            hgt:59cm ecl:zzz
            eyr:2038 hcl:74454a iyr:2023
            pid:3556412378 byr:2007";

        let passports = parse_all_passports(text);

        assert_eq!(0, passports.iter().filter(|p| p.is_valid()).count());
    }

    #[test]
    fn example_3() {
        let text = r"
        pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
        hcl:#623a2f
        
        eyr:2029 ecl:blu cid:129 byr:1989
        iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm
        
        hcl:#888785
        hgt:164cm byr:2001 iyr:2015 cid:88
        pid:545766238 ecl:hzl
        eyr:2022
        
        iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        let passports = parse_all_passports(text);

        assert_eq!(4, passports.iter().filter(|p| p.is_valid()).count());
    }

    #[test]
    fn actual_inputs() {
        let inputs = inputs();
        let passports = parse_all_passports(&inputs);
        assert_eq!(230, passports.iter().filter(|p| p.has_fields()).count());
        assert_eq!(156, passports.iter().filter(|p| p.is_valid()).count());
    }
}
