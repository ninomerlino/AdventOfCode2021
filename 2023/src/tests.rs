use crate::days::{p3,g11, g14, p14, predict_prev_value, g15, p16};

#[test]
fn test_d2_p1(){
	let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
	let value = p3(input);
	assert_eq!(value, 8);
}

#[test]
fn test_d6_p1(){
	let input = r#"Time:      7  15   30
Distance:  9  40  200"#;
	let input = g11(input);
	let strats: Vec<u64> = input.iter().map(|r|r.get_winning_strats_count()).collect();
	dbg!(strats);
}

#[test]
fn test_d7_p2(){
	let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
	let vec = g14(input);
	println!("{:#?}",vec);
	assert_eq!(p14(&vec), 5905)
}

#[test]
fn test_d9_p2(){
	let input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
	let input = g15(input);
	assert_eq!(p16(&input),2)
}