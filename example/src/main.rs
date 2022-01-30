fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Error: Wrong argument count!");
        println!("");
        println!("Usage: {} INPUT_FILE", &args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let mut content: Vec<u8> = vec![0; 20];
    let result = safe_foo::foo_read_file(&filename, &mut content);
    if !result {
        println!("foo_read_file returns {}", result);
    }
    for byte in content.iter() {
        println!("{:#02x}", byte);
    }
}
