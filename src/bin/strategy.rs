trait ExecutionStrategy {
    fn execute_order(&self, order_id: u32, quantity: u32);
}

struct TwapStrategy;
struct VwapStrategy;
struct PovStrategy {
    participation_rate: f64,
}

impl ExecutionStrategy for TwapStrategy {
    fn execute_order(&self, order_id: u32, quantity: u32) {
        println!("Executing order {} using TWAP for {} units.", order_id, quantity);
        // Implement TWAP logic here
    }
}

impl ExecutionStrategy for VwapStrategy {
    fn execute_order(&self, order_id: u32, quantity: u32) {
        println!("Executing order {} using VWAP for {} units", order_id, quantity);
        // Implement VWAP logic here
    }
}

impl ExecutionStrategy for PovStrategy {
    fn execute_order(&self, order_id: u32, quantity: u32) {
        println!(
            "Executing order {} using POV at {}% participation for {} units.",
            order_id, self.participation_rate * 100.0, quantity
        );
        // Implement POV logic here
    }
}

struct OrderExecutor {
    strategy: Box<dyn ExecutionStrategy>,
}

impl OrderExecutor {
    fn new(strategy: Box<dyn ExecutionStrategy>) -> Self {
        Self { strategy }
    }

    fn set_strategy(&mut self, strategy: Box<dyn ExecutionStrategy>) {
        self.strategy = strategy;
    }

    fn execute(&self, order_id: u32, quantity: u32) {
        self.strategy.execute_order(order_id, quantity);
    }
}

fn main() {
    let order_id = 101;
    let quantity = 1000;

    // Using TWAP Strategy
    let twap_strategy = Box::new(TwapStrategy);
    let mut executor = OrderExecutor::new(twap_strategy);
    executor.execute(order_id, quantity);

    // Switching to VWAP Strategy
    let vwap_strategy = Box::new(VwapStrategy);
    executor.set_strategy(vwap_strategy);
    executor.execute(order_id + 1, quantity);

    // Switching to POV Strategy
    let pov_strategy = Box::new(PovStrategy {
        participation_rate: 0.1,
    });
    executor.set_strategy(pov_strategy);
    executor.execute(order_id + 2, quantity);
}