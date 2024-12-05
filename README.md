# advent-of-code-2024
A repository for my solutions to the puzzles in Advent of Code 2024, (hopefully) all done 
in Rust.

## Project structure
The input data for the problems is stored in the input directory. The modules for each 
problem are stored in the src directory, named day1.rs, day2.rs, etc. for each day's 
problem. The main.rs binary can be used to run a certain day's script.

---

### Day 1
The solution for puzzle 1 is the obvious one, just sorting the lists and adding up the 
distances.

In puzzle 2 I first sorted the lists, in order to avoid having to iterate through all of 
list 2 for every item in list 1. This allowed me to only have to iterate through list 2 
once, incrementing the list 1 iterator whenever the current list 2 item exceeded the 
current list 1 item's value.

### Day 2
Not so proud of this one. 

Puzzle 1 is fairly straightforward, simply checking through each report and comparing each 
pair of values to see if they're safe.

For puzzle 2 I tried for a while to find some elegant solution to only iterate through the 
report once, but got frustrated and ended up going with the brute force solution of dropping 
every value until the test passes or the report has been run through. It still completes in 
less than a second on my machine, but I would've liked to make it prettier.

### Day 3
Both puzzle 1 and puzzle 2 were pretty straightforward. 

For puzzle 1 I simply used regex capture groups to iterate through the data and find the 
multiplicand and the multiplier for each operation.

For puzzle 2 I kept the regex, but iterated through the string in slices looking for the 
do() and don't() instructions.

### Day 4
Again, fairly straightforward solutions today.

Puzzle 1 uses a recursive function to find letters of the word 'XMAS' in all directions 
from each X it finds in the matrix of characters.

Puzzle 2 finds every instance of the letter A not on the edge of the matrix, and checks if 
the characters on its opposite diagonals match either ('M', 'S') or ('S', 'M').

Neither of the helper functions is particularly clean, but they do their jobs efficiently.

### Day 5
For both puzzles I used custom orderings. 

In puzzle 1 I simply checked if the update was sorted according to the rules, and summed
up the middle pages for the updates which were.

In puzzle 2 I checked if it wasn't, and used sort_by to sort it if so, then took the sum
of the middle pages just like puzzle 1.