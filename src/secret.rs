use std::collections::HashMap;

fn parse() -> HashMap<String, String> {
    let input = std::fs::read_to_string("./bot.secret").unwrap();
    let parse = input
        .lines()
        .map(|e| e.split("=").collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut out = HashMap::new();
    for i in parse {
        out.insert(i[0].to_owned(), i[1].to_owned());
    }
    out
}
pub fn get_secret(key: &str) -> String {
    let secret = parse();
    secret.get(key).unwrap().to_owned()
}
