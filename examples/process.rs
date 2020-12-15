use std::process::{Command, Stdio};

fn main() {
    test_a_basic_process_launch();
    test_output_to_stdout();
    println!("Done running tests.");
}

fn test_a_basic_process_launch() {
    let output = Command::new("echo")
        .arg("Hello world")
        .output()
        .expect("Failed to execute command");

    assert_eq!(
        b"Hello world\n",
        output.stdout.as_slice(),
        "Assert that the stdout matches what we echoed."
    );
}

fn test_output_to_stdout() {
    let _echo_child = Command::new("echo")
        .arg("Oh no, a tpyo!")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start echo process");
}
