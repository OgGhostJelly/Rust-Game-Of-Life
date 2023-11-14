mod tree;

#[derive(Clone, Debug)]
struct Action<'a> {
    text: &'a str,
}

#[derive(Clone, Debug)]
struct Event<'a> {
    text: &'a str,
    actions: Vec<Action<'a>>,
}

fn main() {
    /*
    tree::a();
    let root: tree::PathedNode<Event> = tree::PathedNode::create(
        Event {
            text: "Welcome!",
            actions: [Action { text: "Attack" }, Action { text: "Escape" }].to_vec(),
        },
        [
            tree::PathedNode::create(
                Event {
                    text: "You attacked!",
                    actions: [Action { text: "Jump" }, Action { text: "Dance" }].to_vec(),
                },
                [
                    tree::PathedNode::new(Event {
                        text: "You jumped",
                        actions: [].to_vec(),
                    }),
                    tree::PathedNode::new(Event {
                        text: "You danced",
                        actions: [].to_vec(),
                    }),
                ]
                .to_vec(),
            ),
            tree::PathedNode::new(Event {
                text: "You escaped",
                actions: [].to_vec(),
            }),
        ]
        .to_vec(),
    );

    println!("{:#?}", root);*/
}
