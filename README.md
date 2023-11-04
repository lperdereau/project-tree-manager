# project-tree-manager

[![Build and release](https://github.com/lperdereau/project-tree-manager/actions/workflows/release.yaml/badge.svg?branch=main)](https://github.com/lperdereau/project-tree-manager/actions/workflows/release.yaml)
[![semantic-release: angular](https://img.shields.io/badge/semantic--release-angular-e10079?logo=semantic-release)](https://github.com/semantic-release/semantic-release)

This project aims to clone all of your git repositories reporting to a config file.

## Install

Go to [project release](https://github.com/lperdereau/project-tree-manager/releases)

Download appropriate compressed assets regarding to your environment.

For MacOS 🍎 and Linux 🐧:

```sh
OS=linux
ARCH=x86_64
xz -d project-tree-manager-$OS-$ARCH.xz
mv project-tree-manager-$OS-$ARCH ~/bin/ptm
chmod +x ~/bin/ptm
```

Coming soon for windows…

## Run

You can project-tree-manager with this CLI:

```sh
ptm -c config.yaml -f ./dest/
```

## Configuration

Bellow two samples who generate this project tree:

```
.
├── projects
│   └── github.com
│       ├── lperdereau
│       │   └── project-tree-manager
│       └── kubernetes
│           └── kubernetes
└── docs.rs
```

<details>
  <summary class="link">YAML Config File</summary>

```yaml
---
- name: projects
  kind: folder
  childs:
    - name: github.com
      kind: folder
      childs:
        - name: lperdereau
          kind: folder
          childs:
            - name: project-tree-manager
              kind: project
              src: https://github.com/lperdereau/project-tree-manager.git
        - name: kubernetes
          kind: folder
          childs:
            - name: kubernetes
              kind: project
              src: https://github.com/kubernetes/kubernetes.git
- name: docs.rs
  kind: project
  src: https://github.com/rust-lang/book.git
```

</details>

<details>
  <summary class="link">JSON Config File</summary>

```json
[
  {
    "name": "projects",
    "kind": "folder",
    "childs": [
      {
        "name": "github.com",
        "kind": "folder",
        "childs": [
          {
            "name": "lperdereau",
            "kind": "folder",
            "childs": [
              {
                "name": "project-tree-manager",
                "kind": "project",
                "src": "https://github.com/lperdereau/project-tree-manager.git"
              }
            ]
          },
          {
            "name": "kubernetes",
            "kind": "folder",
            "childs": [
              {
                "name": "kubernetes",
                "kind": "project",
                "src": "https://github.com/kubernetes/kubernetes.git"
              }
            ]
          }
        ]
      }
    ]
  },
  {
    "name": "doc.rs",
    "kind": "project",
    "src": "https://github.com/rust-lang/book.git"
  }
]
```

<details>

<style>
.link {
  cursor: pointer;
  color: #67A7FF;
}

.link:hover {
  color: #8BB8F8;
  text-decoration: underline;
}
</style>
