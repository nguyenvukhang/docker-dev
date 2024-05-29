IMAGE := ghcr.io/nguyenvukhang/cuda-ubuntu
CONTAINER := khang
ABSOLUTE_WORKDIR := /home/appliedai/Documents/khangs-docker-volume-for-work
HOST_PORT := 616

current:
	@echo 'Makefiles!'

build:
	docker build -t $(IMAGE) .

R := --name $(CONTAINER)
R += --volume /mnt/md0/weijie:/mnt/khang
R += --volume $(ABSOLUTE_WORKDIR):/home/appliedai/repos
R += -p $(HOST_PORT):22
R += --gpus all
R += --detach
R += --tty
RUN_ARGS := $(R)

run: rm-container
	docker run $(RUN_ARGS) $(IMAGE)

start:
	docker start $(CONTAINER)

kill:
	-docker kill $(CONTAINER)

rm-container: kill
	-docker rm $(CONTAINER)

rm-image:
	-docker rmi -f $(IMAGE)

attach:
	docker exec -it $(CONTAINER) zsh
