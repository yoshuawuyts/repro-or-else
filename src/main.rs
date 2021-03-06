use std::error::Error;

fn main() {
    // Return type is `Ok(12)`.
    let res: Result<usize, Result<usize, Box<Error>>> = cache_get1().ok_or_else(|| db_get());
    println!("1. {:?}", res);
    println!("1. {:?}", res.unwrap());

    // Return type is `Err(Ok(13))`.
    let res: Result<usize, Result<usize, Box<Error>>> = cache_get2().ok_or_else(|| db_get());
    println!("2. {:?}", res);
    println!("2. {:?}", res.unwrap());
}

fn cache_get1() -> Option<usize> {
    Some(12)
}

fn cache_get2() -> Option<usize> {
    None
}

fn db_get() -> Result<usize, Box<Error>> {
    Ok(13)
}
