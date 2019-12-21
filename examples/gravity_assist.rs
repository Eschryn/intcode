use intcode::aoc::AocIntCode;

fn main() {
    let input = std::fs::read_to_string("examples/gravity_assist.ic").unwrap();
    let mut code = input.parse::<AocIntCode>().unwrap();
    code.write(1, 12);
    code.write(2, 2);
    let res = code.run().unwrap();

    println!("{}", res);
}