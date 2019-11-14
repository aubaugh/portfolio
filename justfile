# Local Variables:
# mode: makefile
# End:
# vim: set ft=make :

# Build portfolio
build:
    mkdir -p build
    OUTPUT_CSS=build/app.css wasm-pack build --target web --no-typescript --out-dir build
    cp index.html build
