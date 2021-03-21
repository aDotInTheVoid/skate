mod grammar; // synthesized by LALRPOP

mod ast;

fn main() {
    println!("Hello, world!");
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
