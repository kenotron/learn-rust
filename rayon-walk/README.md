This is an ASYNC dir walker with tokio, looking for package.json. Here are some parameters:

1. the target path is HARDCODED
2. the program has just one file
3. it is using rayon

Things I've learned here:

My globber is using a for..in loop - the glob crate actually uses a set of "and_then & map" combinators. 

see https://docs.rs/glob/0.3.0/src/glob/lib.rs.html#854

`rayon` is really neat - it uses work stealing and pays attention to idle cores while firing up work. This is in STARK contrast to any node.js p-queue libraries where they just fulfill promises async without paying attention to system resources.

**I really think there should be a `p-queue` that doesn't need to specify "concurrency".**

Next steps:

1. learn to do some perf measurements
2. understand and_then & map
3. switch to glob and clean up the walk some

Next project: [rayon-walk-cli](../rayon-walk-cli/)