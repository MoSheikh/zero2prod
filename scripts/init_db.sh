#!/usr/bin/bash

set -eo pipefail

if ! [ -x "$(command -v diesel)" ]; then
    echo &2> "Error: diesel is not installed."
    echo &2> "Install it by running the following:
		apt-get update && apt-get install libpq-dev
		cargo install diesel_cli"
    exit 1
fi

CONTAINER_NAME="postgres"
if [ "$(docker container inspect $CONTAINER_NAME 2> /dev/null)" != "[]" ]; then
   echo "Stopping existing container"
   docker rm -f $CONTAINER_NAME
fi

DB_PORT=${POSTGRES_PORT:=5432}

SUPERUSER=${SUPERUSER:=postgres}
SUPERUSER_PWD=${SUPERUSER_PWD:=postgres}

APP_USER=${APP_USER:=newsletter}
APP_USER_PWD=${APP_USER_PWD:=secret}
APP_DATABASE=${APP_DATABASE:=newsletter}

docker run                                                 \
       --env POSTGRES_USER=$SUPERUSER                      \
       --env POSTGRES_PASSWORD=$SUPERUSER_PWD              \
       --env POSTGRES_DB=$APP_DATABASE                     \
       --health-cmd "pg_isready -U $SUPERUSER || exit 1"   \
       --health-interval 1s                                \
       --health-timeout 5s 				   \
       --health-retries 5 				   \
       --publish $DB_PORT:5432 				   \
       --detach 					   \
       --name $CONTAINER_NAME				   \
       postgres -N 1000

until [ \
      "$(docker inspect -f {{.State.Health.Status}} $CONTAINER_NAME 2> /dev/null)" == "healthy" \
]; do
   >&2 echo "Postgres is still unavailable - sleeping"
   sleep 1
done

>&2 echo "Initializing Postgres instance..."

CREATE_QUERY="CREATE USER ${APP_USER} WITH PASSWORD '${APP_USER_PWD}';"
docker exec -it "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -c "${CREATE_QUERY}"

GRANT_QUERY="ALTER DATABASE ${APP_DATABASE} OWNER TO ${APP_USER};";
docker exec -it "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -c "${GRANT_QUERY}"

EXTENSION_QUERY='CREATE EXTENSION "uuid-ossp"';
docker exec -it "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -d ${APP_DATABASE} -c "${EXTENSION_QUERY}"

echo "Postgres is now running and available on port $DB_PORT."

DATABASE_URL=postgres://${APP_USER}:${APP_USER_PWD}@localhost:${DB_PORT}/${APP_DATABASE}
export DATABASE_URL
diesel setup
