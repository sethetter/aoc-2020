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
    if p.byr.is_none() { return false; }
    if p.iyr.is_none() { return false; }
    if p.eyr.is_none() { return false; }
    if p.hgt.is_none() { return false; }
    if p.hcl.is_none() { return false; }
    if p.ecl.is_none() { return false; }
    if p.pid.is_none() { return false; }
    // if p.cid.is_none() { return false; }
    true
}
