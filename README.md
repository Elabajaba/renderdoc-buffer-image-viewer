For when you're stuck using storage buffers instead of textures because wgpu doesn't support read+write storage textures, 
and you still want to see what their contents are.

## Usage

  1. Generate a renderdoc capture
  2. Find the buffer you want to view in renderdoc
  3. Ensure that the buffer is properly formatted (decimal numbers).
  4. Save the buffer as a csv
  5. Run the program, making sure you pass in the correct dimensions and texture format
      - eg. `cargo run --release -- 1280x720 -i my_file.csv -t Bgra8` for a 720p buffer using a Bgra8 texture format

## Troubleshooting

#### Renderdoc isn't showing the buffer as decimal numbers

Ensure that the Format field under the buffer contents is filled in.
If it isn't automatically populated by renderdoc, try finding the buffer in the shader stage you used it in, and see if it's populated there.
![Renderdoc Buffer Format Image](/docs/images/renderdoc_format.png)

#### Can't open PPM files

On Windows, try using [ImageGlass](https://github.com/d2phap/ImageGlass)

## Known Issues

 - Everything is output as an 8 bit PPM file with no tonemapping
 - Alpha channel can be read in, but is currently ignored
 - Output filenames are currently saved with millisecond precision. This is probably fine, but precision can be increased if necessary.
 - Doesn't support raw renderdoc buffers (as in, ones that don't have their format information filled out in renderdoc)
 - Currently only supports floats for input
 - Limited surface format support
    - `--list-supported-texture-formats` shows the currently supported formats
    - see SupportedTextureFormats in [src/texture_format.rs](src/texture_format.rs)
 - Buffer dimensions have to be passed in at runtime
