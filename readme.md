# 10 Kinds of People

### currently doesn't work as expected - work in progress!

https://open.kattis.com/problems/10kindsofpeople

_(from Kattis)_

The world is made up of 10 kinds of people, those who understand binary and those who do not. (...) these two groups have divided the world into two regions, the binary-friendly zones and the decimal-friendly zones. They have even published a map like the following to help people keep up with where the areas are (they have used ones and zeros so nobody would have trouble reading it).

1111100000\n
1111000000\n
1110000011\n
0111100111\n
0011111111\n

Users of binary have to stay in the zones marked with a zero. Users of decimal have to stay in the zones marked with a one. You have to figure out if it is possible for either type of person to get between various locations of interest. 

_People can move north, south, east or west, but cannot move diagonally._

### Input
Input starts with a line containing two positive integers, 1≤r≤1000 and 1≤c≤1000. The next r input lines give the contents of the map, each line containing exactly c characters (which are all chosen from 0 or 1).

The next line has an integer 0≤n≤1000. The following n lines each contain one query, given as four integers: r1,c1 and r2,c2. These two pairs indicate two locations on the map, and their limits are 1≤r1,r2≤r and 1≤c1,c2≤c.

### Output
For each query, output binary if a binary user can start from location r1,c1 and move to location r2,c2. Output decimal if a decimal user can move between the two locations. Otherwise, output neither.

## PROBLEM:

Currently simply checks the values in the two given points, and outputs 'neither' if they're different, otherwise 'binary' if they're zeroes
and 'decimal' if they're ones.

## SOLUTION IDEA:

For a given pair of points(r1,c1) and (r2,c2) check first if user can move in horizontal or vertical, that is if in the same row or column 
there are adjacent points of the same value as user... 
