const std = @import("std");
const stdout = std.io.getStdOut();
const stdoutw = stdout.writer();
const stdin = std.io.getStdIn();
const stdinr = stdin.reader();

var gpa_allocator = std.heap.GeneralPurposeAllocator(.{}){};
var allocator = gpa_allocator.allocator();

const VMIN = 16;
const VTIME = 17;

var stored_settings: std.posix.termios = undefined;

inline fn clear() !void {
    try stdoutw.print("\x1b[2J", .{});
}

inline fn go_to_top() !void {
    try stdoutw.print("\x1b[1;1f", .{});
}

inline fn set_keypress() !void {
    stored_settings = try std.posix.tcgetattr(0);
    var new_settings = stored_settings;

    new_settings.lflag.ICANON = false;
    new_settings.cc[VTIME] = 0;
    new_settings.cc[VMIN] = 1;

    try std.posix.tcsetattr(0, .NOW, new_settings);
}

inline fn reset_keypress() !void {
    try std.posix.tcsetattr(0, .NOW, stored_settings);
}

pub fn main() !void {
    var arena_allocator = std.heap.ArenaAllocator.init(allocator);
    defer arena_allocator.deinit();
    const arena = arena_allocator.allocator();
    const args = try std.process.argsAlloc(arena);
    var files = std.ArrayList([]const u8).init(allocator);
    defer files.deinit();

    try stdoutw.print("Length is:{}\n", .{args.len});

    {
        var i: usize = 1;
        while (i < args.len) : (i += 1) {
            try files.append(args[i]);
        }
    }

    var current_file: u32 = undefined;
    if (files.items.len < 1) {
        current_file = 0;
    } else {
        current_file = 1;
    }
    try stdoutw.print("curfile is:{}\n", .{current_file});
    try stdoutw.print("cwd is:{s}\n", .{try std.fs.cwd().realpathAlloc(allocator, ".")});

    var buf: [1000]u8 = undefined;
    while (true) {
        try stdoutw.print(":", .{});
        const line = (try stdinr.readUntilDelimiterOrEof(&buf, '\n')).?;
        var words = std.mem.split(u8, line, " ");
        const first_word = words.next().?;
        const State = enum (i32) {
            none,
            switch_,
            quit,
        };
        var state = State.none;
        if (std.mem.eql(u8, first_word, "switch")) {
            state = State.switch_;
        } else if (std.mem.eql(u8, first_word, "quit")) {
            state = State.quit;
        } else {
            try stdoutw.print("Unknown command: {s}\n", .{first_word});
        }
        var arguments = std.ArrayList([]const u8).init(allocator);
        while (words.next()) |word| {
            if (word.len < 1) {
                continue;
            }
            try arguments.append(word);
        }
        switch (state) {
            State.none => {},
            State.switch_ => {
                for (arguments.items) |argument| {
                    try stdoutw.print("Arg is:{s}\n", .{argument});
                }
            },
            State.quit => {
                break;
            },
        }
    }
}
