use label_fixer::*;
use std::env::args;

fn main() -> Result<(), Error> {
    let Some(path) = args().skip(1).next() else {
        return Err(Error::Other("must provide a file path"));
    };

    let out_path = fix_label(path)?;

    println!("{}", out_path.display());

    Ok(())
}
