use core::cell::Cell;
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{Datelike, Local, TimeZone, Timelike};

thread_local! {
    static TS: Cell<(i64, [u8; 21])> = const { Cell::new((i64::MIN, [0; 21])) };
}

/// Writes a number into a byte slice in decimal format, padding with leading zeros if necessary.
fn put(dst: &mut [u8], mut n: u32) {
    for d in dst.iter_mut().rev() {
        *d = b'0' + (n % 10) as u8;
        n /= 10;
    }
}

/// Returns a timestamp in the format "[YYYY/MM/DD HH:MM:SS]".
pub(super) fn timestamp() -> [u8; 21] {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs() as i64);

    TS.with(|cell| {
        let (cached, bytes) = cell.get();
        if cached == secs {
            return bytes;
        }

        let t = Local
            .timestamp_opt(secs, 0)
            .single()
            .unwrap_or_else(Local::now);

        let mut b = *b"[0000/00/00 00:00:00]";
        put(&mut b[1..5], t.year() as u32);
        put(&mut b[6..8], t.month());
        put(&mut b[9..11], t.day());
        put(&mut b[12..14], t.hour());
        put(&mut b[15..17], t.minute());
        put(&mut b[18..20], t.second());

        cell.set((secs, b));
        b
    })
}
