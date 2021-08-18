#  PR-Bump

This action uses GitHub releases and pull request labels to bump versions automatically.

ℹ️ This action was designed to integrate easily with
[release-changelog-builder](https://github.com/marketplace/actions/release-changelog-builder). The
configuration format is compatible between both actions, so you can use a single config for both!

### How does it work?

The action does the following, in that order :
1. Finds the latest release for the current repo (defaulting to 0.1.0 if none is found).
2. Loads the last 100 closed pull requests.
3. Filters to keep only pull requests that were merged after the latest release has been created.
4. Reads the labels on those pull requests.
5. Finds the next version based on those labels (and an optional configuration file).

## Example Usage

```yml
- name: "Bump Version"
  uses: marier-nico/pr-bump@1.0
  with:
    configuration: ".github/workflows/pr_bump_config.json"
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

```json
// .github/workflows/pr_bump_config.json

{
  "base_branches": ["main", "master"],
  "bump_files": [
    {
      "path": "Cargo.toml",
      "prefix": "version = \""
    }
  ],
  "categories": [
    {
      "labels": ["fix", "bug", "docs"],
      "semver_part": "patch"
    },
    {
      "labels": ["feature", "enhancement", "a-custom-label"],
      "semver_part": "minor"
    },
    {
      "labels": ["major", "breaking", "milestone"],
      "semver_part": "major"
    }
  ]
}
```

## 📥 Inputs and Outputs

⚠️ You need to make sure the repo's code is available if you want to use a custom configuration file.
You can do this by using the `actions/checkout@v2` action.

| **Input**       | **Required** | **Description**                                                                  |
|-----------------|--------------|----------------------------------------------------------------------------------|
| `configuration` | No           | Relative path from the repo's root to the json configuration file for the action |

| **Output**         | **Required** | **Description**                                                |
|--------------------|--------------|----------------------------------------------------------------|
| `previous_version` | Yes          | The semver version number for the previous version of the repo |
| `next_version`     | Yes          | The semver version number after the version bump               |
| `has_bump`         | Yes          | Whether or not the version was bumped                          |

🔒 For private repos, you need to set the `GITHUB_TOKEN` environment variable.

```yml
- name: "Bump Version"
  uses: marier-nico/pr-bump@1.0
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

## ⚙️ Configuration Format

The configuration must be a json file. If a value is specified in the json file, that value will
override the default for that specific setting. The others will still use the defaults.

**Default Configuration**
```json
{
  "bump_files": [],
  "categories": [
    {
      "labels": ["bug", "docs", "documentation", "fix", "patch"],
      "semver_part": "patch"
    },
    {
      "labels": ["enhancement", "feat", "feature", "minor"],
      "semver_part": "minor"
    },
    {
      "labels": ["breaking", "major"],
      "semver_part": "major"
    }
  ]
}
```

**Possible Values**

- `base_branches`: A pull request is only considered for a version bump if it is merged into a
  branch listed here. If this setting is absent, all base branches are considered.
  ```json
  {
    "base_branches": ["main"]
  }
  ```
- `bump_files`: Update version numbers inside files. The path to the file is relative to the repo's root, and the prefix must be directly before the version number.
  ```toml
  # Cargo.toml

  [package]
  name = "pr-bump"
  version = "0.1.0"
  ```
  ```json
  // pr_bump_config.json

  {
    "bump_files": [
      {
        "path": "Cargo.toml",
        "prefix": "version = \""
      }
    ]
  }
  ```
- `categories`: Associate a label with a version bump level. In the example, if a PR has the `fix` label, the repo would go form 1.2.3 to 1.2.4.
  ```json
  {
    "categories": [
      {
        "labels": ["fix"],
        "semver_part": "patch"
      }
    ]
  }
  ```

## 🚨 Gotchas

- Only pull requests that were merged after the latest release was created are considered. Creating
  a release which doesn't target the `HEAD` of the branch where PRs are merged is not yet supported.
  In other words, if you make a release that does not include all currently merged PRs, then those
  PRs will not be considered when calculating a version bump.
