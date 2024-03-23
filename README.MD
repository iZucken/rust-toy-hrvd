Some time ago I had to implement a generic sort-of human-readable data diffing for audit logs in a dynamic language with reflection capabilities.
There it was pretty much trivial for most cases, even if suboptimal regarding performance.
Features it supported:
- basic diffing on scalars;
- structure fields;
- nested structures;
- partially diffing lists of scalars (iirc didn't have to dive deeper in that regard);
- no extra demarcation or boilerplate for common case;
- attributes to hide specific fields from comparison (e.g. uninteresting ones);
- mark fields as secret (only display the fact of change);
- can't remember...

In this toy project I want to explore approaches to such functionality in rust.
I want to see how much more boilerplate I would have, considering that I would be pretty much bound to marking all structs with derives.
