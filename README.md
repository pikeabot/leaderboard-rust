# Game Leaderboard
Written using Rust just because

Using https://systemdesign.one/leaderboard-system-design/ as a reference for requirements and system design
Would like to include an absolute leaderboard and relative leaderboards

Using https://github.com/wpcodevo/rust-axum-mysql/tree/master as a reference for Rust API design

## Questions
1. What are the primary use cases of the system? Update the score and display the leaderboard
2. Are the clients distributed across the globe? Yes
3. What is the amount of Daily Active Users (DAU) for writes? 50 million DAU
4. What is the anticipated read: write ratio? 5:1
5. Should the leaderboard be available in real-time? Yes

Running everything locally for now including DBs so some of these things may have to be simulated or ignored

## Requirements
### Functional Requirements
- The client (player) can view the top 10 players on the leaderboard in real-time (absolute leaderboard)
- The client can view a specific player’s rank and score
- The client can view the surrounding ranked players to a particular player (relative leaderboard)
- The client can receive score updates through push notifications
- The leaderboard can be configured for global, regional, and friend circles
- The client can view the historical game scores and historical leaderboards
- The leaderboards can rank players based on gameplay on a daily, weekly, or monthly basis
- The clients can update the leaderboard in a fully distributed manner across the globe
- The leaderboard should support thousands of concurrent players

### Non-Functional Requirements
- High availability
- Low latency
- Scalability
- Reliability
- Minimal operational overhead

## Rust build commands
Because I can't remember anything
`
$ cargo build
$ ./target/debug/hello_world
or
$ cargo run
`

