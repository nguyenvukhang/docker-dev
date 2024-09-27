IMAGE            := ghcr.io/nguyenvukhang/uwuntu-cuda12.5.0-ubuntu22.04
CONTAINER        := cachelib
ABSOLUTE_WORKDIR := /media/appliedai/ssd_nvme/khang

HOST_SSH_PORT         := 617
HOST_TENSORBOARD_PORT := 10106
HOST_JUPYTER_PORT     := 10188

current:
	@echo 'Makefiles!'

build:
	docker build -t $(IMAGE) .


R := --name $(CONTAINER)

# volume mounts (HOST:CONTAINER)
R += -v /media/appliedai/ssd_nvme:/mnt/shared
R += -v $(ABSOLUTE_WORKDIR):/home/appliedai/v
R += -v /var/run/docker.sock:/var/run/docker.sock

# port links (HOST:CONTAINER)
R += -p $(HOST_SSH_PORT):22
R += -p $(HOST_TENSORBOARD_PORT):6006
R += -p $(HOST_JUPYTER_PORT):8888

R += --privileged
R += --runtime=nvidia
R += --gpus=all
R += --detach
R += --tty
R += --restart=always

RUN_ARGS := $(R)

# Creates a new container with the configuration.
run:
	docker run $(RUN_ARGS) $(IMAGE)

# Runs a container that already exists.
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
