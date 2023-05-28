use b6_compiler::compile_to_nasm;

#[test]
pub fn main() {
    let nasm_code = compile_to_nasm(include_str!("continue_script.b6strat"));

    println!("{}", nasm_code);
}
