import core
import gleeunit
import gleeunit/should

pub fn main() {
  gleeunit.main()
}

// gleeunit test functions end in `_test`
pub fn part1_test() {
  let input = core.read_input_file()
  let solution = input |> core.solve(True)
  let expected = 53_386
  should.equal(expected, solution)
}

pub fn part2_test() {
  let input = core.read_input_file()
  let solution = input |> core.solve(False)
  let expected = 53_312
  should.equal(expected, solution)
}
