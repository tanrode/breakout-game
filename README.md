# Breakout Arcade Game
The Classic Breakout Arcade game, built in Rust using Raylib.

# Project Demo

# Tech-Stack Overview:
![image](https://github.com/user-attachments/assets/1d2c1e95-4191-48e9-83b8-923c65dbed0c)

Note: This repository contains code only for the front-end. For the back-end code, visit: https://github.com/tanrode/breakout-game-backend

# Crates used:
### For Development
<ul>
  <li><b>raylib</b>: A C library binding for 2D and 3D game development.</li>
  <li><b>bcrypt</b>: Used for hashing and verifying passwords securely.</li>
  <li><b>lazy_static</b>: For safe, one-time, runtime initialization of static variables.</li>
  <li><b>reqwest</b>: A powerful HTTP client for making network requests, with JSON support.</li>
  <li><b>rpassword</b>: Allows secure password input without echoing it to the terminal.</li>
  <li><b>serde</b>: Facilitates serialization and deserialization of data using Rust structs.</li>
  <li><b>serde_json</b>: Provides JSON parsing and serialization functionality using Serde.</li>
  <li><b>sqlx</b>: An async SQL database client supporting various databases with query validation.</li>
  <li><b>tokio</b>: Provides a set of utilities for asynchronous programming.</li>
</ul>

### For Testing
<ul>
  <li><b>mockall</b>: Enables creating and using mocks for unit testing.</li>
  <li><b>mockito</b>: Provides an HTTP mocking framework for simulating external API calls.</li>
</ul>


# Features
<ul>
  <li>User Session: Creating a separate session for each user.</li>
  <li>User Stats: View stats about your most recent as well as your best attempt.</li>
  <li>Leaderboard: View statistics of the top players of the game.</li>
</ul>
