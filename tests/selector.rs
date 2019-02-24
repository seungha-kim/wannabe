extern crate wannabe;

use wannabe::tokenizer::*;
use wannabe::parser::*;
use wannabe::selector::*;

const SIMPLE: &str = "
+hudson
+jimmy
+lance
+dan

~20190225

*dan,lance,jimmy
-hudson
+rio
";

#[test]
fn compute_table_works() {
    fn pretty(members: &Vec<&str>, table: &Vec<Vec<usize>>) {
        
        print!("\t");
        println!("{}", members.join("\t"));
        for (i, row) in table.iter().enumerate() {
            print!("{}\t", members[i]);
            for item in row {
                print!("{}\t", item);
            }
            print!("\n");
        }
    }
    let tk = Tokenizer::new(SIMPLE.to_string());
    let result = tk.tokenize();
    let ps = Parser::new(&result);
    let parse_result = ps.parse();
    let table = compute_table(&parse_result);
    pretty(&parse_result.available_members, &table);
    assert_eq!(&table, &[&[0, 1, 1, 0], &[1, 0, 1, 0], &[1, 1, 0, 0], &[0, 0, 0, 0]]);
}

#[test]
fn selector_works() {
    
}