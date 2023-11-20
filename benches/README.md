# Benchmarks

These are 100% stolen from Bevy's benchmarks, so I can compare SECS' speed to Bevy-ECS' speed. Currently,
only the components benchmarks have been converted, and only those will run. You can run these benchmarks
by running `cargo bench` in this directory - and do the same with Bevy to compare.

Note that many benchmarks have been deleted, because they test other aspects of Bevy besides ECS, and more
will be deleted as I go through and convert these to run on SECS.

For all converted benchmarks, I've commented out the Bevy code, and in the line under that put the SECS code.
This shows how similar the APIs are, and where certain tasks are easier or more difficult.