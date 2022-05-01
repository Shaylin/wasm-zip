# Background Information

The purpose of this document is to present the research done in making certain design decisions. It also serves as a
summary for the subset of the zip specification that this library uses.

## High Level Motivation

The idea behind the Doggy Bag is to offer web applications the ability to allow users to download an archive of
resources that are already present on the client, without the need to contact a server to generate an archive for
download. This allows for increased privacy, lower costs and potentially simpler hosting architecture.

Presently, Doggy Bag creates zip archives without any compression applied. The zip file format was chosen due to its
ubiquity across all operating systems. The absence of compression was a simplifying decision made due to the fact that
there are no bandwidth costs associated with a download directly from the client.

## Zip Archive Creation

Zip archives are split into 2 sections:

1. The file entries.
2. The central directory.

A simplified layout of a zip archive is shown below:

![](diagrams/zip_structure.png)

Doggy Bag produces a binary blob by concatenating rust vectors that represent each of the sections in the above diagram.
While not the most efficient approach, it is significantly more declarative and easy to ensure correctness.

It is important to note that header fields are typically stored in little-endian byte ordering.
This means within a field of data in a zip file header, the least significant byte is stored first.
For example, the local file signature 0x04034B50 would be stored as [0x50, 0x4B, 0x03, 0x04] within the first 4 bytes of memory.

### File Entry Structure

#### Local File Header

| Field Offset | Size (bytes) | Description                                                                                                                                                                                                                               |
|--------------|--------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 0            | 4            | Local file header signature. Always set to 0x04034b50.                                                                                                                                                                                    |
| 4            | 2            | Minimum version needed to extract the file. Set to 0x000A by Doggy Bag.                                                                                                                                                                   |
| 6            | 2            | General purpose bit flag. Used to for additional features such as encryption. Doggy Bag does not utilise these features. Therefore, it is set to 0x0000.                                                                                  |
| 8            | 2            | Compression method. Doggy Bag sets this to 0x0000 to indicate that no compression is used. Data is simply stored.                                                                                                                         |
| 10           | 2            | File last modified time in MS-DOS formatting [2].                                                                                                                                                                                         |
| 12           | 2            | File last modified date in MS-DOS formatting [2].                                                                                                                                                                                         |
| 14           | 4            | CRC-32 of the file data. Doggy Bag uses the ISO HDLC algorithm.                                                                                                                                                                           |
| 18           | 4            | Compressed size of the file data (without the file name). This is the same as the uncompressed size in the case of zips without compression.                                                                                              |
| 22           | 4            | Uncompressed size of the file data (without the file name).                                                                                                                                                                               |
| 26           | 2            | The length of the file name.                                                                                                                                                                                                              |
| 28           | 2            | The extra field length. Doggy Bag does not use extra fields. Therefore this is set to 0x0000.                                                                                                                                             |
| 30           | n            | The file name in bytes. Note that characters are stored in the same order they appear in the string (ignore endian-ness). Also note that folders are represented by including a forward slash in the file name eg. `MyFolder/MyFile.txt`. |
| 30 + n       | m            | Extra field. Unused by Doggy Bag, therefore completely omitted.                                                                                                                                                                           |

### Central Directory Structure

## References

1. PKWARE Inc., ".ZIP File Format Specification", `https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT`, 2020.
2. Microsoft Corporation, "DosDateTimeToFileTime function"
   , `https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-dosdatetimetofiletime?redirectedfrom=MSDN`,
   Apr, 2021.