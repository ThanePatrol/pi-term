use std::net::Ipv4Addr;
use std::process::{Command, Output};

pub fn init_tmux_session(node_name: &String, port: i32) -> Result<Output, std::io::Error> {
    fn create_master_spawner() -> Result<Output, std::io::Error> {
        let tmux_create_cmd = "tmux new -s".to_string()
            + &*dotenvy::var("MASTER_TMUX_SPAWNER_NAME").unwrap() + " -d";

        let output = Command::new("sh")
            .arg("-c")
            .arg(tmux_create_cmd)
            .output()?;
        Ok(output)
    }

    fn init_ttyd(cmd: &String) -> Result<Output, std::io::Error> {
        Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
    }

    fn build_ttyd_cmd(node_name: &String, port: i32) -> String {
        let mut open_ttyd_cmd = "tmux send-keys -t ".to_string();
        open_ttyd_cmd.push_str(&*dotenvy::var("MASTER_TMUX_SPAWNER_NAME").unwrap());
        open_ttyd_cmd.push(' ');
        open_ttyd_cmd.push_str("\"ttyd -p ");
        open_ttyd_cmd.push_str(&*port.to_string());
        open_ttyd_cmd.push_str(" tmux new -A -s ");
        open_ttyd_cmd.push_str(node_name);
        open_ttyd_cmd.push_str(" bash\" C-m");
        open_ttyd_cmd
    }

    create_master_spawner()?;
    let cmd = build_ttyd_cmd(&node_name, port);
    println!("command is: {cmd}");
    init_ttyd(&cmd)
}