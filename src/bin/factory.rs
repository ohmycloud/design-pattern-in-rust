// +------------------+
// |    Creator       |<---------------------------+
// +------------------+                            |
// | + factoryMethod()|                            |
// +------------------+                            |
// ^                                   |
// |                                   |
// +------------------+          +------------------+
// | ConcreteCreator  |          |   Product        |
// +------------------+          +------------------+
// | + factoryMethod()|          | + operation()    |
// +------------------+          +------------------+
// ^
// |
// +---------------------------+
// |                           |
// +------------------+        +------------------+
// | ConcreteProductA |        | ConcreteProductB |
// +------------------+        +------------------+
// | + operation()    |        | + operation()    |
// +------------------+        +------------------+
trait Order {
    fn place(&self);
}

struct MarketOrder {
    symbol: String,
    quantity: u32,
}

struct LimitOrder {
    symbol: String,
    quantity: u32,
    limit_price: f64,
}

struct StopOrder {
    symbol: String,
    quantity: u32,
    stop_price: f64,
}

impl Order for MarketOrder {
    fn place(&self) {
        println!(
            "Placing Market Order: Buy {} units of {} at market price.",
            self.quantity,
            self.symbol
        );
        // Implement order placement logic here
    }
}

impl Order for LimitOrder {
    fn place(&self) {
        println!(
            "Placing Limit Order: Buy {} units of {} at ${}.",
            self.quantity, self.symbol, self.limit_price
        )
        // Implement order placement logic here
    }
}

impl Order for StopOrder {
    fn place(&self) {
        println!(
          "Placing Stop Order: Buy {} units of {} when price reaches ${}.",
            self.quantity, self.symbol, self.stop_price
        );
        // Implement order placement logic here
    }
}

// OrderFactory
enum OrderType {
    Market,
    Limit(f64), // Limit price
    Stop(f64)   // Stop price
}

struct OrderFactory;

impl OrderFactory {
    fn create_order(order_type: OrderType, symbol: String, quantity: u32) -> Box<dyn Order> {
        match order_type {
            OrderType::Market => Box::new(MarketOrder { symbol, quantity }),
            OrderType::Limit(limit_price) => Box::new(LimitOrder {
                symbol,
                quantity,
                limit_price
            }),
            OrderType::Stop(stop_price) => Box::new(StopOrder {
                symbol,
                quantity,
                stop_price,
            }),
        }
    }
}

fn main() {
    let symbol = String::from("AAPL");
    let quantity = 100;

    // Decide which order type to use
    let order_type = OrderType::Limit(149.0);
    // Create the order using the factory
    let order = OrderFactory::create_order(order_type, symbol.clone(), quantity);
    // Place the order
    order.place();
}