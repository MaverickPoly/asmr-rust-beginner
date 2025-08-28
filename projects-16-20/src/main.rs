fn fizz_buzz(n: u32) {
    for i in 1..n+1 {
        let mut output = String::new();

        if i % 3 == 0 {
            output.push_str("Fizz");
        }
        if i % 5 == 0 {
            output.push_str("Buzz");
        }
        if output.is_empty() {
            println!("{}", i);
        } else {
            println!("{}", output);
        }
    }
}

fn is_prime(n: u32) {
    for i in 2..n {
        if n % i == 0 {
            println!("Not Prime!");
            return;
        }
    }
    println!("Prime!");
}

fn is_palindrome(str: &str) {
    if str.chars().rev().collect::<String>().to_lowercase() == String::from(str).to_lowercase() {
        println!("{} is a palindrome!", str);
    } else {
        println!("{} is not palindrome!", str);
    }
}

fn is_anagram(s1: &str, s2: &str) {
    let mut chars1 = s1.to_lowercase().chars().collect::<Vec<char>>();
    let mut chars2 = s2.to_lowercase().chars().collect::<Vec<char>>();
    chars1.sort();
    chars2.sort();

    if chars1 == chars2 {
        println!("{} and {} are anagrams!", s1, s2);
    } else {
        println!("Not anagrams");
    }
}

// Sorting
fn bubble_sort(array: &mut [i32]) {
    for i in 0..array.len() {
        for j in 0..array.len()-i-1 {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }
        }
    }
}

fn selection_sort(array: &mut [i32]) {
    for i in 0..array.len() {
        let mut min = i;
        for j in i+1..array.len() {
            if array[j] < array[min] {
                min = j;
            }
        }
        array.swap(i, min);
    }
}

fn display_array(array: &[i32]) {
    for el in array {
        print!("{} ", el);
    }
    println!();
}

fn main() {
    fizz_buzz(100);

    is_prime(2);
    is_prime(3);
    is_prime(6);
    is_prime(39);

    println!();
    is_palindrome("madam");
    is_palindrome("Altair");
    is_palindrome("level");

    println!();
    is_anagram("Listen", "Silent");
    is_anagram("Unreal", "Unity");
    is_anagram("Real", "Arel");

    println!();
    let mut array = [3, 9, 10, 7, 2, 4, 5, 6, 8, 1];
    display_array(&array);
    bubble_sort(&mut array);
    display_array(&array);

    let mut array = [3, 9, 10, 7, 2, 4, 5, 6, 8, 1];
    selection_sort(&mut array);
    display_array(&array);
}
