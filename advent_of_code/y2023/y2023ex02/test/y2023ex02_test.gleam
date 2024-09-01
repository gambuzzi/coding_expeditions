import core
import gleeunit
import gleeunit/should

pub fn main() {
  gleeunit.main()
}

pub fn parser_test() {
  let input = [
    "Game 1: 4 blue, 16 green, 2 red; 5 red, 11 blue, 16 green",
    "Game 2: 15 green, 20 red, 8 blue; 12 green, 7 red",
  ]
  let parsed = core.parser(input)
  should.equal(
    [
      #(1, [
        [#(4, "blue"), #(16, "green"), #(2, "red")],
        [#(5, "red"), #(11, "blue"), #(16, "green")],
      ]),
      #(2, [
        [#(15, "green"), #(20, "red"), #(8, "blue")],
        [#(12, "green"), #(7, "red")],
      ]),
    ],
    parsed,
  )
}

pub fn part1_test() {
  let input = core.read_input_file()
  let solution = input |> core.parser |> core.check_limits
  let expected = 2685
  should.equal(expected, solution)
}

pub fn part2_test() {
  let input = core.read_input_file()
  let solution = input |> core.parser |> core.part2
  let expected = 83_707
  should.equal(expected, solution)
}
