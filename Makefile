
copy:
	scp -i key.pem *.sh ubuntu@ec2-54-247-35-209.eu-west-1.compute.amazonaws.com
	scp -i key.pem *.conf ubuntu@ec2-54-247-35-209.eu-west-1.compute.amazonaws.com:/etc/nginx/conf.d/

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