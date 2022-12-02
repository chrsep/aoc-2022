#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn calorie_counting() {
        let contents = fs::read_to_string("./data/calorie-count")
            .expect("Something went wrong reading the file");

        let mut largest_calorie = 0;
        for curr in contents.split("\n\n") {
            let mut total_calorie = 0;
            for c in curr.split("\n") {
                let calorie_int = c.parse::<i32>().unwrap();
                total_calorie += calorie_int;
            }

            if largest_calorie < total_calorie {
                largest_calorie = total_calorie;
            }
        }

        println!("Largest calorie: {}", largest_calorie);
    }
}
