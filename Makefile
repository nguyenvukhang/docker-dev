IMAGE := ghcr.io/nguyenvukhang/uwuntu-cuda12.4.1-ubuntu22.04
CONTAINER := khang
ABSOLUTE_WORKDIR := /media/appliedai/ssd_nvme/khang
HOST_SSH_PORT := 616

current:
	@echo 'Makefiles!'

build:
	docker build -t $(IMAGE) .

R := --name $(CONTAINER)
R += --volume /media/appliedai/ssd_nvme:/mnt/shared
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
