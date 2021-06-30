#[cfg(test)]
fn run_stmt(stmt: &str) -> String {
    let tree = parser::StmtParser::new()
        .parse(diagnostics::FileId(0), stmt)
        .unwrap();

    let mut compiler = compiler::FnComping::default();
    compiler.push_stmt(&tree);

    let func = compiler.output;

    let mut code = compiler::bytecode::Code::default();
    let main_key = code.fns.insert(func);

    let mut output = Vec::new();

    let mut vm = vm::VM::new(&mut output);

    vm.run(code, main_key).unwrap();

    String::from_utf8(output).unwrap()
}

#[test]
fn local_vars_basic() {
    let code = "
{
    let a = 2;
    print a + 2;
}
    ";
    assert_eq!(run_stmt(code), "4\n");
}

#[test]
#[should_panic = "No Local Found"]
fn let_a_eq_a() {
    let code = "{let a = a;}";
    run_stmt(code);
}

#[test]
#[should_panic = "Duplicated name"]
fn duplicate_var() {
    let code = "{let a=0; let a = 1;}";
    run_stmt(code);
}

#[test]
fn scoping() {
    let code = "{let a = 0; {let a = 1; print a;} print a;}";
    assert_eq!(run_stmt(code), "1\n0\n");
}

#[test]
fn shadow_from_outer() {
    let code = "{let a = 8; {let a = a; print a;}}";
    assert_eq!(run_stmt(code), "8\n");
}

#[test]
fn assign() {
    let code = "{let a = 2; print a; a = 3; print a;}";
    assert_eq!(run_stmt(code), "2\n3\n");
}

#[test]
fn assign_shadow() {
    let code = "
    {
        let a = \"a\";
        let b = \"b\";
        {
            let a = \"c\";
            b = b + a;
            print a;
            print b;
        }
        print a;
        print b;
    }
    ";
    assert_eq!(run_stmt(code), "c\nbc\na\nbc\n");
}
