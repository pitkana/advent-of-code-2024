# advent-of-code-2024
A repository for my solutions to the puzzles in Advent of Code 2024, (hopefully) all done in Rust.

## Project structure
The input data for the problems is stored in the input directory. The modules for each problem are stored in the src directory, named day1.rs, day2.rs, etc. for each day's problem. The main.rs binary can be used to run a certain day's script.

---

### Day 1
The solution for puzzle 1 is the obvious one, just sorting the lists and adding up the distances.

In puzzle 2 I first sorted the lists, in order to avoid having to iterate through all of list 2 for every item in list 1. This allowed me to only have to iterate through list 2 once, incrementing the list 1 iterator whenever the current list 2 item exceeded the current list 1 item's value.