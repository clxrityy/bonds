# bonds

> [!IMPORTANT]
>
> #### <img src="./docs/content/assets/img/logo.svg" alt="bonds logo" width="16" height="16" /> [bonds.fyi](https://bonds.fyi)

A tool for creating and managing "bonds" between files and directories.

[![Core Crates.io Version](https://img.shields.io/crates/v/bonds-core?style=flat&label=bonds-core)](https://bonds.fyi/latest/api/bonds_core/) [![CLI Crates.io Version](https://img.shields.io/crates/v/bonds-cli?style=flat&label=bonds-cli)](https://bonds.fyi/latest/api/bonds_cli/)

> Build with [symlinks](https://en.wikipedia.org/wiki/Symbolic_link) and a [SQLite](https://www.sqlite.org/)
>> Inspired by [GNU Stow](https://www.gnu.org/software/stow/)

[![CI](https://github.com/clxrityy/bonds/actions/workflows/ci.yml/badge.svg)](https://github.com/clxrityy/bonds/actions/workflows/ci.yml) [![Documentation](https://img.shields.io/badge/docs-blue?logo=rust&logoColor=white)](https://bonds.fyi/latest/api/) [![License](https://img.shields.io/github/license/clxrityy/bonds.svg)](https://github.com/clxrityy/bonds/blob/master/LICENSE)

## Status

- [x] Basic core & CLI
  - [x] API documentation
  - [x] Library API
- [ ] GUI application
- [ ] Ecosystem & plugins

---

#### CLI examples

<table>
  <tr>
    <th>Command</th>
    <th>Description</th>
    <th>Example</th>
  </tr>
  <tr>
    <td>
      <code>bond list</code>
    </td>
    <td>List all bonds</td>
    <td>
      <a href=".github/img/bond-list.png" target="_blank">
        <img src=".github/img/bond-list.png" alt="list" width="100%" height="auto" />
      </a>
    </td>
  </tr>
  <tr>
    <td>
      <code>bond info &lt;id | name&gt;</code>
    </td>
    <td>Show bond details</td>
    <td>
      <a href=".github/img/bond-info.png" target="_blank">
        <img src=".github/img/bond-info.png" alt="info" width="100%" height="auto" />
      </a>
    </td>
  </tr>
  <tr>
    <td>
      <code>bond remove &lt;id | name&gt;</code>
    </td>
    <td>Remove a bond</td>
    <td>
      <a href=".github/img/bond-remove.png" target="_blank">
        <img src=".github/img/bond-remove.png" alt="remove" width="100%" height="auto" />
      </a>
    </td>
  </tr>
</table>
