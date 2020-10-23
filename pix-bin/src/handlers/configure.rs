pub fn config() {
    if cfg!(debug_assertions) {
        println!("development")
    } else {
        println!("production")
    }
}
