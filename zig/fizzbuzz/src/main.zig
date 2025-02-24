const std = @import("std");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer _ = gpa.deinit();

    const argv = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, argv);

    if (argv.len < 2) {
        return try std.io.getStdOut().writeAll("Usage: fizzbuzz <number>");
    }

    const n = try std.fmt.parseInt(i32, std.mem.sliceTo(argv[1], 0), 0);

    if (@rem(n, 3) == 0 and @rem(n, 5) == 0) {
        return try std.io.getStdOut().writeAll("FizzBuzz\n");
    } else if (@rem(n, 3) == 0) {
        return try std.io.getStdOut().writeAll("Fizz\n");
    } else if (@rem(n, 5) == 0) {
        return try std.io.getStdOut().writeAll("Buzz\n");
    }
}
