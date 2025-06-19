# Architecture

The pangalactic system is composed of several "infrastructure" layers for data storage & distribution, synchronization among collaborators, determistic computation. Atop this infrastructure is the flagship `pg` revision control app, although nothing prevents other applications from building on this infrastructure.

## The Store

The *Store* is a universal immutable data storage and distribution layer, spread across a network. The key primitive is hash-based indexing, along with "directory" nodes for linking together data within the Store.

### The Local Store

The *Local Store* is a user-scoped filesystem storage backend containing a subset of the complete universal *Store*.

```admonish info "git comparison"
Because the Local Store is user-scoped, *all* of a user's revision control storage is shared across all projects on the system. For example, if a user clones the same project into multiple different working directories, the revision control storage is deduplicated / cached for all working directories.

There is no equivalent of `.git` blob storage. This means if you need to ensure the complete history of a project is copied to a new host, you must either copy the entire Local Store (which may include other projects), or use the `pg` tool to perform the transfer.
```

### Distribution

The distribution of immutable data, which ensures that the data a user needs is accessible and allows them to retrieve it, is an unspecified feature area. The current code only supports a *Local Store*, and this means currently *projects cannot be shared across hosts*.

```admonish danger "Incomplete Prototype Hazard"
This is an example of how `pangalactic` is in a prototype maturity phase and it is not wise to rely on it solely for revision control or storage needs yet.
```

```TODO unfinished design:
Relationship and terminology between store, distribution, synchronization, and pub/sub crypto:

- Store: universal storage network
- Distribution: how users get the right data (and how to ensure it's available).
- Synchronization: how users share and learn about updates of projects they care about.
- pub/sub: how publisher ids and subscriptions are "storage/network agnostic".

Also, it's not clear yet if pub/sub is a separate layer above Store, or if Store _must_ be pub/sub aware.
```
