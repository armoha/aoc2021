import std/[strutils, parseutils]

type pair = tuple[move: string, x: int]

proc read(file: string): seq[pair] =
  for line in file.lines:
    let move = split(line, {' '})
    var amount: int
    discard parseInt(move[1], amount)
    result.add((move: move[0], x: amount))

func part1(nums: seq[pair]): int =
  var h, v = 0
  for n in nums:
    case n.move
    of "down": v += n.x
    of "up": v -= n.x
    of "forward": h += n.x
    else: discard
  result = h * v

func part2(nums: seq[pair]): int =
  var h, v, aim = 0
  for n in nums:
    case n.move
    of "down": aim += n.x
    of "up": aim -= n.x
    of "forward":
      h += n.x
      v += aim * n.x
    else: discard
  result = h * v

let input = read("input.txt")
echo "Day 2 Part 1: ", part1(input), ", Part 2: ", part2(input)
