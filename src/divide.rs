pub fn divide(dividend: i32, divisor: i32) -> i32 {
  let max: i32 = std::i32::MAX;
  if dividend > max {
    return dividend;
  }
  if divisor > max {
    return divisor;
  }

  let is_negative = (dividend < 0) ^ (divisor < 0);
  let mut dividend = dividend.abs();
  let divisor = divisor.abs();
  let mut result: i32 = 0;
  while dividend >= divisor {
    result += 1;
    dividend -= divisor;
  }
  if is_negative {
    result * -1
  } else {
    result
  }
}

fn main() {
  let divident: i32 = 10;
  let divisor: i32 = 3;
  let result: i32 = divide(divident, divisor);
  println!("{:?}", result);
}
