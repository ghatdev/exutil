build: clean bin
	@mkdir bin
	@cp upper/target/release/upper bin
	@cp lower/target/release/lower bin

clean:
	@rm -r bin

bin:
	@cd upper; cargo build --release;
	@cd lower; cargo build --release;

