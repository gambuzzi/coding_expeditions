import core
import gleam/int
import gleam/io

pub fn main() {
  io.println("Hello from y2023ex01!")
  io.println("Part 1:")
  let input = core.read_input_file()
  let solution = input |> core.solve(True)
  io.println(solution |> int.to_string)
  io.println("Part 2:")
  let solution = input |> core.solve(False)
  io.println(solution |> int.to_string)
}
