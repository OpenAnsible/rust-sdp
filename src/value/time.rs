


#[derive(Debug)]
pub struct Timing {
    // t=<start-time> <stop-time>
    start: usize,
    stop : usize
}
/*
https://tools.ietf.org/html/rfc4566#section-5.10

To make description more compact, times may also be given in units of
days, hours, or minutes.  The syntax for these is a number
immediately followed by a single case-sensitive character.
Fractional units are not allowed -- a smaller unit should be used
instead.  The following unit specification characters are allowed:

    d - days (86400 seconds)
    h - hours (3600 seconds)
    m - minutes (60 seconds)
    s - seconds (allowed for completeness)

Thus, the above session announcement could also have been written:

    r=7d 1h 0 25h


*/
#[derive(Debug)]
pub struct RepeatTimes {
    // r=<repeat interval> <active duration> <offsets from start-time>
    repeat_interval: usize,
    active_duration: usize,
    offsets_from_start_time: usize
}
// t=3034423619 3042462419
// r=604800 3600 0 90000


#[derive(Debug)]
pub struct TimeZone {
    // z=<adjustment time> <offset> <adjustment time> <offset> ....
    // z=2882844526 -1h 2898848070 0
    field: Type
}
