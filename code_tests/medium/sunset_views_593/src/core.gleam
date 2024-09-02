import gleam/list

pub fn solve(input: List(Int)) {
  input
  |> list.fold([], fn(acc, x) { [x, ..{ acc |> list.filter(fn(e) { e > x }) }] })
  |> list.length
}
