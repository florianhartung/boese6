pub fn main() {
    let asm_code = b6_compiler::compile_to_nasm(include_str!("demo_player.b6"));
}
