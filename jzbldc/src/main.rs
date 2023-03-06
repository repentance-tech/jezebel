use classfile::parser;

fn main() -> std::io::Result<()> {
    let f = std::env::args().nth(1).expect("expected file path");

    let fs = std::fs::read(f)?;

    let (_, classfile) = parser::parse_classfile(&fs).expect("correct");

    println!("{:?}", classfile.access_flags);
    println!("{:?}", classfile.constant_pool[classfile.super_class + 1]);

    Ok(())
}
