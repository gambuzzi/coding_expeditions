import core
import gleam/io

pub fn main() {
  let input = [3, 7, 8, 3, 6, 1]
  io.debug(core.solve(input))
}
