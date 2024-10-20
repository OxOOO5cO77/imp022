#!/bin/bash

if [ -z "${REGISTRY}" ]; then
  echo "REGISTRY is unset"
  exit
fi

docker buildx build -f archive/Dockerfile . -t "$REGISTRY/archive:latest"
docker buildx build -f bazaar/Dockerfile . -t "$REGISTRY/bazaar:latest"
docker buildx build -f courtyard/Dockerfile . -t "$REGISTRY/courtyard:latest"
docker buildx build -f drawbridge/Dockerfile . -t "$REGISTRY/drawbridge:latest"
docker buildx build -f forum/Dockerfile . -t "$REGISTRY/forum:latest"
docker buildx build -f gate/Dockerfile . -t "$REGISTRY/gate:latest"
docker buildx build -f hall/Dockerfile . -t "$REGISTRY/hall:latest"
docker buildx build -f jail/Dockerfile . -t "$REGISTRY/jail:latest"
docker buildx build -f lookout/Dockerfile . -t "$REGISTRY/lookout:latest"
docker buildx build -f warehouse/Dockerfile . -t "$REGISTRY/warehouse:latest"

docker push "$REGISTRY/archive:latest"
docker push "$REGISTRY/bazaar:latest"
docker push "$REGISTRY/courtyard:latest"
docker push "$REGISTRY/drawbridge:latest"
docker push "$REGISTRY/forum:latest"
docker push "$REGISTRY/gate:latest"
docker push "$REGISTRY/hall:latest"
docker push "$REGISTRY/jail:latest"
docker push "$REGISTRY/lookout:latest"
docker push "$REGISTRY/warehouse:latest"
