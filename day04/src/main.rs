use regex::Regex;

#[derive(Clone)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Default for Passport {
    fn default() -> Self {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let passports: Vec<Passport> = input.trim().split("\n\n").map(parse_passport).collect();
    let num_valid = passports.into_iter().filter(passport_is_valid).count();

    println!("Part 1: {}", num_valid);
}

fn parse_passport(input: &str) -> Passport {
    let mut p: Passport = Passport::default();
    for entry in input.replace("\n", " ").split(" ") {
        let parts: Vec<&str> = entry.split(":").collect();
        let (label, value) = (parts[0], parts[1]);
        match label {
            "byr" => { p.byr = Some(value.to_owned()) },
            "iyr" => { p.iyr = Some(value.to_owned()) },
            "eyr" => { p.eyr = Some(value.to_owned()) },
            "hgt" => { p.hgt = Some(value.to_owned()) },
            "hcl" => { p.hcl = Some(value.to_owned()) },
            "ecl" => { p.ecl = Some(value.to_owned()) },
            "pid" => { p.pid = Some(value.to_owned()) },
            "cid" => { p.cid = Some(value.to_owned()) },
            _ => {},
        }
    }
    p
}

fn passport_is_valid(p: &Passport) -> bool {
    if let Some(byr) = &p.byr {
        if !check_year_range(byr, 1920, 2002) {
            return false;
        }
    } else { return false; }

    if let Some(iyr) = &p.iyr {
        if !check_year_range(iyr, 2010, 2020) {
            return false;
        }
    } else { return false; }

    if let Some(eyr) = &p.eyr {
        if !check_year_range(eyr, 2020, 2030) {
            return false;
        }
    } else { return false; }

    if let Some(hgt) = &p.hgt {
        let re = Regex::new(r"^([0-9]+)(cm|in)$").unwrap();
        if let Some(caps) = re.captures(hgt) {
            let num_str = caps.get(1).unwrap().as_str();
            let unit = caps.get(2).unwrap();

            let num = match num_str.parse::<usize>() {
                Ok(n) => n,
                Err(_) => { return false; }
            };

            match unit.as_str() {
                "in" if num < 59 || num > 76 => { return false; },
                "cm" if num < 150 || num > 193 => { return false; },
                _ => {},
            }
        } else {
            return false;
        }
    } else { return false; }

    if let Some(hcl) = &p.hcl {
        let re = Regex::new(r"#[0-9A-Fa-f]{6}").unwrap();
        if !re.is_match(hcl) { return false; }
    } else { return false; }

    if let Some(ecl) = &p.ecl {
        match ecl.as_str() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {},
            _ => { return false; },
        }
    } else { return false; }

    if let Some(pid) = &p.pid {
        let re = Regex::new(r"^[0-9]{9}$").unwrap();
        if !re.is_match(pid) { return false; }
    } else { return false; }

    true
}

fn check_year_range(year_str: &String, min: usize, max: usize) -> bool {
    match year_str.parse::<usize>() {
        Ok(year) => {
            if year < min || year > max {
                return false;
            }
        }
        Err(_) => { return false; }
    };
    true
}
