// 给定⼀个整数数组 nums 和⼀个⽬标值 target，请你在该数组中找出和为⽬标值的那 两个 整
// 数，并返回他们的数组下标。
// 你可以假设每种输⼊只会对应⼀个答案。但是，数组中同⼀个元素不能使⽤两遍。
// 示例:
// 给定 nums = [2, 7, 11, 15], target = 9
// 因为 nums[0] + nums[1] = 2 + 7 = 9
// 所以返回 [0, 1]
// ##解题思路
// 对于这道题，我们很容易想到使⽤两层循环来解决这个问题，但是两层循环的复杂度为O(n2)，我们可以考虑能否换⼀种思路，减⼩复杂度。
// 这⾥使⽤⼀个map对象来储存遍历过的数字以及对应的索引值。我们在这⾥使⽤减法进⾏计算
// 1. 计算target和第⼀个数字的差，并记录进map对象中，其中两数差值作为key，其索引值作为value。
// 2. 再计算第⼆个数字与target的差，并与map对象中的数值进⾏对⽐，若相同，直接返回，如果没有相同值，就将这个差值也存⼊map对象中。
// 3. 重复第⼆步，直到找到⽬标值。

use std::{collections::HashMap, usize};

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<usize> {
  let mut res: Vec<usize> = vec![];
  let mut hash_map: HashMap<i32, usize> = HashMap::new();
  for i in 0..nums.len() {
    hash_map.insert(nums[i], i);
  }
  for (index, value) in nums.iter().enumerate() {
    let sub = target - value;
    match hash_map.get(&sub) {
      Some(val) => {
        res.push(index);
        res.push(*val);
        break;
      }
      None => (),
    }
  }
  res
}

fn main() {
  let nums: Vec<i32> = vec![2, 7, 11, 15];
  let target: i32 = 9;
  let result: Vec<usize> = two_sum(nums, target);
  println!("{:?}", result);
}
