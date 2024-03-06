// 给定 n 个⾮负整数表示每个宽度为 1 的柱⼦的⾼度图，计算按此排列的柱⼦，下⾬之后能接多少⾬⽔。
// 上⾯是由数组 [0,1,0,2,1,0,1,3,2,1,2,1] 表示的⾼度图，在这种情况下，可以接 6 个单位的⾬
// ⽔（蓝⾊部分表示⾬⽔）。 感谢 Marcos 贡献此图。
// 示例:
// 输⼊: [0,1,0,2,1,0,1,3,2,1,2,1]
// 输出: 6

pub fn trap(nums: Vec<i32>) -> i32 {
  let mut left_max: Vec<i32> = vec![];
  let mut right_max: Vec<i32> = vec![];
  let mut current_max: i32 = 0;
  let mut result: i32 = 0;
  for (_index, value) in nums.iter().enumerate() {
    current_max = current_max.max(*value);
    left_max.push(current_max);
  }
  current_max = 0;
  for (_index, value) in nums.iter().rev().enumerate() {
    current_max = current_max.max(*value);
    right_max.push(current_max);
  }
  for (index, value) in nums.iter().enumerate() {
    result += left_max[index].min(right_max[nums.len() - index - 1]) - value;
  }
  result
}

fn main() {
  let nums: Vec<i32> = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
  let result: i32 = trap(nums);

  println!("{:?}", result);
}
