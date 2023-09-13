impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut candies: Vec<i32> = Vec::new();
        let mut last_rating: Option<i32> = None;
        for (i, rating) in ratings.iter().enumerate() {
            if last_rating.is_none() {
                candies.push(1);
            }
            else if last_rating.unwrap() == *rating {
                candies.push(1);
            }
            else if last_rating.unwrap() > *rating {
                let last_candy = *candies.last().unwrap();
                if  last_candy > 1 {
                    candies.push(1);
                }
                else {
                    for i in (0..candies.len()).rev() {
                        candies[i] += 1;
                        if i > 0 && (ratings[i] == ratings[i - 1] || candies[i - 1] != candies[i]) {
                            break;
                        }
                    }
                    candies.push(1);
                }
            }
            else if last_rating.unwrap() < *rating {
                let last_candy = *candies.last().unwrap();
                candies.push(last_candy + 1);
            }
            last_rating = Some(*rating);
        }
        return candies.iter().sum();
    }
}

fn main() {
    println!("{}", Solution::candy(2, vec![1,0,2]));
    println!("{}", Solution::candy(2, vec![1,2,2]));
}