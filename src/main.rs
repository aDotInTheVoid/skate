#![allow(clippy::all)]
mod grammar; // synthesized by LALRPOP

mod ast;

fn main() {
    let f = crate::grammar::FunctionParser::new();
    let func = std::env::args().nth(1).unwrap();
    let f = f.parse(&func).unwrap();

    println!("{:?}", f);
}

#[cfg(test)]
mod tests {

    #[track_caller]
    fn t(s: &str) {
        let p = crate::grammar::FunctionParser::new();
        let f = p.parse(s).unwrap();
        insta::assert_yaml_snapshot!(f);
    }

    #[test]
    fn functions() {
        t("fn main(){}");
        t("fn bar(x:int){}");
        t("fn foo(y:int,){}");
        t("fn baz(y:int,z:string){}");
        t("fn baz(y:int, z:string){}");
        t("fn baz(y:int, z:string) -> int {}");
        t("fn baz(y:int,)->bool  {}");
        t("fn baz()->string{}");
        t("fn a(){a;b;c;}");
        t("fn b(){let x = y;}");
        t("fn c(){let x = y}");
        t("fn d(){ let x = {y};}");
        t("fn e(){let x = {x;y;z;}}");
        t("fn f(){ {let a = b; let c = {d;e}}; f}");
        t("fn main() {
            if x {
                y
            }
        }");
        t("fn main() {
            if x {
                xf;
                let z = d;
            } else {
                noo;
            };
            print hell
        }");
        t("fn main() {
            print a;
            print b;
            print xxxxxx;   
        }");
        t("fn main() {
            for i in z {
                print z;
                print i;
                let i = {
                    if a {d};
                    if c {e} else { if f {g} }
                }
            }
        }");
        t("fn main() { let z = a.b }");
        t("fn main() {let z = a.b.c;}");
        t("fn main() { let z = x.y.z[a.b[z].c].d(e,f,g);}");
    }
}
