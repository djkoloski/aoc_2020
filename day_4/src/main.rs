use std::str::FromStr;
use std::num::ParseIntError;
use problem::{Problem, solve};

enum Unit {
    Centimeter,
    Inch,
    None,
}

struct Height {
    pub value: u32,
    pub unit: Unit,
}

#[derive(Debug)]
enum ParseHeightError {
    ParseIntError(ParseIntError),
    InvalidUnit(String),
}

impl From<ParseIntError> for ParseHeightError {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

impl FromStr for Height {
    type Err = ParseHeightError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(unit_start) = s.find(|c: char| !c.is_numeric()) {
            let value = s[..unit_start].parse()?;
            let unit = match &s[unit_start..] {
                "cm" => Unit::Centimeter,
                "in" => Unit::Inch,
                u => return Err(ParseHeightError::InvalidUnit(u.to_string())),
            };
            Ok(Self {
                value,
                unit,
            })
        } else {
            Ok(Self {
                value: s.parse()?,
                unit: Unit::None,
            })
        }
    }
}

enum Color {
    Z,
    Prefixed(u8, u8, u8),
    Unprefixed(u8, u8, u8),
}

#[derive(Debug)]
enum ParseColorError {
    InvalidLength(String),
    ParseIntError(ParseIntError),
}

impl From<ParseIntError> for ParseColorError {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

impl FromStr for Color {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            Err(ParseColorError::InvalidLength(s.to_string()))
        } else {
            let first_char = s.chars().next().unwrap();
            if first_char == 'z' {
                Ok(Self::Z)
            } else {
                if first_char == '#' {
                    if s.len() != 7 {
                        Err(ParseColorError::InvalidLength(s.to_string()))
                    } else {
                        Ok(Self::Prefixed(
                            u8::from_str_radix(&s[1..3], 16)?,
                            u8::from_str_radix(&s[3..5], 16)?,
                            u8::from_str_radix(&s[5..7], 16)?,
                        ))
                    }
                } else {
                    if s.len() != 6 {
                        Err(ParseColorError::InvalidLength(s.to_string()))
                    } else {
                        Ok(Self::Unprefixed(
                            u8::from_str_radix(&s[0..2], 16)?,
                            u8::from_str_radix(&s[2..4], 16)?,
                            u8::from_str_radix(&s[4..6], 16)?,
                        ))
                    }
                }
            }
        }
    }
}

enum EyeColor {
    Gray,
    Brown,
    Amber,
    Green,
    Other,
    Hazel,
    Blue,
    Laser,
    XRay,
    DoesNotExist,
    CoordinatedUniversalTime,
    GreenwichMeanTime,
    Color(Color),
}

#[derive(Debug)]
enum ParseEyeColorError {
    ParseColorError(ParseColorError),
}

impl From<ParseColorError> for ParseEyeColorError {
    fn from(e: ParseColorError) -> Self {
        Self::ParseColorError(e)
    }
}

impl FromStr for EyeColor {
    type Err = ParseEyeColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "gry" => EyeColor::Gray,
            "brn" => EyeColor::Brown,
            "amb" => EyeColor::Amber,
            "grn" => EyeColor::Green,
            "dne" => EyeColor::DoesNotExist,
            "oth" => EyeColor::Other,
            "hzl" => EyeColor::Hazel,
            "lzr" => EyeColor::Laser,
            "blu" => EyeColor::Blue,
            "xry" => EyeColor::XRay,
            "utc" => EyeColor::CoordinatedUniversalTime,
            "gmt" => EyeColor::GreenwichMeanTime,
            c => EyeColor::Color(c.parse()?),
        })
    }
}

enum PassportEntry {
    BirthYear(u32),
    IssueYear(u32),
    ExpirationYear(u32),
    Height(Height),
    HairColor(Color),
    EyeColor(EyeColor),
    PassportId(String),
    CountryId(u32),
}

#[derive(Debug)]
enum ParsePassportEntryError {
    InvalidFormat(String),
    InvalidField(String),
    ParseIntError(ParseIntError),
    ParseHeightError(ParseHeightError),
    ParseColorError(ParseColorError),
    ParseEyeColorError(ParseEyeColorError),
}

