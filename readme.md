# Utilities
> A personal, opinionated collection of useful tools for working in development at IN10

## Installation
1. Clone the repository to your homefolder: `git clone git@github.com:IN10/utilities.git ~/utilities`.
1. Add the folder to your path. On a Mac, open `/etc/paths` and add `/Users/<yourusername>/utilities` as the very first entry.

## Available utilities
1. `deploy_production [name]` Login to the production account, update the code, composer and migrations.
1. `deliver [source] [target]` Checkout the source branch, merge its origin; checkout target branch, merge its origin. Rebase source unto target and push everything.
1. `provision [name]` Update Laravel Homestead configuration for the project, and open it in your browser.
