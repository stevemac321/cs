fn buysell(prices: &[u32]) -> (u32, u32) {
    let mut max_profit = 0;
    let mut buy_index = 0;
    let mut sell_index = 0;

    let mut lo = prices[0];
    let mut hi = prices[0];

    for (i, &price) in prices.iter().enumerate().skip(1) {
        if price < lo {
            lo = price;
            buy_index = i;
        } else if price - lo > max_profit {
            hi = price;
            sell_index = i;
            max_profit = hi - lo;
        }
    }

    (buy_index as u32, sell_index as u32)
}
/*

1. State 1: Price Increases and Max Increases:
   - When the price increases (price > last), and this increase contributes to a higher maximum profit, we update our `hi` (higher) candidate.
   - This corresponds to the match arm for `Greater`.

2. State 2: No Change in Price:
   - When there's no change in price (price == last), we decide whether to update `lo` (lower) or `hi`.
   - This state is encapsulated within the match arm for `Equal`.

3. State 3: Price Decreases and Max Increases:
   - When the price decreases (price < last), but this decrease doesn't affect the maximum profit, we update our `lo` candidate.
   - This corresponds to the match arm for `Less`.

4. State 4: Price Decreases and Max Does Not Increase:
   - When the price decreases (price < last), and this decrease does not lead to a higher maximum profit, we set the new `lo` and reset `hi` to the current `lo`.
   - This state is also encapsulated within the match arm for `Less`.

                   +-------------------+
                   |                   |
                   v                   |
        +-------------------+     +------------------+
        |  State 1: Price    |     |  State 2: No     |
        |  Increases and Max |---->|  Change in Price |
        |  Increases         |<----|                  |
        +-------------------+     +------------------+
            |         ^
            |         |
            |         |
            |         |
            v         |
        +-------------------+     +------------------+
        |  State 3: Price    |     |  State 4: Price  |
        |  Decreases and Max |---->|  Decreases and   |
        |  Increases         |     |  Max Does Not    |
        +-------------------+     |  Increase        |
                   ^                +------------------+
                   |
                   +-------------------+




*/


fn buysell_states(prices: &[u32]) -> (u32, u32) {
    let mut max_profit = 0;
    let mut buy_index = 0;
    let mut sell_index = 0;

    let mut lo = prices[0];
    let mut hi = prices[0];
    let mut last = prices[0];

    for i in 1..prices.len() {
        match prices[i].cmp(&last) {
            std::cmp::Ordering::Greater => {
                // Price increases
                if prices[i] > hi {
                    hi = prices[i];
                    sell_index = i;
                    max_profit = hi - lo;
                }
            }
            std::cmp::Ordering::Equal => {
                // no change
            }
            std::cmp::Ordering::Less => {
                // Price decreases
                if prices[i] < lo {
                    lo = prices[i];
                    buy_index = i;
                    hi = lo; // Reset 'hi' to current 'lo'
                }
            }
        }
        last = prices[i];
    }

    (buy_index as u32, sell_index as u32)
}

fn main() {
    {
        let prices = [100, 113, 85, 105, 110];
        let (buy, sell) = buysell(&prices);
        println!("Buy at index: {}", buy);
        println!("Sell at index: {}", sell);
    }
    {
        let prices = [100, 113, 85, 105, 110];
        let (buy, sell) = buysell_states(&prices);
        println!("Buy at index: {}", buy);
        println!("Sell at index: {}", sell);
    }
}
