#[derive(Debug)]
struct Account {
    balance: i32,
    id: u32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {balance: 0, id, holder}
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }

}

fn print_acc(account: &Account){
    println!("{:#?}",account);
}

fn main() {
    let bank = Bank::new();
    println!("Bank is {:#?}",bank);
    let account = Account::new(1, String::from("me"));
    let mut account_ref = &account;
    let account_ref_2 = &account;
    print_acc(account_ref);
    print_acc(account_ref_2);
    print_acc(account_ref);
}
