// 实现获取下⼀个排列的函数，算法需要将给定数字序列重新排列成字典序中下⼀个更⼤的排列。
// 如果不存在下⼀个更⼤的排列，则将数字重新排列成最⼩的排列（即升序排列）。
// 必须原地修改，只允许使⽤额外常数空间。
// 以下是⼀些例⼦，输⼊位于左侧列，其相应输出位于右侧列。
// 1,2,3 " 1,3,2
// 3,2,1 " 1,2,3
// 1,1,5 " 1,5,1

pub fn swap(nums: &mut Vec<i32>, i: usize, j: usize) {
  let temp: i32 = nums[i];
  nums[i] = nums[j];
  nums[j] = temp;
}

pub fn next_permutation(nums: &mut Vec<i32>) -> &mut Vec<i32> {
  let size: usize = nums.len();
  if size == 0 {
    return nums;
  }
  for (index, value) in nums.iter().enumerate() {
    if index == size - 1 {
      swap(nums, 0, size - 1);
      return nums;
    }
    if nums[size - index - 1] > nums[size - index - 2] {
      swap(nums, size - index - 1, size - index - 2);
      return nums;
    }
  }
  nums
}

fn main() {
  let mut mock: Vec<i32> = vec![1, 1, 5];
  let result: &mut Vec<i32> = next_permutation(&mut mock);
  println!("{:?}", result);
}
