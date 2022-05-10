use grep::grep;

fn main() {
    let sentence = "This is a test sentence.\nThis is also a test sentence.\n";
    let result = grep::find("test", &sentence);
    for r in &result {
        println!("{}", r);  
    }
}
