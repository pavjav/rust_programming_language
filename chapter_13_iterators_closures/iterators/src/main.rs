fn main() {
    println!("Hello, world!");

    // Iterators are used to iterate over references of an object.
    // Iterators can return mutable references or unmutable ones:


    // This is a mutable iterator. We use .iter_mut() method to loop through &mut i32s and then change all their vals
    // to 3.
    let mut v1 = vec![1,2,3];
    let v1_iter = v1.iter_mut();
    for v in v1_iter {
        *v = 3;
    }
    println!("{:?}", v1);

    // Non mutable iterators would look like this. Here v is just &i32.

    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    for v in v1_iter {
        println!{"Got: {v}"};
    }

    //Iterators only need to implement a next() method:
    // pub trait Iterator {
    //      type Item;
    //      fn next(&mut self) -> Option<Self::Item>;
    //
    //}

    // Iterators return Options of an associated type. Our vec above uses type Item = i32.
    // Note that an iterator must be mutable. Most methods/syntax will make an iterator mutable behind the scenes.
    // But if you have to call next() you will have to make the iterator mutable.
    // When we use it in a for loop it will return Some() until None

    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // Here we had to make it mutable to make next() work. Basically next() consumes references to its elements until it reaches
    // None. 

    // We saw iter() make an iterator with non-mutable refs, iter_mut() make an iterator with mutable refs.
    // Both of these do not take ownership of the original object. To take ownership of the original object,
    // We can use into_iter(). Below v just takes on i32 type.

    let v1 = vec![1,2,3];
    let v1_iter = v1.into_iter();
    for v in v1_iter{
        println!{"{v}"};
    }

    // An iterator that uses next() is called a consuming adapter because it uses up the iterator.
    // We also move it into the method so that it is no longer usable
    // For example sum()
    // This is a method that uses next to sum up its contents:

    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total,6);
    // v1_iter is no longer usable

    // An iterator that does not use up the iterator is called an iterator adapter. They produce iterators from other iterators.
    // For example, .map() takes an FnMut closure (Does not take ownership of input)
    // And mutates the contents
    // This returns a Map < Iter<'_, i32>, impl FnMut(&i32) -> i32>
    // type. This is  a special iterator, where its next() method returns transformations of its original input container
    // This makes next() a little more complex, but it is ultimately still satisfying the trait Iterator.

    // We include the collect() method on an iterator so that it returns an object

    let map = v1.iter().map(|x| x+1);
    let res: Vec<_> = map.collect();
    println!("{:?}", res);

    // collect() is very useful to collect() an iterator back into an object.
    // filter() is a similar iterator adapter that takes an FnMut()->bool closure to return an iterator of 
    // items that satisfy a particular trait.
    // For example, we take a vector of shoes:

    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker")},
        Shoe { size: 13, style: String::from("sandal")},
        Shoe { size: 10, style: String::from("boot")},
    ];

    // We consume it into an iterator with into_iter()
    // Then use filter() to create an iterator from it of shoes of size 10
    // Collect this into a new vector of Shoes
    // This new vector effectively replaces the original

    let in_my_size: Vec<Shoe> = shoes.into_iter().filter(|s| s.size == 10).collect();
    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker")},
            Shoe { size: 10, style: String::from("boot")},
        ]
    );

    // If we do not consume the original, we obtain a vector of &i32 values:
    // However, this is still distinct from the original vector
    // Below we use iter(), only borrow v1 to filter to the ones,
    // And show that altering the new collection does not alter the old one

    let v1 = vec![1,2,3];
    let mut ones : Vec<&i32> = v1.iter().filter(|x| **x == 1).collect();
    for i in ones.iter_mut() {
        *i = &50;
    }
    println!("{:?}", ones);
    println!("{:?}", v1);

    // Zipping is also very powerful to concatenate containers along a new axis
    // We can then pass a zipped iterator into a map iterator.
    let v1: Vec<i32> = vec![1,2,3];
    let v2: Vec<i32> = vec![4,5,6];
    let prod: Vec<i32> = v1.iter()
        .zip(&v2) // zip will consume a vector unless its borrowed
        .map(|(l,r)| l*r)
        .collect();
    let sum: i32 =  v1.iter()
        .zip(&v2) // zip will consume a vector unless its borrowed
        .map(|(l,r)| l*r)
        .sum();


    println!("The Hadamar product of {:?} and {:?} is {:?}. Its sum is {sum}", v1,v2, prod,);


    


}

#[derive(PartialEq,Debug)]
struct Shoe {
    size: u32,
    style: String
}