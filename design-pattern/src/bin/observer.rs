use std::cell::RefCell;
use std::rc::Rc;

/// The `Observer` design pattern is a behavioral design pattern that enables an object, know as
/// the `Subject`, to maintain a list of its dependents, called `Observers`,
/// and automatically notify them of any state changes, typically by calling one of their methods.
/// This pattern is particularly useful in `Event-Driven Systems`, when you need to multiple objects
/// about events without tightly coupling them. The definition from the "Head First Design Pattern":
/// > The observer pattern defines a one to many dependency between objects so that when one object
/// changes state, all of its dependents are notified and updated automatically.
/// In the context of algorithmic trading, the Observer pattern can be applied to scenarios such as:
/// - Market Data Feeds: Notifying trading strategies when new market data arrives.
/// - Order Execution Updates: Informing interested parties when an order is executed or its status changes.
///
/// # Structure
/// - Subject(Observable or Publisher): The Subject issue events of interest to other objects. These events
/// occur when the subject(publisher) changes its state or executes some behaviors. Subjects maintains a subscription
/// infrastructure, list of observers and provides methods to attach, detach, and notify them.
/// - Observer(Subscriber): The Observer interface declares the notification interface.
/// In most case, it contains a single `update` method.
/// - Concrete Subject: Implements the subject interface and holds the state of interest.
/// - Concrete Observer: Concrete Observer perform some actions in response to notifications issued by
/// the subject. All of these classes must implement the same interface so the subject isn't coupled to concrete classes.
///

trait Observer {
    fn update(&self, instrument_id: &str, price: f64);
}

struct MomentumStrategy {
    name: String,
    threshold: f64,
}

impl Observer for MomentumStrategy {
    fn update(&self, instrument_id: &str, price: f64) {
        if price > self.threshold {
            println!(
                "{}: [{}] price crossed above threshold! Price: {}",
                self.name, instrument_id, price
            );
            // Implement buy logic
        } else {
            println!(
                "{}: [{}] Price below threshold. Price: {}",
                self.name, instrument_id, price
            );
            // Implement hold or sell logic
        }
    }
}

struct MeanReversionStrategy {
    name: String,
    average_price: RefCell<f64>,
}

impl Observer for MeanReversionStrategy {
    fn update(&self, instrument_id: &str, price: f64) {
        let mut avg = self.average_price.borrow_mut();
        *avg = (*avg * 0.9) + (price * 0.1); // Updating moving average
        if price < *avg {
            println!(
              "{}: [{}] Price below average! Price: {}, Average: {:.2}",
                self.name, instrument_id, price, *avg
            );
            // Implement buy logic
        } else {
            println!(
                "{}: [{}] Price above average. Price: {}, Average: {:.2}",
                self.name, instrument_id, price, *avg
            );
        }
        // Implement sell logic
    }
}

// Define the Subject Trait
trait Subject {
    fn attach(&mut self, observer: Rc<dyn Observer>);
    fn detach(&mut self, observer: &Rc<dyn Observer>);
    fn notify(&self);
}

// Implement the Concrete Subject(Market Data Feed)
struct MarketDataFeed {
    instrument_id: String,
    observers: RefCell<Vec<Rc<dyn Observer>>>,
    price: RefCell<f64>,
}

impl Subject for MarketDataFeed {
    fn attach(&mut self, observer: Rc<dyn Observer>) {
        self.observers.borrow_mut().push(observer);
    }

    fn detach(&mut self, observer: &Rc<dyn Observer>) {
        let mut observers = self.observers.borrow_mut();
        if let Some(pos) = observers.iter().position(|x| Rc::ptr_eq(x, observer)) {
            observers.remove(pos);
        }
    }

    fn notify(&self) {
        let price = *self.price.borrow();
        let instrument_id = &self.instrument_id;
        for observer in self.observers.borrow_mut().iter() {
            observer.update(instrument_id, price);
        }
    }
}

impl MarketDataFeed {
    fn new(instrument_id: &str) -> Self {
        MarketDataFeed {
            instrument_id: instrument_id.to_string(),
            observers: RefCell::new(Vec::new()),
            price: RefCell::new(0.0),
        }
    }
    fn set_price(&self, new_price: f64) {
        *self.price.borrow_mut() = new_price;
        self.notify();
    }
}

fn main() {
    // Create market data feed for AAPL
    let mut market_data_feed = MarketDataFeed::new("AAPL");

    // Create observers
    let momentum_strategy: Rc<dyn Observer> = Rc::new(MomentumStrategy {
        name: String::from("MomentumStrategy"),
        threshold: 150.0,
    });
    let mean_reversion_strategy: Rc<dyn Observer> = Rc::new(MeanReversionStrategy {
       name: String::from("MeanReversionStrategy"),
        average_price: RefCell::new(145.0),
    });

    // Attach observers
    market_data_feed.attach(momentum_strategy.clone());
    market_data_feed.attach(mean_reversion_strategy.clone());

    // Simulate market data updates
    let price_updates = vec![148.0, 151.0, 149.5, 152.5, 147.0];
    for price in price_updates {
        println!("\nMarketDataFeed [{}]: New price is {}", "AAPL", price);
        market_data_feed.set_price(price);
    }

    // Detach momentum strategy
    market_data_feed.detach(&momentum_strategy);

    // More updates
    let more_price_updates = vec![153.0, 146.5];
    for price in more_price_updates {
        println!("\nMarketDataFeed [{}]: New price is {}", "AAPL", price);
        market_data_feed.set_price(price);
    }

}