# Do Pls

A simple command line tool to open your projects in editors(currently only supports `nvim` and `code`:) ) without leaving the root directory.

## Installation

```
git clone https://github.com/srj31/dopls.git
cd ./dopls
cargo install --path .
```

## Usage

- To alias the the directory ./project as name1

```
dopls add name1 ./project
```

- To see the list of aliases saved

```
$ dopls list

name1   /Users/srj31/Documents/dopls/project
```

- Open the directory in the editor

```
# to open in nvim
$ dopls code name1

# to open in vscode
$ dopls code name1 -c
```

The default editor is `nvim` and if you want to use another editor use the respective flags
