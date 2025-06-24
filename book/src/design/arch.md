# Architecture

The pangalactic system is composed of several "infrastructure" layers for data storage & distribution, synchronization among collaborators, determistic computation. Atop this infrastructure is the flagship `pg` revision control app, although nothing prevents other applications from building on this infrastructure.

**Warning:** The text here is not fully reconciled with the code, including command examples!

## Concepts

Before describing the pangalactic architecture, we introduce a few important concepts that help clarify the architectural design:

### Signing and Verifying

Pangalactic makes heavy use of cryptographic signature schemes. However, it's important to dispel two persistent confusions about these schemes:

- Signing and verification keys are not identified with people or connected to human "identity". Instead, they are "mere components" of the system. Code that relies on verifying signatures does this so the user can rely on beliefs about how the associated signing keys are controlled. Humans and/or code may control and manage any number of signing keys (via software) in pangalactic.
- Signing and verification keys are cryptographic values used to sign data and verify signatures, and nothing about them is _inherently_ public or private. A verification key may be broadcast widely, or it may be controlled in a small private extent. Likewise with signing keys.

### Validity and Attestations

Below we often describe various kinds of data and their relation to each other. It is often the case that relationship between data items "should be" something described here. What happens when it is not?

Since pangalactic is designed to encompass a universal network of collaborators sharing a universal data set, and any kind of participants may collaborate, we want to protect a user from the malicious usage of other collaborators in the network.

Pangalactic software uses three methods to protect users:

- *zero-knowledge proofs of computational integrity* aka ZKPs: when possible, metadata representing a relationship between two data items proves that the relationship correctly holds by include a third item which is a ZKP for the relationship predicate.

