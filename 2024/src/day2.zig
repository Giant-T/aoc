const std = @import("std");

fn part_one(buff: []const u8) !u32 {
    var line_iter = std.mem.split(u8, buff, "\n");

    var nb_of_safe: u32 = 0;
    outer: while (line_iter.next()) |report| {
        if (report.len == 0) break;
        var num_iter = std.mem.split(u8, report, " ");
        var prev_str = num_iter.next() orelse break;
        var report_is_increasing: ?bool = null;

        while (num_iter.next()) |str| {
            if (str.len == 0) break;
            const level = try std.fmt.parseInt(i32, str, 10);
            const prev_level = try std.fmt.parseInt(i32, prev_str, 10);

            const diff = level - prev_level;
            if (diff == 0) {
                continue :outer;
            } else if (report_is_increasing == null) {
                report_is_increasing = diff > 0;
            } else if ((diff > 0) != report_is_increasing.?) {
                continue :outer;
            }

            const is_safe = switch (diff) {
                -3...3 => true,
                else => false,
            };

            if (!is_safe) {
                continue :outer;
            }

            prev_str = str;
        }

        std.debug.print("{s}\n", .{report});

        nb_of_safe += 1;
    }

    return nb_of_safe;
}

const input = @embedFile("input/day2.txt");

pub fn main() !void {
    const timestamp = std.time.nanoTimestamp();
    const res = try part_one(input);

    std.debug.print("time: {}, value: {}\n", .{ std.time.nanoTimestamp() - timestamp, res });
}
