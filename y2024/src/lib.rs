use seq_macro::seq;
seq!(N in 1..9 {
    pub mod day0~N;
});
// seq!(N in 10..=25 {
//     pub mod day~N;
// });
