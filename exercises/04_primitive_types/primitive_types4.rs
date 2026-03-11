fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i
        }
    }

    s.len()
}

fn main() {
        let a = [1, 2, 3, 4, 5];
        let nice_slice = &a[1..4];
        let s = String::from("Greetings world");
        let word = first_word(&s);
        println!("Word {}", word);

        println!("Nice slice: {:#?}", nice_slice);
}

#[cfg(test)]
mod tests {
    #[test]


    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];
        let nice_slice = &a[1..4];

        println!("Nice slice: {:#?}", nice_slice);

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        // let nice_slice = ???

        assert_eq!([2, 3, 4], nice_slice);
    }
}
