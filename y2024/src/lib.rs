use seq_macro::seq;
seq!(N in 1..10 {
    pub mod day0~N;
});
seq!(N in 10..=15 {
    pub mod day~N;
});
