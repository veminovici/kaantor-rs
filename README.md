# KANTOR
A crate!

## Traversal Algorithms
Different traversal algorithms for distributed systems.

### Flooding
A traversal using the *flooding* algorith. You can read more at [source](./kantor/examples/flooding.rs).

```bsh
RUST_LOG=debug cargo run --example flooding

INFO  RECV | on 1 from 1 | api->1 | 50 | Start(999)
INFO  SEND | from 1 to all | 1->all | 50 | Forward(999)
DEBUG do_send [1-->2]
DEBUG do_send [1-->3]
INFO  RECV | on 2 from 1 | 1->all | 50 | Forward(999)
INFO  SEND | from 2 to all-[1] | 1->all | 50 | Forward(999)
DEBUG do_send [2-->4]
INFO  RECV | on 3 from 1 | 1->all | 50 | Forward(999)
INFO  SEND | from 3 to all-[1] | 1->all | 50 | Forward(999)
DEBUG do_send [3-->5]
INFO  RECV | on 4 from 2 | 1->all | 50 | Forward(999)
INFO  SEND | from 4 to all-[2] | 1->all | 50 | Forward(999)
DEBUG do_send [4-->5]
INFO  RECV | on 5 from 3 | 1->all | 50 | Forward(999)
INFO  SEND | from 5 to all-[3] | 1->all | 50 | Forward(999)
DEBUG do_send [5-->4]
INFO  RECV | on 5 from 4 | 1->all | 50 | Forward(999)
INFO  RECV | on 4 from 5 | 1->all | 50 | Forward(999)
```