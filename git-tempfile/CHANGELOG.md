# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 3.0.1 (2023-01-10)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 49 calendar days.
 - 49 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - prepare changelogs prior to release ([`93bef97`](https://github.com/Byron/gitoxide/commit/93bef97b3c0c75d4bf7119fdd787516e1efc77bf))
    - Merge branch 'patch-1' ([`b93f0c4`](https://github.com/Byron/gitoxide/commit/b93f0c49fc677b6c19aea332cbfc1445ce475375))
    - thanks clippy ([`9e04685`](https://github.com/Byron/gitoxide/commit/9e04685dd3f109bfb27663f9dc7c04102e660bf2))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
</details>

## 3.0.0 (2022-11-21)

### New Features (BREAKING)

 - <csr-id-3d8fa8fef9800b1576beab8a5bc39b821157a5ed/> upgrade edition to 2021 in most crates.
   MSRV for this is 1.56, and we are now at 1.60 so should be compatible.
   This isn't more than a patch release as it should break nobody
   who is adhering to the MSRV, but let's be careful and mark it
   breaking.
   
   Note that `git-features` and `git-pack` are still on edition 2018
   as they make use of a workaround to support (safe) mutable access
   to non-overlapping entries in a slice which doesn't work anymore
   in edition 2021.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 2 calendar days.
 - 14 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-hash v0.10.0, git-features v0.24.0, git-date v0.3.0, git-actor v0.14.0, git-glob v0.5.0, git-path v0.6.0, git-quote v0.4.0, git-attributes v0.6.0, git-config-value v0.9.0, git-tempfile v3.0.0, git-lock v3.0.0, git-validate v0.7.0, git-object v0.23.0, git-ref v0.20.0, git-sec v0.5.0, git-config v0.12.0, git-command v0.2.0, git-prompt v0.2.0, git-url v0.11.0, git-credentials v0.7.0, git-diff v0.23.0, git-discover v0.9.0, git-bitmap v0.2.0, git-traverse v0.19.0, git-index v0.9.0, git-mailmap v0.6.0, git-chunk v0.4.0, git-pack v0.27.0, git-odb v0.37.0, git-packetline v0.14.0, git-transport v0.23.0, git-protocol v0.24.0, git-revision v0.7.0, git-refspec v0.4.0, git-worktree v0.9.0, git-repository v0.29.0, git-commitgraph v0.11.0, gitoxide-core v0.21.0, gitoxide v0.19.0, safety bump 28 crates ([`b2c301e`](https://github.com/Byron/gitoxide/commit/b2c301ef131ffe1871314e19f387cf10a8d2ac16))
    - prepare changelogs prior to release ([`e4648f8`](https://github.com/Byron/gitoxide/commit/e4648f827c97e9d13636d1bbdc83dd63436e6e5c))
    - Merge branch 'version2021' ([`0e4462d`](https://github.com/Byron/gitoxide/commit/0e4462df7a5166fe85c23a779462cdca8ee013e8))
    - upgrade edition to 2021 in most crates. ([`3d8fa8f`](https://github.com/Byron/gitoxide/commit/3d8fa8fef9800b1576beab8a5bc39b821157a5ed))
</details>

## 2.0.6 (2022-11-06)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 47 calendar days.
 - 47 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-features v0.23.1, git-glob v0.4.1, git-config-value v0.8.1, git-tempfile v2.0.6, git-object v0.22.1, git-ref v0.18.0, git-sec v0.4.2, git-config v0.10.0, git-prompt v0.1.1, git-url v0.10.1, git-credentials v0.6.1, git-diff v0.21.0, git-discover v0.7.0, git-index v0.7.0, git-pack v0.25.0, git-odb v0.35.0, git-transport v0.21.1, git-protocol v0.22.0, git-refspec v0.3.1, git-worktree v0.7.0, git-repository v0.26.0, git-commitgraph v0.10.0, gitoxide-core v0.19.0, gitoxide v0.17.0, safety bump 9 crates ([`d071583`](https://github.com/Byron/gitoxide/commit/d071583c5576fdf5f7717765ffed5681792aa81f))
    - prepare changelogs prior to release ([`423af90`](https://github.com/Byron/gitoxide/commit/423af90c8202d62dc1ea4a76a0df6421d1f0aa06))
    - Merge branch 'main' into write-sparse-index (upgrade to Rust 1.65) ([`5406630`](https://github.com/Byron/gitoxide/commit/5406630466145990b5adbdadb59151036993060d))
    - thanks clippy ([`04cfa63`](https://github.com/Byron/gitoxide/commit/04cfa635a65ae34ad6d22391f2febd2ca7eabca9))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
</details>

## 2.0.5 (2022-09-20)

Maintenance release without observable changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 27 calendar days.
 - 27 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#450](https://github.com/Byron/gitoxide/issues/450), [#470](https://github.com/Byron/gitoxide/issues/470)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - add safety notes ([`910fa06`](https://github.com/Byron/gitoxide/commit/910fa06f39c9839c58d59467bca05ad511c23bef))
 * **[#470](https://github.com/Byron/gitoxide/issues/470)**
    - update changelogs prior to release ([`caa7a1b`](https://github.com/Byron/gitoxide/commit/caa7a1bdef74d7d3166a7e38127a59f5ab3cfbdd))
 * **Uncategorized**
    - Release git-hash v0.9.10, git-features v0.22.5, git-date v0.2.0, git-actor v0.12.0, git-glob v0.4.0, git-path v0.5.0, git-quote v0.3.0, git-attributes v0.4.0, git-config-value v0.8.0, git-tempfile v2.0.5, git-validate v0.6.0, git-object v0.21.0, git-ref v0.16.0, git-sec v0.4.0, git-config v0.8.0, git-discover v0.5.0, git-traverse v0.17.0, git-index v0.5.0, git-worktree v0.5.0, git-testtools v0.9.0, git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
    - Merge branch 'filter-refs-by-spec' ([`5c05198`](https://github.com/Byron/gitoxide/commit/5c051986bd89590a9287d85d84c713d83dfab83a))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
</details>

## 2.0.4 (2022-08-24)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 5 calendar days.
 - 6 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`4974eca`](https://github.com/Byron/gitoxide/commit/4974eca96d525d1ee4f8cad79bb713af7a18bf9d))
    - Merge branch 'main' into remote-ls-refs ([`95f2f4f`](https://github.com/Byron/gitoxide/commit/95f2f4f17f7eae174a64c7d9f6a1513d73b21bbb))
    - thanks clippy ([`f90d772`](https://github.com/Byron/gitoxide/commit/f90d772cf625afea605e42c92db15ed870d7e120))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - remove default link to cargo doc everywhere ([`533e887`](https://github.com/Byron/gitoxide/commit/533e887e80c5f7ede8392884562e1c5ba56fb9a8))
    - Merge branch 'main' into remote-ls-refs ([`bd5f3e8`](https://github.com/Byron/gitoxide/commit/bd5f3e8db7e0bb4abfb7b0f79f585ab82c3a14ab))
</details>

## 2.0.3 (2022-08-17)

A maintenance release without user facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 26 calendar days.
 - 26 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-date v0.0.3, git-actor v0.11.1, git-attributes v0.3.1, git-tempfile v2.0.3, git-object v0.20.1, git-ref v0.15.1, git-config v0.6.1, git-diff v0.17.1, git-discover v0.4.0, git-bitmap v0.1.1, git-index v0.4.1, git-mailmap v0.3.1, git-traverse v0.16.1, git-pack v0.21.1, git-odb v0.31.1, git-packetline v0.12.6, git-url v0.7.1, git-transport v0.19.1, git-protocol v0.18.1, git-revision v0.4.0, git-worktree v0.4.1, git-repository v0.21.0, safety bump 5 crates ([`c96473d`](https://github.com/Byron/gitoxide/commit/c96473dce21c3464aacbc0a62d520c1a33172611))
    - prepare changelogs prior to reelase ([`c06ae1c`](https://github.com/Byron/gitoxide/commit/c06ae1c606b6af9c2a12021103d99c2810750d60))
    - Merge pull request #2 from SidneyDouw/main ([`ce885ad`](https://github.com/Byron/gitoxide/commit/ce885ad4c3324c09c83751c32e014f246c748766))
    - Merge branch 'Byron:main' into main ([`9b9ea02`](https://github.com/Byron/gitoxide/commit/9b9ea0275f8ff5862f24cf5a4ca53bb1cd610709))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - Merge branch 'main' into pathspec ([`7b61506`](https://github.com/Byron/gitoxide/commit/7b615060712565f515515e35a3e8346278ad770c))
    - Merge branch 'kianmeng-fix-typos' ([`4e7b343`](https://github.com/Byron/gitoxide/commit/4e7b34349c0a01ad8686bbb4eb987e9338259d9c))
    - Fix typos ([`e9fcb70`](https://github.com/Byron/gitoxide/commit/e9fcb70e429edb2974afa3f58d181f3ef14c3da3))
</details>

## 2.0.2 (2022-07-22)

This is a maintenance release with no functional changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 21 calendar days.
 - 110 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-hash v0.9.6, git-features v0.22.0, git-date v0.0.2, git-actor v0.11.0, git-glob v0.3.1, git-path v0.4.0, git-attributes v0.3.0, git-tempfile v2.0.2, git-object v0.20.0, git-ref v0.15.0, git-sec v0.3.0, git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0, safety bump 22 crates ([`4737b1e`](https://github.com/Byron/gitoxide/commit/4737b1eea1d4c9a8d5a69fb63ecac5aa5d378ae5))
    - prepare changelog prior to release ([`3c50625`](https://github.com/Byron/gitoxide/commit/3c50625fa51350ec885b0f38ec9e92f9444df0f9))
    - Merge pull request #1 from Byron/main ([`085e76b`](https://github.com/Byron/gitoxide/commit/085e76b121291ed9bd324139105d2bd4117bedf8))
    - Merge branch 'main' into pathspec ([`89ea12b`](https://github.com/Byron/gitoxide/commit/89ea12b558bcc056b892193ee8fb44b8664b5da4))
    - Merge branch 'main' into cont_include_if ([`daa71c3`](https://github.com/Byron/gitoxide/commit/daa71c3b753c6d76a3d652c29237906b3e28728f))
    - thanks clippy ([`e1003d5`](https://github.com/Byron/gitoxide/commit/e1003d5fdee5d4439c0cf0286c67dec9b5e34f53))
</details>

## 2.0.1 (2022-04-03)

A maintenance release without any changes on the surface.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 42 calendar days.
 - 44 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#298](https://github.com/Byron/gitoxide/issues/298), [#328](https://github.com/Byron/gitoxide/issues/328), [#364](https://github.com/Byron/gitoxide/issues/364)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - refactor ([`591b533`](https://github.com/Byron/gitoxide/commit/591b5338ecdc0da33151baa0781fd8dc1ee8d5a9))
 * **[#328](https://github.com/Byron/gitoxide/issues/328)**
    - a stress test to try conjure deadlocks in handlers ([`3bdecb5`](https://github.com/Byron/gitoxide/commit/3bdecb54570b46e4c140e783f49ff23ef465582d))
 * **[#364](https://github.com/Byron/gitoxide/issues/364)**
    - update changelogs prior to release ([`746a676`](https://github.com/Byron/gitoxide/commit/746a676056cd4907da7137a00798344b5bdb4419))
 * **Uncategorized**
    - Release git-diff v0.14.0, git-bitmap v0.1.0, git-index v0.2.0, git-tempfile v2.0.1, git-lock v2.0.0, git-mailmap v0.1.0, git-traverse v0.13.0, git-pack v0.17.0, git-quote v0.2.0, git-odb v0.27.0, git-packetline v0.12.4, git-url v0.4.0, git-transport v0.16.0, git-protocol v0.15.0, git-ref v0.12.0, git-worktree v0.1.0, git-repository v0.15.0, cargo-smart-release v0.9.0, safety bump 5 crates ([`e58dc30`](https://github.com/Byron/gitoxide/commit/e58dc3084cf17a9f618ae3a6554a7323e44428bf))
    - make fmt ([`7cf3545`](https://github.com/Byron/gitoxide/commit/7cf354509b545f7e7c99e159b5989ddfbe86273d))
    - Merge branch 'short-id' ([`5849d5b`](https://github.com/Byron/gitoxide/commit/5849d5b326b83f98a16cf1d956c720c7f0fd4445))
</details>

## 2.0.0 (2022-02-17)

### Bug Fixes (BREAKING)

 - <csr-id-c863ea5b34fa9ee3dac21c1f85587da16045f8d8/> do not install signal handlers by default.
   
   The previous behaviour is meant to be convenient for the casual
   user even though it
   ends up being surprising when used in applications that install
   their own signal handlers and need more control over how the program
   shuts down.
   
   This is now fixed by **requiring an explicit `setup()`** call before
   the first tempfile is created, which makes it a breaking change.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 1 calendar day.
 - 10 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#298](https://github.com/Byron/gitoxide/issues/298), [#336](https://github.com/Byron/gitoxide/issues/336)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - Use hash_hasher based hash state for better keys/less collisions ([`814de07`](https://github.com/Byron/gitoxide/commit/814de079f4226f42efa49ad334a348bce67184e4))
 * **[#336](https://github.com/Byron/gitoxide/issues/336)**
    - update changelog ([`2cfbe9c`](https://github.com/Byron/gitoxide/commit/2cfbe9ce81f611c0b541b9c0fd4fffd2d99dfa0a))
    - do not install signal handlers by default ([`c863ea5`](https://github.com/Byron/gitoxide/commit/c863ea5b34fa9ee3dac21c1f85587da16045f8d8))
 * **Uncategorized**
    - Release git-tempfile v2.0.0, safety bump 6 crates ([`90b1c42`](https://github.com/Byron/gitoxide/commit/90b1c42d5487904a9f329362d185b035d0ddb975))
</details>

## 1.0.6 (2022-02-07)

<csr-id-25209454d3f7e27e12e8ddca92e43b1ff01d58aa/>

Fixes a potential deadlock in in an interrupt handler attempting to cleanup tempfiles.

### Chore

 - <csr-id-25209454d3f7e27e12e8ddca92e43b1ff01d58aa/> upgrade dashmap to 5.1.0 (with security fix)

### Bug Fixes

 - <csr-id-81ed5f5e7a3634f0fab681ca59e40099f0118f75/> Assure interrupt based tempfile cleanup can't deadlock.
   
   We do this by using the `try_entry()` API provided by the most recent
   dashmap, at the cost of potentially trying to access a lot of indices
   that don't exist in the map anymore. The cost of this are expected
   to be low though especially since interrupts are the uncommmon case.
   
   A side-effect of this is that tempfiles that are currently being
   removed for writing to them, for example, won't be cleaned up.
   
   It will be up to the code running after the interrupt, if the case
   at all, to deal with the tempfile, which is what it does anyway.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 5 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#328](https://github.com/Byron/gitoxide/issues/328)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#328](https://github.com/Byron/gitoxide/issues/328)**
    - Prepare changelog ([`8e92494`](https://github.com/Byron/gitoxide/commit/8e924948dfa366d3d39227b63053c7ff00a5382a))
    - Assure interrupt based tempfile cleanup can't deadlock ([`81ed5f5`](https://github.com/Byron/gitoxide/commit/81ed5f5e7a3634f0fab681ca59e40099f0118f75))
 * **Uncategorized**
    - Release git-tempfile v1.0.6 ([`bd3f8ee`](https://github.com/Byron/gitoxide/commit/bd3f8ee28b51fa556a0f37c9bd62569f0ce7d49d))
    - upgrade dashmap to 5.1.0 (with security fix) ([`2520945`](https://github.com/Byron/gitoxide/commit/25209454d3f7e27e12e8ddca92e43b1ff01d58aa))
</details>

## 1.0.5 (2022-02-01)

### Bug Fixes

 - <csr-id-d9451e8d7fc39c252042f9d2447061262c16ae7a/> downgrade dashmap to 4.0 to avoid unsoundness.
   See https://github.com/xacrimon/dashmap/issues/167 for tracking
   progress on resolving the issue.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 8 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-tempfile v1.0.5 ([`d811154`](https://github.com/Byron/gitoxide/commit/d81115473cf04f5c3ae25b657b88c3f049451227))
    - downgrade dashmap to 4.0 to avoid unsoundness. ([`d9451e8`](https://github.com/Byron/gitoxide/commit/d9451e8d7fc39c252042f9d2447061262c16ae7a))
</details>

## 1.0.4 (2022-01-23)

A maintenance release thanks to upgraded dependencies.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 35 calendar days.
 - 100 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#266](https://github.com/Byron/gitoxide/issues/266)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#266](https://github.com/Byron/gitoxide/issues/266)**
    - upgrade dashmap to latest version ([`52d4fe5`](https://github.com/Byron/gitoxide/commit/52d4fe55b6dd88f72479abd4015cab063ddaaf97))
 * **Uncategorized**
    - Release git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`1b76119`](https://github.com/Byron/gitoxide/commit/1b76119259b8168aeb99cbbec233f7ddaa2d7d2c))
    - Release git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`8f57c29`](https://github.com/Byron/gitoxide/commit/8f57c297d7d6ed68cf51415ea7ede4bf9263326e))
    - Release git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`d78aab7`](https://github.com/Byron/gitoxide/commit/d78aab7b9c4b431d437ac70a0ef96263acb64e46))
    - Release git-hash v0.9.1, git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0, safety bump 4 crates ([`373cbc8`](https://github.com/Byron/gitoxide/commit/373cbc877f7ad60dac682e57c52a7b90f108ebe3))
    - prepare changelogs for release ([`674ec73`](https://github.com/Byron/gitoxide/commit/674ec73b0816baa2c63b4ef1b40b7a41849c5e95))
    - prepar changelogs for cargo-smart-release release ([`8900d69`](https://github.com/Byron/gitoxide/commit/8900d699226eb0995be70d66249827ce348261df))
</details>

## v1.0.3 (2021-10-15)

This release contains no functional changes, but a more useful changelog.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 14 commits contributed to the release over the course of 11 calendar days.
 - 34 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#198](https://github.com/Byron/gitoxide/issues/198)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#198](https://github.com/Byron/gitoxide/issues/198)**
    - Adjust all changelogs to fulfil requirements for publishing ([`04b9ca0`](https://github.com/Byron/gitoxide/commit/04b9ca025a1667529b2221ab4280bd3c8dae01cf))
    - Handle changelogs with upcoming version section if they were left for editing ([`0f5f47d`](https://github.com/Byron/gitoxide/commit/0f5f47da4662b596cbbbd9c0d83e135e2cc52c11))
    - deduplicate conventional message ids ([`e695eda`](https://github.com/Byron/gitoxide/commit/e695eda8cd183f703d9a3e59b7c3c7fa496ea1d2))
    - regenerate all changelogs to get links ([`0c81769`](https://github.com/Byron/gitoxide/commit/0c817690bd444f52bed2936b2b451cafd87dde92))
    - Mention actual issues that where worked on ([`a517e39`](https://github.com/Byron/gitoxide/commit/a517e39a81145b331f6c7a6cc2fc22e25daf42e2))
    - respect release-wide ignore list to allow removing entire conventional headlines ([`145103d`](https://github.com/Byron/gitoxide/commit/145103d4aa715386da9d4953f7f85fadc49fff9a))
    - Rebuild all changelogs to assure properly ordered headlines ([`4a9a05f`](https://github.com/Byron/gitoxide/commit/4a9a05f95930bad5938d4ce9c517ebf0e0b990f1))
    - Sort all commits by time, descending… ([`f536bad`](https://github.com/Byron/gitoxide/commit/f536bad20ffbac4dc353dfeb1a917bb88becbb78))
    - greatly reduce changelog size now that the traversal fix is applied ([`a0bc98c`](https://github.com/Byron/gitoxide/commit/a0bc98c06c349de2fd6e0d4593606e68b98def72))
    - Fixup remaining changelogs… ([`2f75db2`](https://github.com/Byron/gitoxide/commit/2f75db294fcf20c325555822f65629611be52971))
 * **Uncategorized**
    - Release git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-config v0.1.7, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0 ([`59ffbd9`](https://github.com/Byron/gitoxide/commit/59ffbd9f15583c8248b7f48b3f55ec6faffe7cfe))
    - Adjusting changelogs prior to release of git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0, safety bump 3 crates ([`a474395`](https://github.com/Byron/gitoxide/commit/a47439590e36b1cb8b516b6053fd5cbfc42efed7))
    - make fmt, but now it picked up some parts that usually don't get altered… ([`01f7b72`](https://github.com/Byron/gitoxide/commit/01f7b729337bd2c99498321c479a9a13b1858e3e))
    - Update changelogs just for fun ([`21541b3`](https://github.com/Byron/gitoxide/commit/21541b3301de1e053fc0e84373be60d2162fbaae))
</details>

## v1.0.2 (2021-09-10)

- Compatibility with Rust 1.55. It informed about the incorrect usage of `std::io::ErrorKind::Other` which this crate also dependent on in its tests.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-tempfile v1.0.2 ([`310ea73`](https://github.com/Byron/gitoxide/commit/310ea7336e78fbedb2cefa1ecb773b25e6a77e0a))
    - Update changelogs once more… ([`d57d279`](https://github.com/Byron/gitoxide/commit/d57d279dc603cf450c151bbb6dc6c6505cc6da10))
    - Update changelogs retro-actively… ([`78cfe0a`](https://github.com/Byron/gitoxide/commit/78cfe0ac341c6c2257743d913884b50042078e6c))
</details>

## v1.0.1 (2021-09-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 15 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-tempfile v1.0.1 ([`295eb37`](https://github.com/Byron/gitoxide/commit/295eb374d104ac2775b9f864ef3234e2c5832b54))
    - [tempfile #195] adapt to Rust 1.55 ([`d9e71ac`](https://github.com/Byron/gitoxide/commit/d9e71acc5d619b5e78673da4fbb5a531c97ad6dd))
    - thanks clippy ([`4701296`](https://github.com/Byron/gitoxide/commit/4701296bd5e2c4ad2f80f4e1de498db49f93385a))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v1.0.0 (2021-08-25)

- initial release

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 8 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-tempfile v1.0.0 ([`1238535`](https://github.com/Byron/gitoxide/commit/123853539dc30ddea2d822ab177ee09b191bdf1b))
    - [stability #171] prepare git-lock and git-tempfile release ([`3a1cf4d`](https://github.com/Byron/gitoxide/commit/3a1cf4d441b53c880b5c887916302a493ad28b41))
    - [stability #171] Prime git-tempfile and git-lock for release ([`01278fe`](https://github.com/Byron/gitoxide/commit/01278fe4e28bf97ce6a2b8947198683646e361ee))
</details>

## v1.0.0 (2021-08-25)

## v0.6.1 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 5 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-tempfile v0.6.1 ([`eda952f`](https://github.com/Byron/gitoxide/commit/eda952f95e9ece78bbdbe6c26dd78f7ac5365d86))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com/Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
</details>

## v0.6.0 (2021-08-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.6.0 ([`d58f37e`](https://github.com/Byron/gitoxide/commit/d58f37e3b5a000fbe069aa869bd84f66d5c3210b))
    - [utils #154] refactor: bool.then(||this) - neat ([`1dec1c4`](https://github.com/Byron/gitoxide/commit/1dec1c49032c8acb449e463fde41f403cb640e45))
</details>

## v0.5.0 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release over the course of 40 calendar days.
 - 48 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 ([`0e11e98`](https://github.com/Byron/gitoxide/commit/0e11e98f0562c7baa9c90e18db6240731d165217))
    - [pack #153] implement io traits for tempfiles ([`59d03d6`](https://github.com/Byron/gitoxide/commit/59d03d6133b301a19adfab212cf2c946110fc53b))
    - clippy on tests and thanks clippy ([`a77a71c`](https://github.com/Byron/gitoxide/commit/a77a71cf02d328a2a964388928d6b2a235a0aa85))
    - thanks clippy ([`e1964e4`](https://github.com/Byron/gitoxide/commit/e1964e43979b3e32a5d4bfbe377a842d2c0b10ea))
    - Remove unnecessary pub(crate) exports ([`3d2456e`](https://github.com/Byron/gitoxide/commit/3d2456e11709f0461b37c6df55ecc3861ca4cab5))
    - [lock] support recoverable commits ([`b2217e7`](https://github.com/Byron/gitoxide/commit/b2217e7d25df9801354f702b0625d3168f8d3271))
    - [ref] support for persistence with recovery ([`d8b2d66`](https://github.com/Byron/gitoxide/commit/d8b2d661b9cf644b52950b9dedf8dbce0d348098))
    - [ref] refactor ([`a261b82`](https://github.com/Byron/gitoxide/commit/a261b82c1ee7ebdbbc82ce1c8286ca6a0f221cea))
    - [ref] allow reflogs to be created in place of empty directory trees ([`80a6e0e`](https://github.com/Byron/gitoxide/commit/80a6e0e1ff2321d9162e799d5fc0f457e7de4ade))
    - [tempfile] a way to delete empty dirs recursively ([`6025aa0`](https://github.com/Byron/gitoxide/commit/6025aa08d93cd5124c8df38c51b795b9c7d1c911))
    - Bump libc from 0.2.97 to 0.2.98 ([`caf6495`](https://github.com/Byron/gitoxide/commit/caf6495b08f77d7e4eaa058074693fffb5c5644b))
    - [tempfile] close a tempfile while keeping track of it ([`aa96ed1`](https://github.com/Byron/gitoxide/commit/aa96ed1776a615446b9864b1231f9f33805ab178))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.4.0 (2021-06-23)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 2 calendar days.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 ([`4512798`](https://github.com/Byron/gitoxide/commit/45127986daba0a409f5b405d463fa23f5c4a053b))
    - [lock] add [must_use = "reason"] attribute where it matters ([`813c46b`](https://github.com/Byron/gitoxide/commit/813c46b1ac9ed5454c7832a6bad5a112f145b565))
    - [lock] refactor, remaining docs ([`956e69f`](https://github.com/Byron/gitoxide/commit/956e69fcb96085d96124b6c56d829607b36adf9f))
    - [lock] tests green ([`3706b26`](https://github.com/Byron/gitoxide/commit/3706b2669ebee5cd25a75a42d9b0a4a380707ee1))
    - [lock] cleanup signal handling even more… ([`9fb13d2`](https://github.com/Byron/gitoxide/commit/9fb13d27ccce5b0742ee9289fca891dbeb8a65de))
    - [lock] first tests and a lot of refactoring ([`3c34194`](https://github.com/Byron/gitoxide/commit/3c34194b6c0fd5ab22eb91081a563ba3bfa19110))
    - [lock] refactor; Marker is definitely not necessary… ([`6af84c9`](https://github.com/Byron/gitoxide/commit/6af84c92dbe049068be795ef4870fd830baf5384))
    - [lock] test what happens if multiple tempfiles are created ([`17942c7`](https://github.com/Byron/gitoxide/commit/17942c7960f25ad1f8f7fb2c94f251d39bb03c6e))
    - [lock] sketch API ([`f0e1427`](https://github.com/Byron/gitoxide/commit/f0e142734c1b09e6c4175b3c1b232d886449e280))
</details>

## v0.3.0 (2021-06-20)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 ([`92f3a83`](https://github.com/Byron/gitoxide/commit/92f3a830457766c88c68f8424828bfd9b5145f86))
    - [tempfile] refactor ([`f3144a8`](https://github.com/Byron/gitoxide/commit/f3144a897b4e10742fef47fadd350b4e9ddf316a))
    - [tempfile] remaining tests ([`450db66`](https://github.com/Byron/gitoxide/commit/450db6609eb3dad10deed3f9769a21ae8c4b3be8))
    - [tempfile] refactor ([`3bafa7b`](https://github.com/Byron/gitoxide/commit/3bafa7b2a3cf8efd0769564026ce7b757cb8c09b))
    - [tempfile] implement 'closed' version of tempfile ([`d4bb61d`](https://github.com/Byron/gitoxide/commit/d4bb61dbc99b4270464d903978222d31c7e7dc5e))
    - [tempfile] refactor ([`4788222`](https://github.com/Byron/gitoxide/commit/4788222c28605118c03ce9f3a4dfccc26e7f7b60))
    - [tempfile] fix docs ([`3cd6712`](https://github.com/Byron/gitoxide/commit/3cd6712c22dae2e804573bca0b7a687c36066c29))
    - [tempfile] sketch of a closed registration with different types ([`db9bb11`](https://github.com/Byron/gitoxide/commit/db9bb11a3132961029e04f1cf761bfc3c96ec33d))
    - [tempfile] refactor ([`8a0ce64`](https://github.com/Byron/gitoxide/commit/8a0ce64baf5a3d77a08aa68c3245be8e7964be70))
    - [tempfile] typesafe diffentiation between writable tempfiles and closed ones ([`3b424b1`](https://github.com/Byron/gitoxide/commit/3b424b1cc071580303d37b7459e80036635eb4aa))
    - [tempfile] refactor ([`913f301`](https://github.com/Byron/gitoxide/commit/913f3014313fe0c03cd8f0af88944d8d514d89d9))
    - [tempfile] refactor ([`9384617`](https://github.com/Byron/gitoxide/commit/9384617dbe00dd59726cc418f23fb0a6e6dde415))
    - [tempfile] implement 'map' on tempfile to realize that 'close()' can't be done… ([`f4a1d33`](https://github.com/Byron/gitoxide/commit/f4a1d33e994e986604d18a85b7f85e1cea063dcf))
</details>

## v0.2.0 (2021-06-19)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 46 commits contributed to the release over the course of 3 calendar days.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.2.0 ([`7c2eb36`](https://github.com/Byron/gitoxide/commit/7c2eb36274d13646956ac850bee90abbbac91c5b))
    - [tempfile] improve docs ([`d311b08`](https://github.com/Byron/gitoxide/commit/d311b082cdec323eb76363d986064fe771aa2bfd))
    - thanks clippy ([`a0f9803`](https://github.com/Byron/gitoxide/commit/a0f9803533e5684cfed5ab50cd8145d071e978b2))
    - [tempfile] refactor ([`3a0f1ad`](https://github.com/Byron/gitoxide/commit/3a0f1ad0963c77a07f1984c39b127337463c030b))
    - [tempfile] refactor ([`9b8abd0`](https://github.com/Byron/gitoxide/commit/9b8abd0336d6ea1d7c088c78fc09fa935408896f))
    - [tempfile] cleanup control for named and unnamed tempfiles ([`0ef85a2`](https://github.com/Byron/gitoxide/commit/0ef85a247d60332ca232d6d993987c0b89e34466))
    - [tempfile] all remaining remove_dir tests I can think of ([`3e45e5f`](https://github.com/Byron/gitoxide/commit/3e45e5fef4f0bbd8736ae3f197f15813290fe8dc))
    - [tempfile] first bunch of tests for error handling and basic function of rmdir ([`ba41a15`](https://github.com/Byron/gitoxide/commit/ba41a15d874a2709ab92a8680d9e168ece7b4676))
    - [tempfile] quick impl of remove-dir iter without tests ([`bf48913`](https://github.com/Byron/gitoxide/commit/bf48913b3bc1a8c3ebaa230880f573ad01982f08))
    - [tempfile] refactor ([`9226dbe`](https://github.com/Byron/gitoxide/commit/9226dbe18127d7e85ba2779393cb7263e87cfbf8))
    - [tempfile] refactor ([`730b733`](https://github.com/Byron/gitoxide/commit/730b733a1a5b2c3911849eef6ffe0833e12daf73))
    - [tempfile] refactor ([`1da35ce`](https://github.com/Byron/gitoxide/commit/1da35ce045609296c189133ca439a74b550ccc6c))
    - [tempfile] improve Retries documentation; retries docs for remove_dir ([`e665a5f`](https://github.com/Byron/gitoxide/commit/e665a5fcd38c7002545079c63f0bf35dee11876d))
    - [tempfile] sketch how tempfile cleanup should be configured… ([`71acede`](https://github.com/Byron/gitoxide/commit/71acede9cba6fc222d0bde1a3fd0c232d3c877ab))
    - [tempfile] logic fixed, it's working ([`6ad4946`](https://github.com/Byron/gitoxide/commit/6ad4946e2ee603c69dad1da3e1e996cd1d4ca075))
    - [tempfile] better counting, but… ([`972113f`](https://github.com/Byron/gitoxide/commit/972113f1ea72674db61867b14f0eed0de498b310))
    - [tempfile] better retry counts ([`c7a35ca`](https://github.com/Byron/gitoxide/commit/c7a35caa295580a1b9d4f8b77eb8ded9d9c88703))
    - [tempfile] refactor ([`273d722`](https://github.com/Byron/gitoxide/commit/273d72276a73d49a633b9be1c66f1a2357dfcb0f))
    - [tempfile] a better way to count retries… ([`e110b97`](https://github.com/Byron/gitoxide/commit/e110b97b4925a10fa9a62576daf9f078508b6760))
    - [tempfile] trying to implement remove_dir really shows that… ([`1319b90`](https://github.com/Byron/gitoxide/commit/1319b908cc42ef5114d22957ebed9ed2ced11391))
    - [tempfile] docs for create_dir; frame for remove_dir; ([`aa6b6d1`](https://github.com/Byron/gitoxide/commit/aa6b6d14236c817ecc64390b110069c4c1340c03))
    - [tempfile] tests for automatic directory creation ([`3bd5cee`](https://github.com/Byron/gitoxide/commit/3bd5cee0dc0811ff3b1ab3d1a93e7dca8ae06b69))
    - [tempfile] refactor ([`d441312`](https://github.com/Byron/gitoxide/commit/d4413125c432da2e7ef809aca61973f5f55dbd5c))
    - [tempfile] use create_dir::all based on configuration… ([`156c021`](https://github.com/Byron/gitoxide/commit/156c021ac8aaabe8fed60ac4681f365c75e0e165))
    - [tempfile] remove todo ([`5a14ab6`](https://github.com/Byron/gitoxide/commit/5a14ab63555d6e3a58ce32b68d4b47287869b73f))
    - [tempfile] more information about error cases, too ([`7415141`](https://github.com/Byron/gitoxide/commit/74151415f0019a32b4759cf01873acb4102f2d1e))
    - [tempfile] refactor ([`ae0c97a`](https://github.com/Byron/gitoxide/commit/ae0c97a59d9cc56e19d3ce6fcc12b4564a66298a))
    - [tempfile] refactor ([`7c7658d`](https://github.com/Byron/gitoxide/commit/7c7658d3390fdf1b5348a482c71a9fb20a815cca))
    - [tempfile] test for racy directory creation… ([`c9073bf`](https://github.com/Byron/gitoxide/commit/c9073bf0d6dff3165cfd43733a602127b8835727))
    - [tempfile] verify existing files are handled correctly ([`28fee55`](https://github.com/Byron/gitoxide/commit/28fee552718cbbed067b8a16631aaa1080886e00))
    - [tempfile] a test for directory creation limits ([`584b4b7`](https://github.com/Byron/gitoxide/commit/584b4b7a1e6997594f1234d5feab1bc82a83b859))
    - [tempfile] limit retries for directory creation… ([`1536c7a`](https://github.com/Byron/gitoxide/commit/1536c7a58f1da4b80e83f1169b3f865f12a3d9a2))
    - [tempfile] refactor ([`fa7b8e9`](https://github.com/Byron/gitoxide/commit/fa7b8e99d2613297127b044605a2314b878d3ab9))
    - [tempfile] handle interrupts and assure there is an end to it ([`dc0afbd`](https://github.com/Byron/gitoxide/commit/dc0afbdf08c44237b6749426ebacbded6cf8a647))
    - [tempfile] first recursive directory creation ([`b01faa9`](https://github.com/Byron/gitoxide/commit/b01faa9fdc371c1226978e32a3c71dbf3be600ec))
    - [tempfile] refactor ([`7b59392`](https://github.com/Byron/gitoxide/commit/7b59392fec4c80eddd9f82271eb1a5671e44aa5a))
    - [tempfile] another test ([`9e4834d`](https://github.com/Byron/gitoxide/commit/9e4834df1142fd0ffdbf5ffba1aed63bc67b66b3))
    - [tempfile] first sketch of iterator based create directory all… ([`314693c`](https://github.com/Byron/gitoxide/commit/314693c6a4577f0b2b00274a55ec99e87c17918f))
    - [lock] frame for git-lock crate ([`e6bc87d`](https://github.com/Byron/gitoxide/commit/e6bc87d77f9b397b25694f58d347de2fc38bf71d))
    - [tempfile] add journey test to validate operation on process level ([`2d1efd4`](https://github.com/Byron/gitoxide/commit/2d1efd4915d66890b1132d5b39e08027a83047ba))
    - [tempfile] more docs ([`d0c5e6b`](https://github.com/Byron/gitoxide/commit/d0c5e6b96f27d7ae708e7182b4cd5dbaceecd3cd))
    - refactor ([`e0b7f69`](https://github.com/Byron/gitoxide/commit/e0b7f695ee6bd1032544a29d91906f9b75e12d46))
    - [tempfile] clean cargo manifest ([`d43f514`](https://github.com/Byron/gitoxide/commit/d43f51438937d5bdd2bb2e02c355dcd4ee2b8680))
    - [tempfile] fix windows for good ([`3192a47`](https://github.com/Byron/gitoxide/commit/3192a47b730245f2656ccf8cd82394ec31e13126))
    - [tempfile] fix windows build (hopefully) ([`6c1df66`](https://github.com/Byron/gitoxide/commit/6c1df667031084a9e6fe9676534f80aae9a95789))
    - [tempfile] refactor ([`4a45df0`](https://github.com/Byron/gitoxide/commit/4a45df02340b55d34534be89934d2201dda261f9))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.1.0 (2021-06-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [tempfile] prepare release ([`c0f7fde`](https://github.com/Byron/gitoxide/commit/c0f7fde70b28526ad52dce9e2314a25af1531689))
    - [tempfile] an example to show off signal handlers ([`f325e69`](https://github.com/Byron/gitoxide/commit/f325e696c64e3f61f64c0a8d8c4e8af38104a713))
    - [tempfile] remaining docs ([`d334dc0`](https://github.com/Byron/gitoxide/commit/d334dc004d8b8eea5b586c6ada173d28d380ccce))
    - [tempfile] restore original signal handler behaviour. ([`9f91dd8`](https://github.com/Byron/gitoxide/commit/9f91dd8e95e1e51a8e0a4f7ba45628b3d93fc5de))
    - [tempfile] process-id filter on deletion to support forks ([`611056f`](https://github.com/Byron/gitoxide/commit/611056f431dc793684efad668d40b93b1cfec21e))
    - [tempfile] implement handler correctly, probably. ([`8cb0bbc`](https://github.com/Byron/gitoxide/commit/8cb0bbcf2d022401886071f9c91498977cea185c))
    - [tempfile] remove tempfiles on shutdown, but… ([`954b760`](https://github.com/Byron/gitoxide/commit/954b76029e4d9e331738137ec2c9804b0e06a890))
    - [tempfile] switch to dashmap as slab ([`6164719`](https://github.com/Byron/gitoxide/commit/61647195ce8fd0be1b3b63f19e8aaec392f33f19))
    - [tempfile] a more realistic slab test shows the index can get quite high. ([`915f14c`](https://github.com/Byron/gitoxide/commit/915f14c41145dbd3f63bd798e6d0cc18d51fef6f))
    - [tempfile] first step towards clearing tempfiles… ([`b0e0cee`](https://github.com/Byron/gitoxide/commit/b0e0cee866b643f9f9e4ebdc495abed5f5c6abf9))
    - [tempfile] precisely named tempfiles ([`edc74f0`](https://github.com/Byron/gitoxide/commit/edc74f0e8f04f45661d4909bb3e6c62f4ac1b453))
    - [tempfile] `take()` method ([`d377397`](https://github.com/Byron/gitoxide/commit/d3773976b86ad294528997104b9cfa0c803f9d6a))
    - [tempfile] basic operation of a tempfile ([`a692950`](https://github.com/Byron/gitoxide/commit/a692950ae7c32ed9dd040c0aebde494ef3029a30))
    - [tempfile] show that slabs can store a lot actually ([`0cc5b33`](https://github.com/Byron/gitoxide/commit/0cc5b33f0e421ed761e5c350fb4d3ad85ef3e884))
    - [tempfile] initial docs as there is a lot to consider ([`9dffc2b`](https://github.com/Byron/gitoxide/commit/9dffc2b918178650a3b40adfcc35730e48f46951))
    - [tempfile] crate frame ([`1b04c39`](https://github.com/Byron/gitoxide/commit/1b04c39030b436fb6850fbfa0c39a4fed7df727c))
</details>

