# Regalis

This is a ***work-in-progress*** attempt at writing a chess engine from scratch.

## Current Status

- Working (slowly, due to holidays) on getting a basic model of a game set up that is playable by two players

## Roadmap
- Develop an algorithm for finding legal moves
- Non-terminal GUI
- Begin developing an actual engine algorithm

## Contributing

As I am a student with little free time, this is going to be a long project, as evidenced by it taking a few weeks to
program the ruleset. That being said, I welcome _all_ contrbutions - anyone is welcome to fork, contribute, and make a
PR for me to review. A good place to look for ideas would be the above roadmap.

## Building

### Dependencies
- rustc
- cargo

### Compiling
`git clone --branch rust-rewrite https://github.com/John123Allison/regalis.git`
`cd regalis`
`cargo build`
`./target/debug/regalis`