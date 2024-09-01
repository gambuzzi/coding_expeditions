import gleam/dict
import gleam/int
import gleam/list
import gleam/result
import gleam/string
import simplifile

pub fn read_input_file() -> List(String) {
  let file = simplifile.read(from: "./input.txt")
  case file {
    Ok(contents) ->
      contents
      |> string.split("\n")
      |> list.map(string.trim)
    Error(_) -> []
  }
}

pub fn parser(lines: List(String)) {
  lines
  |> list.map(fn(line) {
    let game_rounds = line |> string.split(":")
    let game_id =
      game_rounds
      |> list.first()
      |> result.unwrap("Game -1")
      |> string.split(" ")
      |> list.last()
      |> result.unwrap("-1")
      |> int.parse()
      |> result.unwrap(-1)
    let rounds =
      game_rounds |> list.last() |> result.unwrap("") |> string.split("; ")
    let rounds =
      rounds
      |> list.map(fn(round) {
        round
        |> string.trim
        |> string.split(", ")
        |> list.map(fn(color) {
          let splitted_color = color |> string.split(" ")
          #(
            splitted_color
              |> list.first()
              |> result.unwrap("0")
              |> int.parse()
              |> result.unwrap(0),
            splitted_color |> list.last() |> result.unwrap("white"),
          )
        })
      })
    #(game_id, rounds)
  })
}

pub fn check_limits(games: List(#(Int, List(List(#(Int, String)))))) -> Int {
  let limits = dict.from_list([#("red", 12), #("green", 13), #("blue", 14)])
  games
  |> list.map(fn(game) {
    let game_id = game.0
    let rounds = game.1
    let valid =
      rounds
      |> list.all(fn(round) {
        round
        |> list.all(fn(sub_round) {
          sub_round.0 <= limits |> dict.get(sub_round.1) |> result.unwrap(-1)
        })
      })

    case valid {
      True -> game_id
      False -> 0
    }
  })
  |> list.reduce(fn(a, b) { a + b })
  |> result.unwrap(0)
}

pub fn part2(games: List(#(Int, List(List(#(Int, String)))))) -> Int {
  games
  |> list.map(fn(game) {
    let least = dict.from_list([#("red", 0), #("green", 0), #("blue", 0)])
    game.1
    |> list.fold(least, fn(acc, round) {
      round
      |> list.fold(acc, fn(acc2, cube) {
        let count = cube.0
        let color = cube.1
        let new_value = case acc2 |> dict.get(color) {
          Ok(current_count) -> int.max(current_count, count)
          Error(_) -> count
        }
        acc2 |> dict.insert(color, new_value)
      })
    })
    |> dict.values
    |> list.fold(1, fn(acc, v) { acc * v })
  })
  |> list.fold(0, fn(a, b) { a + b })
}
