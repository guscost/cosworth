
.PHONY: build* debug* deploy clean

# Build and deploy targets
build/debug:
	docker build -f ./debug/Dockerfile -t cosworth-example/debug:latest .

build/deploy:
	docker build -f ./build/Dockerfile --target build \
		-t cosworth-example/build:latest .
	docker build -f ./build/Dockerfile --target deploy \
		-t cosworth-example/deploy:latest .

build/cleanup:
	docker images -qa -f "dangling=true" | xargs -I {} docker rmi {}

deploy: build/deploy build/cleanup
	docker run -it --rm -p 8080:8080 cosworth-example/deploy:latest

# Debug targets
debug/network:
	-docker network create cosworth-example

debug/database: debug/network
	docker build -f ./database/Dockerfile -t cosworth-example/database:latest .
	-docker run -d --rm \
		--network cosworth-example \
		--name cosworth-example-database \
		cosworth-example/database:latest

debug/migrate: command=sh -c "/root/.cargo/bin/diesel migration --migration-dir ./src/migrations run"
debug/migrate: debug

debug: build/debug build/cleanup debug/network debug/database
	docker run -it --rm \
		--network cosworth-example \
		--name cosworth-example \
		-e DATABASE_URL=postgres://cosworth-example-database/cosworth_example \
		-v $(shell pwd)/src:/build/src \
		-p 8080:8080 \
		cosworth-example/debug:latest $(command)

# Clean target
clean:
	docker ps -a \
		| awk -F' +' '/cosworth-example/{print $$1}' \
		| xargs -I {} docker stop {} \
		| xargs -I {} docker rm {}
	docker images \
		| awk -F' +' '/cosworth-example/{print $$1":"$$2}' \
		| xargs -I {} docker rmi {}
