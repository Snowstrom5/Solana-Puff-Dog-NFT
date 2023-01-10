#!/bin/bash

#configuration settings
DOCKER_SWARM_SERVICE_NAME=climate_news_stack_db
BACKUP_PATH=/backups/2023-01/2023-01-10
BACKUP_FILE=climate_news_10-Jan-2023_00_00_UTC.tar.gz

####################################
# Restore PostgreSQL database backup
# prints out commands to run

postgres_container_id=$(docker service ps -f "name=$DOCKER_SWARM_SERVICE_NAME" $DOCKER_SWARM_SERVICE_NAME -q --no-trunc | head -n1)
echo "commands to run:"
echo "tar -xf  $BACKUP_PATH/$BACKUP_FILE -C $BACKUP_PATH/"
echo "sudo docker exec -it $DOCKER_SWARM_SERVICE_NAME.1.$postgres_container_id /bin/bash"
echo "psql climate_news < $BACKUP_PATH/dump_climate_news.sql -U climate_news"



##### Restore backup locally
# edit /etc/postgresql/12/main/pg_hba.conf to change the peer to md5 
# sudo service postgresql restart
# psql -U climate_news -d postgres
# DROP DATABASE climate_news;
# CREATE DATABASE climate_news;
# psql climate_news < dump_climate_news.sql -U climate_news