# advent_of_code

This is my showcase for (eventually) solving all the problems on [adventofcode.com](https://adventofcode.com) using Rust.

## Testing

All the examples can be tested to work by using

    cargo test example

This will show you that my code works for the same example listed on [adventofcode.com](https://adventofcode.com) for each puzzle. For example, an output line like this:

    test aoc_2021_8::tests::example_2 ... ok

Is testing for puzzle number 8, for 2021, for the example input on the 2nd task.

I encourage you to try and solve the problems yourself, you will have a lot of fun. But if you are looking for the solutions to your own inputs, see the next segment.

## Testing with your input

###### (Cheating)

Since every user on [adventofcode.com](https://adventofcode.com) get his own unique input to puzzles, mine are placed in the input directory in this repo. If you want to get the answers to your own input, simply replace the file contents for the specified puzzle.

You can then run tests with:

    cargo test

This will pass if you used my input files, but will fail if you replaced them. That's good! You can see the correct answer by looking at the output.

For example, assuming you replaced the contents of [input/aoc_2021_7.txt](input/aoc_2021_7.txt), you will something like this output:

    thread 'aoc_2021_7::tests::task_1' panicked at 'assertion failed: `(left == right)`
    left: `123456`,
    right: `356958`', src\aoc_2021_7.rs:84:9

This means that my expected output (for puzzle 2021, puzzle 7, part 1) was `356958` but your correct answer is `123456` !

## Thanks

- I've used some small part of code from [TheAlgorithms/Rust](https://github.com/TheAlgorithms/Rust)
- I've learned Rust mainly from YouTubers such as [Let's get Rusty](https://www.youtube.com/@letsgetrusty) | [Derek Banas](https://www.youtube.com/@derekbanas) | [Code to the Moon](https://www.youtube.com/@codetothemoon) | [No Boilerplate](https://www.youtube.com/@NoBoilerplate) > These have been extremely helpful even as I improved in my abilities and I highly recommend you check out their channels!
