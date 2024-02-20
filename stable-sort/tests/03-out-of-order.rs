use stable_sort::stable_sorted;

#[stable_sorted]
pub enum Error {
    ThatFailed,
    ThisFailed,
    SomethingFailed,
    WhoKnowsWhatFailed,
}

fn main() {}
