use calculator::try_eval;
use std::io::{self, BufRead};


fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut buf = String::new();
    loop {
        buf.clear();
        println!("Enter expression:");
        stdin.lock().read_line(&mut buf)?;
        let buf= buf.trim();
        if buf.is_empty() {
            break;
        }

        match try_eval(&buf) {
            Ok(res) => {
                println!("{buf} = {res}",);
            }
            Err(err) => {
                println!("Could not evaluate due to: {err}",);
            }
        }
    }

    Ok(())
}
