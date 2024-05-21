use ecosystem::MyError;

fn main() -> Result<(), MyError> {
    println!("size of MyError is {}", std::mem::size_of::<MyError>());

    // let filename = "non_existent_file.txt";
    // let fd = fs::File::open(filename).with_context(|| format!("file: {}", filename))?;

    fail_with_error()?;

    Ok(())
}

fn fail_with_error() -> Result<(), MyError> {
    Err(MyError::Custom("An error message".to_string()))
}
