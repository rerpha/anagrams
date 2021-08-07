#[macro_use]
extern crate clap;
use clap::App;
use itertools::Itertools;

fn generate_anagrams(word: &str) -> Vec<String> {
    let mut anagrams = Vec::new();
    let vec = word.chars().collect::<Vec<char>>();
    for perm in vec.iter().permutations(vec.len()).unique() {
        if perm.len() > 0{
            let s: String = perm.into_iter().collect();
            anagrams.push(s);}
    }
    return anagrams;
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let inp_word = matches.value_of("WORD").unwrap();
    println!("your input was: {}\nAnagrams of this are:\n", inp_word);

    let anagrams = generate_anagrams(inp_word);
    for i in anagrams.iter() {
        print!("{} ", &i);
    }
}


#[cfg(test)]
mod test {
    use super::*;

    fn vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
        let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
        matching == a.len() && matching == b.len()
    }
    
    #[test]
    fn test_given_multiple_letter_word_when_generating_anagrams_then_all_possible_anagrams_are_contained() {
        let word = "dir";
        let expected = vec!("dir".to_string(), "dri".to_string(), "idr".to_string(), "ird".to_string(), "rdi".to_string(), "rid".to_string());
        let actual = generate_anagrams(word);
        println!("{:?}", actual);
        assert!(vecs_match(&expected, &actual));
    }

    #[test]
    fn test_given_empty_string_when_generating_anagrams_then_generated_list_is_empty() {    
        let actual = generate_anagrams("");
        assert_eq!(actual.len(), 0)
    }

    #[test]
    fn test_given_string_with_two_chars_when_generating_anagrams_then_generated_list_contains_two_elements() {
        let word = "it";
        let expected = vec!("it".to_string(), "ti".to_string());
        let actual = generate_anagrams(word);
        assert!(vecs_match(&expected, &actual));
    }

    #[test]
    fn test_given_string_with_identical_chars_when_generating_anagrams_then_generated_list_contains_only_original_word(){
        let word = "aaa";
        let expected = vec!(word.to_string());
        let actual = generate_anagrams(word);
        assert!(vecs_match(&expected, &actual));
    }
}