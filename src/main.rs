use lazy_static::lazy_static;
use minijinja::{Environment, Source};
use std::collections::BTreeMap;

lazy_static! {
    static ref ENV: Environment<'static> = create_env();
}

fn create_env() -> Environment<'static> {
    let mut env = Environment::new();
    let mut source = Source::new();
    source.load_from_path("templates", &["j2"]).unwrap();
    env.set_source(source);
    env
}

fn main() {
    let mut ctx = BTreeMap::new();
    ctx.insert("name", "Apache Config");
    let tmpl = ENV.get_template("hello.j2").unwrap();
    println!("{}", tmpl.render(&ctx).unwrap());
}
