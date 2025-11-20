fn main() {
    // put your code here to launch it
}

struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl PartialEq for Ticket {
    fn eq(&self, other: &Self) -> bool {
        let Ticket {
            title,
            description,
            status,
        } = self;
        let Ticket {
            title: other_title,
            description: other_description,
            status: other_status,
        } = other;
        [...]
    }
}