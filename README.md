# wasm-qrcode

A QR Code WebAssembly library implemented with Rust.

## Methods

`qrcode(data: string, width: number, height: number)`

<span style="display: inline-block; margin-left: 20px;">Generate the SVG format QR Code image with the given data and image widht/height.<span>

<br>

`qrcode_base64(data: string, width: number, height: number)`

<span style="display: inline-block; margin-left: 20px;">Generate a base64 encoded SVG QR Code image with the given data and image widht/height.<span>

## Example

```javascript
import * as wasm from "wasm-qrcode";

window.onload = function() {
  const qr_b64img = wasm.qrcode_base64(
    "https://github.com/yishiashia",
    36, 36
  );
  const img = document.createElement('img');

  img.src = `data:image/svg+xml;base64,${qr_b64img}`;
  img.alt = "QR Code";
  img.style.width = "100px";
  img.style.height = "100px";

  document.body.append(img);
}
```