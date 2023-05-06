#[warn(unused_assignments)]
use std::sync::Arc;
use std::sync::{Mutex, RwLock};
// use libc::{pthread_rwlock_t, pthread_rwlock_destroy, free};
struct Account {
    account_id: usize,
    balance: RwLock<i64>,
}

impl Account {
    fn new(id: usize) -> Account {
        Account {
            account_id: id,
            balance: RwLock::new(0),
        }
    }
}

pub struct Bank {
    accounts: Vec<Account>,
    num_succ: Arc<Mutex<i32>>,
    num_fail: Arc<Mutex<i32>>,
}

impl Bank {
    pub fn new(num_accounts: usize) -> Bank {
        // let accounts: Vec<Account> = (0..num_accounts).map(Account::new).collect();
        // let accounts = Box::into_raw(vec![Account; num_accounts].into_boxed_slice());
        let mut accounts: Vec<Account> = Vec::with_capacity(num_accounts);
        // let accounts: Vec<Account> = vec![Account::default(); num_accounts];
        for i in 0..num_accounts {
            // accounts[i] = Account::new(i);
            accounts.push(Account::new(i));
        }
        Bank {
            accounts,
            num_succ: Arc::new(Mutex::new(0)),
            num_fail: Arc::new(Mutex::new(0)),
        }
    }

    pub fn print_account(&self) {
        for account in &self.accounts {
            let balance_ref = account.balance.read().unwrap();
            println!("ID# {} | {}", account.account_id, balance_ref);
        }
        let succ = *self.num_succ.lock().unwrap();
        let fail = *self.num_fail.lock().unwrap();
        println!("Success: {} Fails: {}", succ, fail);
    }

    fn record_fail(&self, msg: &str) {
        // *self.num_succ.lock().unwrap() += 1;
        let mut num_fail_ref = self.num_fail.lock().unwrap();
        *num_fail_ref += 1;
        drop(num_fail_ref);
        println!("{}", msg);
    }

    fn record_succ(&self, msg: &str) {
        let mut num_succ_ref = self.num_succ.lock().unwrap();
        *num_succ_ref += 1;
        drop(num_succ_ref);
        println!("{}", msg);
    }

    pub fn deposit(&self, worker_id: i32, ledger_id: i32, account_id: usize, amount: i64) {
        let account = &self.accounts[account_id];
        let mut balance = account.balance.write().unwrap();
        *balance += amount;
        let msg = format!(
            "Worker {} completed ledger {}: deposit {} into account {}\n",
            worker_id, ledger_id, amount, account_id
        );
        self.record_succ(&msg);
        drop(balance);
    }

    pub fn withdraw(&self, worker_id: i32, ledger_id: i32, account_id: usize, amount: i64) -> i32 {
        let mut res = 0;
        let account = &self.accounts[account_id];
        let mut balance = account.balance.write().unwrap();
        if *balance >= amount {
            *balance -= amount;
            let msg = format!(
                "Worker {} completed ledger {}: withdraw {} from account {}\n",
                worker_id, ledger_id, amount, account_id
            );
            self.record_succ(&msg);
        } else {
            let msg = format!(
                "Worker {} failed to complete ledger {}: withdraw {} from account {}\n",
                worker_id, ledger_id, amount, account_id
            );
            self.record_fail(&msg);
            res = -1;
        }
        drop(balance);
        return res;
    }

    pub fn transfer(
        &self,
        worker_id: i32,
        ledger_id: i32,
        src_id: usize,
        dest_id: usize,
        amount: i64,
    ) -> i32 {
        let mut res = 0;
        let src_account = &self.accounts[src_id];
        let dest_account = &self.accounts[dest_id];
        if src_id == dest_id {
            let msg = format!("Worker {} failed to complete ledger {}: transfer {} from account {} to account {}\n", worker_id, ledger_id, amount, src_id, dest_id);
            self.record_fail(&msg);
            return -1;
        }

        let mut src_balance_ref = src_account.balance.write().unwrap();
        let mut dest_balance_ref = dest_account.balance.write().unwrap();

        if *src_balance_ref >= amount {
            *src_balance_ref -= amount;
            *dest_balance_ref += amount;
            let msg = format!(
                "Worker {} completed ledger {}: transfer {} from account {} to account {}\n",
                worker_id, ledger_id, amount, src_id, dest_id
            );
            self.record_succ(&msg);
        } else {
            let msg = format!("Worker {} failed to complete ledger {}: transfer {} from account {} to account {}\n", worker_id, ledger_id, amount, src_id, dest_id);
            self.record_fail(&msg);
            res = -1;
        }

        drop(src_balance_ref);
        drop(dest_balance_ref);
        return res;
    }

    pub fn check_balance(&self, worker_id: i32, ledger_id: i32, account_id: usize) {
        let account = &self.accounts[account_id];
        let balance = account.balance.read().unwrap();
        let msg = format!(
            "Worker {} completed ledger {}: check balance amount of {} in account {}\n",
            worker_id, ledger_id, balance, account_id
        );
        self.record_succ(&msg);
        drop(balance);
    }
}

// impl Drop for Bank {
//     fn drop(&mut self) {
//         for account in &self.accounts {
//             unsafe {
//                 pthread_rwlock_destroy(&mut account.lock);
//             }
//         }
//         unsafe {
//             free(self.accounts as *mut c_void);
//             pthread_mutex_destroy(&mut self.bank_lock);
//         }
//     }
// }
