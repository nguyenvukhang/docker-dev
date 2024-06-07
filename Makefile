IMAGE := ghcr.io/nguyenvukhang/cuda-ubuntu
CONTAINER := khang2
ABSOLUTE_WORKDIR := /home/khang/docker-workdir2
ABSOLUTE_WORKDIR := /home/appliedai/Documents/khangs-docker-volume
HOST_SSH_PORT := 617

current:
	@echo 'Makefiles!'

build:
	docker build -t $(IMAGE) .

R := --name $(CONTAINER)
R += --volume /mnt/md0/weijie:/mnt/khang
R += --volume $(ABSOLUTE_WORKDIR):/home/appliedai/v
R += -p $(HOST_SSH_PORT):22
R += --gpus all
R += --detach
R += --tty
RUN_ARGS := $(R)

run:
	docker run $(RUN_ARGS) $(IMAGE)

start:
	docker start $(CONTAINER)

stop:
	docker stop $(CONTAINER)

kill:
	-docker kill $(CONTAINER)

rm-container: kill
	-docker rm $(CONTAINER)

rm-image:
	-docker rmi -f $(IMAGE)

attach:
	docker exec --user appliedai --workdir /home/appliedai -it $(CONTAINER) zsh
