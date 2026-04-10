## 2024-04-10 - O(N) String Allocations in TUI Filter Loops
**Learning:** In Ratatui applications, iterators that perform `.to_lowercase()` inside a `.filter()` closure reallocate strings for every single tick/render cycle, compounding latency dramatically on large lists.
**Action:** Always hoist invariant string transformations (like lowercasing the search query) outside of filter closures.
