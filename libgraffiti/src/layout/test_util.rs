// helper macro for layout testing
//
// let calculate = layout_tree! (node(style_prop = val, ...)
//   (node(...) ...)
//   (text "hello")
//   ...
// )
//
// let results = calculate(Size::new(400., 300.))
// assert_eq!(results[0], ...)
macro_rules! layout_tree {
    ($root:tt) => ({
        struct Tree(Vec<LayoutStyle>, Vec<Vec<usize>>);
        struct Para;

        impl crate::layout::Paragraph for Para {
            fn measure(&self, _: f32) -> (f32, f32) {
                todo!()
            }
        }

        impl crate::layout::LayoutTree for Tree {
            type NodeRef = usize;
            type Paragraph = Para;

            fn root(&self) -> usize {
                0
            }

            fn children(&self, node: usize) -> &[usize] {
                &self.1[node]
            }

            fn style(&self, node: usize) -> &LayoutStyle {
                &self.0[node]
            }

            fn paragraph(&self, node: usize) -> Option<&Para> {
                Option::None
            }
        }

        |avail_size: Size<f32>| {
            let layout_engine = LayoutEngine::new();
            let mut styles: Vec<LayoutStyle> = Vec::new();
            let mut children: Vec<Vec<usize>> = Vec::new();

            layout_tree!(styles, children, $root);
            let tree = Tree(styles, children);

            let mut results: Vec<LayoutResult> = Vec::new();
            results.resize_with(tree.0.len(), Default::default);

            layout_engine.calculate(avail_size, &tree, &mut results);
            results
        }
    });
    // TODO: text
    ($styles:expr, $children:expr, ( text $text:literal )) => ($tree.create_node());
    ($styles:expr, $children:expr, ( $tag:ident ($($($prop:ident).+ = $val:expr),*) $($inner:tt)* )) => ({
        let mut style = crate::layout::LayoutStyle::default();
        $(style.$($prop).+ = $val;)*

        let id = $styles.len();
        $styles.push(style);
        $children.push(Vec::new());

        let children = vec![ $(layout_tree!($styles, $children, $inner)),* ];
        $children[id] = children;

        id
    });
}
