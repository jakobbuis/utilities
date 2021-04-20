# Utilities
> A personal, opinionated collection of useful tools for working in development

## Installation
1. Clone the repository to your homefolder.
1. Add the folder to your path. On a Mac, open `/etc/paths` and add `/Users/<yourusername>/utilities` as the very first entry.

## Assumptions
These utilities, especially the `vagrant_*` commands, make a couple of assumptions on how your system is organised:
1. Your projects are all `~/code/*`. For example, a project named "waterbus" on my PC would be in `/Users/jakobbuis/code/waterbus`.
1. All projects have their web roots in `./code/{name}/public`. Moving the web root will require manual tuning.
1. You have a single Homestead installation for all projects, and it's located in `~/homestead`.
1. You have DNSMasq or similar installed and all your projects are reachable over `http://{name}.test`.

## Available utilities

### Finish a release branch
```bash
rf
```
Fetches changes, and merges and pushes a finished release branch to the appropriate targets.

### Start a release branch
```bash
rs
```
Fetches changes, and starts a release branch with the right number automatically.

### Bring master and develop up to date
```bash
update_local
```
Fetches changes, and brings your develop and master branch up to date.

### Ensure vagrant is running
```bash
vagrant_ensure_up
```
Restarts your vagrant machine to ensure that it is running.

### Open a vagrant connection from anywhere
```bash
vagrant_open
```
Opens a ssh-connection to your default homestead install from any directory. Once logged-in to the vagrant machine, this will immediately run `cd code/{dirname}` where dirname is the name of the current folder you're in.

### Provision a vagrant site
```bash
vagrant_provision [name]
```
Update Laravel Homestead configuration for the project, and open it in your browser.

### Start a new feature
```bash
vfs [ticket number] [description]
vfs [ticket number]
```
Fetches changes, and creates a new feature branch using common conventions. The description is optional, and must be in kebab-case without the leading dash.
