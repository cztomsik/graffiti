use super::LayoutContext;

impl<K: Copy> LayoutContext<'_, K> {
    //   pub fn compute_table(&self, table: &mut LayoutBox, parent_size: Size<f32>) {
    //         // TODO: flat_map TableRowGroup(s)
    //         let rows = table.children.iter_mut().filter(|ch| ch.style.display == Display::TableRow);

    //         let mut x;
    //         let mut y = 0.;

    //         for row in rows {
    //             x = 0.;

    //             self.init_box(row, parent_size);

    //             for cell in &mut row.children.iter_mut().filter(|ch| ch.style.display == Display::TableCell) {
    //                 self.compute_box(cell, Size::new(100., f32::NAN));
    //                 cell.x = x;
    //                 cell.y = y;

    //                 x += cell.size.width;

    //                 if !cell.size.height.is_nan() {
    //                     row.size.height = row.size.height.max(cell.size.height);
    //                 }
    //             }

    //             y += row.size.height;
    //             table.size.height += row.size.height;
    //         }
    //     }
}
