import std/[sequtils, parseutils]

proc read(file: string): seq[int] =
  for line in file.lines:
    var res: int
    if parseInt(line, res) == 0:
      break
    result.add(res)

proc zipcmp(arr: seq[int], start: int): int =
  for (p, q) in arr.zip(arr[start .. ^1]):
    if p < q:
      result += 1

let s = read("input.txt")
echo "Day 1 Part 1: ", zipcmp(s, 1), ", Part 2: ", zipcmp(s, 3)
