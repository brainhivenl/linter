fn main() {
    let userId = 1;

    unsafe {
        println!("test");
    }
}

async fn test() {
    println!("test");
    std::thread::sleep(std::time::Duration::from_secs(1));
}