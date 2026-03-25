<a href="https://www.buymeacoffee.com/milkandsugar" target="_blank"><img src="https://cdn.buymeacoffee.com/buttons/v2/default-blue.png" alt="Buy Me A Coffee" style="height: 40px !important;width: 145px !important;" ></a>

# rmdsstore

## Usage

```plaintext
This program deletes all .DS_Store files that exist under the specified directory.

Usage: rmdsstore [OPTIONS] [Target Directory]

Arguments:
  [Target Directory]  File search root. Default: current directory

Options:
  -f, --force    Remove files without confirmation.
  -n, --dry-run  Show files that would be deleted without actually deleting them.
  -q, --quiet    Suppress non-critical output.
  -h, --help     Print help
  -V, --version  Print version
```

## Notes

- Hidden directories and `node_modules`/`target` are skipped automatically.
- `--force` and `--dry-run` cannot be used together.
- Exit code is `1` if any file fails to delete.
