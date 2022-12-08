# Mighty Mancala

## About
Mighty mancala is the best two player mancala gaming experience, ran on the command line. 
The Demo can be watched [here](https://youtu.be/tgR5kKDAFYU).

Play with a friend and see who is the best at mancala
![home page](https://github.com/Dgawel77/MightyMancala/blob/main/images/homepage.png)

## Install
`cargo install mighty-mancala`

## Usage
run the game with 

`mancala`

Move your selection around the screen with the arrow keys `← → ↑ ↓`.

Confirm a selection with `enter`.

To quit the program press `q`.

## Features
Mighty mancala has an easy to use text user interface. 
![game play page](https://github.com/Dgawel77/MightyMancala/blob/main/images/gameplay.png)

It also has multiple game modes to play with 
- Capture
- Avalanche

There are also diffrent difficulties to choose from
- Normal (4 stones per bowl)
- Random

There are more features to come...
- Play against a bot
- More customizable board
- Online multiplayer
- Leader boards
- and more!

## Technical Specifications
Mighty Mancala uses the [cusive tui library](https://github.com/gyscos/cursive) to make all the graphics for the game.
There is also a dependency on the rand crate and the cursive-aligned-view crate.  

Cursive uses a cross-term backend. This backend works with many different operating systems.

Because terminals are not standard some colors or charachters may look different on different terminals. If a specific terminal is giving you trouble please use a different one.

The techincal overview video can be found [here](https://www.youtube.com/watch?v=ZhTjssDRq6w).

This repo is a clone of a private repo that I used for class. 