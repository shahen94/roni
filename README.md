# <b>Roni</b> - A command-line tool for managing your projects


- [Roni - A command-line tool for managing your projects](#roni---a-command-line-tool-for-managing-your-projects)
- [](#)
  - [Purpose](#purpose)
- [](#-1)
    - [Usage](#usage)
    - [Add new project](#add-new-project)
    - [Print all projects](#print-all-projects)
    - [Print one project configuration](#print-one-project-configuration)
    - [Add files to the project](#add-files-to-the-project)
    - [Unlink file from the project](#unlink-file-from-the-project)
    - [Remove project and all files connected to it](#remove-project-and-all-files-connected-to-it)

#
## Purpose
When working on multiple projects, it's easy to spread files related to it(shell scripts that should not be in the repo, config files, etc.). And when finishing work on the project, these files are easy to forget to remove.

And here is where **Roni** can help
#

### Usage

Roni Help
```sh
$ roni
```

### Add new project
```sh
$ roni add react-native-video-processing
```

### Print all projects
```sh
$ roni print
```

### Print one project configuration
```sh
$ roni print react-native-video-processing
```

### Add files to the project
```sh
$ roni link react-native-video-processing ~/Projects/personal/rn-p/docker.compose.yml
```

If we'll print project again, new file will be added there

### Unlink file from the project
```sh
$ roni unlink react-native-video-processing ~/Projects/personal/rn-p/docker.compose.yml
```

### Remove project and all files connected to it
```sh
$ roni remove react-native-video-processing
```