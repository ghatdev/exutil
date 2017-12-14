build: clean bin
	@mkdir bin
	@cp target/release/upper bin
	@cp target/release/lower bin

clean:
	@rm -r bin

bin:
	@cargo build  --release

install: bin
	@cp target/release/upper /usr/local/bin
	@cp target/release/lower /usr/local/bin
