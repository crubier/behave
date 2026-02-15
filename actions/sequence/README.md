# Action: Sequence

Composite action that ticks children left-to-right. Succeeds only if **all** children succeed. Fails immediately when any child fails, skipping remaining children.

Children are managed lazily -- each child is started on its first tick, not when the tree is built.
