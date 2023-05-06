use crate::bank::Bank;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;
#[derive(Debug)]

struct Ledger {
    from: usize,
    to: usize,
    amount: i64,
    mode: i32,
    ledger_id: i32,
}

impl Ledger {
    fn new(from: usize, to: usize, amount: i64, mode: i32, ledger_id: i32) -> Ledger {
        Ledger {
            from,
            to,
            amount,
            mode,
            ledger_id,
        }
    }
}

// static ledger: Arc<Mutex<Vec<Ledger>>> = Arc::new(Mutex::new(Vec::new()));
// static  bank: Bank = Bank::new(0);

pub fn init_bank(num_workers: usize, filename: &str) {
    let bank = Arc::new(Bank::new(10));
    let mut ledger: Arc<Mutex<VecDeque<Ledger>>> = Arc::new(Mutex::new(VecDeque::new()));
    load_ledger(filename, &mut ledger);
    let mut handles = vec![];

    for worker_id in 0..num_workers {
        let bank_ref = bank.clone();
        let ledger_ref = ledger.clone();
        let handle = thread::spawn(move || {
            worker(worker_id, bank_ref, ledger_ref);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    bank.print_account();
}

fn load_ledger(filename: &str, ledger: &mut Arc<Mutex<VecDeque<Ledger>>>) {
    let file = File::open(filename).expect("Cannot open file");
    let buffered = BufReader::new(file);
    let mut ledger_id = 0;

    for line in buffered.lines() {
        let line = line.expect("Cannot read line");
        let line_arr: Vec<&str> = line.split_whitespace().collect();
        let mut ledger_ref = ledger.lock().unwrap();
        if line_arr.len() == 4 {
            let from = line_arr[0].parse().expect("Cannot parse from");
            let to = line_arr[1].parse().expect("Cannot parse to");
            let amount = line_arr[2].parse().expect("Cannot parse amount");
            let mode = line_arr[3].parse().expect("Cannot parse mode");
            ledger_ref.push_back(Ledger::new(from, to, amount, mode, ledger_id));
            ledger_id += 1;
        }
    }
}

fn worker(worker_id: usize, bank: Arc<Bank>, ledger: Arc<Mutex<VecDeque<Ledger>>>) {
    loop {
        // let found_ledger = {
        //     let ledger_ref = ledger.lock().unwrap();
        //     match ledger_ref.first() {
        //         Some(first_element) => Some(first_element),
        //         None => None,
        //     }
        // };

        let mut ledger_ref = ledger.lock().unwrap();

        if let Some(found_ledger) = ledger_ref.pop_front() {
            drop(ledger_ref);
            match found_ledger.mode {
                0 => {
                    bank.deposit(
                        worker_id as i32,
                        found_ledger.ledger_id,
                        found_ledger.from,
                        found_ledger.amount,
                    );
                }
                1 => {
                    bank.withdraw(
                        worker_id as i32,
                        found_ledger.ledger_id,
                        found_ledger.from,
                        found_ledger.amount,
                    );
                }
                2 => {
                    bank.transfer(
                        worker_id as i32,
                        found_ledger.ledger_id,
                        found_ledger.from,
                        found_ledger.to,
                        found_ledger.amount,
                    );
                }

                3 => {
                    bank.check_balance(
                        worker_id as i32,
                        found_ledger.ledger_id,
                        found_ledger.from,
                    );
                    
                }
                _ => {
                    eprintln!("Unknown mode: {}", found_ledger.mode);
                }
            }
        } else {
            break;
        }
    }
}
