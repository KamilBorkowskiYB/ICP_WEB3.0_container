use std::cell::RefCell;

thread_local! {
    static LEADERBOARD: RefCell<Vec<(String, u64)>> = RefCell::default();
}

#[ic_cdk::update]
fn add_record(name: String, score: u64){
    LEADERBOARD.with(|records|{
        records.borrow_mut().push((name, score));
    });
}

#[ic_cdk::query]
fn read_leaderboard() -> Vec<(String, u64)> {
    LEADERBOARD.with(|records|{
        records.borrow().clone()
    })
}