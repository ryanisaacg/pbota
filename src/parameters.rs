use serenity::model::channel::Message;

pub struct Parameters {
    pub modifier: Option<i32>,
    pub hope: bool,
    pub despair: bool,
}

pub fn parameters(msg: &Message) -> Parameters {
    let mut parameters = Parameters {
        modifier: None,
        hope: false,
        despair: false,
    };

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
                let val = param.parse::<i32>().ok();
                if sign == '-' {
                    parameters.modifier = val.map(|x| -x);
                } else {
                    parameters.modifier = val;
                }
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
