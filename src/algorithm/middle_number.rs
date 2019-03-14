pub struct Solution {}
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let num1_len = nums1.len();
        let num2_len = nums2.len();
        let sum = num1_len + num2_len;
        let (mut walker, mut nums) = if sum % 2 == 0 {
            (sum / 2 - 1, 2)
        } else {
            (sum / 2, 1)
        };

        let mut nums1_waler = 0;
        let mut nums2_waler = 0;

        while walker != 0 {
            if nums1_waler >= nums1.len() && nums2_waler >= nums2.len() {
                break;
            } else if nums1_waler >= nums1.len() {
                nums2_waler += 1
            } else if nums2_waler >= nums2.len() {
                nums1_waler += 1
            } else {
                if nums1[nums1_waler] < nums2[nums2_waler] {
                    nums1_waler += 1;
                } else {
                    nums2_waler += 1;
                }
            }
            walker -= 1;
        }

        let mut pre_sum = 0;
        let mut i_nums = nums;
        while nums != 0 {
            if nums1_waler >= nums1.len() && nums2_waler >= nums2.len() {
                break;
            } else if nums1_waler >= nums1.len() {
                pre_sum += nums2[nums2_waler];
                nums2_waler += 1;
            } else if nums2_waler >= nums2.len() {
                pre_sum += nums1[nums1_waler];
                nums1_waler += 1;
            } else {
                if nums1[nums1_waler] < nums2[nums2_waler] {
                    pre_sum += nums1[nums1_waler];
                    nums1_waler += 1;
                } else {
                    pre_sum += nums2[nums2_waler];
                    nums2_waler += 1;
                }
            }
            nums -= 1;
        }
        pre_sum as f64 / i_nums as f64
    }
}
