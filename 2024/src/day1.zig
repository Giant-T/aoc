const std = @import("std");
const Allocator = std.mem.Allocator;
const GeneralPurposeAllocator = std.heap.GeneralPurposeAllocator;

const MinHeap = struct {
    buff: []?u32,
    size: usize = 0,
    allocator: Allocator,

    fn init(allocator: Allocator, size: usize) !MinHeap {
        return .{
            .buff = try allocator.alloc(?u32, size),
            .allocator = allocator,
        };
    }

    fn deinit(self: *MinHeap) void {
        self.allocator.free(self.buff);
    }

    fn push(self: *MinHeap, val: u32) void {
        if (self.size >= self.buff.len) return;

        var idx = self.size;
        self.size += 1;

        while (idx >= 0) {
            if (idx == 0) {
                self.buff[0] = val;
                return;
            }

            const parent_idx = (idx - 1) / 2;

            if (val > self.buff[parent_idx].?) {
                self.buff[idx] = val;
                return;
            }

            self.buff[idx] = self.buff[parent_idx];

            idx = parent_idx;
        }
    }

    fn pop(self: *MinHeap) ?u32 {
        if (self.size == 0) return null;

        const res = self.buff[0];
        self.size -= 1;
        self.buff[0] = self.buff[self.size];
        self.buff[self.size] = null;

        if (self.size == 0) return res;

        var idx: usize = 0;
        while (idx < (self.buff.len - 2) / 2 and self.buff[idx] != null) {
            const curr_value = self.buff[idx].?;
            const l_child_idx = idx * 2 + 1;
            const l_child = self.buff[l_child_idx];
            const r_child_idx = idx * 2 + 2;
            const r_child = self.buff[r_child_idx];

            if (l_child == null) break;

            if (r_child == null) {
                if (l_child.? < curr_value) {
                    self.buff[idx] = l_child;
                    self.buff[l_child_idx] = curr_value;
                    idx = l_child_idx;
                }
                break;
            }

            if (l_child.? < r_child.? and l_child.? < curr_value) {
                self.buff[idx] = l_child;
                self.buff[l_child_idx] = curr_value;
                idx = l_child_idx;
            } else if (r_child.? < curr_value) {
                self.buff[idx] = r_child;
                self.buff[r_child_idx] = curr_value;
                idx = r_child_idx;
            } else {
                break;
            }
        }

        return res;
    }

    fn validate(self: *MinHeap) bool {
        if (self.size == 0) return true;
        var idx = self.size - 1;
        while (idx > 0) : (idx -= 1) {
            const parent_idx = (idx - 1) / 2;
            if (self.buff[idx].? < self.buff[parent_idx].?) {
                return false;
            }
        }

        return true;
    }
};

const input = @embedFile("input/day1.txt");

pub fn main() !void {
    var gpa = GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const timestamp = std.time.nanoTimestamp();
    const res = try part_two(allocator, input);
    std.debug.print("time: {}, value: {}\n", .{ std.time.nanoTimestamp() - timestamp, res });
}

fn part_one(allocator: Allocator, buff: []const u8, lines: usize) !u32 {
    var l_heap = try MinHeap.init(allocator, lines);
    defer l_heap.deinit();
    var r_heap = try MinHeap.init(allocator, lines);
    defer r_heap.deinit();

    var line_iter = std.mem.split(u8, buff, "\n");

    while (line_iter.next()) |line| {
        if (line.len == 0) break;
        var nums = std.mem.split(u8, line, "   ");
        l_heap.push(try std.fmt.parseInt(u32, nums.next().?, 10));
        r_heap.push(try std.fmt.parseInt(u32, nums.next().?, 10));
    }

    var sum: u32 = 0;
    while (l_heap.pop()) |l_num| {
        const r_num = r_heap.pop().?;

        const diff = if (l_num > r_num) l_num - r_num else r_num - l_num;
        sum += diff;
    }

    return sum;
}

const CountMap = std.AutoHashMap(u32, u32);

fn part_two(allocator: Allocator, buff: []const u8) !u32 {
    var l_map = CountMap.init(allocator);
    defer l_map.deinit();
    var r_map = CountMap.init(allocator);
    defer r_map.deinit();

    var line_iter = std.mem.split(u8, buff, "\n");
    while (line_iter.next()) |line| {
        if (line.len == 0) break;
        var nums = std.mem.split(u8, line, "   ");
        const l_num = try std.fmt.parseInt(u32, nums.next().?, 10);

        if (l_map.getPtr(l_num)) |count| {
            count.* += 1;
        } else {
            try l_map.put(l_num, 1);
        }

        const r_num = try std.fmt.parseInt(u32, nums.next().?, 10);

        if (r_map.getPtr(r_num)) |count| {
            count.* += 1;
        } else {
            try r_map.put(r_num, 1);
        }
    }

    var map_iter = l_map.iterator();
    var res: u32 = 0;
    while (map_iter.next()) |entry| {
        const count = r_map.get(entry.key_ptr.*) orelse 0;
        res += entry.key_ptr.* * entry.value_ptr.* * count;
    }

    return res;
}

test "pop" {
    var gpa = GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var l_heap = try MinHeap.init(allocator, 1000);
    defer l_heap.deinit();

    var line_iter = std.mem.split(u8, input, "\n");

    while (line_iter.next()) |line| {
        if (line.len == 0) break;
        var nums = std.mem.split(u8, line, "   ");
        l_heap.push(try std.fmt.parseInt(u32, nums.next().?, 10));
    }

    while (l_heap.pop()) |_| {
        try std.testing.expect(l_heap.validate());
    }
}
