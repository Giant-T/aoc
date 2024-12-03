const std = @import("std");

pub fn build(b: *std.Build) !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{ .preferred_optimize_mode = .ReleaseFast });

    // running the unit tests.
    const test_step = b.step("test", "Run unit tests");

    const src_dir = try std.fs.cwd().openDir("src", .{ .iterate = true });

    var walker = try src_dir.walk(allocator);

    while (try walker.next()) |entry| {
        if (entry.kind != std.fs.Dir.Entry.Kind.file) continue;

        const len_without_ext = entry.basename.len - 4;

        const ext = entry.basename[len_without_ext..];
        if (!std.mem.eql(u8, ext, ".zig")) continue;

        const basename = try allocator.alloc(u8, len_without_ext);
        @memcpy(basename, entry.basename[0..len_without_ext]);

        const path = try allocator.alloc(u8, 4 + entry.path.len);
        _ = try std.fmt.bufPrint(path, "src/{s}", .{entry.path});

        const exe = b.addExecutable(.{
            .name = basename,
            .root_source_file = b.path(path),
            .target = target,
            .optimize = optimize,
        });

        b.installArtifact(exe);

        const run_cmd = b.addRunArtifact(exe);
        run_cmd.step.dependOn(b.getInstallStep());

        const run_step = b.step(basename, "Run the app");
        run_step.dependOn(&run_cmd.step);

        const exe_unit_tests = b.addTest(.{
            .root_source_file = b.path("src/day1.zig"),
            .target = target,
            .optimize = optimize,
        });

        const run_exe_unit_tests = b.addRunArtifact(exe_unit_tests);
        test_step.dependOn(&run_exe_unit_tests.step);
    }
}
