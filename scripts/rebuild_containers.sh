#! /user/bin/env bash
set -x
set -eo pipefail

# docker ps -q lists the IDs of all currently running containers.
# docker stop $(docker ps -q) stops all running containers.
# docker ps -a -q lists the IDs of all containers, including stopped ones.
# docker rm $(docker ps -a -q) removes all containers.

# docker stop $(docker ps --filter 'name=redis' --format '{{.ID}}') && docker rm $(docker ps -a -q)
#>&2 echo "STOPPING docker containers for Traders"
#docker stop $(docker ps --filter 'name=redis' --format '{{.ID}}')
#docker stop $(docker ps --filter 'name=traders-psql-db' --format '{{.ID}}')
#>&2 echo "PRUNING stopped containers"
#docker container prune -f

# docker ps -a: Lists all containers, including those that are not running.
# grep 'foo\|bar': Filters the output to only include lines containing 'foo' or 'bar'. This will match any part of the container details that include 'foo' or 'bar', including the names 'foomie' and 'barmie'.
# awk '{print $1}': Extracts the first field from each line of the filtered output, which corresponds to the container ID.
# xargs -r docker rm: Takes the list of container IDs and passes them to docker rm to remove the containers. The -r option for xargs ensures that docker rm is only run if there are any inputs, preventing an error if no containers match the pattern.

>&2 echo "REMOVING all traders containers"
docker ps -a --format '{{.ID}} {{.Names}}' | grep 'traders_' | awk '{print $1}' | xargs -r docker rm -f

>&2 echo "STARTING start scripts for Traders"
./scripts/init_db.sh
./scripts/init_redis.sh

