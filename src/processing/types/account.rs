use iban::Iban;

#[derive(Hash, Debug)]
pub enum AccountType {
    /// Your everyday account: pay bills, buy stuff, receive salary. Cash is flowing here.
    /// No interest though most of the time.
    Checking,
    /// A place to save money. Less accessible, usually provides some interest.
    Saving,
    /// Also for saving money. Generally higher interest than savings account, but you
    /// sign a contract with the bank to leave the money there for a fixed amount of time.
    // Deposit,
    /// This is where you put your money if you want to participate in the stock market.
    Brokerage,
}

/// A proper bank account that is guaranteed to have an Iban.
#[derive(Hash, Debug)]
pub struct Account {
    pub iban: Iban,
    pub name: String,
    pub account_type: Option<AccountType>, // TODO: Can we actually derive this information from the raw data though?
}

/// Almost a bank account, except it's tied to a real account and as such doesn't have
/// an official iban.
#[derive(Hash, Debug)]
pub struct SubAccount {
    /// Bank Sub Account Number. This is not a "real" thing (I think), but it serves
    // its purpose.
    pub bsan: String,
    pub name: String,
    pub parent_account: Account, // Might be nice to have?
    pub account_type: Option<AccountType>,
}
