# Making a release

- in a branch:
  - update [CHANGELOG.md](../CHANGELOG.md)
  - update all occurrences of `0.14.2`
  - ship into `main`
- create a new tag:

  ```bash
  git checkout main && git tag v0.14.2 && git push --tags
  ```
- the CI server creates the release fully automatically
