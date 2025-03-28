# Breakout Arcade Game
The Classic Breakout Arcade game, built in Rust using Raylib.

# Project Demo
#### Note: 
Some sections of the game experience appear glitchy in the video, likely due to an issue with the recording tool. Please be rest assured that the game functions seamlessly :)

https://github.com/user-attachments/assets/2d43761f-f94f-479a-a2d1-51880e46a971

# Tech-Stack Overview:
![image](https://github.com/user-attachments/assets/1d2c1e95-4191-48e9-83b8-923c65dbed0c)

Note: This repository contains code only for the front-end. For the back-end code, visit: https://github.com/tanrode/breakout-game-backend

# Crates used:
### For Development
<ul>
  <li><b>raylib</b>: A C library binding for 2D and 3D game development.</li>
  <li><b>lazy_static</b>: For safe, one-time, runtime initialization of static variables.</li>
  <li><b>reqwest</b>: A powerful HTTP client for making network requests, with JSON support.</li>
  <li><b>rpassword</b>: Allows secure password input without echoing it to the terminal.</li>
  <li><b>serde</b>: Facilitates serialization and deserialization of data using Rust structs.</li>
  <li><b>serde_json</b>: Provides JSON parsing and serialization functionality using Serde.</li>
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

# Screens
### 1. User Authentication: <br><br> ![image](https://github.com/user-attachments/assets/bb7fb3f6-b5ec-47a5-82c4-63e9db4739db)
### 2. Welcome Screen: <br><br> ![image](https://github.com/user-attachments/assets/78a1ed0a-f558-4d69-971c-4bbc20b023c4)
### 3. Game Screen: <br><br> ![image](https://github.com/user-attachments/assets/94c05a43-dc3e-41e3-91f7-6a9a0fba8222)
### 4. User Stats Screen: <br><br> ![image](https://github.com/user-attachments/assets/1764b10b-5226-4291-aafd-c3e608da6bbd)
### 5. Leaderboard: <br><br> ![image](https://github.com/user-attachments/assets/db0f1714-e180-4938-a993-19239bfce0fa)


