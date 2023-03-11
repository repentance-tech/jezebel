use classfile::parser;

// a convenient type alias which allows using `?` on almost any error type
// Box<dyn Error> is similar to what anyhow and eyre do
type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() -> Result {
    // - use descriptive variable names, `file_name` instead of `f`
    // - don't use expect/unwrap, especially if inside a Result-returning function, ok_or returns
    //   the result of the closure if the Option is None, turning it into a Result
    let file_name = std::env::args()
        .nth(1)
        .ok_or("no filepath given, usage `jzbldc <path>`")?;

    // see above regarding names
    let file_content = std::fs::read(file_name)?;

    // see review of classfile
    let classfile = parser::parse(&file_content)?;

    println!("{:?}", classfile.access_flags);
    println!("{:?}", classfile.constant_pool[classfile.super_class + 1]);

    Ok(())
}
