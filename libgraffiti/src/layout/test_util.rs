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
        |avail_size: Size<f32>| {
            let layout_engine = LayoutEngine::new();
            let mut styles: Vec<LayoutStyle> = Vec::new();
            let mut children: Vec<Vec<usize>> = Vec::new();

            layout_tree!(styles, children, $root);

            let mut results: Vec<LayoutResult> = Vec::new();
            results.resize_with(styles.len(), Default::default);

            layout_engine.calculate(avail_size, 0, &crate::util::index_with(&children, |chs, node: usize| &chs[node][..]), &styles, &mut results);
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
