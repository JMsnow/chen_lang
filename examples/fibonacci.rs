fn main() {
    simple_logger::init().unwrap();
    let cpde: String = r#"
let n = 1
let i = 1
let j = 2
for n <= 10 {
   println(i)
   let tmp  = i
   i = j
   j = tmp + j
   n = n + 1
}
"#
    .to_string();

    chen_lang::run(cpde).unwrap();
}
