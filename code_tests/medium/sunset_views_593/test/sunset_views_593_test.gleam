import core
import gleeunit
import gleeunit/should

pub fn main() {
  gleeunit.main()
}

// gleeunit test functions end in `_test`
pub fn core_test() {
  let input = [3, 7, 8, 3, 6, 1]
  should.equal(3, core.solve(input))
}
