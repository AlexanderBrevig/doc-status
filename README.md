# Doc Status

![Current](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fgithub.com%2FAlexanderBrevig%2Fdoc-status%2Fraw%2Fmain%2F.doc-status.json&query=%24.files%5B%22README.md%22%5D.status&label=Current&color=white)

> [!WARNING]  
> This project is still in the early prototyping phases!
> If you want to join, feel free to open issues and PRs, but do not expect a stable API.

## What is this?

`doc-status` is a tool for helping maintainers keep their documentation up to date.
If a file has not been changed for an `interval` number of seconds, an issue will be created. When that issue is closed, its status is set to OK again.

## TODO

* [ ] make file types for scan configurable (.md, .org etc)
* [ ] refresh timestamps during check (if a change has been made, do not create issue)
* [ ] create a workflow for closing task => set timestamp and status ok
