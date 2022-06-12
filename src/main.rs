fn main() {
    pub fn foo(vector_of_vectors: Vec<Vec<usize>>) {
        vector_of_vectors.iter().map(|vector_of_number|
            {
            let bar = vector_of_number.iter().find(|number|
            number == &&1).expect("this is a long message, which makes rustfmt fails if long enough");
            bar});
    }
    foo(vec![vec![1, 2]]);
}
