
# WORK IN PROGRESS - REPOSITORY


This is a rust implementation of the iotmonitor project. 



## GNU eabihf cross compilation (on raspian manner)

	# build the image
	cd docker
	docker build -t rscross .
	cd ..

	# cross compile
	docker run -ti -v `pwd`:/code rscross cargo build


	docker run --rm -ti -v `pwd`:/code rscross bash -c "cargo build --target=armv7-unknown-linux-gnueabihf"



## X64 with static glibc

Command line to statically compile glibc into the exe

	RUSTFLAGS="-C target-feature=+crt-static" cargo build --target x86_64-unknown-linux-gnu





### MUSL COMPILE EVALUATION 


	target for old glibc

	info: downloading component 'rust-std' for 'armv7-unknown-linux-musleabihf'

	install musl-dev musl-tools packages

	docker run --rm -ti -v `pwd`:/code rscross bash -c "CC=/usr/bin/musl-gcc cargo build --target=armv7-unknown-linux-musleabihf"


	CC=/usr/bin/musl-gcc cargo build --target=armv7-unknown-linux-musleabihf


