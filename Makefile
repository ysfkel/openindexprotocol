
# Define the target
.PHONY: deploy_tv permit  test  test_validator build

permit:
	chmod +x ./deploy/*.sh

gen_new_program_key:
	solana-keygen new --outfile target/deploy/openindex-keypair.json

build:
	 cargo build-sbf

deploy_dev:
	./deploy/devnet.sh

deploy_tv:
	./deploy/test_validator.sh

test:
	cargo test --workspace -- --nocapture --test-threads 1 

test_validator:
	cargo test -p test-validator --features test-validator -- --nocapture
