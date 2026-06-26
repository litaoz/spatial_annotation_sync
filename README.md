# Product Requirements Document: Collaborative Spatial Annotation Sync Library

## Problem Statement
Building collaborative AR applications requires a data layer that can handle concurrent edits to shared spatial content — even when users are offline. This project solves the core synchronization problem: how do two clients independently edit the same spatial annotations and converge to the same state when they reconnect, without a server arbitrating conflicts?

The intended audience is developers who would build on top of this as a library, with a CLI demo as the proof-of-concept interface.

## Goals & Success Criteria
- A working sync demo: two separate processes communicate over localhost, make conflicting edits while "offline," reconnect, and converge to identical state
- A clear README explaining the AR use case and how to run the demo
- A gif demonstrating the sync operation end-to-end
- Property-based tests (via proptest) that assert convergence under arbitrary operation orderings — the mathematical proof of correctness

## Core Features
### 1. Annotation CRUD
Users can create, edit, and delete spatial annotations. Each annotation has:
- A unique ID
- Spatial coordinates (x, y, z)
- Text content
- A LWW-Element-Set CRDT representation of its state

### 2. Offline Editing
Each client can create, edit, and delete annotations independently with no network connection. Changes are tracked locally and queued for sync.

### 3. Peer Sync
Two clients connect over localhost (plain TCP via tokio) and exchange their full state. After syncing, both clients converge to identical state. No reconnection handling is required for the MVP.

### 4. Convergence Verification
Property-based tests assert that any two clients always reach the same final state regardless of the order operations are applied or received.

## User Stories
### Scenario 1: Concurrent Edits to Coordinates
Alice and Bob both run the app as separate processes. While offline, Alice moves an annotation to (1, 2, 3) and Bob moves the same annotation to (4, 5, 6). When they reconnect and sync, both clients converge to (4, 5, 6) because Bob's write carried a later timestamp (Last Write Wins).

### Scenario 2: Delete vs. Edit Conflict
Alice deletes an annotation at T=10 while Bob edits the same annotation at T=8, both while offline. When they sync, the annotation is deleted on both clients because Alice's delete has the later timestamp. If Bob's edit had occurred at T=12, the annotation would survive with Bob's content.

## Scope & Constraints
### In Scope
- LWW-Element-Set CRDT for all annotation state (coordinates, content, existence)
- Last Write Wins conflict resolution across all fields, including deletes
- Two-peer sync over plain TCP on localhost
- In-memory state only (no disk persistence)
- CLI demo harness showing two processes syncing
- Property-based convergence tests via proptest
- README and demo gif

### Out of Scope
- Collaborative text editing within an annotation (e.g., RGA/LSEQ)
- More than 2 peers syncing simultaneously
- Authentication or security on the sync connection
- Disk persistence
- Reconnection handling or fault tolerance
- Real network deployment (localhost only)

### External Dependencies
- tokio — async runtime and TCP transport
- proptest — property-based testing for convergence verification
