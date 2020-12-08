use core::str::FromStr;
trait InRange {
    fn is_value_in_range<T: PartialOrd + FromStr>(&self, min: T, max: T) -> bool;
}

impl InRange for str {
    fn is_value_in_range<T: PartialOrd + FromStr>(&self, min: T, max: T) -> bool {
        match self.parse::<T>() {
            Err(_) => false,
            Ok(x) => x >= min && x <= max,
        }
    }
}
#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
}

impl Passport {
    fn new() -> Self {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
        }
    }

    fn from_lines(input: &str) -> Self {
        let mut p = Passport::new();
        for l in input.lines() {
            for f in l.split_ascii_whitespace() {
                let mut fields = f.split(':');
                match fields.next() {
                    Some("byr") => p.byr = fields.next().map(|x| x.to_string()),
                    Some("iyr") => p.iyr = fields.next().map(|x| x.to_string()),
                    Some("eyr") => p.eyr = fields.next().map(|x| x.to_string()),
                    Some("hgt") => p.hgt = fields.next().map(|x| x.to_string()),
                    Some("hcl") => p.hcl = fields.next().map(|x| x.to_string()),
                    Some("ecl") => p.ecl = fields.next().map(|x| x.to_string()),
                    Some("pid") => p.pid = fields.next().map(|x| x.to_string()),
                    Some("cid") => {},
                    _ => unreachable!(),
                }
            }
        }
        p
    }
    fn is_valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_byr_valid(&self) -> bool {
        match &self.byr {
            None => false,
            Some(x) => x.is_value_in_range(1920, 2002),
        }
    }

    fn is_iyr_valid(&self) -> bool {
        match &self.iyr {
            None => false,
            Some(x) => x.is_value_in_range(2010, 2020),
        }
    }

    fn is_eyr_valid(&self) -> bool {
        match &self.eyr {
            None => false,
            Some(x) => x.is_value_in_range(2020, 2030),
        }
    }

    fn is_hgt_valid(&self) -> bool {
        match &self.hgt {
            None => false,
            Some(x) => {
                let hgt_digits = x.chars().filter(|x| x.is_ascii_digit()).collect::<String>();
                let hgt_unit = x.split_at(hgt_digits.len());
                match hgt_unit.1 {
                    "cm" => hgt_digits.is_value_in_range(150, 193),
                    "in" => hgt_digits.is_value_in_range(59, 76),
                    _ => false,
                }
            }
        }
    }

    fn is_hcl_valid(&self) -> bool {
        match &self.hcl {
            Some(x) if x.len() == 7 => {
                x[0..1] == String::from("#")
                    && x[1..].chars().filter(|x| x.is_digit(16)).count() == 6
            }
            _ => false,
        }
    }

    fn is_ecl_valid(&self) -> bool {
        match self.ecl.as_ref().map(|x| x.as_str()) {
            Some("amb") | Some("blu") | Some("brn") | Some("gry") | Some("grn") | Some("hzl")
            | Some("oth") => true,
            _ => false,
        }
    }

    fn is_pid_valid(&self) -> bool {
        match &self.pid {
            Some(pid) => pid.chars().filter(|x| x.is_ascii_digit()).count() == 9,
            None => false,
        }
    }
    
    fn is_valid_extended(&self) -> bool {
        self.is_byr_valid()
            && self.is_iyr_valid()
            && self.is_eyr_valid()
            && self.is_hgt_valid()
            && self.is_hcl_valid()
            && self.is_ecl_valid()
            && self.is_pid_valid()
    }
}

fn read_passports(input: &str) -> Vec<Passport> {
    input.split("\n\n").map(|p| Passport::from_lines(p)).collect::<Vec<_>>()
}

use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = std::fs::File::open("input.txt")?;
    let mut buffer = String::new();
    input.read_to_string(&mut buffer)?;
    dbg!(&buffer);
    //let passports = read_passports(buffer.lines().collect::<Vec<_>>());
    let passports = read_passports(&buffer);

    println!(
        "valid passports {}",
        passports.iter().filter(|x| x.is_valid()).count()
    );
    println!(
        "extended check valid passports {}",
        passports.iter().filter(|x| x.is_valid_extended()).count()
    );
    Ok(())
}

#[test]

fn test0() {
    let example =
        r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm

        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929

        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm

        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in"#;
    

        assert_eq!(
        read_passports(example)
            .iter()
            .filter(|x| x.is_valid())
            .count(),
        2
    );
}

#[test]

fn test1() {
    let example = 
        r#"eyr:1972 cid:100
        hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

        iyr:2019
        hcl:#602927 eyr:1967 hgt:170cm
        ecl:grn pid:012533040 byr:1946

        hcl:dab227 iyr:2012
        ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

        hgt:59cm ecl:zzz
        eyr:2038 hcl:74454a iyr:2023
        pid:3556412378 byr:2007"#;    
    assert_eq!(
        read_passports(example)
            .iter()
            .filter(|x| x.is_valid_extended())
            .count(),
        0
    );
}

#[test]

fn test2() {
    let example = 
        r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
        hcl:#623a2f

        eyr:2029 ecl:blu cid:129 byr:1989
        iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

        hcl:#888785
        hgt:164cm byr:2001 iyr:2015 cid:88
        pid:545766238 ecl:hzl
        eyr:2022

        iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"#;
    
    assert_eq!(
        read_passports(example)
            .iter()
            .filter(|x| x.is_valid_extended())
            .count(),
        4
    );
}