- *signed attestations*: where ZKPs are not-yet-implemented or impossible (because a relation is not mathematically provable), we rely on signatures to *attest* to the validity of a relationship. An example of where this when a publisher publishes a new update, that update may be arbitrary (and not a provable derivation of previous updates) so the signature serves to indicate the publisher authorized the update. (Recall that the signature scheme does not *identify* any human participant.

- *sandboxing*: where feasible, pangalactic sandboxes computations so that maliciously authored computations, or buggy ones, cannot cause harm beyond the sandbox.

With this section in mind, the rest of this chapter describes how data "should be" related to each other descriptively, without using qualifiers like "should".

### Capabilities

We use the term *capability* inspired by the capabilities security mindset. To say "X is a *capability* to Y" indicates that (if the network is properly available) knowing X is both sufficient and necessary to accomplish Y.

### Verification can be Outsourced

By relying on signatures, encryption, and ZKPs (especially the *zero-knowledge* property) we can often oursource verification so that a third party service can verify a data relation is valid and available without the capability to read that data! This outsourcing relationship can be expressed with *verification capabilities*.[^1]

[^1]: This possibility, as well as most of the authorization architecture of pangalactic is directly inspired by the [Tahoe-LAFS](https://www.tahoe-lafs.org/) project.

### Authorization / Distribution Layering

Pangalactic separates *authorization* decisions (such as which components can authorize updates to data or which may read data) from *distribution* of data across the network.

Authorization *never* relies on third party authorities, including within the network such as routing, naming, or remote host access control mechanisms. It relies only on capabilities, ZKPs, and attestations.

However, authorization and distribution are not entirely orthogonal because a good distribution design ensures that data flows to where it can be read rather than to areas where it cannot be read, and also that updates reach interested parties efficiently.

**TODO:** This chapter currently _ignores_ most of the distribution design. This part of the design is underdeveloped at present.

## Revision Control

The flagship application of pangalactic, `pg` is a revision control tool.

### Filesystem Structure

A user Alice can create a new *workspace* which consists of a local filesystem working directory and revision control *book-keeping*. 

The book-keeping is stored within the `.pg` subdirectory of the working directory. It is further split into *tracked* and *untracked* state. `.pg/UNTRACKED` directory contains untracked state and every other child of `.pg` is tracked. The book-keeping metadata consists of small *links* into *the Store*, which are both described below in [The Store](#the-store) section.

A *workspace* is always associated with a *publisher* which is a means of controlling how users can *subscribe* to the changes of the workspace via a *pubid* identifier.

The *untracked book-keeping* contains at least:

- The *publisher's key*, which is a publication capability for producing new records of changes.[^2]

[^2]: A hypothesized hazard is that users will relatively frequently `cp -r` their workspace, then continue revision control in both locations. This will create colliding records. This should be detected and the user could be guided to convert one of the copies to a *fork*.
 
The *tracked book-keeping* contains at least the most recent *record* of the changes in the revision history: `.pg/tip.pgl`.

### Creating Records

A user *saves* changes in their working directory to create a new *record*. This follows the following process:

1. The working directory, excluding `.pg/UNTRACKED` and any path configured to be ignored within `.pg/config.toml`, is inserted into the Store to produce a *CID* (described in [The Store](#the-store) section).
2. If `.pg/policy.pgx` file exists, it is used to derive a *revision* (described below). If it is not present a hardcoded *no-op policy* is used instead.
3. A new directory is stored is inserted in the Store with two links:
  - `prev`: a link to the previous record (which comes from `.pg/tip.pgl` in the no-op policy.
  - `rev`: a link to the *revision* from step 2.
4. A *record* is created with the contents of `3`, signed by the publisher.
5. That record is used to overwrite `.pg/tip.pgl`.
6. The as-of-yet-unspecified "distribution system" ensures subscribers to this workspace receive the new record.

### Store-Space vs Filesystem-Space

[The Store](#the-store) (described below) provides a directory & file abstraction. The complete revision history at any point is always "just" a directory structure in the Store. We refer to the layout and contents of this structure as *Store-space* to distinguish that it is the layout within the *Store*.

Meanwhile, the actual files and directories on a users computer that make up a workspace have a different but related layout and contents, and we distinguish this as the Filesystem-space, or often we'll just say "on the fs" for short.

The no-op policy converts the fs-space layout from step 1 into a *revision*, where the layout of *revision* isn't well specified yet...

Some things a revision needs to do:

- Specify the revision.
- Track public peer subscriptions?
- Contain a snapshot of the working directory contents.

### The Conventional Policy

By default, when a user creates a new workspace with `pg init`, a record is immediately published, implicitly, that installs *the Conventional Policy* (aka *the CoPo*) as the policy for that record, which is permitted by the no-op policy.

The CoPo, among other things, provides new revision control features atop the lower host-platform-coded layer, in contrast the the CoPo which is at the sandboxed deterministic computation layer of derivations.

#### Transactions

CoPo requires every change to a repository to be a *transaction*. Transactions are high-level user-facing concepts that map well to the UX, such as "save the current working directory state", "annotate a recent revision with this descriptive log message", and "if the given revision passes language and application-specific deterministic checks, then append a new record attesting to that fact, else do nothing." 

CoPo guarantees that the saved working directory state is the deterministic outcome of applying the associated transaction to the previous history. In this way, users can explore either snapshots of working directories or high-level transaction logs to understand the revision history, content with the knowledge that the relationship between these is preserved by the policy.

### Narratives and Releases

 Since a workspace is associated with a single publisher, and a publisher distributes a *linear sequence* of *records*, the revision control history produced by a given workspace is a linear history.

CoPo introduces *narratives* to facilitate common revision control management techniques. A narrative is a name referring to a "managed" history of changes which can be modified semi-arbitrarily. CoPo revisions store a mapping of the narrative names to "narrative revisions" within the top-level revision.

When users need a complete auditable record of changes (for example while investigating the insertion of a backdoor), including changes to narrative histories, they can review the log of top-level records. OTOH, when they need to review managed, curated, or pedagogical histories, they can rely on narratives.

Narratives are vaguely akin to `git` branches, that is, mutable names referring to different semi-related sub-histories of the complete revision history. However, with CoPo there are many important differences:

- Narratives are revision controlled, whereas git branches are local mutable state.
- Narratives can be semi-arbitrarily modified to "ret-con" the presented sequence of changes (similar to `git` branch modifications like squashing, rebasing, etc...). Unlike `git`, the sequence of changes to narratives themselves are tracked in the linear revision history. This strikes a balance between the needs for revision history to either provide a complete auditable record versus a well-edited pedagogical narrative of how the latest revision could ideally / should have been developed.
- Narratives are published to all subscribers of a workspace. (**TODO:** We *could* have a "local narrative" feature where they are stored in the untracked book-keeping area.)

*Releases* are narratives that live under the `/release/...` namespace and are more constrained by CoPo so that they cannot be overwritten and follow a constrained versioning format. (**TODO:** Should we allow overwrites of releases for "hot-fixes" or retractions?)

## Pub/Sub

A core primitive of pangalactic is the *Pub/Sub* system, which has an authorization layer and (one or more) outer distribution layers.

Note: while there is low-level commandline access to direct pub/sub features via `pg uth pubsub`, all of the examples describe using the higher level revision control use case which manages pub/sub usage under the hood.

### The Pub/Sub Capabilities Model

The *Pub/Sub Capabilities Model* or PSCM is the authoritzation layer of pub/sub: a network/IO-agnostic cryptographic data format which defines how users can *publish* and *subscribe* to update *records*.

#### Publishers

Any time Alice wishes to share a sequence of related update *records* to Bob and Charlie, she generates a new *publisher*. This typically occurs under the hood when using `pg` for revision control when a user runs `pg init`.

A *publisher* is entirely controlled by a cryptographic signing key, which Alice typically stores privately. Again, when using `pg` for revision control, this is managed under the hood. With that in place, Alice then shares her *publisher ID* aka *pubid* with Bob and Charlie. The pubid for a revision control *workspace* is available with the `pg info` command.

#### Subscriptions

Bob and Charlie then take Alice's pubid and *subscribe* to it. Suppose the publisher Alice created is for a revision control workspace and Bob wants to acquire a new local copy, then he uses the `pg fork` command to create a new local workspace and subscribe to Alice's pubid. Meanwhile, let's suppose the workspace Alice is hacking on is part of a project that Charlie is also hacking on, but the two haven't yet directly collaborated. In that case Charlie uses `pg peer` in his workspace to subscribe to Alice's workspace updates.

#### Records

Now that Bob and Charlie are subscribed to Alice's pubid, they can receive *records* from it. A publisher always produces a *linear sequence of records*. Records contain a *sequence number* which starts at 0 and increments each time Alice *publishes* a record with her publisher. Additionally it contains a *previouis CID* and a *current CID*. (CIDs are described soon in [The Store](#the-store) section).

A record provides the pubid which produced it (either because it is literally stored in the record or it is derivable given the signature scheme and signature).

#### PSCM Confidentiality

An important feature of the pub/sub PSCM is that an arbitary third party, Mal, *cannot* track updates from Alice's workspace *without* subscribing to the pubid for it. If Alice is working on a private project she could ensure she sends the pubid to Bob and Charlie privately, and rely on them not to share the pubid elsewhere without checking with her first.

In other words a pubid is a capability for two actions: *retrieving* and *reading* records from the publisher.

## The Store

The *Store* is a universal immutable data storage and distribution layer, spread across a network. The term "the Store" refers to both the Authorization / Distribution distinction, but in this section we describe only the *Store Capabilities Model*.

The key primitive is hash-based indexing, along with "directory" nodes for linking together data within the Store.

*Content Identifiers*, or *CIDs* for short, are (relatively) short self-authenticating immutable data references. At an elementary level, the data they refer to is an arbitrary sequence of bytes, called a *blob*. CIDs provide the following properties:

- *Compactness*: CIDs are relatively short with a capped length (~32 or ~64 bytes).
- *Determinism*: any two devices compute the same CID given the same blobs as inputs.
- *Collision Resistance*: no two distinct blobs result in the same CID value.
- *Read Capability*: a blob *cannot* be retrieved and read from any source without the corresponding CID. (**TODO:** Introduce immutable *VerifyCap* vs *ReadCap* terminology and semantics.)

**Rust Crate:** `pangalactic-cid`

*Directories* are blobs which contain a pangalactic-specific serialization of a directory structure which contains a set of child *links*.

Each child link contains a *link name* which is unique within a given directory. It also includes a *link kind* which claims the referent blob is either a directory or a *file*. Finally it contains a *link reference*.

**Rust Crates:** `pangalactic-dir`, `pangalactic-linkkind`, `pangalactic-link`

A *link reference* is either just a bare CID _or_ it is a `(record, path)` tuple. A link reference can be resolved to a "direct CID" which is either just the bare CID in the first case, or the result of traversing the `path` from the `current CID` of the `record`. When a link reference is a bare CID, it's called a *hard link* and when it is a `(record, path)` it is called a *splice link* or *splice* for short. The `path` selects a subpath within the `record` which can be used to splice to previous history, specific narratives (especially releases), etc...

**Note:** Splices are not yet implemented.

*Splices* are designed to support several important `pg` use cases:

- *Unified Subscriptions & Locking*: An important property is that splices provide a *subscription* to receive future updates and simultaneously *locking*. Locking (also called *pinning*) is a common feature of dependency management systems to ensure a particular revision of software is tied to specific revisions of all of its dependencies. Transitive pinning is necessary to ensure any two hosts that build the software are building with the same inputs. One of `pg`'s target use cases is deterministic builds, so making pinning a first-class feature meets this goal.

- Maintaining *peer subscriptions* of a workspace. These are subscriptions to peer workspaces which enable users to share code patches with each other. This is somewhat akin to the `git` revision control tool's `.git/refs/remotes` tracking, except that every peer is a signed record (and revision controlled).

- Maintaining "embedded workspaces" in a workspace. Embedded workspaces allow a user to embed the source code of other projects into their source code via a slice, which enables explicit version control across the containing and embedded projects. For example, a rust project could splice many dependencies into a "cargo vendor" directory structure so that cargo will build the primary (containing) workspace without needing network access. Meanwhile, the containing workspace also is guaranteed to have pinned subscriptions to the dependencies and so dependency management is explicitly controlled by the containing workspace author. Avoiding network access is an important requirement for deterministic builds.[^3]

[^3]: This enables `pg` repositories meet the use cases of `git subtree` and/or `git submodules` except as a single first class feature that behaves the same as other uses of splice links.

Finally, because splices are first class in the PSCM, "lock updates" are also a first class feature via the `pg update` command. This can be used to discover and pin newer updates to dependencies, peer repositories, or embedded workspaces. Those changes, of course, will be part of the revision history of the workspace.

### The Local Store

The *Local Store* is a user-scoped filesystem storage backend containing a subset of the complete universal *Store*.

**Note:** Because the Local Store is user-scoped, *all* of a user's revision control storage is shared across all projects on the system. For example, if a user clones the same project into multiple different working directories, the revision control storage is deduplicated / cached for all working directories.

**Note:** There is no equivalent of `.git` blob storage. This means if you need to ensure the complete history of a project is copied to a new host, you must either copy the entire Local Store (which may include other projects), or use the `pg` tool to perform the transfer.

## Derivation

The pangalactic derivation system is a first-class deterministic computation system. Purpose-built code directs how to construct new files and directories in the Store given existing files and directories as the sole input.

The current implementation is build on WASM (via `wasmtime`) with a special-purpose binary "host call API".

A *plan* is a directory that specifies both the executable and inputs. A host can *derive* an *attestation* from a plan. An *attestation* contains the originating plan, the generated output (or deterministic error), a log, and supporting evidence that this is the correct attestation for the plan.

The supporting evidence should eventually be a ZKP, but in the short term we will implement signature-based attestation. (**Note**: When Alice derives outputs as part of the standard revision control process, the resulting attestations are implicitly signed by Alice's publisher.)

