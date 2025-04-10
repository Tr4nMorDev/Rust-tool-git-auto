
use std::process::Command;

fn run(cm : &str ,  args : &[&str]){
    let status = Command::new(cm)
    .args(args)
    .status()
    .expect("fail");
    if !status.success(){
        println!("Comand `{}` failes",status);
    }
}

fn main() {
    let remote_url = "https://github.com/Tr4nMorDev/Rust-tool-git-auto.git";

    println!("🌀 Initializing git repo...");
    run("git", &["init"]);

    println!("🔗 Adding remote origin...");
    run("git", &["remote", "add", "origin", remote_url]);

    println!(" Add file readme.md");
    run("touch" , &["README.md"]);

    println!("📦 Adding all files...");
    run("git", &["add", "."]);

    println!("📝 Committing...");
    run("git", &["commit", "-m", "initial commit"]);

    println!("🚀 Pushing to remote...");
    run("git", &["push", "-u", "origin", "master"]);

    println!("✅ Done!");
}
