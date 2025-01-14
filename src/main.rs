use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, Instant};
use std::{process, thread};

fn main() {
    let num_threads = thread::available_parallelism().unwrap().get() as i32;
    println!("Available threads: {}", num_threads);
    threads(num_threads, 12_000_000);
    process(16);
}

fn threads(max_threads: i32, number_max: i32) {
    println!("Numeros primos procurados {}", number_max);
    let mut relatorio = vec![];
    for num_threads in 1..=max_threads {
        let now = Instant::now();
        let divide = number_max as f64 / num_threads as f64;

        let mut handlers = vec![];
        for thread in 1..=num_threads {
            handlers.push(thread::spawn(move || {
                let mut prims = vec![];
                let mut size_final = (divide * thread as f64) as usize;
                let mut size_start = (size_final as f64 - divide + 1.0) as usize;
                println!(
                    "Thread {} started - range: {}..{}",
                    thread, size_start, size_final
                );
                for number in size_start..size_final {
                    if is_prime_number(number) {
                        prims.push(number);
                    }
                }
                println!("Thread {} finished", thread);
                prims
            }));
        }

        let mut results = vec![];
        for handle in handlers {
            results.push(handle.join().unwrap())
        }

        let elapsed_time = now.elapsed();
        let seconds = elapsed_time.as_secs_f64();
        println!("{} seconds.", seconds);
        relatorio.push(seconds);
    }

    let mut writter = BufWriter::new(File::create(PathBuf::from("relatorio.txt")).unwrap());
    for reg in relatorio.iter().enumerate() {
        let line = format!(
            "Numero de threads: {} - Tempo: {} segundos\n",
            reg.0 + 1,
            reg.1
        );
        writter.write_all(line.as_bytes()).unwrap();
    }
    writter.flush().unwrap();
}

fn process(num_process: i32) {
    println!("Process Number: {}", process::id());
    let mut process = vec![];
    let mut handlers = vec![];
    for _ in 1..=num_process {
        handlers.push(thread::spawn(move || {
            let mut pro = Command::new("echo").arg("hello").spawn().unwrap();
            println!("Process Number: {}", pro.id());
            thread::sleep(Duration::from_secs(30));
            pro
        }));
    }
    for handle in handlers {
        process.push(handle.join().unwrap());
    }
    while !process.is_empty() {
        process.pop().unwrap().kill().unwrap();
    }
}
fn is_prime_number(num: usize) -> bool {
    if num < 2 {
        return false;
    }
    for i in 2..=((num as f64).sqrt() as usize) {
        if num % i == 0 {
            return false;
        }
    }
    true
}
