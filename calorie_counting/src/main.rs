const INPUTFILE: &str = include_str!("elf_calories");
fn main() {
    let mut highest_Total = 0;
    let mut current_Total = 0;
    let mut sec_highest_Total = 0;
    let mut sec_current_Total = 0;
    let mut third_highest_Total = 0;
    let mut third_current_Total = 0;
    let mut sum_of_totals = 0;

    //find the three highest totals
    INPUTFILE.lines().for_each(|current_line| {
        if current_line.is_empty() {
            if current_Total > highest_Total {
                sec_highest_Total = highest_Total;
                highest_Total = current_Total;
                println!("New Highest Total:{} ", highest_Total);
            } else if current_Total > sec_highest_Total {
                sec_highest_Total = current_Total;
                println!("New Second Highest Total:{} ", sec_highest_Total);
            } else if current_Total > third_highest_Total {
                third_highest_Total = current_Total;
                println!("New Third Highest Total:{} ", third_highest_Total);
            }
            current_Total = 0;
        } else {
            current_Total += current_line.parse::<i32>().unwrap();
        }
        //add the three highest totals together
        sum_of_totals = highest_Total + sec_highest_Total + third_highest_Total;
        println!("The second highest...:{} ", sec_highest_Total);
        println!("The third highest...:{} ", third_highest_Total);
        println!("The winner is!:{} ", highest_Total);
        println!("The sum of the three highest totals is:{}", sum_of_totals);
    });
}



//     INPUTFILE.lines().for_each(|current_line| {
//         if current_line.is_empty() {
//             println!("Total:{} ", current_Total);
//             if current_Total > highest_Total {
//                 highest_Total = current_Total;
//
//             }
//             current_Total = 0;
//         } else {
//             current_Total += current_line.parse::<i32>().unwrap();
//         }
//     });
//     if current_Total > highest_Total {
//         highest_Total = current_Total;
//     }
//     println!("The winner is!:{} ", highest_Total);
// }
