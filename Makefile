comp:
	cargo build && ./osx_vst_bundler.sh CVOUT target/debug/liboscvst.dylib && rm -rf ~/Library/Audio/Plug-Ins/VST/CVOUT.vst && cp -r CVOUT.vst ~/Library/Audio/Plug-Ins/VST/
