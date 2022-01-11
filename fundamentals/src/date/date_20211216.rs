use std::usize;

/// 给定一个数组list，一个参数k，一个最大值t
/// 从数组中找到k个元素，他们的和为sum
/// 找到一个最大的sum <= t
/// 若找到->sum
/// 找不到->-1

// 数学组合的实现

/// 全组合的实现
/// list 原始数组
/// k 每一个组中得到数字个数
/// start_pos 每一次DFS从数组的start_pos开始
/// path 每一个组合
/// ans 所有的组合
pub fn combine(
    list: &Vec<i32>,
    k: usize,
    start_pos: usize,
    path: &mut Vec<i32>,
    ans: &mut Vec<i32>,
) {
    if path.len() == k {
        println!("{:?}", path);
        let sum = path.iter().sum();
        ans.push(sum);
        return;
    }
    let mut i = 0;
    for num in &list[start_pos..] {
        path.push(*num);
        i += 1;
        combine(list, k, i, path, ans);
        path.pop();
    }
}
fn choose_best_sum(t: i32, k: i32, ls: &Vec<i32>) -> i32 {
    if k > ls.len() as i32 {
        return -1;
    }
    let mut path: Vec<i32> = Vec::new();
    let mut ans: Vec<i32> = Vec::new();
    combine(ls, k as usize, 0, &mut path, &mut ans);
    println!("{:?}", ans);
    ans.into_iter().filter(|&res| res > t).max().unwrap()
}

#[cfg(test)]
mod date20211216_tests {
    use super::*;
    fn testing(t: i32, k: i32, ls: &Vec<i32>, exp: i32) -> () {
        assert_eq!(choose_best_sum(t, k, ls), exp)
    }

    #[test]
    fn basics_choose_best_sum() {
        let ts = &vec![50, 55, 56, 57, 58];
        testing(163, 3, ts, 163);
        let ts = &vec![50];
        testing(163, 3, ts, -1);
        let ts = &vec![91, 74, 73, 85, 73, 81, 87];
        testing(230, 3, ts, 228);
        testing(331, 2, ts, 178);
    }
}
