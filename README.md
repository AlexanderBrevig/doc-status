# README

![Current](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fgithub.com%2FAlexanderBrevig%2Fdoc-status%2Fraw%2Fmain%2F.doc-status.json&query=%24.files%5B%22README.md%22%5D.status&label=Current&color=white)

## Notes

* tool scans all markdown files (configurable)
* if a doc has badge, it _must_ have status OK

This means that if a document in repo has a badge, but is not kept up to date, then an issue should be crated

* a cron based job for checking status
* for all broken statuses find or create the issue for that
* when issue is closed, trigger workflow that fixes that article in the statuses
