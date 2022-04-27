# Background Information

The purpose of this document is to present the research done in making certain design decisions. It also serves as a summary for the subset of the zip specification that this library uses.

## High Level Motivation

The idea behind the Doggy Bag is to offer web applications the ability to allow users to download an archive of resources that are already present on the client, without the need to contact a server to generate an archive for download. This allows for increased privacy, lower costs and potentially simpler hosting architecture.

Presently, Doggy Bag creates zip archives without any compression applied. The zip file format was chosen due to its ubiquity across all operating systems. The absence of compression was a simplifying decision made due to the fact that there are no bandwidth costs associated with a download directly from the client.

## Zip Archive Creation

Zip archives are split into 2 sections:
1. The file entries.
2. The central directory.

A simplified layout of a zip archive is shown below:

![](diagrams/zip_structure.png)

### File Entries



### Central Directory


## References
1. PKWARE Inc., ".ZIP File Format Specification", `https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT`, 2020.