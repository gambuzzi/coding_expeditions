import gleam/erlang
import gleam/int
import gleam/io
import gleam/list

pub fn main() {
  io.println("Hello from out_of_order_44!")
  let a = [5, 4, 3, 2, 1]
  io.debug(a)
  io.debug(count_swaps(a))
  let b = [2, 4, 1, 3, 5]
  io.debug(b)
  io.debug(count_swaps(b))

  let a = list.range(1000, 0)

  let t0 = erlang.system_time(erlang.Millisecond)
  list.range(0, 1000)
  |> list.each(fn(_) {
    let _ = count_swaps(a)
  })
  let t1 = erlang.system_time(erlang.Millisecond)
  let duration = t1 - t0
  io.println("Duration " <> duration |> int.to_string <> "ms")
}

pub fn count_swaps(arr: List(Int)) -> Int {
  merge_sort(arr).0
}

pub fn merge(
  a1: List(Int),
  a2: List(Int),
  acc: List(Int),
  swaps: Int,
) -> #(Int, List(Int)) {
  let len_a1 = a1 |> list.length
  case a1, a2 {
    [], _ -> #(swaps, acc |> list.append(a2))
    _, [] -> #(swaps, acc |> list.append(a1))
    [head1, ..tail1], [head2, ..tail2] -> {
      case head1 > head2 {
        True -> merge(a1, tail2, acc |> list.append([head2]), swaps + len_a1)
        False -> merge(tail1, a2, acc |> list.append([head1]), swaps)
      }
    }
  }
}

fn merge_sort(arr: List(Int)) -> #(Int, List(Int)) {
  case arr |> list.length {
    0 -> #(0, arr)
    1 -> #(0, arr)
    2 -> {
      let assert Ok(first) = arr |> list.first
      let assert Ok(last) = arr |> list.last
      case first > last {
        True -> #(1, arr |> list.reverse)
        False -> #(0, arr)
      }
    }
    _ -> {
      let half = { arr |> list.length } / 2
      let #(cnt1, a1) = merge_sort(arr |> list.take(half))
      let #(cnt2, a2) = merge_sort(arr |> list.drop(half))
      let #(cnt_merged, arr_merged) = merge(a1, a2, [], 0)
      #(cnt1 + cnt2 + cnt_merged, arr_merged)
    }
  }
}
