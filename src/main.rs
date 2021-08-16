use std::io::{Write,Result};
use std::process::{Command, Stdio, Output};
//use std::error::Error;

//fn parallelize(command: &str, input: Vec<String>) -> Result<Output> {
/// Execute command in a subprocess using Gnu Parallel with given input
/// Runs parallel instances of command with on item of input per instance
/// E.g. input ['a','b','c'] -> parallel-exec [command 'a', command 'b', command 'c']
fn parallelize(command: &str, input: &'static [String]) -> Result<Output> {
    
    let mut child = Command::new("parallel")
        .arg(command)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin.write_all(input.join("\n").as_bytes()).expect("Failed to write to stdin");
    });
    
    child.wait_with_output()//.expect("Failed to read stdout");
}

fn main() {

    
    loop {
        // 1. get a whole set of input (e.g. names of all files in some directory)
        //    if there's no input sleep for a while and try again
        let v: Vec<String> = (0..101).map(|x| x.to_string()).collect();
        let v2 = vec!["Hello", "world!"];
        let v3 = vec!["Hello".to_owned(), "world!".to_owned()];

        // 2. break it into chucks of a specified size
        let chunksize = 50;
        let numchunks = v.len()/chunksize; // 2
        let leftover = v.len() % chunksize; // 1
        println!("number of chunks {}", numchunks);
        println!("leftover {}", leftover);

        // ??? how to break input up into chunks
  
        // Each chunk is a reference to slice of our Vec<String>:  &[String] &["foo", "bar",...,"baz"]
        // chunk 1: &v[index..(start+chunksize)] [0..50] 0-49 50-things
        // chunk 2: &v[index..chunksize] [50..100] 50-99 50-things
        // chunk 3: &v[index..(chunksize*2+1)] [100..101] 100 1-thing

        let mut chunks = vec![];
        for i in 0..numchunks {
            println!("chunk {}", i + 1);
            let index = chunksize*i;
            // Each chunk is reference into the String Vector?
            chunks.push(&v[index..(index+chunksize)]);
        }
        if leftover != 0 {
            let index = chunksize*numchunks;
            chunks.push(&v[index..(index+leftover)])
        }

        let mut i = 0;
        for c in chunks {
            println!("chunk {}", i);
            for s in c {
                println!("{}", s);
            }
            i = i + 1;
        }

        break;
        // 3. process each chunk in parallel 
        //for c in chunks {
        //    // c is a chunk w/ chunksize elements
        //    let output = parallelize("echo {%}-{#}-{}", c);
        //    println!("{}", String::from_utf8_lossy(&output.unwrap().stdout));
        //}

        // 4. Do any necessary postprocessing
    }
}