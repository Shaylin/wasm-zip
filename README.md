# Doggy Bag

Generate a file archive blob to download and take away. No file system needed.

## Usage

## Supported Inputs

```json
{
  "MyStringTextFile.txt": "Hello World!",
  "MyJSONFile.json": {
    "message": "Hello Again!"
  },
  "FirstFolder": {
    "SecondFolder": {
      "AnotherJSONFile.json": {
        "name": "Doggy Bag",
        "count": 12
      }
    },
    "AnotherTextFile.txt": "I'm Inside The Folder!"
  }
}
```