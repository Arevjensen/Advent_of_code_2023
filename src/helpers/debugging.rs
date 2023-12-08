use std::io::{stdin, stdout, Read, Write};

//add to loops to pause each iteration after printing some dbg!() info
//Then press enter for next iteration
pub fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
