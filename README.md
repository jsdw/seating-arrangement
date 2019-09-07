# seating-arrangement

A small binary for discovering "good" seating arrangements across a set of tables.

# Usage

First, you need to compile a CSV whereby each row contains a _cost_ and a pair of individuals that this cost applies to. The lower the cost, the more you believe that those individuals *should* sit together. The higher the cost, the more we believe they should be kept apart. We do not need to provide a cost for every possible pair of individuals; if no cost is provided it is assumed to be 0.

An example CSV might look like this:

```
-100,John,Chris
-50,John,Barry
50,Barry,Chris
100,Barry,Steve
```

In this example, we want John and Chris to sit together a lot, and we want John and Barry to sit together quite a lot (but half as much as John and Chris). We don't want Barry and Chris to sit together, and we **really** don't want Barry and Steve to sit together.

With this in mind, the `seating-arrangement` binary can be called like so (assuming it is available on your `$PATH`):

```
seating-arrangement --scores path/to/costs.csv --tables 10x12 5x8 3x20 7
```

This uses the `csv` file that you've created, and is told that there are 10 12-seat tables, 5 8-seat tables, 3 20-seat tables and a single 7 seat table.

On running this, you'll see output like the following:

```
Starting search (lower score is better). Hit CTRL+C at any time to return the current best result.
0: current -785 (best -785)
10000: current -6277 (best -6277)
20000: current -6477 (best -6477)
30000: current -6477 (best -6477)
```

On the left is the number of _moves_ we've performed to try and improve our seating plan. We can see the current score being worked with (lower is better; this is a sum of how much each person likes each other person on each table). We can also see the best score that we've found so far. If you hit CTRL+C at any point, you'll be shown the seating arrangement for the best score we've found so far. Otherwise, you can leave the program running in the hopes that it will find a better arrangement.

# Installing

To use this program, you'll need to install it. Currently, that will have to be from source, and will require `rust` to be installed.

With Rust installed, just run `cargo install --path .` in this folder to build a binary and place it into a `$PATH`.

# The Algorithm

The algorithm used to find a good seating arrangement is reasonably naive, and essentially starts with some seating arrangement, and then generates a short random sequence of random _swaps_, applying them if they improve the seating arrangement. If the seating arrangement isn't improved after a while, we start looking at moves that might not improve the score.