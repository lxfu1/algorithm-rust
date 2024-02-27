// 给定⼀个排序数组，你需要在 原地 删除重复出现的元素，使得每个元素只出现⼀次，返回移除
// 后数组的新⻓度。

// 示例 1:
// 给定数组 nums = [1,1,2],
// 函数应该返回新的⻓度 2, 并且原数组 nums 的前两个元素被修改为 1, 2。
// 你不需要考虑数组中超出新⻓度后⾯的元素。
// 示例 2:
// 给定 nums = [0,0,1,1,1,2,2,3,3,4],
// 函数应该返回新的⻓度 5, 并且原数组 nums 的前五个元素被修改为 0, 1, 2, 3, 4。
// 你不需要考虑数组中超出新⻓度后⾯的元素。

pub fn remove_duplicates(nums: &mut Vec<i32>) -> usize {
  let size: usize = nums.len();
  if size == 0 {
    return 0;
  }
  let mut slow: usize = 0;
  for fast in 1..size {
    if nums[fast] != nums[slow] {
      slow += 1;
      nums[slow] = nums[fast];
    }
  }
  return slow + 1;
}

fn main() {
  let mut mock: Vec<i32> = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
  println!("{:?}", remove_duplicates(&mut mock));
  println!("{:?}", mock);
}
