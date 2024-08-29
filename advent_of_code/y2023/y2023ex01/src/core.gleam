import gleam/bool.{and}
import gleam/list
import gleam/result
import gleam/string
import simplifile

pub fn read_input_file() -> List(String) {
  let file = simplifile.read(from: "./input.txt")
  case file {
    Ok(contents) -> contents |> string.split("\n")
    Error(_) -> []
  }
}

fn min(a: Int, b: Int) -> Int {
  case a < b {
    True -> a
    False -> b
  }
}

fn find(line: String, ptr: Int, only_digits: Bool) -> Result(Int, Nil) {
  let digits = [
    #("one", 1),
    #("two", 2),
    #("three", 3),
    #("four", 4),
    #("five", 5),
    #("six", 6),
    #("seven", 7),
    #("eight", 8),
    #("nine", 9),
    #("0", 0),
    #("1", 1),
    #("2", 2),
    #("3", 3),
    #("4", 4),
    #("5", 5),
    #("6", 6),
    #("7", 7),
    #("8", 8),
    #("9", 9),
  ]

  let max_ptr = string.length(line)

  digits
  |> list.find_map(fn(kv) {
    case only_digits |> and(string.length(kv.0) > 1) {
      True -> Error(Nil)
      False ->
        case
          string.slice(line, ptr, min(max_ptr, ptr + string.length(kv.0)) - ptr)
          == kv.0
        {
          True -> Ok(kv.1)
          False -> Error(Nil)
        }
    }
  })
}

pub fn solve(lines: List(String), only_digits: Bool) -> Int {
  lines
  |> list.map(fn(line) {
    let left =
      list.range(0, string.length(line))
      |> list.find_map(fn(i) { find(line, i, only_digits) })
      |> result.unwrap(0)
    let right =
      list.range(0, string.length(line))
      |> list.reverse
      |> list.find_map(fn(i) { find(line, i, only_digits) })
      |> result.unwrap(0)
    Ok(left * 10 + right)
  })
  |> list.reduce(fn(a, b) { Ok(result.unwrap(a, 0) + result.unwrap(b, 0)) })
  |> result.unwrap(Ok(0))
  |> result.unwrap(0)
}
