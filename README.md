# KANTOR
A crate!

## Traversal Algorithms
Different traversal algorithms for distributed systems.

### Flooding
A traversal using the *flooding* algorith. You can read more at [source](./kantor/examples/flooding.rs).

```bsh
RUST_LOG=debug cargo run --example flooding

INFO  kantor::node::actor RECV'ng on 1 from 1 | api->1 | 50 | Start(999)
INFO  kantor::node::actor SEND'ng from 1 to all | 1->all | 50 | Forward(999)
DEBUG kantor::proxy do_send'ng [1-->2]
DEBUG kantor::proxy do_send'ng [1-->3]
INFO  kantor::node::actor RECV'ng on 2 from 1 | 1->all | 50 | Forward(999)
INFO  kantor::node::actor SEND'ng from 2 to all-- | 1->all | 50 | Forward(999)
DEBUG kantor::proxy do_send'ng [2-->4]
INFO  kantor::node::actor RECV'ng on 3 from 1 | 1->all | 50 | Forward(999)
kantor::node::actor SEND'ng from 3 to all-- | 1->all | 50 | Forward(999)
DEBUG kantor::proxy do_send'ng [3-->5]
INFO  kantor::node::actor RECV'ng on 4 from 2 | 1->all | 50 | Forward(999)
INFO  kantor::node::actor SEND'ng from 4 to all-- | 1->all | 50 | Forward(999)
DEBUG kantor::proxy do_send'ng [4-->5]
INFO  kantor::node::actor RECV'ng on 5 from 3 | 1->all | 50 | Forward(999)
INFO  kantor::node::actor SEND'ng from 5 to all-- | 1->all | 50 | Forward(999)
DEBUG kantor::proxy do_send'ng [5-->4]
INFO  kantor::node::actor RECV'ng on 5 from 4 | 1->all | 50 | Forward(999)
DEBUG flooding] Received a message for a recorded sessions 50
INFO  kantor::node::actor RECV'ng on 4 from 5 | 1->all | 50 | Forward(999)
DEBUG flooding Received a message for a recorded sessions 50
```