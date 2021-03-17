#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    // n<= 2000 なのでO(n^2)なら間に合う?
    // a[i]よりも小さい数字の個数を保存する配列を左右分２つ作る
    let mut left = vec![0; n];
    let mut right = vec![0; n];
    let waru = 1_000_000_007;
    let mut ans = 0;
    for i in 0..n {
        // a[i] よりもa[j]が小さい時, i < jのとき
        for j in 0..n {
            if a[i] > a[j] {
                if i < j {
                    right[i] += 1;
                } else {
                    left[i] += 1;
                }
            }
        }
        // a[i]に関して, left[i] * (k - 1) + right[i] * kをそれぞれ足していく
        ans += ((right[i] + left[i]) % waru) * (k * (k - 1) / 2 % waru);
        ans += right[i] * k % waru;
        ans %= waru;
    }
    println!("{}", ans);
}
