#[test]
fn empty_module() {
    let module = nasm_codegen::Module::new();

    let asm_code = module.into_generated_nasm();

    assert_eq!(asm_code, "".to_owned())
}
