# Design Patterns in Rust

https://siddharthqs.com/design-patterns-in-rust

## Strategy pattern Class Diagram

```markdown
+-----------------------------------------------------------------+
| <<interface>>                                                   |
| ExecutionStrategy                                               |
|-----------------------------------------------------------------|
| + execute_order(...)                                            |
+-----------------------------------------------------------------+

     /|\                  /|\                   /|\
      |                    |                     |
      |                    |                     |
+---------------+     +--------------+       +--------------+
| TwapStrategy  |     | VwapStrategy |       | PovStrategy  |
|---------------|     |--------------|       |--------------|
|               |     |              |       | - participation_rate: f64 |
|---------------|     |--------------|       |--------------|
| + execute_order(...)|+ execute_order(...)  |+ execute_order(...)|
+---------------+     +--------------+       +--------------+

+-----------------------------+
| OrderExecutor               |
|-----------------------------|
| - strategy: ExecutionStrategy |
|-----------------------------|
| + new(strategy)             |
| + set_strategy(strategy)    |
| + execute(order_id, quantity) |
+-----------------------------+

```