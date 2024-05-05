#!/bin/bash

authNetwork=`docker network ls | grep auth-net | wc -l`
authNetwork=$(($authNetwork + 0))

if [[ $authNetwork = "0" ]]
then
  docker network create --driver bridge auth-net
fi

# Run the Docker container
docker run -dit --rm -p 8810:8810 app/hashing-salting
