# Piston-Tutorials [![Build Status](https://travis-ci.org/PistonDevelopers/Piston-Tutorials.svg)](https://travis-ci.org/PistonDevelopers/Piston-Tutorials)

This is a repository for examples of Piston projects that have are
accompanied by written tutorials explaining core concepts for that
tutorial.

## Current Tutorials

#### [Getting Started Spinning Square](./getting-started)
A "tutorial" with instructions on compiling and running a very
simple Piston.

#### [Sudoku](./sudoku) (work in progress)
Write a Sudoku game with Piston.

## Planned Tutorials

* graphics
* piston

## Troubleshooting

* [I get `ld: library not found for -lSDL2` error on OSX](https://github.com/PistonDevelopers/rust-empty/issues/175)

* I get "GL context creation failed" when running an example.

  It's likely your hardware or driver doesn't support PistonWindow's default OpenGl spec. Just change it to something
  you can support at the beginning of the example. See hello_world.rs for an example.

## Making changes to the tutorials
Because most of the tutorials will contain heavy amounts of 
code, TyOverby developed a markdown pre-processor that takes 
`readme.dev.md` files and includes code from the surrounding 
project.  This way you don't need to make a change in the code
for the tutorial and then also make the same change in the 
readme.md file; the preprocessor will do that for you!

In order to run the pre-processor, simply invoke `cargo run` 
from the root directory (not the sub-tutorial directory) and 
it will rebuild all the markdown files that it knows about.

[How to contribute](https://github.com/PistonDevelopers/piston/blob/master/CONTRIBUTING.md)
