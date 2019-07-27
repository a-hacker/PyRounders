build:
	cargo build --release
	cp target/release/librounders.so py/rounders.so

startpy:
	source venv/bin/activate.fish
	cd py
	jupyter notebook

run: build py