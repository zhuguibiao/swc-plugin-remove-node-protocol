# swc-plugin-remove-node-protocol
An SWC plugin to remove "node:" 

## Installation
Install with your favorite package manager as devDependency.

```bash
npm i -D swc-plugin-remove-node-protocol

or pnpm add -D swc-plugin-remove-node-protocol 

or yarn add -D swc-plugin-remove-node-protocol 
```
You can check the compatibility of versions on https://plugins.swc.rs/

## Usage
Via `.swcrc`

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        [
          "swc-plugin-remove-node-protocol",
        ]
      ]
    }
  }
}
```

## Dev
environment:
- [swc](https://swc.rs/docs/plugin/ecmascript/getting-started)
- [rust](https://doc.rust-lang.org/book/)
```
pnpm install

pnpm run prepack 

pnpm run test

cargo test
```