BINARY := "target/release/ransomware"

release:
	cargo build --release
	strip {{BINARY}}
	upx --best --lzma {{BINARY}}
	ls -lh {{BINARY}}
