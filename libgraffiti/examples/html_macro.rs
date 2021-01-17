use graffiti::Document;

fn main() {
    let mut doc = Document::new(|e| println!("{:?}", e));

    // super-simple prefix macro
    macro_rules! html {
        ($text:literal) => (doc.create_text_node($text));
        ([ $tag:ident $(. $cls:ident)*: $($inner:tt)* ]) => ({
            let el = doc.create_element(stringify!($tag));
            doc.set_attribute(el, "class", stringify!($($cls)*));
            
            for (i, &child) in [ $(html!($inner)),* ].iter().enumerate() {
                doc.insert_child(el, child, i)
            }

            el
        });
    }

    let div = html! (
        [div.bar:
            [h1: "Hello macro!"]
            [p:
                "xxx"
                "yyy"
                [strong: "zzz"]
            ]
        ]
    );

    println!("{:#?}", div);
}
