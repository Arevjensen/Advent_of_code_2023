use std::io::{stdin, stdout, Read, Write};

//add to loops to pause each iteration after printing some dbg!() info
//Then press enter for next iteration
pub fn pause() {
    let mut stdout = stdout();
    stdout.write_all(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read_exact(&mut [0]).unwrap();
}
