// 给定⼀个⽆重复元素的数组 candidates 和⼀个⽬标数 target ，找出 candidates 中所有可以使
// 数字和为 target 的组合。
// candidates 中的数字可以⽆限制重复被选取。
// 说明：
// 所有数字（包括 target）都是正整数。
// 解集不能包含重复的组合。
// 示例 1：
// 输⼊：candidates = [2,3,6,7], target = 7,
// 所求解集为：
// [
// [7],
// [2,2,3]
// ]
// 示例 2：
// 输⼊：candidates = [2,3,5], target = 8,
// 所求解集为：
// [
// [2,2,2,2],
// [2,3,3],
// [3,5]
// ]
// 提示：
// 1 <= candidates.length <= 30
// 1 <= candidates[i] <= 200
// candidate 中的每个元素都是独⼀⽆⼆的。
// 1 <= target <= 500

type Candidate = Vec<i32>;

pub fn back_track(
  result: &mut Vec<Candidate>,
  temp: &mut Candidate,
  nums: &[i32],
  target: i32,
  start: usize,
) {
  if target == 0 {
    // 当目标值为 0 时，说明找到了一组合适的组合，将其添加到结果中
    result.push(temp.clone());
    return;
  }

  for i in start..nums.len() {
    if nums[i] > target {
      // 如果当前数大于目标值，跳过
      continue;
    }

    // 选择当前数，加入到当前组合中
    temp.push(nums[i]);
    // 继续递归探索，注意新的目标值要减去当前数
    back_track(result, temp, nums, target - nums[i], i);
    // 撤销选择
    temp.pop();
  }
}

pub fn combination_sum(nums: Candidate, target: i32) -> Vec<Candidate> {
  let mut result: Vec<Candidate> = vec![];
  let mut temp: Candidate = vec![];

  back_track(&mut result, &mut temp, &nums, target, 0);

  result
}

fn main() {
  let candidates: Candidate = vec![2, 3, 6, 7];
  let target = 7;
  let result: Vec<Candidate> = combination_sum(candidates, target);

  for combination in &result {
    for &num in combination {
      print!("{} ", num);
    }
    println!("");
  }
}
