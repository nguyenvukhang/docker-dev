IMAGE := ghcr.io/nguyenvukhang/uwuntu-cuda12.4.1-ubuntu22.04
CONTAINER := khang
ABSOLUTE_WORKDIR := /media/appliedai/ssd_nvme/khang
HOST_SSH_PORT         := 616
HOST_TENSORBOARD_PORT := 9106
HOST_JUPYTER_PORT     := 9188

current:
	@echo 'Makefiles!'

build:
	docker build -t $(IMAGE) .

R := --name $(CONTAINER)
R += --volume /media/appliedai/ssd_nvme:/mnt/shared
R += --volume $(ABSOLUTE_WORKDIR):/home/appliedai/v
R += -p $(HOST_SSH_PORT):22
R += -p $(HOST_TENSORBOARD_PORT):6006
R += -p $(HOST_JUPYTER_PORT):8888
R += --privileged
R += --runtime=nvidia
# R += --gpus all
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
