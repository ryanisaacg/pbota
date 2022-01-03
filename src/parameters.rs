use serenity::model::channel::Message;

#[derive(Default)]
pub struct Parameters<'a> {
    pub modifier: Option<Modifier<'a>>,
    pub character: Option<&'a str>,
    pub hope: bool,
    pub despair: bool,
}

#[derive(Debug)]
pub struct Modifier<'a> {
    pub value: ModifierValue<'a>,
}

#[derive(Debug)]
pub enum ModifierValue<'a> {
    Number(i32),
    Stat {
        sign: i32,
        stat: &'a str,
    },
}

pub fn parameters<'a>(msg: &'a Message) -> Parameters<'a> {
    let mut parameters = Parameters::default();

   let content = &msg.content;

   let (_, params) = match content.find(' ') {
       Some(idx) => content.split_at(idx + 1),
       None => {
           return parameters;
       }
    };

    println!("{}", params);

    for param in params.split(&[',', ' '][..]) { 
        match param.chars().next() {
            Some(sign @ '+' | sign @ '-') => {
                parameters.modifier = Some(Modifier {
                    value: match param.parse::<i32>() {
                        Ok(val) => ModifierValue::Number(val),
                        Err(_) => ModifierValue::Stat { 
                            sign: if sign == '-' { -1 } else { 1 },
                            stat: &param[1..]
                        },
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
