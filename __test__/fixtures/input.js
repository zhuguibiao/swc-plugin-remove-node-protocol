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

/** ignore */
console.log('node:fs');
fn('node:fs')