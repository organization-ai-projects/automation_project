pub struct DiffWidget;

impl DiffWidget {
    pub fn render(before: &str, after: &str) {
        if before == after {
            println!("no diff");
        } else {
            println!("diff detected");
            println!("before: {before}");
            println!("after: {after}");
        }
    }
}
