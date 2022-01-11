use fundamentals;

fn main() {
    let mut path = Vec::new();
    let mut ans = Vec::new();
    let list = vec![50, 55, 56, 57, 58];
    fundamentals::tool_or_grammar::combine(&list, 3, 0, &mut path, &mut ans);
    println!("{:?} \n count => {}", ans,ans.len());
}
