# Bonds | Command Line Interface

![Crates.io Size](https://img.shields.io/crates/size/bonds-cli?style=flat)

**Install the CLI globally:**

```bash
cargo install bonds-cli
```

---

## Commands

- [bond](#bond)
- [add](#add)
- [list](#list)
- [info](#info)
- [remove](#remove)
- [update](#update)
- [migrate](#migrate)
- [config](#config)
- [metadata](#metadata)

<table style="width:fit-content; border-collapse: collapse; text-align: left;">
  <thead>
    <tr>
      <th>Command</th>
      <th>Args</th>
      <th>Description</th>
      <th>Example(s)</th>
    </tr>
  </thead>
  <tbody style="background-color: transparent;">
    <tr id="bond">
      <td><code>bond</code></td>
      <td></td>
      <td>Main entry point, shows help if no subcommand is provided.</td>
      <td>
        <pre><code lang="bash">bond</code></pre>
        <pre><code lang="bash">bond --help</code></pre>
      </td>
    </tr>
    <tr id="add">
      <td><code>add</code></td>
      <td><code>&lt;source&gt;</code> <code>[--name &lt;name&gt;]</code></td>
      <td>Create a bond (symlink) from source to target.</td>
      <td>
        <pre><code lang="bash">bond add ~/projects/my-app</code></pre>
        <pre><code lang="bash">bond add ~/projects/my-app --name foo</code></pre>
      </td>
    </tr>
    <tr id="list">
      <td><code>list</code></td>
      <td></td>
      <td>List all bonds.</td>
      <td>
        <pre><code lang="bash">bond list</code></pre>
      </td>
    </tr>
    <tr id="info">
      <td><code>info</code></td>
      <td><code>&lt;name|id&gt;</code></td>
      <td>Show details about a specific bond.</td>
      <td>
        <pre><code lang="bash">bond info foo</code></pre>
      </td>
    </tr>
    <tr id="remove">
      <td><code>remove</code></td>
      <td><code>&lt;name|id&gt;</code> <code>[--with-target]</code></td>
      <td>Remove a bond, with optional target deletion.</td>
      <td>
        <pre><code lang="bash">bond remove foo</code></pre>
      </td>
    </tr>
    <tr id="update">
      <td><code>update</code></td>
      <td><code>&lt;name|id&gt;</code> <code>[--source &lt;new-source&gt;]</code> <code>[--target &lt;new-target&gt;]</code></td>
      <td>Update a bond's source and/or target.</td>
      <td>
        <pre><code lang="bash">bond update foo --source ~/projects/new-app</code></pre>
        <pre><code lang="bash">bond update foo --target ~/bonds/new-app</code></pre>
      </td>
    </tr>
    <tr id="migrate">
      <td><code>migrate</code></td>
      <td><code>&lt;name|id&gt;</code> <code>[new-target]</code></td>
      <td>Move existing bonds to a new target location (with auto-backup). Moves to the default target if none is provided.</td>
      <td>
        <pre><code lang="bash">bond migrate foo</code></pre>
        <pre><code lang="bash">bond migrate foo ~/new-bonds</code></pre>
      </td>
    </tr>
    <tr id="config">
      <td><code>config</code></td>
      <td><code>&lt;get|set&gt;</code> <code>&lt;key&gt;</code></td>
      <td>Get or set configuration values.</td>
      <td>
        <pre><code lang="bash">bond config set default ~/my-bonds</code></pre>
        <pre><code lang="bash">bond config get default</code></pre>
      </td>
    </tr>
    <tr id="metadata">
      <td><code>metadata</code></td>
      <td><code>&lt;get|set|remove&gt;</code> <code>&lt;name|id&gt;</code> <code>[key] [value]</code></td>
      <td>Read or modify metadata for a bond.</td>
      <td>
        <pre><code lang="bash">bond metadata get foo</code></pre>
        <pre><code lang="bash">bond metadata set foo project my-app</code></pre>
        <pre><code lang="bash">bond metadata get foo project</code></pre>
        <pre><code lang="bash">bond metadata remove foo project</code></pre>
      </td>
    </tr>
  </tbody>
</table>
