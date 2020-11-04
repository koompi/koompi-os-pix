pub fn config() {
    if cfg!(debug_assertions) {
        println!("development")
    } else {
        println!("production")
    }
}
// Check development or production environment
// if cfg!(debug_assertions) {
//     println!("dev")
// } else {
//     println!("pro")
// }
