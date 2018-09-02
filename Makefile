
.PHONY: build/* debug deploy

build/debug:
	@time docker build -f ./debug.dockerfile -t cosworth/debug:latest .

build/deploy:
	@time docker build -f ./deploy.dockerfile --target build \
		-t cosworth/build:latest .
	@time docker build -f ./deploy.dockerfile --target deploy \
		-t cosworth/deploy:latest .

build/cleanup:
	docker images -qa -f "dangling=true" | xargs -I {} docker rmi {}

debug: build/debug build/cleanup
	docker run -it --rm -p 8080:8080 cosworth/debug:latest

deploy: build/deploy build/cleanup
	docker run -it --rm -p 8080:8080 cosworth/deploy:latest
