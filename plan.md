# Overview
I want to write a game about a player that pilots a starship in a solar system as a merchant trader. They will start off with a small amount of money, travel to different planets in the solar system as the planets orbit the sun, trade goods for different prices, eventually being able to buy factories and make investments that produce the good.

The goal of the game is to make as much money as possible in a fixed number of turns, which will act as the high-score board.

# Main Design Choices

- Single Player: Local game only
- Game data format: Use YAML for config & save files
- Turn based: Player will input actions, game engine will process, and then wait for user to input next move
- UI: initially command line - can add Web interface later 
- Language: Written in Rust

# Game Map / Solar System Setup

- There are Multiple Planets with different orbits around the central sun, assuming uniform circular motion
- Each planet has a total orbit length, determined by distance from sun
- Planets all orbit in the same direction (initial simplification, can revisit this assumption later)
- Given time advancing (measured in months), a single planet will travel a distance along it's predefined orbit. Speed is goverened by kepler's laws
- Player can go from a single planet, to another planet Location. 
	- Player has a constant travel speed, but needs to take into account orbit speed of target planet
	- Time taken is based on the distance and speeds of orbit
- Define calcs from current plane to each other planet

# Movement system

Movement system: Player action is to move point to point between planets
- Calculation of the travel time depends on the relative position and speeds of rotation of planets in orbit around the sun  
- Assume Ship travel speed is constant, but taking into account the movement of planets. 
- Game should calculate the fastest way between either travelling in the direction of orbit of all planets, or in reverse orbit direction to meet the target as it revolves around the sun. Player should only be given the fastest of the options.


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
