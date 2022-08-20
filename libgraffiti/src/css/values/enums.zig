pub const Align = enum {
    @"auto",
    @"flex-start",
    @"center",
    @"flex-end",
    @"stretch",
    @"baseline",
    @"space-between",
    @"space-around",
};

pub const BorderStyle = enum {
    @"none",
    @"hidden",
    @"dotted",
    @"dashed",
    @"solid",
    @"double",
    @"groove",
    @"ridge",
    @"inset",
    @"outset",
};

pub const Display = enum {
    @"none",
    @"block",
    @"inline",
    @"inline-block",
    @"flex",
    @"table",
    @"table-header-group",
    @"table-row-group",
    @"table-row",
    @"table-cell",
};

pub const FlexDirection = enum {
    @"column",
    @"column-reverse",
    @"row",
    @"row-reverse",
};

pub const FlexWrap = enum {
    @"nowrap",
    @"wrap",
    @"wrap-reverse",
};

pub const Justify = enum {
    @"flex-start",
    @"center",
    @"flex-end",
    @"space-between",
    @"space-around",
    @"space-evenly",
};

pub const Overflow = enum {
    @"visible",
    @"hidden",
    @"scroll",
    @"auto",
};

pub const Position = enum {
    @"static",
    @"relative",
    @"absolute",
    @"sticky",
};

pub const TextAlign = enum {
    @"left",
    @"right",
    @"center",
    @"justify",
};

pub const Visibility = enum {
    @"visible",
    @"hidden",
    @"collapse",
};
