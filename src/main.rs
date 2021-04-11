use log::{ error };
use git2::{Repository, Oid};
use std::path::Path;
use std::process::Command;

fn main() {
    env_logger::init();

    let repo = match clone_repo() {
        Ok(repo) => repo,
        Err(e) => log_and_exit(e)
    };

    match checkout_commit(&repo){
        Ok(_) => (),
        Err(e) => log_and_exit(e)
    };



    match build_image() {
        Ok(_) => { println!("build success"); },
        Err(e) => log_and_exit(e)
    };
}

fn log_and_exit(message: String) -> ! {
    error!("{}", message);
    std::process::exit(1)
}

fn clone_repo() -> Result<Repository, String> {
    let repo_url = match std::env::var("REPOSITORY_URL") {
        Ok(s) => s,
        Err(e) => {
            error!("REPOSITORY_URL not set");
            return Err(e.to_string());
        }
    };
    let work_dir = match std::env::var("WORKING_DIRECTORY") {
        Ok(s) => s,
        Err(_) => String::from("/work")
    };


    let repo = match Path::new("/etc/hosts").exists() {
        true => Repository::open(Path::new(&work_dir)).unwrap(),  // FIXME git pull?
        false => match Repository::clone(&repo_url, work_dir) {
            Ok(repo) => repo,
            Err(e) => {
                error!("Unable to clone git repository: {}", e.to_string());
                return Err(e.to_string());
            }
        }
    };

    Ok(repo)
}

fn checkout_commit(repo: &Repository) -> Result<(), String> {

    let commit_hash = match std::env::var("COMMIT_HASH") {
        Ok(s) => s,
        Err(e) => {
            error!("COMMIT_HASH not set");
            return Err(e.to_string()); },
    };

    let oid = Oid::from_str(&commit_hash).unwrap();

//    let oid = Oid::from_str(my_oid_str).unwrap();
    // let commit = repo.find_commit(commit_hash).unwrap();

    // let branch = repo.branch(
    //     commit_hash,
    //     &commit,
    //     false,
    // );

    // let obj = repo.revparse_single(&("refs/heads/".to_owned() + commit_hash)).unwrap();

    // repo.checkout_tree(
    //     &obj,
    //     None,
    // );

    // repo.set_head(&("refs/heads/".to_owned() + commit_hash));

    match repo.set_head_detached(oid) {
        Ok(()) => Ok(()),
        Err(e) => Err(e.to_string())
    }
}



fn build_image() -> Result<(), String> {

    // TODO check for .build-tomo

    // FIXME
    let repo_url = std::env::var("REPOSITORY_URL").unwrap();
    let mut repo_parts = repo_url.split('/');
    repo_parts.next();
    repo_parts.next();
    repo_parts.next();

    let commit_hash = match std::env::var("COMMIT_HASH") {
        Ok(s) => s,
        Err(e) => {
            error!("COMMIT_HASH not set");
            return Err(e.to_string()); },
    };
    

    // FIXME deal with .git ending
    let image_name = format!("{}/{}:{}", repo_parts.next().unwrap(), repo_parts.next().unwrap(), commit_hash.get(0..8).unwrap());

    // FIXME redundant
    let work_dir = match std::env::var("WORKING_DIRECTORY") {
        Ok(s) => s,
        Err(_) => String::from("/work")  // default to /work
    };

    match std::env::set_current_dir(Path::new(&work_dir)) {
        Ok(_) => (),
        Err(e) => return Err(e.to_string()) }

    println!("Build image: {}", image_name);

    let output =  Command::new("/app/img")
        .arg("build")
        .arg("-t")
        .arg(image_name)
        .arg("-s")
        .arg(work_dir)
        .arg(".")
        .output()
        .expect("failed to build image");

    println!("{:?}", output);

    Ok(())

}
