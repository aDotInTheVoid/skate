#[cfg(test)]
fn run_stmt(stmt: &str) -> String {
    let src = format!("fn main () {{ {} }}", stmt);

    let tree = parser::ProgramParser::new()
        .parse(diagnostics::FileId(0), &src)
        .unwrap();

    let (code, main_key) = compiler::compile(&tree);

    let mut output = Vec::new();

    let mut vm = vm::VM::new(&mut output);

    vm.run(code, main_key).unwrap();

    assert_eq!(vm.stack_len(), 0);

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

#[test]
fn if_() {
    let code = "
    {
    let x = true;
    print \"a\";
    if x {
        print \"t\";
    }
    print \"b\";
    }
    ";
    assert_eq!(run_stmt(code), "a\nt\nb\n")
}

#[test]
fn if2() {
    let code = "
    {
    let x = false;
    print \"a\";
    if x {
        print \"t\";
    }
    print \"b\";
    }
    ";
    assert_eq!(run_stmt(code), "a\nb\n");
}

#[test]
fn while_() {
    let code = "
    {
    let x = 0;
    while x <=10 {
        print x;
        if x == 3 {
            print \"THREE\";
        } else {
            if x == 10 {
                print \"TEN\";
            }
        }
        x = x + 1;
    }
    }
    ";
    assert_eq!(
        run_stmt(code),
        "0\n1\n2\n3\nTHREE\n4\n5\n6\n7\n8\n9\n10\nTEN\n"
    );
}

#[test]
fn array() {
    let code = "
    {
        let x = [1,2,3,4,5];
        print (x);
    }
    ";
    assert_eq!(run_stmt(code), "[1, 2, 3, 4, 5]\n");
}
