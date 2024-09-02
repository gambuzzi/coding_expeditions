import gleeunit
import gleeunit/should
import out_of_order_44

pub fn main() {
  gleeunit.main()
}

// gleeunit test functions end in `_test`
pub fn merge_test() {
  let a1 = [1, 4, 6, 7, 10]
  let a2 = [2, 3, 5, 8, 9]
  should.equal(
    out_of_order_44.merge(a1, a2, [], 0),
    #(13, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
  )
}

pub fn asimmetric_merge_test() {
  let a1 = [1, 4, 6, 7, 10]
  let a2 = [2, 3]
  should.equal(
    out_of_order_44.merge(a1, a2, [], 0),
    #(8, [1, 2, 3, 4, 6, 7, 10]),
  )
}

pub fn full_inversion_test() {
  let a = [5, 4, 3, 2, 1]
  should.equal(out_of_order_44.count_swaps(a), 10)
}

pub fn some_inversion_test() {
  let a = [2, 4, 1, 3, 5]
  should.equal(out_of_order_44.count_swaps(a), 3)
}
