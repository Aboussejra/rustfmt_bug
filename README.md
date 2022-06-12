# rustfmt_bug

Here is a minimal example I created after I found at work a case that yields a bug in rustfmt:

The given code here in src/main.rs is badly formatted by rustfmt:

```
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
```

After investigation the long message in .expect() makes rustfmt fail.

Try changing from:
(Here rustfmt does not work)
`.expect("this is a long message, which makes rustfmt fails if long enough, adding those words do the trick")`

to:
(Here rustfmt does work)
`.expect("this is a long message, which makes rustfmt fails if long enough")`

When rustfmt does not work, code is badly formatted and we have :

```
fn main() {
    pub fn foo(vector_of_vectors: Vec<Vec<usize>>) {
        vector_of_vectors.iter().map(|vector_of_number|
            {
            let bar = vector_of_number.iter().find(|number|
            number == &&1).expect("this is a long message, which makes rustfmt fails if long enough, adding those words do the trick");
            bar});
    }
    foo(vec![vec![1, 2]]);
}
```

When it does work (with the shorter message), we have:
```
fn main() {
    pub fn foo(vector_of_vectors: Vec<Vec<usize>>) {
        vector_of_vectors.iter().map(|vector_of_number| {
            let bar = vector_of_number
                .iter()
                .find(|number| number == &&1)
                .expect("this is a long message, which makes rustfmt fails if long enough");
            bar
        });
    }
    foo(vec![vec![1, 2]]);
}
```

This repo was created in order to report this bug, the issue may be found here : 

https://github.com/rust-lang/rustfmt/issues/5381
