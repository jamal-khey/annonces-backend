run:
	./target/debug/rust-server-bin
build:
	cargo build
dev:
	systemfd --no-pid -s http::5000 -- cargo watch -x run

deploy:
	make bundle
	make upload_bundle
	make upload_deploy_script
	make execute_deploy_script

bundle:
	zip -r bundle-rust.zip . -x target/\* -x .git/\* -x .next/\* -x bundle-rust.zip -x .env -x *.pem

upload_bundle:
	scp -i key.pem bundle-rust.zip ubuntu@${HOST}:

upload_deploy_script:
	scp -i key.pem deploy-rust.sh ubuntu@${HOST}:

execute_deploy_script:
	ssh -i key.pem ubuntu@${HOST} "bash deploy-rust.sh"