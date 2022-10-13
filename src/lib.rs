pub struct Account {
    balance: u128,
}

impl Account {
    /// Creates a new account with 0 balance
    ///
    /// # Examples
    /// ```
    /// use savings_account::Account;
    /// let account = Account::new();
    /// assert_eq!(account.get_balance(), 0);
    /// ```
    pub fn new() -> Account {
        Account { balance: 0 }
    }

    pub fn get_balance(&self) -> u128 {
        self.balance
    }

    pub fn deposit(&mut self, amount: u128) {
        self.balance += amount;
        println!(
            "Deposited {} into the account and new balance is {}",
            amount,
            self.get_balance()
        );
    }

    pub fn withdraw(&mut self, amount: u128) {
        assert!(amount <= self.get_balance(), "Not enough balance");
        self.balance -= amount;
        println!(
            "Withdrawn {} from account and new balance is {}",
            amount,
            self.get_balance()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_have_zero_balance_on_creation() {
        let account = Account::new();
        let balance = account.get_balance();
        assert_eq!(balance, 0, "Balance is not 0 in creation");
    }

    #[test]
    fn it_should_update_balance_on_deposit() {
        let mut account = Account::new();
        account.deposit(10);
        let balance = account.get_balance();
        assert_eq!(balance, 10, "Balance is not updated in creation");
    }

    #[test]
    #[should_panic(expected = "Not enough balance")]
    fn it_should_not_allow_to_withdraw_more_than_balance() {
        let mut account = Account::new();
        account.deposit(10);
        account.withdraw(11);
    }

    #[test]
    fn it_withdraw_successfully() {
        let mut account = Account::new();
        account.deposit(10);
        account.withdraw(2);
        assert_eq!(account.get_balance(), 8, "Withdraw not successfull");
    }
}
