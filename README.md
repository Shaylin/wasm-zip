# Wasm-Zip

## Introduction

Wasm-Zip is a Web Assembly library that allows you to generate a zip archive directly within memory in your browser. No
servers or filesystem access required.

The bulk of the heavy lifting is done using the Rust programming language, with some input pre-processing handled by
JavaScript.

This library is packaged for inclusion in any JavaScript or TypeScript browser project
by [wasm-pack](https://rustwasm.github.io/wasm-pack/).

Potential use cases include:

1. Packaging up user generated assets, such as drawings or files already loaded by the client, as downloadable zip.
   Removing the need for a server to process these files.
2. Pre-packaging multiple files on the client as a single zip file before sending it to a server on your back end,
   effectively reducing the amount of requests needed.
3. Packaging files from multiple remote sources as single convenient download for users. This can be used to simplify
   hosting requirements as even static web pages can package files.

To view an example project, [Click Here](https://gitlab.com/Shaylin/wasm-zip-example).

For more background information, see [BACKGROUND.md](https://gitlab.com/Shaylin/wasm-zip/-/blob/main/BACKGROUND.md).

To get started with development, see [DEVELOPING.md](https://gitlab.com/Shaylin/wasm-zip/-/blob/main/DEVELOPING.md).

## Usage

### Installation

```
npm install wasm-zip --save
```

### API

Wasm-Zip accepts a JSON object as an input that represents a mapping between file names and file contents. Folders are
represented by nesting objects. Strings and Uint8Arrays are accepted as file contents data types.

Example Input:

```javascript
const directoryMapping = {
    "MyFile.txt": "Hello World!",
    "MyFolder": {
        "FileInsideFolder.json": JSON.stringify({isInsideFolder: true, someData: 2})
    }
};
```

This input object is then supplied to the library where it is processed and single binary zip file representing a
zip file is returned to the caller.

```rust
pub fn generate_zip_binary(zip_contents: Object) -> Box<[u8]>
```

### Multi-File JavaScript Example

```javascript
import * as wasm from "wasm-zip";

const imageRequest = new Request("https://picsum.photos/640");

const reportData = {
    userName: "Wasm-Zip",
    id: 2231,
    url: "https://gitlab.com/Shaylin/wasm-zip"
};

fetch(imageRequest).then(async (response) => {
    let imageArrayBuffer = await response.arrayBuffer();

    const directoryMapping = {
        "MyRandomImage.jpeg": new Uint8Array(imageArrayBuffer),
        "ReportFolder": {
            "ZippedReport.json": JSON.stringify(reportData)
        }
    }

    //Generate a zip blob from the retrieved image and report data
    const zipBinary = wasm.generate_zip_binary(directoryMapping);
    const downloadableBlob = new Blob([zipBinary], {
        type: "application/zip;charset=utf-8"
    });
});
```

### Downloading The Generated Zip From The Browser

Having already generated a zip blob in the previous example, you may create an object URL for a user to download the zip
file.

```javascript
const downloadableBlob = new Blob([zipBinary], {
    type: "application/zip;charset=utf-8"
});

let blobURL = window.URL.createObjectURL(downloadableBlob);

//Upon clicking the <a> element within the HTML, the zip file will be downloaded.
document.getElementById("MyDownloadLink").href = blobURL;

//IMPORTANT - Object URLs must be revoked after use to avoid memory leaks.
window.URL.revokeObjectURL(blobURL);
```

## Limitations

1. Only zip stored archives are supported at this moment. No compression options are available. This was a simplifying
   design decision since there is no bandwidth cost associated with downloading a blob from within your own client
   memory.
2. The permitted maximum blob size for web browsers typically does not exceed a few hundred MiB. The exact number varies
   per browser.