fn main() {
    let parser = tera::parsing::parser::Parser::new("{{ 1 + 1 }}");
    let ast = parser.parse().expect("should parse correctly");
    println!("{:?}", ast.nodes)
}
