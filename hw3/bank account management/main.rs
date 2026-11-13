use std::io;
use std::thread;
use std::time::Duration;

enum AccountStatus {
    Active,
    Frozen,
}

enum TransactionKind {
    Deposit,
    Withdraw,
    TransferIn,
    TransferOut,
}
 enum BankError {
    AccountNotFound,
    InvalidAmount,
    InsufficientFunds,
    AccountFrozen,
    SameAccount,
 }

struct Account {
 id: u64,
 owner_name: String,
 balance: u64,
 status: AccountStatus,
}

struct Transaction {
account_id: u64,
kind: TransactionKind,
amount: u64,  
}
//----------------------------------------------------------------------------------------

fn read_input(message: &str) -> Option<String> {
    println!("{}", message);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(0) => None,
        Ok(_) => Some(input.trim().to_string()),
        Err(_) => None,
    }
}

fn read_number(message: &str) -> Option<u64> {
    loop {
        let number = read_input(message)?;
        match number.parse::<u64>() {
            Ok(number) => return Some(number),
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

fn read_name(message: &str) -> Option<String> {
    loop {
        let name = read_input(message)?;
        if !name.is_empty() { return Some(name); }
        println!("Name can't be empty.");
    }
}
//----------------------------------------------------------------------------------------

fn create_account(accounts: &mut Vec<Account>, next_id: &mut u64 ) { 
    println!("================Create a new account===================");

    let name = match read_name("Enter the account holder's name: ") {
        Some(value) => value,
        None => return,
    };
    let balance = match read_number("Enter the initial balance: ") {
        Some(value) => value,
        None => return,
    };
    let id = *next_id;

    let account = Account {
        id: id,
        owner_name: name,
        balance: balance,
        status: AccountStatus::Active,
    };

    accounts.push(account);
    *next_id += 1;

    println!("Your account has been created successfully.");
    println!("Your account id: {}", id);
}

//---------------------------------------------------------------------------------------

fn print_menu() {
    println!();
    println!("====================================================");
    println!("         Bank Account Management  (Rust)            ");
    println!("====================================================");
    println!("  1) Create account");
    println!("  2) List accounts");
    println!("  3) Find account by ID");
    println!("  4) Search accounts by name");
    println!("  5) Deposit");
    println!("  6) Withdraw");
    println!("  7) Transfer money");
    println!("  8) Freeze / unfreeze account");
    println!("  9) Close account");
    println!(" 10) Transaction history");
    println!(" 11) Filter transactions by type");
    println!(" 12) Bank statistics");
    println!(" 13) Sort accounts");
    println!("  0) Exit");
    println!("----------------------------------------------------");
}


fn main() {
    let mut accounts: Vec<Account> = Vec::new();
    let mut transaction: Vec<Transaction> = Vec::new();
    let mut next_id: u64 = 1000;

    loop{
         print_menu();

         let choice = match read_number("your choice: ") {
            Some(number) => number,
            None => {
                println!();
                break;
            }
        };

         match choice {
             0 => {
                println!("Goodbye!");
                break; }
             1 => create_account(&mut accounts, &mut next_id),
             _ => {
                println!(" Unknown option. Please enter a number from 0 to 13.");}
        }
        thread::sleep(Duration::from_secs(2));
            
    }

}