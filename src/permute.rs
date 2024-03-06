// 给定⼀个 没有重复 数字的序列，返回其所有可能的全排列。
// 示例:
// 输⼊: [1,2,3]
// 输出:
// [
//   [1,2,3],
//   [1,3,2],
//   [2,1,3],
//   [2,3,1],
//   [3,1,2],
//   [3,2,1]
// ]

fn back_track(result: &mut Vec<Vec<i32>>, temp: &mut Vec<i32>, nums: &[i32]) {
  if temp.len() == nums.len() {
    result.push(temp.clone());
    return;
  }
  for i in 0..nums.len() {
    if temp.contains(&nums[i]) {
      continue;
    }
    temp.push(nums[i]);
    back_track(result, temp, nums);
    temp.pop();
  }
}

fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
  let mut result: Vec<Vec<i32>> = vec![];
  let mut temp: Vec<i32> = vec![];
  back_track(&mut result, &mut temp, &nums);
  result
}

fn main() {
  let nums: Vec<i32> = vec![1, 2, 3];
  let result: Vec<Vec<i32>> = permute(nums);
  println!("{:?}", result);
}
