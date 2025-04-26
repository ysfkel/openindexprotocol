
# Define the target
.PHONY: deploy_tv permit  test  test_validator  

permit:
	chmod +x ./deploy/*.sh

deploy_tv:
	./deploy/test_validator.sh

test:
	cargo test --workspace -- --nocapture

test_validator:
	cargo test -p test-validator --features test-validator -- --nocapture