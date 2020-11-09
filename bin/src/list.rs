use colored::Colorize;

pub fn list_online(color: bool, name: String, cmd: String, desc: String) {
    if color {
        println!(
            "{:30} {:50} {:50}",
            name.green().bold(),
            cmd.green().bold(),
            desc.green().bold()
        )
    } else {
        println!("{:30} {:50} {:50}", name, cmd, desc,)
    }
}
