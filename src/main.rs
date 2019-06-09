use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process::{exit, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

// Args:
// 0: Progname
// 1: Listfile 
// 2: Command pattern ('%' substitutes filename)
// 3: Number of threads to use (optional, default 1)
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <listfile> <command> [nthreads]\n    \
                       <listfile> should contain a list of filenames, one per line.\n    \
                       <command> is the template to run, with '%' substituted for the filename.\n    \
                       It is executed with `sh -c`, so usual shell quoting rules apply.\n    \
                       [nthreads] is optional, it's recommended to use the output of `nproc`.", args[0]);
        exit(1);
    }

    let filelist: Vec<String> = BufReader::new(File::open(&args[1])?)
                                          .lines()
                                          .map(|l| l.unwrap())
                                          .collect();

    let filelist = Arc::new(Mutex::new(filelist));

    let mut threads: Vec<JoinHandle<()>> = Vec::new();

    let nthreads = match args.get(3) {
        Some(s) => s.parse().expect("nthreads must be a number."),
        None    => 1
    };

    for n in 0..nthreads {
        let cmd   = args[2].clone();
        let files = filelist.clone();

        let t = thread::spawn(move || {
            loop {
                let file = {
                    let mut guard = files.lock().unwrap();

                    match guard.pop() {
                        Some(f) => f,
                        None    => return
                    }
                };

                let cmd = cmd.replace("%", &file);

                println!("[Thread {}] {}", n, cmd);

                let res = Command::new("sh")
                                  .arg("-c")
                                  .arg(&cmd)
                                  .stdin(Stdio::null())
                                  .stdout(Stdio::null())
                                  .stderr(Stdio::null())
                                  .status();
                
                match res {
                    Ok(status) => {
                        if !status.success() {
                            eprintln!("[Thread {}] Error: process exited with {}.", n, status.code().unwrap());
                        }
                    },
                    Err(e) => {
                        eprintln!("[Thread {}] Couldn't spawn process: {:?}", n, e);
                    }
                };
            }
        });

        threads.push(t);
    };

    for t in threads {
        t.join().expect("Couldn't join thread.");
    }

    Ok(())
}
