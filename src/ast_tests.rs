use crate::diagnostics;

const FID: diagnostics::FileId = diagnostics::FileId(100);

#[track_caller]
fn e(s: &str) {
    p(&format!("fn rand(){{ {}; }}", s));
}
#[track_caller]
fn s(s: &str) {
    p(&format!("fn rand(){{ {} }}", s));
}

#[track_caller]
fn p(s: &str) {
    let p = crate::grammar::ProgramParser::new();
    let f = p.parse(FID, s).unwrap();
    insta::assert_yaml_snapshot!(f);
}

#[test]
fn functions() {
    p("fn main(){}");
    p("fn bar(x:int){}");
    p("fn foo(y:int,){}");
    p("fn baz(y:int,z:string){}");
    p("fn baz(y:int, z:string){}");
    p("fn baz(y:int, z:string) -> int {}");
    p("fn baz(y:int,)->bool  {}");
    p("fn baz()->string{}");
    p("fn a(){a;b;c;}");
    p("fn b(){let x = y;}");
    p("fn c(){let x = y}");
    p("fn d(){ let x = {y};}");
    p("fn e(){let x = {x;y;z;}}");
    p("fn f(){ {let a = b; let c = {d;e}}; f}");
    p("fn main() {
            if x {
                y
            }
        }");
    p("fn main() {
            if x {
                xf;
                let z = d;
            } else {
                noo;
            };
            print hell
        }");
    p("fn main() {
            print a;
            print b;
            print xxxxxx;   
        }");
    p("fn main() {
            for i in z {
                print z;
                print i;
                let i = {
                    if a {d};
                    if c {e} else { if f {g} }
                }
            }
        }");
    p("fn main() { let z = a.b }");
    p("fn main() {let z = a.b.c;}");
    p("fn main() { let z = x.y.z[a.b[z].c].d(e,f,g);}");
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
    e("{2}");
    e("{2;}")
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
    s("print 1 + (if z {}else{})");
    s("print 1 + if z {}else{}");
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
    p("fn main(){}");
}
