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
    }
}
