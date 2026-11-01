'use strict';

const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const zlib = require('node:zlib');

function crc32(buffer) {
  let crc = 0xffffffff;
  for (const byte of buffer) {
    crc ^= byte;
    for (let bit = 0; bit < 8; bit += 1) {
      crc = (crc >>> 1) ^ ((crc & 1) ? 0xedb88320 : 0);
    }
  }
  return (crc ^ 0xffffffff) >>> 0;
}

function pngChunk(type, data) {
  const typeBytes = Buffer.from(type, 'ascii');
  const length = Buffer.alloc(4);
  length.writeUInt32BE(data.length, 0);
  const checksum = Buffer.alloc(4);
  checksum.writeUInt32BE(crc32(Buffer.concat([typeBytes, data])), 0);
  return Buffer.concat([length, typeBytes, data, checksum]);
}

function makePng() {
  const width = 16;
  const height = 16;
  const header = Buffer.alloc(13);
  header.writeUInt32BE(width, 0);
  header.writeUInt32BE(height, 4);
  header.writeUInt8(8, 8);  // bit depth
  header.writeUInt8(6, 9);  // RGBA
  header.writeUInt8(0, 10);
  header.writeUInt8(0, 11);
  header.writeUInt8(0, 12);

  const rows = [];
  for (let y = 0; y < height; y += 1) {
    const row = Buffer.alloc(1 + width * 4);
    row[0] = 0; // PNG filter: None
    for (let x = 0; x < width; x += 1) {
      const gold = x >= 5 && x <= 10 && y >= 5 && y <= 10;
      const offset = 1 + x * 4;
      row[offset] = gold ? 217 : 95;
      row[offset + 1] = gold ? 177 : 64;
      row[offset + 2] = gold ? 62 : 48;
      row[offset + 3] = 255;
    }
    rows.push(row);
  }
  const imageData = zlib.deflateSync(Buffer.concat(rows));
  return Buffer.concat([
    Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]),
    pngChunk('IHDR', header),
    pngChunk('IDAT', imageData),
    pngChunk('IEND', Buffer.alloc(0)),
  ]);
}

function makeIco(png) {
  const header = Buffer.alloc(22);
  header.writeUInt16LE(0, 0);       // reserved
  header.writeUInt16LE(1, 2);       // icon
  header.writeUInt16LE(1, 4);       // one image
  header.writeUInt8(16, 6);         // width
  header.writeUInt8(16, 7);         // height
  header.writeUInt8(0, 8);          // palette
  header.writeUInt8(0, 9);          // reserved
  header.writeUInt16LE(1, 10);      // color planes
  header.writeUInt16LE(32, 12);     // bits per pixel
  header.writeUInt32LE(png.length, 14);
  header.writeUInt32LE(22, 18);     // image offset
  return Buffer.concat([header, png]);
}

const outputPath = path.join(__dirname, 'icons', 'icon.ico');
fs.mkdirSync(path.dirname(outputPath), { recursive: true });
const icon = makeIco(makePng());
fs.writeFileSync(outputPath, icon);
const digest = crypto.createHash('sha256').update(icon).digest('hex');
process.stdout.write(`Generated deterministic 16x16 Windows ICO: ${outputPath} (${digest})\n`);
