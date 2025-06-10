# Trading System

- Player has a cargo hold, this holds a fixed number of goods, which can be upgraded
- There are different types of goods merchandise, all take up 1 unit of cargo
- Goods have an inherent underlying value - this is modified by planetary local supply/demand
- Player has Cash, used to buy/sell goods at different prices
- Player can buy / sell goods with the trader according to the prevailing prices in the market, at the particular location.

# Solar System Economy

- Each Planet has an underlying economy which is abstracted in the form of goods it buys or sells
- Planets all produce at least 1 item and demand at least 1 item, randomised upon initial game setup
- Goods that planets produced are the cheapest, while goods that planet demand is the most expensive
- There is a margin between buy/sell price of all goods, this also varies by planet and is randomly generated
- Every Turn/Month that passes planets will randomly modify their supply/demand of different goods, which impacts the price.
- There are also events which can impact the supply/demand of goods