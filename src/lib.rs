#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn calorie_counting() {
        let contents = fs::read_to_string("./data/calorie-count")
            .expect("Something went wrong reading the file");

        let mut top_three = vec![0, 0, 0];
        for curr in contents.split("\n\n") {
            let mut total_calorie = 0;
            for c in curr.split("\n") {
                let calorie_int = c.parse::<i32>().unwrap();
                total_calorie += calorie_int;
            }

            if top_three[0] < total_calorie {
                top_three.drain(0..1);
                top_three.push(total_calorie);
                top_three.sort()
            }
        }

        println!(
            "Top three calorie total: {}", top_three[0] + top_three[1] + top_three[2]
        );
    }
}
