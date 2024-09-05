import gleeunit
import gleeunit/should
import y2023ex03.{Number, Symbol, read_input_file}

pub fn main() {
  gleeunit.main()
}

// gleeunit test functions end in `_test`
pub fn parse_row_numbers_test() {
  let input = "123...231...43...*.24"
  let expected = [
    Number(24, 19, 0, 2),
    Number(43, 12, 0, 2),
    Number(231, 6, 0, 3),
    Number(123, 0, 0, 3),
  ]
  should.equal(y2023ex03.parse_row_numbers(input, 0), expected)

  let input = "123...231...43...*.24."
  let expected = [
    Number(24, 19, 0, 2),
    Number(43, 12, 0, 2),
    Number(231, 6, 0, 3),
    Number(123, 0, 0, 3),
  ]
  should.equal(y2023ex03.parse_row_numbers(input, 0), expected)
}

pub fn parse_row_symbol_test() {
  let input = "123...231...43...*.24"
  let expected = [Symbol("*", 17, 1)]
  should.equal(y2023ex03.parse_row_symbols(input, 1), expected)

  let input = "123...231.&.43...*.24."
  let expected = [Symbol("*", 17, 2), Symbol("&", 10, 2)]
  should.equal(y2023ex03.parse_row_symbols(input, 2), expected)
}

pub fn parser_test() {
  let input = read_input_file("./readme_input.txt")

  let expected = #(
    [
      Number(114, 5, 0, 3),
      Number(467, 0, 0, 3),
      Number(633, 6, 2, 3),
      Number(35, 2, 2, 2),
      Number(617, 0, 4, 3),
      Number(58, 7, 5, 2),
      Number(592, 2, 6, 3),
      Number(755, 6, 7, 3),
      Number(598, 5, 9, 3),
      Number(664, 1, 9, 3),
    ],
    [
      Symbol("*", 3, 1),
      Symbol("#", 6, 3),
      Symbol("*", 3, 4),
      Symbol("+", 5, 5),
      Symbol("*", 5, 8),
      Symbol("$", 3, 8),
    ],
  )
  should.equal(y2023ex03.parser(input), expected)
}

pub fn part1_readme_test() {
  let input = read_input_file("./readme_input.txt")
  let solution = input |> y2023ex03.parser |> y2023ex03.part1
  should.equal(solution, 4361)
}

pub fn part1_test() {
  let input = read_input_file("./input.txt")
  let solution = input |> y2023ex03.parser |> y2023ex03.part1
  should.equal(solution, 553_079)
}

pub fn part2_test() {
  let input = read_input_file("./input.txt")
  let solution = input |> y2023ex03.parser |> y2023ex03.part2
  should.equal(solution, 84_363_105)
}
