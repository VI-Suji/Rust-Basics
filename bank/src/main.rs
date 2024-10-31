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

    fn deposit(&mut self, amount: i32) -> i32{
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32{
        self.balance -= amount;
        self.balance
    }

    fn summary(&self) -> String {
        format!("{} has a balance of {}",self.holder,self.balance)
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
    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|account| account.balance).sum()
    }

    fn summary(&self) -> Vec<String> {
        self.accounts.iter().map(|account|account.summary()).collect::<Vec<String>>()
    }
}


fn main() {
    let mut bank = Bank::new();
    let mut account = Account::new(1,String::from("new_user"));
    let mut account1 = Account::new(2,String::from("new_user_1"));
    let mut account2 = Account::new(3,String::from("new_user_2"));

    // bank.add_account(account);
    // println!("{:#?}", bank);

    account.deposit(500);
    account.withdraw(250);
    account1.deposit(1000);
    account1.withdraw(250);
    account2.deposit(100);
    account2.withdraw(25);

    println!("{:#?}", account.summary());
    bank.add_account(account);
    bank.add_account(account1);
    bank.add_account(account2);
    println!("{:#?}", bank);

    println!("{:#?}", bank.summary());
    println!("{}", bank.total_balance());
}
