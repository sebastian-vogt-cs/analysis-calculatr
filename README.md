# analysis-calculatr
Provides many useful tools for mathamtical functions

## what this application is supposed to do in the future:
calculate zeros, calculate extrema, derive, integrate, ... in a command line interface

## what this application can do right now
  - provide a simple command line interface which enables you to enter functions and the 'help' command
  - define as many polynomal functions as you want and print them to the screen by entering their keyletter. Internally the function is translated into a tree-like data structure, mor precisely a Vec<(i8, u32, u32)> and then stored in a hash-map. When printing it to the screen by typing its name (eg f(x)), it's translated back to a string and printed out.
