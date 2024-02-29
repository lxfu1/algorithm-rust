// 给你⼀个升序排列的整数数组 nums ，和⼀个整数 target 。
// 假设按照升序排序的数组在预先未知的某个点上进⾏了旋转。（例如，数组 [0,1,2,4,5,6,7] 可能变
// 为 [4,5,6,7,0,1,2] ）。
// 请你在数组中搜索 target ，如果数组中存在这个⽬标值，则返回它的索引，否则返回 -1 。
// 示例 1：
// 输⼊：nums = [4,5,6,7,0,1,2], target = 0
// 输出：4
// 示例 2：
// 输⼊：nums = [4,5,6,7,0,1,2], target = 3
// 输出：-1
// 示例 3：
// 输⼊：nums = [1], target = 0
// 输出：-1
// 提示：
// 1 <= nums.length <= 5000
// -10^4 <= nums[i] <= 10^4
// nums 中的每个值都 独⼀⽆⼆
// nums 肯定会在某个点上旋转
// -10^4 <= target <= 10^4

use std::f32;

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
  let size: usize = nums.len();
  let mut start: usize = 0;
  let mut end: usize = size - 1;

  while start <= end {
    let mid: usize = start + (f32::ceil((end - start) as f32 / 2.0) as usize);
    if nums[mid] == target.try_into().unwrap() {
      return mid as i32;
    }
    // 左侧有序
    if nums[mid] > nums[start] {
      if target >= nums[start] && target < nums[mid] {
        end = mid - 1;
      } else {
        start = mid + 1;
      }
    } else {
      // 右侧有序
      if target > nums[mid] && target <= nums[end] {
        start = mid + 1;
      } else {
        end = mid - 1;
      }
    }
  }
  -1
}

fn main() {
  let mock: Vec<i32> = vec![4, 5, 6, 7, 0, 1, 2];
  let target: i32 = 7;
  let result = search(mock, target);
  println!("{:?}", result);
}
