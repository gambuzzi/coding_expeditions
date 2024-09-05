import gleam/dict
import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/set.{type Set}
import gleam/string
import simplifile

pub fn main() {
  let input = read_input_file("./input.txt")

  io.println("Part 1:")
  let solution = input |> parser |> part1
  io.println(solution |> int.to_string)

  let input = read_input_file("./input.txt")
  io.println("Part 2:")
  let solution = input |> parser |> part2
  io.println(solution |> int.to_string)
}

pub fn read_input_file(fname: String) -> List(String) {
  let file = simplifile.read(fname)
  case file {
    Ok(contents) -> contents |> string.split("\n")
    Error(_) -> []
  }
}

pub type Star {
  Number(number: Int, x: Int, y: Int, length: Int)
  Symbol(symbol: String, x: Int, y: Int)
  NaN
}

pub fn parse_row_symbols(line: String, y: Int) -> List(Star) {
  line
  |> string.trim
  |> string.split("")
  |> list.index_fold([], fn(acc, char, idx) {
    case char {
      "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "." -> acc
      _ -> [Symbol(char, idx, y), ..acc]
    }
  })
}

pub fn parse_row_numbers(line: String, y: Int) -> List(Star) {
  let ret =
    line
    |> string.split("")
    |> list.index_fold([], fn(acc, char, idx) {
      let c = char |> int.parse |> result.unwrap(0)
      case acc {
        [] ->
          case char {
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" -> [
              Number(c, idx, y, 1),
            ]
            _ -> []
          }
        [head, ..tail] ->
          case char {
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ->
              case head {
                NaN -> [
                  Number(char |> int.parse |> result.unwrap(0), idx, y, 1),
                  ..tail
                ]
                Number(n, xx, yy, l) -> [
                  Number(n * 10 + c, xx, yy, l + 1),
                  ..tail
                ]
                _ -> []
              }
            _ ->
              case head {
                NaN -> acc
                _ -> [NaN, ..acc]
              }
          }
      }
    })
  ret |> list.drop_while(fn(x) { x == NaN })
}

pub fn parser(lines: List(String)) -> #(List(Star), List(Star)) {
  let numbers =
    lines
    |> list.index_map(parse_row_numbers)
    |> list.flatten
  let symbols =
    lines
    |> list.index_map(parse_row_symbols)
    |> list.flatten

  #(numbers, symbols)
}

fn touch(x: Int, y: Int, length: Int, sym_xy: Set(#(Int, Int))) -> Bool {
  sym_xy |> set.contains(#(x - 1, y))
  || sym_xy |> set.contains(#(x + length, y))
  || {
    list.range(x - 1, x + length)
    |> list.any(fn(x) { sym_xy |> set.contains(#(x, y - 1)) })
  }
  || {
    list.range(x - 1, x + length)
    |> list.any(fn(x) { sym_xy |> set.contains(#(x, y + 1)) })
  }
}

fn touch_get(x: Int, y: Int, length: Int, sym_xy: Set(#(Int, Int))) {
  let touched =
    sym_xy
    |> set.filter(fn(e) { touch(x, y, length, set.new() |> set.insert(e)) })
  touched |> set.to_list |> list.first
}

pub fn part1(numbers_symbols: #(List(Star), List(Star))) -> Int {
  let #(numbers, symbols) = numbers_symbols
  let sym_xy =
    symbols
    |> list.fold(set.new(), fn(acc, symbol) {
      case symbol {
        Symbol(_, x, y) -> acc |> set.insert(#(x, y))
        _ -> acc
      }
    })

  numbers
  |> list.fold(0, fn(acc, number) {
    case number {
      Number(n, x, y, l) ->
        case touch(x, y, l, sym_xy) {
          True -> acc + n
          False -> acc
        }
      _ -> acc
    }
  })
}

pub fn part2(numbers_symbols: #(List(Star), List(Star))) -> Int {
  let #(numbers, symbols) = numbers_symbols
  let gears =
    symbols
    |> list.fold(set.new(), fn(acc, symbol) {
      case symbol {
        Symbol("*", x, y) -> acc |> set.insert(#(x, y))
        _ -> acc
      }
    })

  let ratios =
    numbers
    |> list.fold(dict.new(), fn(acc, star) {
      case star {
        Number(n, x, y, l) ->
          case touch_get(x, y, l, gears) {
            Ok(#(gear_x, gear_y)) -> {
              case acc |> dict.get(#(gear_x, gear_y)) {
                Ok(elem) -> {
                  let #(num, cnt) = elem
                  acc |> dict.insert(#(gear_x, gear_y), #(n * num, cnt + 1))
                }
                _ -> acc |> dict.insert(#(gear_x, gear_y), #(n, 1))
              }
            }
            _ -> acc
          }
        _ -> acc
      }
    })

  // ratios.values().filter(|x| x.1 == 2).map(|x| x.0).sum()
  ratios
  |> dict.values
  |> list.filter(fn(x) { x.1 == 2 })
  |> list.map(fn(x) { x.0 })
  |> list.fold(0, fn(a, b) { a + b })
}
