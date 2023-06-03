const std = @import("std");

/// Mixin for types that can be iterated over.
pub fn Iterator(comptime T: type) type {
    return struct {
        /// Returns the first element in the iterator.
        /// Note: creates a copy because the `x.childNodes().next()` would cause
        /// a discard of the const qualifier
        pub fn first(self: *const T) @typeInfo(@TypeOf(T.next)).Fn.return_type.? {
            var copy = self.*;
            return copy.next();
        }
    };
}
