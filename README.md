# Root - Powerful on-chain market making

<div align="center">
  <img height="500x" src="https://i.ibb.co/mc6TBCW/d.png" />
</div>

Root is an AMM protocol built on top of Phoenix. Root lets users to deploy market making vaults with trading strategies such as Avellaneda Stoikov in a permission-less and user-friendly way.

For the OPOS hackathon, we have implemented a simplified Avellaneda-Stoikov market making strategy with following features:

 - Volatility-adjusted dynamic spreads
 - Targeted rebalancing to minimize inventory skew
 - Customizable risk management
 - Workable without any oracle dependency

This strategy can be used for market making on Phoenix  and can be deployed by users in a permission-less and "set and forget" way.

We have also implemented a simplified VolatilityAccumulator that calculates historical volatility based on logarithmic price returns of the asset.


# Why Root?

The core mission of Root is to democratize profitable market making for all users and build trading tools that are powerful yet easy to use.

# Technical Architecture
 
 At the core of Root are vaults, and strategies. Users can create vaults for every fungible token market on Phoenix. Each vault has an associated strategy that defines the trading logic on how the user funds are to be used for trading. Users can customize the strategies by selecting their preferences over particular strategies.
 
This repository is a template POC on how Root is structured at a component level. Currently, a simplified Avellaneda-Stoikov strategy is implemented and on our Rust backend and is market making on Phoenix SOL-USDC market on Solana mainnet.

# Roadmap

Private Alpha - Coming soon (Q3 2023)