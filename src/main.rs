fn main() {
    let start = std::time::Instant::now();
    let result =
        count_prefix_suffix_pairs(&["aba", "bcb", "ece", "aa", "e"], &[[0, 2], [1, 4], [1, 1]]);
    println!("Time: {:?}", start.elapsed());
    println!("Result: {result:?}");
}

fn count_prefix_suffix_pairs(words: &[&str], queries: &[[i32; 2]]) -> Vec<i32> {
    let vowel_set = std::collections::HashSet::from(['a', 'e', 'i', 'o', 'u']);
    let mut prefix_cnt = vec![0; words.len() + 1];
    let mut prev = 0;
    words.iter().enumerate().for_each(|(i, w)| {
        if vowel_set.contains(&w.chars().next().unwrap())
            && vowel_set.contains(&w.chars().last().unwrap())
        {
            prev += 1;
        }
        prefix_cnt[i + 1] = prev;
    });
    queries
        .iter()
        .enumerate()
        .map(|(_, q)| {
            let [l, r] = *q;
            prefix_cnt[(r + 1) as usize] - prefix_cnt[l as usize]
        })
        .collect::<Vec<_>>()
}
