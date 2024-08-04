use std::process::Command;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    prepare_system()?;
    install_kubernetes()?;
//    configure_cluster()?;
//    run_benchmarks()?;
    Ok(()) 
}

fn prepare_system() -> Result<(), Box<dyn Error>> {
    println!("preparing system");
    Command::new("apt").args(&["update"]).status()?;
    Command::new("apt").args(&["upgrade", "-y"]).status()?;
    Command::new("apt").args(&["install", "-y", "docker.io"]).status()?;
    Ok(())
}

fn install_kubernetes() -> Result<(), Box<dyn Error>> {
    println!("installing kubernetes...");

    // add the k8's repository to the system, if not already added
    Command::new("sh").args(&["-c", "curl -s https://packages.cloud.google.com/apt/doc/apt-key.gpg | apt-key add -"]).status()?;
    Command::new("sh").args(&["-c", "echo 'deb https://apt.kubernetes.io/ kubernetes-xenial main' > /etc/apt/sources.list.d/kubernetes.list"]).status()?;
    
    println!("completed k8's installation steps");
    Ok(())
}

