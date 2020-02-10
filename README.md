# StalkerSim
This program is supposed to simulate the Zone from Stalker: Anomaly in the following ways:
+ Movement of NPC Stalkers, faction members.
+ Movement of mutants, mutant spawns.
+ Occurences of blowouts and psy-storms.
+ Placement of anomalies, artifacts as the result of blowouts.
+ Interaction of the above elements - faction conflict, mutant attacks, deaths by anomalies, etc

There will also be the ability to interact with the simulation:
+ The user will be able to place PC Stalkers on the map and move them manually
+ The user will be able to mark NPCs and mutants as "killed" as the result of PC actions
+ The user will be able to press a button to advance the simulation to the next "day", or some other "tick" of the simulation.

The general idea is that this simulation would be useful for running a table-top RPG set in the Zone.

## Building
I'm using features from the nightly rust channel. So to build, you may need to install the nightly version using rustc:
```bash
rustup install nightly
```
and then build with:
```bash
cargo +nightly build
# or
cargo +nightly run
```

This project is powered by [amethyst](https://amethyst.rs/). The first-time build might take a long time, since it pulls down a bunch of dependencies.

## Running
### Windows
`cargo +nightly run` should work out of the box on windows.

### Linux
Install the dependencies required by Amethyst to compile and run. You can find the depndencies here: https://github.com/amethyst/amethyst#dependencies

Then, use `export WINIT_UNIX_BACKEND=x11` to avoid an issue documented here: https://github.com/amethyst/space-menace/issues/32

Afterwards, `cargo +nightly run` should function.