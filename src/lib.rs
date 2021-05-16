pub mod basic_calculations {
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct CalcResults {
        pub mean: f32,
        pub median: f32,
        pub mode: u32,
    }

    pub fn calculate_median_with_sort(nums: &mut Vec<u32>) -> f32 {
        nums.sort();

        let nums_len = nums.len();
        let median = if nums_len % 2 == 0 {
            let m_nums = (&nums[(nums_len / 2) - 1], &nums[(nums_len / 2)]);
            (m_nums.0 + m_nums.1) as f32 / 2.0
        } else {
            nums[((nums_len + 1) / 2) - 1] as f32
        };

        median
    }

    pub fn calculate_mean(nums: &Vec<u32>) -> f32 {
        let mut sum = 0;
        for num in nums {
            sum += num;
        }

        let mean: f32 = sum as f32 / nums.len() as f32;
        mean
    }

    pub fn calculate_mode(nums: &Vec<u32>) -> u32 {
        let mut mode_map: HashMap<u32, u32> = HashMap::new();
        for num in nums {
            let count = mode_map.entry(*num).or_insert(0);
            *count += 1;
        }

        let mut max = 0;
        let mut mode = 0;
        for (num, count) in mode_map {
            if count > max {
                max = count;
                mode = num;
            }
        }
        mode
    }
}

pub mod pig_latin {
    pub fn cleaner_pig_latin(input: &str) -> String {
        let mut chars = input.chars().peekable();
        let mut new_s = String::new();
        while let Some(c) = chars.next() {
            let suffix = match c {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    new_s.push(c);
                    String::from("-hay")
                }
                'a'..='z' | 'A'..='Z' => {
                    format!("-{}ay", c)
                }
                _ => {
                    new_s.push(c);
                    continue;
                }
            };

            while let Some(&c) = chars.peek() {
                match c {
                    'a'..='z' | 'A'..='Z' => {
                        chars.next();
                        new_s.push(c);
                    }
                    _ => break,
                }
            }

            new_s += &suffix;
        }

        new_s
    }

    pub fn crappy_pig_latin(text: &str) -> String {
        let mut pig_latin = String::new();

        for word in text.split_whitespace() {
            let mut new_word = String::new();
            let mut suffix = String::new();
            let chars_len = word.char_indices().count();
            let chars = word.char_indices();
            for (i, char) in chars {
                if i == 0 {
                    match char {
                        'a' | 'e' | 'i' | 'o' | 'u' => {
                            suffix.push_str("-hay");
                        }
                        _ => {
                            suffix = format!("-{}ay", char);
                            continue;
                        }
                    }
                }

                match char {
                    'a'..='z' | 'A'..='Z' => {
                        new_word.push(char);
                    }
                    _ => {
                        if chars_len - 1 == i {
                            suffix.push(char);
                        } else {
                            new_word.push(char);
                        }
                    }
                }
            }

            pig_latin += &(new_word + &suffix + " ");
        }

        pig_latin
    }
}