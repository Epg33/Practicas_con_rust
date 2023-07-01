use std::collections::HashSet;

#[allow(unused)]

fn main() {
    println!("{}", grow(vec![1, 2, 3, 4, 5]))
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec!(0, 0);
    for i in nums.iter() {
       for j in nums.iter() {
            if i + j == target {
                result = vec!(i.clone(), j.clone())
            }
       } 
    }

    return result
} 

fn contains_duplicate(nums: Vec<i32>) -> bool {
    let dep: HashSet<&i32> = nums.iter().collect();
    return dep.len() != nums.len()
}

fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    return accounts.iter().map(|acc| acc.iter().sum()).max().unwrap()
}
 
fn grow(nums: Vec<i32>) -> i32 {
    nums.iter().fold(1, |acc, elm| acc * elm)
}

fn is_square(n: i64) -> bool {
    let sqr = (n as f64).sqrt();
    sqr % 1.0 == 0.0
}

fn count_by(x: u32, n: u32) -> Vec<u32> {
    let mut nums: Vec<u32> = Vec::new();
    for i in 0..n{ 
        if i * x % x == 0 {
            nums.push(i*x)
        }
    }
    nums
}

// checks if a string end with the other string that is passed
fn solution(word: &str, ending: &str) -> bool {
    word.ends_with(ending) 
}

// squares every digit of a given number a return a number that is the 
// concatenation of all theese numbers
fn square_digits(num: u64) -> u64 {
    let num_string = num.to_string();
    let mut string_for_num = String::new();
    for i in num_string.chars() {
        let i_num = i.to_digit(10).unwrap();
        string_for_num.push_str((i_num*i_num).to_string().as_str())
    }
    return string_for_num.parse::<u64>().expect("expected an unsigned number of 64 bits");
}

// takes a string and returns a string with the position in the alphabet of every char from 
// the first string
fn alphabet_position(text: &str) -> String {
    let mut result = String::new();
    for ch in text.chars() {
        if ch.is_alphabetic() && ch!=' ' {
            let position = (ch as u8 - b'a') +1;
            result.push_str(position.to_string().as_str());
            result.push_str(" ");
        }
    }
    return result;
}



/*Once upon a time, on a way through the old wild mountainous west,…

… a man was given directions to go from one point to another. The directions were "NORTH", "SOUTH", "WEST", "EAST". Clearly "NORTH" and "SOUTH" are opposite, "WEST" and "EAST" too.

Going to one direction and coming back the opposite direction right away is a needless effort. Since this is the wild west, with dreadful weather and not much water, it's important to save yourself some energy, otherwise you might die of thirst!
How I crossed a mountainous desert the smart way.

The directions given to the man are, for example, the following (depending on the language):

["NORTH", "SOUTH", "SOUTH", "EAST", "WEST", "NORTH", "WEST"].
or
{ "NORTH", "SOUTH", "SOUTH", "EAST", "WEST", "NORTH", "WEST" };
or
[North, South, South, East, West, North, West]

You can immediately see that going "NORTH" and immediately "SOUTH" is not reasonable, better stay to the same place! So the task is to give to the man a simplified version of the plan. A better plan in this case is simply:

["WEST"]
or
{ "WEST" }
or
[West]

Other examples:

In ["NORTH", "SOUTH", "EAST", "WEST"], the direction "NORTH" + "SOUTH" is going north and coming back right away.

The path becomes ["EAST", "WEST"], now "EAST" and "WEST" annihilate each other, therefore, the final result is [] (nil in Clojure).

In ["NORTH", "EAST", "WEST", "SOUTH", "WEST", "WEST"], "NORTH" and "SOUTH" are not directly opposite but they become directly opposite after the reduction of "EAST" and "WEST" so the whole path is reducible to ["WEST", "WEST"].
Task

Write a function dirReduc which will take an array of strings and returns an array of strings with the needless directions removed (W<->E or S<->N side by side).

    The Haskell version takes a list of directions with data Direction = North | East | West | South.
    The Clojure version returns nil when the path is reduced to nothing.
    The Rust version takes a slice of enum Direction {North, East, West, South}.

See more examples in "Sample Tests:"
Notes

    Not all paths can be made simpler. The path ["NORTH", "WEST", "SOUTH", "EAST"] is not reducible. "NORTH" and "WEST", "WEST" and "SOUTH", "SOUTH" and "EAST" are not directly opposite of each other and can't become such. Hence the result path is itself : ["NORTH", "WEST", "SOUTH", "EAST"].
    if you want to translate, please ask before translating.

*/
