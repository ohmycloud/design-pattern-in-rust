trait OrderExecutor {
    fn execute_order(&self, order: &Order) -> Result<(), String>;
}

#[derive(Debug)]
struct Order {
    symbol: String,
    quantity: i32,
    price: f64,
}

struct BasicOrderExecutor;
struct LoggingDecorator<T: OrderExecutor> {
    executor: T,
}

struct ValidationDecorator<T: OrderExecutor> {
    executor: T,
}

impl<T: OrderExecutor> ValidationDecorator<T> {
    fn new(executor: T) -> Self {
        Self { executor }
    }
}

impl<T: OrderExecutor> ValidationDecorator<T> {
    fn validate(&self, order: &Order) -> bool {
        // Implement validation logic
        order.quantity > 0 && order.price > 0.0
    }
}

impl<T: OrderExecutor> OrderExecutor for ValidationDecorator<T> {
    fn execute_order(&self, order: &Order) -> Result<(), String> {
        if self.validate(order) {
            println!("Validated Order: {:?}", order);
            self.executor.execute_order(order)
        } else {
            Err(String::from("Validation failed"))
        }
    }
}

impl<T: OrderExecutor> LoggingDecorator<T> {
    fn new(executor: T) -> Self {
        Self { executor }
    }
}

impl<T: OrderExecutor> OrderExecutor for LoggingDecorator<T> {
    fn execute_order(&self, order: &Order) -> Result<(), String> {
        println!("LoggingDecorator: Order received: {:?}", order);
        let result = self.executor.execute_order(order);
        println!("LoggingDecorator: Order executoresult: {:?}", result);
        result
    }
}

impl OrderExecutor for BasicOrderExecutor {
    fn execute_order(&self, order: &Order) -> Result<(), String> {
        // Simulate order executor logic
        println!("Executing order: {:?}", order);
        Ok(())
    }
}

fn main() {
    let order = Order {
        symbol: "AAPL".to_string(),
        quantity: 100,
        price: 150.0,
    };

    // Basic executor
    let basic_executor = BasicOrderExecutor;

    // Decorate with validation
    let validated_executor = ValidationDecorator::new(basic_executor);

    // Further decorate with logging
    let logged_executor = LoggingDecorator::new(validated_executor);

    // Execute the order
    let result = logged_executor.execute_order(&order);
    match result {
        Ok(_) => println!("Order executed successfully."),
        Err(e) => println!("Order execution failed: {}", e),
    }
}