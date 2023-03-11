use classfile::parser;

// a convenient type alias which allows using `?` on almost any error type
// Box<dyn Error> is similar to what anyhow and eyre do
type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() -> Result {
    // - don't use expect/unwrap, especially if inside a Result-returning function, ok_or returns
    //   the result of the closure if the Option is None, turning it into a Result
    let f = std::env::args()
        .nth(1)
        .ok_or("no filepath given, usage `jzbldc <path>`")?;

    let fs = std::fs::read(f)?;

    // see review of classfile
    let classfile = parser::parse(&fs)?;

    println!("{:?}", classfile.access_flags);
    println!("{:?}", classfile.constant_pool[classfile.super_class + 1]);

    Ok(())
}
