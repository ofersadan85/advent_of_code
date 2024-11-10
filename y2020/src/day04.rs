use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Passport {
    byr: String, // birth_year
    iyr: String, // issue_year
    eyr: String, // expiration_year
    hgt: String, // height
    hcl: String, // hair_color
    ecl: String, // eye_color
    pid: String,
    cid: Option<String>,
}

impl Passport {
    fn new(raw_json: &str) -> Option<Self> {
        let json = format!(
            "{{\"{}\"}}",
            raw_json
                .trim()
                .replace(' ', "\", \"")
                .replace(':', "\": \"")
        );
        serde_json::from_str(&json).ok()
    }

    fn is_valid(&self) -> bool {
        self._is_valid_opt().unwrap_or(false)
    }

    #[allow(clippy::similar_names)] // This is the task
    fn _is_valid_opt(&self) -> Option<bool> {
        let byr = (1920..=2002).contains(&self.byr.parse::<usize>().ok()?);
        let iyr = (2010..=2020).contains(&self.iyr.parse::<usize>().ok()?);
        let eyr = (2020..=2030).contains(&self.eyr.parse::<usize>().ok()?);

        let (hgt_str, hgt_unit) = self.hgt.split_at(self.hgt.len() - 2);
        let hgt_int: usize = hgt_str.parse().ok()?;
        let hgt = (hgt_unit == "cm" && (150..=193).contains(&hgt_int))
            || (hgt_unit == "in" && (59..=76).contains(&hgt_int));

        let hcl = self.hcl.starts_with('#')
            && self.hcl.len() == 7
            && usize::from_str_radix(&self.hcl[1..], 16).is_ok();
        let valid_ecl = [
            "amb".to_string(),
            "blu".to_string(),
            "brn".to_string(),
            "gry".to_string(),
            "grn".to_string(),
            "hzl".to_string(),
            "oth".to_string(),
        ];
        let ecl = valid_ecl.contains(&self.ecl);
        let pid = self.pid.len() == 9 && self.pid.parse::<usize>().is_ok();
        Some(byr && iyr && eyr && hgt && hcl && ecl && pid)
    }
}

fn input(example: bool) -> Vec<String> {
    const PATH: &str = "inputs/day04.txt";
    if example {
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm
        
        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929
        
        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm
        
        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in"
            .to_string()
    } else {
        std::fs::read_to_string(PATH).unwrap()
    }
    .trim()
    .lines()
    .map(|row| row.trim().to_string())
    .collect()
}

fn parse_passports(data: &[String]) -> Vec<Option<Passport>> {
    let mut potentials: Vec<Option<Passport>> = vec![];
    let mut current = String::new();
    for row in data {
        if row.trim().is_empty() {
            potentials.push(Passport::new(&current));
            current = String::new();
        } else {
            current += " ";
            current += row.trim();
        }
    }
    potentials.push(Passport::new(&current));
    potentials
}

fn part_1(data: &[String]) -> usize {
    parse_passports(data).iter().flatten().count()
}

fn part_2(data: &[String]) -> usize {
    parse_passports(data)
        .iter()
        .flatten()
        .filter(|p| p.is_valid())
        .count()
}

#[test]
fn example_1() {
    assert_eq!(part_1(&input(true)), 2);
}

#[test]
fn solution_1() {
    assert_eq!(part_1(&input(false)), 219);
}

#[test]
fn example_2() {
    let example_invalid =
        "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926    
    iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946
    hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277    
    hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007"
            .split('\n')
            .filter_map(|row| Passport::new(row.trim()))
            .filter(Passport::is_valid)
            .count();
    let example_valid = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f
    eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm    
    hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022    
    iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
        .split('\n')
        .filter_map(|row| Passport::new(row.trim()))
        .filter(Passport::is_valid)
        .count();
    assert_eq!(example_valid, 4);
    assert_eq!(example_invalid, 0);
}

#[test]
fn solution_2() {
    assert_eq!(part_2(&input(false)), 127);
}
