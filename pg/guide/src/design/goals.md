# Goals

Pangalactic is designed with these goals in mind:

The primary goal is to enable decentralized revision control, build, and composition.

## Decentralization

Decentralization embodies several sub-goals:

- **Permissionlessness Use:** any set of users can use Pangalactic to collaborate without censorship or an intemediary of the content they collaborate on.
- **Permissionlessness Composition:** a user may use any third party projects to which they subscribe as dependencies or forks for their own projects without requiring the permission or notification of the third party authors.
- **Stand-alone Capabilities:** the capabilities which enable collaboration between users are stand-alone and do not rely on intermediaries or access controls. For example, the capability to publish revisions to a project or the capability to subscribe to new revisions of a project are independent of the network (such as DNS or TLS/PKI).
- **Publication and Subscription Capabilities:** users can create any number of publication capabilities freely for one or more projects, and then share subscriptions in any manner of their choosing. Only users with a subscription can retrieve revisions for the associated project. Thus, by default, all projects have confidentiality. (Of course a publisher may choose to post a subscription publicly for an open source project.)
- **Resilience:** Pangalactic users should be able to collaborate when they have sufficient connectivity to each other without over-reliance or vulnerability on centralized networks or protocols.
- **Network Independence:** Pangalactic should be usable and useful without excessive network requirements. For example, users can use the complete system on a local machine without internet access. Network access should only be used to provide availability and connectivity to content.

## Revision Control

- The Revision Control UX of `pg` should be high-level, streamlined, and widely scoped. For example, it should be simpler than `git` for a user new to both systems.
- The `pg` Revision Control application provides high-level features and concepts in addition to standard modern revision control systems: issue tracking, dependency management, sub-projects, etc…

## Build

- The Pangalactic Derivation System should provide a hermetically sealed deterministic build-like system, and as much functionality as possible should be built inside that framework. Ideally this would include a self-hosted compiler.
- The `pg build` system should provide pragmatic host system build orchestration which aims for serving the ~80% of use cases.
- Both the PDS and `pg build` are streamlined into both revision control and dependency management.

## Composition

- The revision control and build functionality are composable by relying on the Pangalactic Store & PubSub platform for dependency management.

## Why These Goals?

The next evolution of development collaboration requires decentralization, not just "nuts and bolts" feature improvements, because intermediaries and centralized control points are ultimately antithetical to free software development. These intermediaries are both authority risks as well as simpler reliability risks.
