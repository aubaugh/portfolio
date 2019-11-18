# Local Variables:
# mode: makefile
# End:
# vim: set ft=make :

output-dir := "austinsheep.github.io"

# Build portfolio
build:
    mkdir -p {{output-dir}}
    OUTPUT_CSS={{output-dir}}/app.css wasm-pack build --target web --no-typescript --out-dir {{output-dir}}
    rm {{output-dir}}/.gitignore
    cp index.html {{output-dir}}
