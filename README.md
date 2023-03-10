# KAANTOR
A crate!

## Traversal Algorithms
Different traversal algorithms for distributed systems.

### Flooding
A traversal using the *flooding* algorith. You can read more at [source](./kantor/examples/flooding.rs).
The graph is made of the nodes: 1, 2, 3, 4, and 5, and the edges 1-2, 1-3, 2-4, 4-5, and 3 -5

```bsh
RUST_LOG=debug cargo run --example flooding

INFO  RECV | on 1 from 1 | api->1 | 50 | Start(999)
INFO  SEND | from 1 to all | 1->all | 50 | Forward(999)
INFO  RECV | on 2 from 1 | 1->all | 50 | Forward(999)
INFO  SEND | from 2 to all-[1] | 1->all | 50 | Forward(999)
INFO  RECV | on 3 from 1 | 1->all | 50 | Forward(999)
INFO  SEND | from 3 to all-[1] | 1->all | 50 | Forward(999)
INFO  RECV | on 4 from 2 | 1->all | 50 | Forward(999)
INFO  SEND | from 4 to all-[2] | 1->all | 50 | Forward(999)
INFO  RECV | on 5 from 3 | 1->all | 50 | Forward(999)
INFO  SEND | from 5 to all-[3] | 1->all | 50 | Forward(999)
INFO  RECV | on 5 from 4 | 1->all | 50 | Forward(999)
INFO  RECV | on 4 from 5 | 1->all | 50 | Forward(999)
```

### Spanning Tree Propagation Feedback
A traversal using *propagation and feedback* algorithm. You can read more at [source](./kantor/examples/span_tree_propagation_feedback.rs). The graph is made of the nodes: 1, 2, 3, 4, and 5, and the edges 1-2, 1-3, 2-4, 4-5, and 3 -5.
The result should be:

```bsh
RUST_LOG=debug cargo run --example span_tree_propagation_feedback

RECV | on 1 from 1 | api->1 | 50 | Start
SEND | from 1 to all-[] | 1->all | 50 | GO
RECV | on 2 from 1 | 1->all | 50 | Go
SEND | from 2 to all-[1] | 2->all | 50 | GO
RECV | on 3 from 1 | 1->all | 50 | Go
SEND | from 3 to all-[1] | 3->all | 50 | GO
RECV | on 4 from 2 | 2->all | 50 | GO
SEND | from 4 to all-[2] | 4->all | 50 | GO
RECV | on 5 from 3 | 3->all | 50 | GO
SEND | from 5 to all-[3] | 5->all | 50 | GO
RECV | on 5 from 4 | 4->all | 50 | GO
SEND | from 5 to node 4 | 5->4 | 50 | BACK NOT A CHILD
RECV | on 4 from 5 | 5->all | 50 | GO
SEND | from 4 to node 5 | 4->5 | 50 | BACK NOT A CHILD
RECV | on 4 from 5 | 5->4 | 50 | BACK NOT A CHILD
SPANNING TREE NODE: 4 p=Parent(2) cs=[]
SEND | from 4 to node 2 | 4->2 | 50 | BACK CHILD
RECV | on 5 from 4 | 4->5 | 50 | BACK NOT A CHILD
SPANNING TREE NODE: 5 p=Parent(3) cs=[]
SEND | from 5 to node 3 | 5->3 | 50 | BACK CHILD
RECV | on 2 from 4 | 4->2 | 50 | BACK CHILD
SPANNING TREE NODE: 2 p=Parent(1) cs=[4]
SEND | from 2 to node 1 | 2->1 | 50 | BACK CHILD
RECV | on 3 from 5 | 5->3 | 50 | BACK CHILD
SPANNING TREE NODE: 3 p=Parent(1) cs=[5]
SEND | from 3 to node 1 | 3->1 | 50 | BACK CHILD
RECV | on 1 from 2 | 2->1 | 50 | BACK CHILD
RECV | on 1 from 3 | 3->1 | 50 | BACk CHILD
SPANNING TREE NODE: 1 p=Root cs=[2, 3]

1
?????? 2
???  ?????? 4
?????? 3
   ?????? 5
```

### Spanning Tree Breadth-First
A traversal using *breadth-first* algorithm. You can read more at [source](./kantor/examples/span_tree_breadth_fitst.rs). The graph is made of the nodes: 1, 2, 3, 4, and 5, and the edges 1-2, 1-3, 2-4, 4-5, and 3 -5.
The result should be:

```bsh
RUST_LOG=debug cargo run --example span_tree_breadth_first
```

### Spanning Tree Depth-First
A traversal using *breadth-first* algorithm. You can read more at [source](./kantor/examples/depth_fitst.rs). The graph is made of the nodes: 1, 2, 3, 4, and 5, and the edges 1-2, 1-3, 2-4, 4-5, and 3 -5.
The result should be:

```bsh
RUST_LOG=debug cargo run --example depth_first
```
