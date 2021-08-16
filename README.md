#  PR-Bump

GitHub action to automatically bump versions using releases and pull requests labels.

## Gotchas

- Only pull requests that were merged after the latest release was created are considered. Creating
  a release which doesn't target the `HEAD` of the branch where PRs are merged is not yet supported.
  In other words, if you make a release that does not include all currently merged PRs, then those
  PRs will not be considered when calculating a version bump.
