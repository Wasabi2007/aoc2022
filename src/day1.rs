use std::str;
use std::str::Lines;
use std::vec;

fn split_by<F>(lines:Lines, f:F) -> Vec<Vec<&str>>
    where F: Fn(&str) -> bool
{
    let mut current_group : Vec<&str> = Vec::new();
    let mut result : Vec<Vec<&str>> = vec![];
    for line in lines{
        if f(line)
        {
            result.push(current_group);
            current_group = Vec::new();            
            continue;
        }
        current_group.push(line);
    }
    result.push(current_group);

    return result;
}
//
pub fn parse_input(input: &str){
    let backpacks = split_by(input.lines(),|l| l.is_empty());
    let mut backpack_sumed : Vec<i32> = backpacks
                        .iter()
                        .map(|b| b.iter().map(|i| i.parse::<i32>().unwrap()))
                        .map(|b| b.sum::<i32>())
                        .collect();

    backpack_sumed.sort();

    for elf_bagpack in backpack_sumed.iter()
    {
        println!("Bag Cal:{}", elf_bagpack);
    }
    
    let maxcal = backpack_sumed.iter().rev().take(3).sum::<i32>();
    println!("Bigest 3 Bag Cal:{}", maxcal);
}