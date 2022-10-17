use crate::reset::return_string;

mod reset;
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
    pub fn new() -> Self {
        Self { balance: 0 }
    }

    /// Returns balance of a given account
    ///
    /// # Examples
    /// ```
    /// use savings_account::Account;
    /// let mut account = Account::new();
    /// account.deposit(10);
    /// let balance = account.get_balance();
    /// assert_eq!(account.get_balance(), 10);
    /// ```
    pub fn get_balance(&self) -> u128 {
        self.balance
    }

    /// Should allow the user to add funds
    ///
    /// # Examples
    /// ```
    /// use savings_account::Account;
    /// let mut account = Account::new();
    /// let mut balance = account.get_balance();
    /// assert_eq!(account.get_balance(), 0);
    /// account.deposit(10);
    /// balance = account.get_balance();
    /// assert_eq!(account.get_balance(), 10);
    /// account.deposit(10);
    /// balance = account.get_balance();
    /// assert_eq!(account.get_balance(), 20);
    /// ```
    pub fn deposit(&mut self, amount: u128) {
        self.balance += amount;
        return_string();
        println!(
            "Deposited {} into the account and new balance is {}",
            amount,
            self.get_balance()
        );
    }

    /// Should allow the user to withdraw funds
    ///
    /// # Examples
    /// ```
    /// use savings_account::Account;
    /// let mut account = Account::new();
    /// account.deposit(10);
    /// let mut balance = account.get_balance();
    /// assert_eq!(account.get_balance(), 10);
    /// account.withdraw(10);
    /// balance = account.get_balance();
    /// assert_eq!(account.get_balance(), 0);
    /// ```
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
    use crate::reset::return_string;

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
    #[should_panic]
    fn it_should_panic_if_deposit_overflows() {
        let mut account = Account::new();
        let amount = 10;
        let max = u128::MAX;
        account.deposit(amount);
        account.deposit(max);
    }

    #[test]
    fn it_withdraw_successfully() {
        let mut account = Account::new();
        account.deposit(10);
        account.withdraw(2);
        assert_eq!(account.get_balance(), 8, "Withdraw not successfull");
    }

    #[test]
    fn it_should_return_string() {
        let string = return_string();
        assert_eq!("Test", string, "string mismatch");
    }
}
