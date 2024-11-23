#[derive(Debug, PartialEq)]
enum TransactionKind {
    Credit,
    Debit,
}

#[derive(Debug)]
struct Date {
    month: u32,
    day: u32,
    year: u32,
}

#[derive(Debug)]
struct Amount {
    value_times_100: i32,
}

#[derive(Debug)]
struct Transaction {
    kind: TransactionKind,
    date: Date,
    description: String,
    amount: Amount,
}

#[derive(Debug)]
enum ParserState {
    Kind,
    Date,
    Description,
    Amount,
}

fn parse_transactions(input: &str) -> Vec<Transaction> {
    let mut transactions = Vec::new();
    let mut current_transaction = Transaction {
        kind: TransactionKind::Credit,
        date: Date { month: 0, day: 0, year: 0 },
        description: String::new(),
        amount: Amount { value_times_100: 0 },
    };
    let mut state = ParserState::Kind;
    let mut buffer = String::new();

    for c in input.chars() {
        match state {
            ParserState::Kind => {
                if c.is_whitespace() && !buffer.is_empty() {
                    current_transaction.kind = match buffer.as_str() {
                        "CREDIT" => TransactionKind::Credit,
                        "DEBIT" => TransactionKind::Debit,
                        _ => panic!("Invalid transaction kind"),
                    };
                    buffer.clear();
                    state = ParserState::Date;
                } else if !c.is_whitespace() {
                    buffer.push(c);
                }
            },
            ParserState::Date => {
                if c.is_whitespace() && buffer.len() == 8 {
                    current_transaction.date = Date {
                        month: buffer[0..2].parse().unwrap(),
                        day: buffer[2..4].parse().unwrap(),
                        year: buffer[4..8].parse().unwrap(),
                    };
                    buffer.clear();
                    state = ParserState::Description;
                } else if !c.is_whitespace() {
                    buffer.push(c);
                }
            },
            ParserState::Description => {
                if c == '$' {
                    current_transaction.description = buffer.trim().to_string();
                    buffer.clear();
                    state = ParserState::Amount;
                } else {
                    buffer.push(c);
                }
            },
            ParserState::Amount => {
                if c.is_whitespace() && !buffer.is_empty() {
                    let amount: f64 = buffer.parse().unwrap();
                    current_transaction.amount = Amount {
                        value_times_100: (amount * 100.0) as i32,
                    };
                    transactions.push(current_transaction);
                    current_transaction = Transaction {
                        kind: TransactionKind::Credit,
                        date: Date { month: 0, day: 0, year: 0 },
                        description: String::new(),
                        amount: Amount { value_times_100: 0 },
                    };
                    buffer.clear();
                    state = ParserState::Kind;
                } else if !c.is_whitespace() {
                    buffer.push(c);
                }
            },
        }
    }

    // Handle the last transaction if it exists
    if !buffer.is_empty() {
        let amount: f64 = buffer.parse().unwrap();
        current_transaction.amount = Amount {
            value_times_100: (amount * 100.0) as i32,
        };
        transactions.push(current_transaction);
    }

    transactions
}

fn main() {
    let input = "CREDIT 04062020 PayPal transfer $4.99
CREDIT 04032020 Payroll $69.73
DEBIT 04022020 ACH transfer $38.25
DEBIT 03242020 IRS tax kind $52249.98";

    let transactions = parse_transactions(input);
    for transaction in transactions {
        println!("{:?}", transaction);
    }
}