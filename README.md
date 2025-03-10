# swc-plugin-remove-node-protocol&middot; [![npm version](https://img.shields.io/npm/v/swc-plugin-remove-node-protocol)](https://www.npmjs.com/package/swc-plugin-remove-node-protocol)

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
          "swc-plugin-remove-node-protocol"
        ]
      ]
    }
  }
}
```
Input Code:
```ts
    import fs from 'node:fs';
    import path from 'node:path';
    
    const dynamic = import('node:url');
    const fs = require('node:fs');
    const path = require.resolve('node:path');
    
    export * as fs from 'node:fs';
    export * from 'node:crypto';
    export { readFile } from 'node:fs';
    
    module.exports = { fs: require('node:fs') };
    module.exports.fs = require('node:fs');
    
    console.log('node:fs');
    fn('node:fs')
```
Output:  
``` ts
    import fs from 'fs';
    import path from 'path';
    
    const dynamic = import('url');
    const fs = require('fs');
    const path = require.resolve('path');
    
    export * as fs from 'fs';
    export * from 'crypto';
    export { readFile } from 'fs';

    module.exports = {fs: require('fs')};
    module.exports.fs = require('fs');

    console.log('node:fs');
    fn('node:fs');

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