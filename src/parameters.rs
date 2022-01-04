
#[derive(Default)]
pub struct Parameters<'a> {
    pub modifier: Option<Modifier<'a>>,
    pub character: Option<&'a str>,
    pub hope: bool,
    pub despair: bool,
}

#[derive(Debug)]
pub enum Modifier<'a> {
    Number(i32),
    Stat {
        sign: i32,
        stat: &'a str,
    },
}

pub fn parameters<'a>(contents: &'a str) -> Parameters<'a> {
    let mut parameters = Parameters::default();

    for param in contents.split(&[',', ' '][..]) { 
        match param.chars().next() {
            Some(sign @ '+' | sign @ '-') => {
                parameters.modifier = Some(match param.parse::<i32>() {
                        Ok(val) => Modifier::Number(val),
                        Err(_) => Modifier::Stat { 
                            sign: if sign == '-' { -1 } else { 1 },
                            stat: &param[1..]
                        },
                    });
            }
            Some('@') => {
                parameters.character = Some(&param[1..]);
            }
            _ => {},
        }

        if param == "h" || param == "hope" || param == "a" || param == "adv" {
            parameters.hope = true;
        }
        if param == "d" || param == "despair" || param == "dis" {
            parameters.despair = true;
        }
    }

    parameters
}
