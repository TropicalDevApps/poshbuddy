## 2024-04-12 - Avoid Allocation in ASCII Searches
**Learning:** In Rust `to_lowercase().contains(&filter_lower)` requires allocating a new String for each item in the collection during every keystroke. For TUI filters that run frequently over potentially large lists, this heap allocation can cause stuttering.
**Action:** Use byte-level sliding windows without allocation: `line.as_bytes().windows(len).any(|w| w.eq_ignore_ascii_case(needle))` for case-insensitive ASCII substring matching.
