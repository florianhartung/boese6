#[test]
fn raw_function() {
    let mut module = nasm_codegen::Module::new();

    module.label("_main");
    module.instruction("add", &["eax", "ebx"]);
    module.instruction("averylonginstruction", &["ecx"]);
    module.instruction("hlt", &[]);

    module.r#extern("_GetStdOut@4");

    let asm_code = module.into_generated_nasm();

    let expected = "\
extern _GetStdOut@4

_main:
\tadd       eax, ebx
\taverylonginstruction ecx
\thlt
";
    assert_eq!(asm_code, expected.replace("\n", "\r\n"))
}
