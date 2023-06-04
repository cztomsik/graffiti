const std = @import("std");

/// Mixin for types that can be iterated over.
pub fn Iterator(comptime T: type) type {
    const Item = @typeInfo(@TypeOf(T.next)).Fn.return_type.?;

    return struct {
        /// Returns the first element in the iterator.
        /// Note: this creates a copy because the `el.childNodes().next()` would
        /// cause a discard of the const qualifier
        pub fn first(self: *const T) Item {
            var copy = self.*;
            return copy.next();
        }
    };
}
