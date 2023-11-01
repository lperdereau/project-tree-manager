# project-tree-manager
[![Build and release](https://github.com/lperdereau/project-tree-manager/actions/workflows/release.yaml/badge.svg?branch=main)](https://github.com/lperdereau/project-tree-manager/actions/workflows/release.yaml)
[![semantic-release: angular](https://img.shields.io/badge/semantic--release-angular-e10079?logo=semantic-release)](https://github.com/semantic-release/semantic-release)


This project aims to clone all of your git repositories reporting to a config file.


# Run

You can project-tree-manager with this CLI:
```sh
ptm -c config.yaml
```

For exemple we have the next configuration file:
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
```

or in json

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
  }
]
```
