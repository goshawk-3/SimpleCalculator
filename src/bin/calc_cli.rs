use calculator::try_eval;
use std::io::{self, BufRead};


fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut buf = String::new();
    loop {
        buf.clear();
        println!("> expr:");
        stdin.lock().read_line(&mut buf)?;
        if buf.trim().is_empty() {
            break;
        }

        match try_eval(&buf) {
            Ok(res) => {
                println!("result: {res}",);
            }
            Err(err) => {
                println!("Could not evaluate due to: {err}",);
            }
        }
    }

    Ok(())
}
