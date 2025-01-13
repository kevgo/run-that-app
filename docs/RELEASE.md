# Making a release

- in a branch:
  - update [CHANGELOG.md](../CHANGELOG.md)
  - update all occurrences of `0.10.1`
  - ship into `main`
- create a new tag:

  ```bash
  git tag v0.10.1 && git push --tags
  ```
- the CI server creates the release fully automatically
