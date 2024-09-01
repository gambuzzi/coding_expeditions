import core
import gleam/int
import gleam/io

pub fn main() {
  io.println("Hello from y2023ex02!")
  io.println("Part 1:")
  let input = core.read_input_file()
  let solution = input |> core.parser |> core.check_limits
  io.println(solution |> int.to_string)
  io.println("Part 2:")
  let solution = input |> core.parser |> core.part2
  io.println(solution |> int.to_string)
}
