# Do Pls

A simple command line tool to open your projects in editors(currently only supports `nvim` :) ) without leaving the root directory.

## Installation

```
git clone https://github.com/srj31/dopls.git
cd ./dopls
cargo install --path .
```

## Usage

- To alias the current directory as name1

```
dopls add name1 .
```

- To see the list of aliases saved

```
$ dopls list

name1   /Users/srj31/Documents/dopls
```

- Open the directory in the editor

```
$ dopls code name1
```
