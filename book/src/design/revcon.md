# Revision Control

Revision control is the "flagship app" built atop the pangalactic infrastructure, primarily accessible throught the `pg` end-user tool.

In addition to the top-level pangalactic [architecture](./arch.md), this chapter assumes you are familiar with decentralized revision control tools, such as git, hg, monotone, darcs, etc...

## Concepts

### Publications and Subscriptions

A *publication* refers to a self-authenticated revision control history. 

### Workspaces

When a user initializes a new project with `pg init` they convert a host directory into a newly created *workspace* with these components:

- `<workdir>` - the *working directory* is the directory which was passed in the `--workdir` option to `pg init` (which defaults to the current directory) and we will refer to it as *workdir* for short.
- `<workdir>/.pg` - the *book-keeping directory* contains revision control metadata. This is the *only* subdirectory within the workdir which `pg` treats specially. (**TODO:** Review this claim as the design evolves.)
- `<workdir>/.pg/prev.pglink` - the *previous transaction link* is present for every 

