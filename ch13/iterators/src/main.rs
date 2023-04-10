fn main() {
    println!("Hello, world!");

    let v1 = vec![1,2,3];

    // Iterators are lazy
    // Have no effect until you consume them
    let mut v1_iter = v1.iter();

    // for val in v1_iter {
    //     println!("Got: {}", val);
    // }
    //
    //


    // The iter method produces an iterator over immutable references. If 
    // we want to create an iterator that takes ownership of v1 and returns owned values, 
    // we can call into_iter instead of iter. Similarly, if we want to iterate over 
    // mutable references, we can call iter_mut instead of iter.

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);


    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // we call sum() a consuming adaptor, a method that takes ownership of the iterator
    // sum() takes ownership of v1_iter and consumes it
    let total: i32 = v1_iter.sum();
    // therefore we cannot use v1_iter after this 

    assert_eq!(total, 6);


    // iterator adaptors: don't consume the iterator, but produce a different iterator
    let v1: Vec<i32> = vec![1, 2, 3];

    // use collect() to consume to iterator and return a collection
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}
