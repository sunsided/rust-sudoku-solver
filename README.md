# A Sudoku Solver in Rust

This project is an attempt at familiarizing myself with Rust in terms of both the
language itself, patterns and workspaces. To do something at least remotely meaningful,
it implements a Sudoku solver.

Only trivial strategies (such as lone and hidden singles) are implemented. If these strategies are not
sufficient, state-space search is used to explore the solutions. For computational efficiency, the candidate
set mutable per branch; however, there's a bug hidden somewhere that will not explore the full set of
branches; combined with the non-deterministic sorting strategies used, this will erratically
report some Sudokus as unsolvable when - in really - the aren't, and futher runs of the solver
_will_ find a solution.

In any case, given that this is just a toy project to get used to Rust, I may or may not attempt to fix it in the future.
For now, the solver works well enough with simple games - YMMV and caveat emptor, as usual.

## Build an run

To build and run, execute

```bash
cargo build
cargo run
```

## Example Puzzle

For reference, here's an example puzzle from the Wikipedia [Sudoku](https://en.wikipedia.org/wiki/Sudoku) page:

![](.readme/puzzle.png)

The following is a solution to the above picture:

![](.readme/solution.png)

## Example Nonomino

For reference, here's an example Nonomino from the Wikipedia [Sudoku](https://en.wikipedia.org/wiki/Sudoku) page:

![](.readme/nonomino.png)

The following is a solution to the above picture:

![](.readme/nonomino-solution.png)

## License

Copyright © 2020 Markus Mayer

This software is licensed under the European Union Public License (EUPL) version
1.2, which is available in the 22 official languages of the EU. Only the English
one is provided in [LICENSE.md](LICENSE.md); for official translations see [here](https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12).
