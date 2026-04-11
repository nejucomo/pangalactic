# Capabilities Security

## What is a Capability?

Pangalactic is designed around the *capabilities security* mindset. A **capability** is a piece of data such that:

- *Knowing* it is **sufficient** to perform the associated action (given network availability).
- *Knowing* it is **necessary** to perform that action — no other path grants the ability.

Capabilities are just data: they can be stored, transmitted, kept secret, or shared selectively, and the system's access-control logic follows entirely from who possesses which capabilities. There are no separate "identity" or "permission" systems layered on top.

> **Inspiration:** This design is directly inspired by [Tahoe-LAFS](https://www.tahoe-lafs.org/), which pioneered the use of cryptographic capabilities for storage authorization in a decentralised setting.

## The `<verb>Cap` Naming Convention

Pangalactic names every capability type using a `<verb>Cap` pattern, where `<verb>` names the action that possession of the capability enables:

- A `ReadCap` enables *reading* the associated data.
- A `VerifyCap` enables *verifying* the integrity of the associated data (without necessarily reading it).
- A `PublishCap` enables *publishing* new records to an associated publication stream.
- A `SubscribeCap` enables *subscribing to* and reading records from an associated publication stream.

This naming convention makes the relationship between data and authority explicit and uniform across the whole system.

## `PublishCap` and `SubscribeCap`

The Pub/Sub system (described in detail in the [Architecture](arch.md) chapter) uses two complementary capabilities:

**`PublishCap`** — a cryptographic *signing key* held privately by the author of a publication stream. Possessing a `PublishCap` is both necessary and sufficient to append new records to that stream. It is never shared; leaking it would allow anyone to publish on the author's behalf.

**`SubscribeCap`** — a cryptographic *verification key* (or a value from which one is derivable) that allows its holder to locate, retrieve, and verify records from the associated publication stream. A `SubscribeCap` is what an author shares with collaborators to invite them to follow the stream.

Crucially, **a `SubscribeCap` is the only identifier for a publication stream** — there is no separate "publisher ID" for general-purpose identification. Knowing a `SubscribeCap` is what it means to have access to a publication stream, and not knowing it means the stream is inaccessible.

This has an important privacy consequence: a third party who does not possess a `SubscribeCap` cannot discover or track the associated publication stream, even if they can observe network traffic.
