// #![warn(missing_docs)]
//! 数组双指针算法相关题目

/// 26 Remove Duplicates from Sorted Array
/// **easy**
///
/// 给你一个 升序排列 的数组 nums ，请你 原地 删除重复出现的元素，使每个元素 只出现一次 ，返回删除后数组的新长度。元素的 相对顺序 应该保持 一致 。然后返回 nums 中唯一元素的个数。
/// 考虑 nums 的唯一元素的数量为 k ，你需要做以下事情确保你的题解可以被通过：
/// 更改数组 nums ，使 nums 的前 k 个元素包含唯一元素，并按照它们最初在 nums 中出现的顺序排列。nums 的其余元素与 nums 的大小不重要。
/// 返回 k 。
#[doc = include_str!("./docs/0026. Remove Duplicates from Sorted Array.md")]
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    22
}

/// 27 Remove Element  
/// **easy**  
/// 给你一个 升序排列 的数组 nums ，请你 原地 删除重复出现的元素，使每个元素 只出现一次 ，返回删除后数组的新长度。  
/// 元素的 相对顺序 应该保持 一致 。然后返回 nums 中唯一元素的个数。  

#[doc = include_str!("./docs./0027. Remove Element.md")]
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut slow = 0;

    for i in 0..nums.len() {
        if nums[i] != val {
            nums[slow] = nums[i];
            slow += 1;
        }
    }
    return slow as i32;
}
