use std::error::Error;
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

pub fn init_new_web_terminal(node_ip: &Ipv4Addr, port: i32) -> Result<(), Box<dyn Error>> {
    let ip_without_dots = &node_ip.clone().to_string().replace(".", "");

    let _ = init_tmux_session(ip_without_dots, port)
        .expect("Error executing new tmux cmd");
    //println!("output was {:?}", output);

    Ok(())
}

pub fn start_ssh_session_in_ttyd(
    node_ip: &Ipv4Addr,
    port: i32,
    user: &String,
) -> Result<(), Box<dyn Error>> {
    let url_string = "http://127.0.0.1:".to_string() + &*port.to_string();

    let _ = Command::new("python3")
        .arg("./resources/python_scripts/ssh_init.py")
        .arg(&url_string)
        .arg(user)
        .arg(node_ip.to_string())
        .arg("/home/hugh/.local/bin/chromedriver")
        .output()
        .unwrap();

    //println!("{:?}", output);

    Ok(())
}

pub fn start_minicom_session()