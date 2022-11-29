# aocfetch

Use this tool to get your input files for Advent of Code.

The tool downloads and caches the files, so it's safe to use frequently as it won't download the file every time.

## Requirements

The tool needs your Advent of Code session to work. You can find your `session` in your browser, look into the cookies
for https://adventofcode.com/ and copy the value for the `session` cookie.

Put the value in an environment variable called `AOC_SESSION` for the tool to work.

## Examples

The following example will print out the input for the [day 1 of 2021](https://adventofcode.com/2021/day/1)
```
$ export AOC_SESSION=12345
$ aocfetch --year 2021 --day 1
123
126
130
137
140
...
```

You can pipe it to your implementation and read the contents from the stdin. The following example calls a ruby implementation:
```
$ export AOC_SESSION=12345
$ aocfetch --year 2021 --day 1 | ruby src/2011/01/main.rb
Part 1: 1799
Part 2: 1899
```