impl From<ParseIntError> for ParsePassportEntryError {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

impl From<ParseHeightError> for ParsePassportEntryError {
    fn from(e: ParseHeightError) -> Self {
        Self::ParseHeightError(e)
    }
}

impl From<ParseColorError> for ParsePassportEntryError {
    fn from(e: ParseColorError) -> Self {
        Self::ParseColorError(e)
    }
}

impl From<ParseEyeColorError> for ParsePassportEntryError {
    fn from(e: ParseEyeColorError) -> Self {
        Self::ParseEyeColorError(e)
    }
}

impl FromStr for PassportEntry {
    type Err = ParsePassportEntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pieces = s.split(':').collect::<Vec<_>>();
        if pieces.len() != 2 {
            Err(ParsePassportEntryError::InvalidFormat(s.to_string()))
        } else {
            let field = pieces[0];
            let value = pieces[1];

            Ok(match field {
                "byr" => PassportEntry::BirthYear(value.parse()?),
                "iyr" => PassportEntry::IssueYear(value.parse()?),
                "eyr" => PassportEntry::ExpirationYear(value.parse()?),
                "hgt" => PassportEntry::Height(value.parse()?),
                "hcl" => PassportEntry::HairColor(value.parse()?),
                "ecl" => PassportEntry::EyeColor(value.parse()?),
                "pid" => PassportEntry::PassportId(value.to_string()),
                "cid" => PassportEntry::CountryId(value.parse()?),
                _ => return Err(ParsePassportEntryError::InvalidField(field.to_string())),
            })
        }
    }
}

struct PassportLine {
    pub entries: Vec<PassportEntry>,
}

impl FromStr for PassportLine {
    type Err = <PassportEntry as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "" {
            Ok(Self {
                entries: Vec::new(),
            })
        } else {
            Ok(Self {
                entries: s.split(' ').map(|p| p.parse()).collect::<Result<_, _>>()?,
            })
        }
    }
}

fn count_valid_passports(input: &Vec<PassportLine>, validate: bool) -> Result<usize, ()> {
    let mut has_birth_year = false;
    let mut has_issue_year = false;
    let mut has_expiration_year = false;
    let mut has_height = false;
    let mut has_hair_color = false;
    let mut has_eye_color = false;
    let mut has_passport_id = false;
    let mut is_valid = true;

    let mut valid_passports = 0;

    for line in input {
        if line.entries.len() == 0 {
            if has_birth_year && has_issue_year && has_expiration_year && has_height && has_hair_color & has_eye_color && has_passport_id && (is_valid || !validate) {
                valid_passports += 1;
            }

            has_birth_year = false;
            has_issue_year = false;
            has_expiration_year = false;
            has_height = false;
            has_hair_color = false;
            has_eye_color = false;
            has_passport_id = false;
            is_valid = true;
        } else {
            for entry in line.entries.iter() {
                match entry {
                    PassportEntry::BirthYear(year) => {
                        has_birth_year = true;
                        if *year < 1920 || *year > 2002 {
                            is_valid = false;
                        }
                    },
                    PassportEntry::IssueYear(year) => {
                        has_issue_year = true;
                        if *year < 2010 || *year > 2020 {
                            is_valid = false;
                        }
                    },
                    PassportEntry::ExpirationYear(year) => {
                        has_expiration_year = true;
                        if *year < 2020 || *year > 2030 {
                            is_valid = false;
                        }
                    },
                    PassportEntry::Height(height) => {
                        has_height = true;
                        let is_height_valid = match height.unit {
                            Unit::Centimeter => height.value >= 150 && height.value <= 193,
                            Unit::Inch => height.value >= 59 && height.value <= 76,
                            _ => false,
                        };
                        if !is_height_valid {
                            is_valid = false;
                        }
                    },
                    PassportEntry::HairColor(color) => {
                        has_hair_color = true;
                        let is_hair_color_valid = match color {
                            Color::Prefixed(..) => true,
                            _ => false,
                        };
                        if !is_hair_color_valid {
                            is_valid = false;
                        }
                    },
                    PassportEntry::EyeColor(eye_color) => {
                        has_eye_color = true;
                        let is_eye_color_valid = match eye_color {
                            EyeColor::Amber | EyeColor::Blue | EyeColor::Brown | EyeColor::Gray | EyeColor::Green | EyeColor::Hazel | EyeColor::Other => true,
                            _ => false,
                        };
                        if !is_eye_color_valid {
                            is_valid = false;
                        }
                    },
                    PassportEntry::PassportId(passport_id) => {
                        has_passport_id = true;
                        if passport_id.len() != 9 || !passport_id.chars().all(|c| c.is_numeric()) {
                            is_valid = false
                        }
                    },
                    PassportEntry::CountryId(_) => (),
                }
            }
        }
    }

    if has_birth_year && has_issue_year && has_expiration_year && has_height && has_hair_color & has_eye_color && has_passport_id && (is_valid || !validate) {
        valid_passports += 1;
    }

    Ok(valid_passports)
}

struct Day4;
impl Problem for Day4 {
    type Input = Vec<PassportLine>;
    type Part1Output = usize;
    type Part2Output = usize;
    type Error = ();

    fn part_1(input: &Self::Input) -> Result<Self::Part1Output, Self::Error> {
        count_valid_passports(input, false)
    }

    fn part_2(input: &Self::Input) -> Result<Self::Part2Output, Self::Error> {
        count_valid_passports(input, true)
    }
}

fn main() {
    solve::<Day4>("input").unwrap();
}
