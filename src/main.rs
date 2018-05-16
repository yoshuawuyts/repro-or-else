extern crate failure;

use failure::Error;

fn main() {
    let res: Result<usize, Error> = cache_get().ok_or_else(|| db_get());
    println!("{:?}", res);
}

fn cache_get() -> Option<usize> {
    Some(12)
}

fn db_get() -> Result<usize, Error> {
    Ok(13)
}
