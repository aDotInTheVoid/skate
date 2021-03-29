#[track_caller]
fn f(s: &str) {
    let p = crate::grammar::FunctionParser::new();
    let f = p.parse(s).unwrap();
    insta::assert_yaml_snapshot!(f);
}
#[track_caller]
fn e(s: &str) {
    let p = crate::grammar::ExprParser::new();
    let f = p.parse(s).unwrap();
    insta::assert_yaml_snapshot!(f);
}
#[track_caller]
fn s(s: &str) {
    let p = crate::grammar::StmtParser::new();
    let f = p.parse(s).unwrap();
    insta::assert_yaml_snapshot!(f);
}

#[track_caller]
fn i(s: &str) {
    let p = crate::grammar::ItemParser::new();
    let f = p.parse(s).unwrap();
    insta::assert_yaml_snapshot!(f);
}

#[track_caller]
fn p(s: &str) {
    let p = crate::grammar::ProgramParser::new();
    let f = p.parse(s).unwrap();
    insta::assert_yaml_snapshot!(f);
}

#[test]
fn functions() {
    f("fn main(){}");
    f("fn bar(x:int){}");
    f("fn foo(y:int,){}");
    f("fn baz(y:int,z:string){}");
    f("fn baz(y:int, z:string){}");
    f("fn baz(y:int, z:string) -> int {}");
    f("fn baz(y:int,)->bool  {}");
    f("fn baz()->string{}");
    f("fn a(){a;b;c;}");
    f("fn b(){let x = y;}");
    f("fn c(){let x = y}");
    f("fn d(){ let x = {y};}");
    f("fn e(){let x = {x;y;z;}}");
    f("fn f(){ {let a = b; let c = {d;e}}; f}");
    f("fn main() {
            if x {
                y
            }
        }");
    f("fn main() {
            if x {
                xf;
                let z = d;
            } else {
                noo;
            };
            print hell
        }");
    f("fn main() {
            print a;
            print b;
            print xxxxxx;   
        }");
    f("fn main() {
            for i in z {
                print z;
                print i;
                let i = {
                    if a {d};
                    if c {e} else { if f {g} }
                }
            }
        }");
    f("fn main() { let z = a.b }");
    f("fn main() {let z = a.b.c;}");
    f("fn main() { let z = x.y.z[a.b[z].c].d(e,f,g);}");
}

#[test]
fn exprs() {
    e("1");
    e("1 + {if 1 {2} else {3}}");
    e("2+4*3");
    e("4*2+3");
    e("1==2==3+3");
    e(r#""""#);
    e(r#""ab\"""#);
    e("(-1 + 2) * 3 - -4");
    e("24+24*32/-12+2");
}

#[test]
fn stmt() {
    s(r#"print 1"#);
    s(r#"print "2""#);
    s(r#"print "2" + "3""#);
    s("let x = if a {b} else {c}");
    s("for i in if a {} else {} {e;}");
    s("while if x{false}else{true} {
        print 1 + (if z {a} else {d});
        for i in while foo {bar} {
            print zany;
        }
    }");
    // TODO: Dont require parens
    s("print 1 + (if z {}else{})");
}

#[test]
fn progs() {
    p(r#"fn hello() {print "hello";}
            fn world() { print "world"; }
            fn main() {
                hello();
                world();
            }
            "#);
}

#[test]
fn items() {
    i("fn main(){}");
}
