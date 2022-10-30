// test {
//     const Foo = struct {
//         x: f32,
//         pub fn y() f32 {
//             @panic("todo");
//         }
//         pub fn setY(_: f32) void {}
//     };

//     inline for (comptime properties(Foo)) |p| {
//         std.debug.print("{s}: {s}\n", .{ p.name, @typeName(p.field_type) });
//     }
//     std.debug.print("........\n\n", .{});
//     return error.Foo;
// }

// fn properties(comptime T: type) []Property {
//     comptime {
//         // fields and getter/setter pairs
//         const max_len = std.meta.fields(T).len + (std.meta.declarations(T).len / 2);
//         var props: [max_len]Property = undefined;

//         var i = 0;
//         for (std.meta.fields(T)) |f| {
//             props[i] = .{
//                 .name = f.name,
//                 .field_type = f.field_type,
//             };
//             i += 1;
//         }

//         for (std.meta.declarations(T)) |d| {
//             if (d.is_pub and @hasDecl(T, setterName(d.name))) {
//                 props[i] = .{
//                     .name = d.name,
//                     .field_type = @typeInfo(@TypeOf(@field(T, d.name))).Fn.return_type orelse continue,
//                 };
//                 i += 1;
//             }
//         }

//         return props[0..i];
//     }
// }

// const Property = struct {
//     name: []const u8,
//     field_type: type,
// };

// fn setterName(comptime prop_name: []const u8) []const u8 {
//     comptime {
//         var buf: [3 + prop_name.len]u8 = undefined;
//         std.mem.copy(u8, &buf, "set");
//         buf[3] = std.ascii.toUpper(prop_name[0]);
//         std.mem.copy(u8, buf[4..], prop_name[1..]);
//         return &buf;
//     }
// }

// fn setterName(comptime prop_name: []const u8) []const u8 {
//     comptime {
//         var buf: [3 + prop_name.len]u8 = undefined;
//         std.mem.copy(u8, &buf, "set");
//         buf[3] = std.ascii.toUpper(prop_name[0]);
//         std.mem.copy(u8, buf[4..], prop_name[1..]);
//         return &buf;
//     }
// }
