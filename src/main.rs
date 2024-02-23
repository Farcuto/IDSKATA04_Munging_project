

use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn find_day_with_min_temp_spread(filename: &str) -> io::Result<usize>
{
    let file = File::open(filename)?;
    let mut min_spread_day = 0;
    let mut min_spread_temp = usize::MAX;

    for lines_file in BufReader::new(file).lines() 
    {
        let values: Vec<String> = lines_file?.split_whitespace().map(|s| s.to_string()).collect();

        if values.len() >= 3 
        {
            if let (Ok(day), Ok(max_temp), Ok(min_temp)) =
            (
                values[0].parse::<usize>(),
                values[1].parse::<usize>(),
                values[2].parse::<usize>(),
            ) {
    
               let temp_spread = max_temp - min_temp;
    
                //println!("This day is {0} and the spread is {1} ",day,temp_spread);
    
                if temp_spread < min_spread_temp
                {
                    min_spread_temp = temp_spread;
                    min_spread_day = day
                }
    
            }
        }

    }

    Ok(min_spread_day)
}

fn find_soccer_team_with_minimun_spread(filename: &str) -> io::Result<String>
{
    let file = File::open(filename)?;
    //let mut min_team_number = 10;
    let mut min_spread_team = usize::MAX;
    let mut team_name: String = "-".to_string();


    for lines_file in BufReader::new(file).lines().skip(1) 
    {
        let values: Vec<String> = lines_file?.split_whitespace().map(|s| s.to_string()).collect();
        
        if values.len() == 1{
            continue;
        }
        //println!("{}, {}", values[0],values[8]);
        if values.len() >= 1 
        {
            if let ( Ok(four_team), Ok(against_team)) =
            (
                
                values[6].parse::<usize>(),
                values[8].parse::<usize>(),
            ) {
    
                
            
                if four_team > against_team{
                    let team_spread = four_team - against_team;

                    if team_spread < min_spread_team
                    {
                    min_spread_team = team_spread;
                    //min_team_number = team_number;
                    team_name = values[2].to_string();

                    }
                }
                else {
                    let team_spread = against_team -four_team;

                    if team_spread < min_spread_team
                    {
                    min_spread_team = team_spread;
                    //min_team_number = team_number;
                    team_name = values[2].to_string();

                    }
                }
               
    
                //println!("This day is {0} and the spread is {1} ",four_team,against_team);
                
                
    
            }
        }

    }
    

    Ok(team_name)
}

fn main() {
    match find_soccer_team_with_minimun_spread("./data/football.dat") {
        Ok(team_name) => println!("The team with the smallest difference in ‘for’ and ‘against’ goals is {}",team_name),
        Err(e) => println!("Error reading football data {}", e)
        
    }
    match find_day_with_min_temp_spread("./data/weather.dat") {
        Ok(day) => println!("Day with smallest temperature spread is {}",day),
        Err(e) => println!("Error reading weather data {}", e)
        
    }
}


#[cfg(test)]
mod test_wheather 
{
    


    use crate::find_day_with_min_temp_spread;
    

    #[test]
    fn find_day_with_min_temp_spread_test_1_invalid_input() {
        let file_test: &str = "./../test1.dat";

        assert_eq!(find_day_with_min_temp_spread(file_test).unwrap(),0);       
    }

    #[test]
    fn find_day_with_min_temp_spread_test_2_valid_input() {
        let file_test: &str = "./../test2.dat";

        assert_eq!(find_day_with_min_temp_spread(file_test).unwrap(),4);       
    }

    #[test]
    fn find_day_with_min_temp_spread_test_3_invalid_input_empty() {
        let file_test: &str = "./../test3.dat";

        assert_eq!(find_day_with_min_temp_spread(file_test).unwrap(),0);       
    }

    #[test]
    fn find_day_with_min_temp_spread_test_3_invalid_input_negative_spread() {
        let file_test: &str = "./../test4.dat";

        assert_ne!(find_day_with_min_temp_spread(file_test).unwrap(),0);       
    }
}

#[cfg(test)]
mod test_soccer 
{
    


    use crate::find_day_with_min_temp_spread;
    

    #[test]
    fn find_day_with_min_temp_spread_test_1_invalid_input() {
        let file_test: &str = "./../test1.dat";

        assert_eq!(find_day_with_min_temp_spread(file_test).unwrap(),0);       
    }

    #[test]
    fn find_day_with_min_temp_spread_test_2_valid_input() {
        let file_test: &str = "./../test2.dat";

        assert_eq!(find_day_with_min_temp_spread(file_test).unwrap(),4);       
    }

    #[test]
    fn find_day_with_min_temp_spread_test_3_invalid_input_empty() {
        let file_test: &str = "./../test3.dat";

        assert_eq!(find_day_with_min_temp_spread(file_test).unwrap(),0);       
    }

    #[test]
    fn find_day_with_min_temp_spread_test_3_invalid_input_negative_spread() {
        let file_test: &str = "./../test4.dat";

        assert_ne!(find_day_with_min_temp_spread(file_test).unwrap(),0);       
    }
}