build:
	@cd upper; cargo build --release;
	@cd lower; cargo build --release;
	@mkdir bin
	@cp upper/target/release/upper bin
	@cp lower/target/release/lower bin

clean:
	@rm -r bin
