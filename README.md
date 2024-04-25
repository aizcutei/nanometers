# nanometers

Crossplatform audio visualizer with Rust & WebGPU.

A practice that mimics but not to replace the Minimeters.

Under development. Unstable. Not ready for use. **Not responsible for loss of project files due to plugin crashes.**

Use together with [nanometers-plugin](https://github.com/aizcutei/nanometers_plug) (VST3 & CLAP).

### Testing locally

Make sure you are using the latest version of stable rust by running `rustup update`.

`cargo run --release`

On Linux you need to first run:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`

On Fedora Rawhide you need to run:

`dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel`