fn main() {
    let input_file_path = "day01_input.txt";
    let file_content = read_file_content(input_file_path);
    let elv_list = file_content_to_elvs_vec(file_content);
    let elv_carrying_most_calories = find_elv_carrying_most_calories(&elv_list);
    println!(
        "\x1b[35m ==> The elv carrying higher amount of calories is carrying {} calories !",
        { elv_carrying_most_calories.get_total_of_calories() }
    );
}

fn read_file_content(file_path: &str) -> String {
    match std::fs::read_to_string(file_path) {
        Err(e) => panic!("Error reading file {:?}: \n {:?}", file_path, e),
        Ok(file_content) => file_content,
    }
}

#[derive(PartialEq, Clone, Debug)]
struct Elv {
    carried_calories: Vec<i32>,
}

impl Elv {
    fn _new(carried_calories: Vec<i32>) -> Self {
        Self { carried_calories }
    }

    fn get_total_of_calories(&self) -> i32 {
        self.carried_calories.iter().sum()
    }
}

impl FromIterator<i32> for Elv {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        Elv {
            carried_calories: iter.into_iter().collect(),
        }
    }
}

fn file_content_to_elvs_vec(file_content: String) -> Vec<Elv> {
    let res = file_content.split("\n\n").map(|split| {
        split
            .trim_end()
            .split('\n')
            .map(|number| number.parse::<i32>().unwrap())
    });

    res.map(|split| split.collect::<Elv>())
        .collect::<Vec<Elv>>()
}

fn find_elv_carrying_most_calories(elvs: &Vec<Elv>) -> &Elv {
    elvs.iter()
        .max_by_key(|elv| elv.get_total_of_calories())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elv_should_return_his_total_of_calories() {
        // GIVEN
        let expected_total_calories_of_elv = 1200;
        let elv = Elv {
            carried_calories: vec![600, 300, 300],
        };

        // WHEN
        let total_calories = elv.get_total_of_calories();

        // THEN
        assert_eq!(total_calories, expected_total_calories_of_elv)
    }

    #[test]
    fn should_read_txt_file_content() {
        // GIVEN
        let file_path = "day01_input_test.txt";
        let expected_file_content = "\
            100\n\
            200\n\
               \n\
            100\n\
            200\n\
            300\n\
        ";

        // WHEN
        let file_content = read_file_content(file_path);

        // THEN
        assert_eq!(file_content, expected_file_content);
    }

    #[test]
    fn should_return_vector_of_elvs() {
        // GIVEN
        let file_content = "\
            100\n\
            200\n\
                \n\
            100\n\
            200\n\
            300\n\
        "
        .to_string();

        let expected_elvs = vec![
            Elv {
                carried_calories: vec![100, 200],
            },
            Elv {
                carried_calories: vec![100, 200, 300],
            },
        ];

        // WHEN
        let elvs_vec = file_content_to_elvs_vec(file_content);

        // THEN
        assert_eq!(elvs_vec, expected_elvs);
    }

    #[test]
    fn should_return_elv_carrying_most_calories() {
        // GIVEN
        let elv_carrying_most_calories: Elv = Elv {
            carried_calories: vec![600, 300, 300],
        };
        let elvs = vec![
            elv_carrying_most_calories.clone(),
            Elv {
                carried_calories: vec![100, 200, 140],
            },
            Elv {
                carried_calories: vec![300, 200],
            },
            Elv {
                carried_calories: vec![200, 200],
            },
        ];

        // WHEN
        let found_elv_carrying_most_calories: &Elv = find_elv_carrying_most_calories(&elvs);

        // THEN
        assert_eq!(
            &elv_carrying_most_calories,
            found_elv_carrying_most_calories
        );
    }
}
