use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

fn main() {
    let matcher = SkimMatcherV2::default();
    assert_eq!(None, matcher.fuzzy_match("abc", "abx"));
    assert!(matcher.fuzzy_match("axbycz", "abc").is_some());
    assert!(matcher.fuzzy_match("axbycz", "xyz").is_some());

    print_score("axbycz", "abc");
    print_score("axbycz", "axbycz");
    print_score("axbycz", "JJJ");
    print_score("run", "rn");
    print_score("run-integration-test", "rit");
    print_score("run-integration-test", "ruit");
}

// こんなかんじでtarget一覧すべてに対してscoreを計算して降順で並べ替えればskimのようなことができそう
fn print_score(s: &str, p: &str) {
    let matcher = SkimMatcherV2::default();
    let (score, indices) = match matcher.fuzzy_indices(s, p) {
        Some((score, indices)) => (score, indices),
        None => (0, vec![]),
    };
    println!("{}, {}: score:{}, indices: {:?}", s, p, score, indices);
}
