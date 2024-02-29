// 给定两个整数，被除数 dividend 和除数 divisor。将两数相除，要求不使⽤乘法、除法和 mod 运
// 算符。
// 返回被除数 dividend 除以除数 divisor 得到的商。
// 整数除法的结果应当截去（truncate）其⼩数部分，例如：truncate(8.345) = 8 以及
// truncate(-2.7335) = -2
// 示例 1:
// 输⼊: dividend = 10, divisor = 3
// 输出: 3
// 解释: 10/3 = truncate(3.33333..) = truncate(3) = 3
// 示例 2:
// 输⼊: dividend = 7, divisor = -3
// 输出: -2
// 解释: 7/-3 = truncate(-2.33333..) = -2
// 提示：
// 被除数和除数均为 32 位有符号整数。
// 除数不为 0。
// 假设我们的环境只能存储 32 位有符号整数，其数值范围是 [−231, 231 − 1]。本题中，如果除法结
// 果溢出，则返回 231 − 1。

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
