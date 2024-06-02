#!/bin/bash
podman build -f archive/Dockerfile . -t registry.impending.org:5050/archive:latest
podman build -f bazaar/Dockerfile . -t registry.impending.org:5050/bazaar:latest
podman build -f courtyard/Dockerfile . -t registry.impending.org:5050/courtyard:latest
podman build -f drawbridge/Dockerfile . -t registry.impending.org:5050/drawbridge:latest
podman build -f forum/Dockerfile . -t registry.impending.org:5050/forum:latest
podman build -f gate/Dockerfile . -t registry.impending.org:5050/gate:latest
podman build -f hall/Dockerfile . -t registry.impending.org:5050/hall:latest
podman build -f jail/Dockerfile . -t registry.impending.org:5050/jail:latest
podman build -f lookout/Dockerfile . -t registry.impending.org:5050/lookout:latest
podman build -f watchtower/Dockerfile . -t registry.impending.org:5050/watchtower:latest

podman push registry.impending.org:5050/archive:latest
podman push registry.impending.org:5050/bazaar:latest
podman push registry.impending.org:5050/courtyard:latest
podman push registry.impending.org:5050/drawbridge:latest
podman push registry.impending.org:5050/forum:latest
podman push registry.impending.org:5050/gate:latest
podman push registry.impending.org:5050/hall:latest
podman push registry.impending.org:5050/jail:latest
podman push registry.impending.org:5050/lookout:latest
podman push registry.impending.org:5050/watchtower:latest
