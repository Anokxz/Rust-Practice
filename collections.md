# Collections
## Vectors

1. Create
    i. `let v: Vec<i32> = Vec::new();`
    ii. `let v = vec![1, 2, 3];`
2. Update
    i. Use push to add to vector : `v.push(5);` (should be mutable to edit)
3. Reading
    i. using indexing : `let third: &i32 = &v[2];`6
    ii. using get
    ```rust
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    ```