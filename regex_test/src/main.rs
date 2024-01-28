use regex::Regex;

fn main() {
    println!("Hello, world!");


println!("find_iter----------------------------");
let re = Regex::new(r"\d+").unwrap();
let input = "server: ok=100 changed=50 unreachable=0 failed=3";
for m in re.find_iter(input) {
    println!("Found `{}` at {}-{}", m.as_str(), m.start(), m.end());
}


println!("captures_iter----------------------------");
let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
let input = "Eiichi Shibusawa was born on 1840-02-13 and died on 1931-11-11.";
for caps in re.captures_iter(input) {
    println!("年: {}, 月: {}, 日: {}", &caps[1], &caps[2], &caps[3]);
}


println!("replace_all ----------------------------");
let re = Regex::new(r"[m|M]aku").unwrap();
let input = "I am maku. You are not Maku.";
let output = re.replace_all(input, "####");
println!("{}", output); //=> "I am ####. Your are not ####."

println!("is_match rstr ----------------------------");
let re = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();
let input = "Today's date is 2023-01-07.";
if re.is_match(input) {
    println!("日付らしき文字列が見つかりました");
}

println!("is_match str ----------------------------");
let key = "\\d{4}-\\d{2}-\\d{2}";
let re = Regex::new(key).unwrap();
let input = "Today's date is 2023-01-07.";
if re.is_match(input) {
    println!("日付らしき文字列が見つかりました");
}

let key = "(\\d{4})-(\\d{2})-(\\d{2})";
let re = Regex::new(key).unwrap();
let input = "Eiichi Shibusawa was born on 1840-02-13 and died on 1931-11-11.";
for caps in re.captures_iter(input) {
    println!("年: {}, 月: {}, 日: {}", &caps[1], &caps[2], &caps[3]);
}
}
