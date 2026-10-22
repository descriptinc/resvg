Local patches to usvg 0.48.1

Based on the crates.io 0.48.1 source, licensed under MIT or Apache-2.0.

Changes cover invalid SVG point lists, CSS dash-array validation and cascade, CSS transforms on paint servers, and filter primitive default regions. Paint-server transforms accept absolute CSS lengths and angles; relative lengths and percentages remain unsupported.

The parsed stroke retains `vector-effect="non-scaling-stroke"` through serialization and accounts for it in bounds. The NME scene renderer consumes this flag; the unmodified resvg rasterizer does not.

An optional raster-glyph provider embeds browser-rendered emoji in the parsed text tree, preserving grapheme boundaries, bidi order, SVG positioning, clipping, and opacity. Native callers retain font-based emoji rendering.

Parser and CPU rendering regressions live in render-core/src/raster/svg_*_test.rs.
