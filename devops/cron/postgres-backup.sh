#!/bin/bash

#configuration settings
CURRENT_MONTH=$(date +%Y-%m)
CURRENT_DATE=$(date +%Y-%m-%d)
CURRENT_DATETIME=$(date +%d-%b-%Y_%H_%M_%Z)
BACKUPS_PATH=/backups
DOCKER_SWARM_SERVICE_NAME=climate_news_stack_db
S3_BACKUP_BUCKET=climate-news-db-backup

####################################
#backup PostgreSQL database
BACKUP_FOLDER=$BACKUPS_PATH/$CURRENT_MONTH/$CURRENT_DATE
if [ ! -d "$BACKUP_FOLDER" ]; then
    mkdir -p "$BACKUP_FOLDER"
fi

echo 'Creating PostgreSQL backup...'
cd "$BACKUP_FOLDER"
if [ -f 'dump_'"$POSTGRES_DB"'.sql' ]; then
   rm 'dump_'"$POSTGRES_DB"'.sql'
fi
db_backup_filename=$POSTGRES_DB'_'$CURRENT_DATETIME'.tar.gz'
postgres_container_id=$(docker service ps -f "name=$DOCKER_SWARM_SERVICE_NAME" $DOCKER_SWARM_SERVICE_NAME -q --no-trunc | head -n1)
docker exec -t $DOCKER_SWARM_SERVICE_NAME.1."$postgres_container_id" pg_dump -U $POSTGRES_USER $POSTGRES_DB > 'dump_'"$POSTGRES_DB"'.sql'
tar -cf - 'dump_'"$POSTGRES_DB"'.sql' | gzip -9 > "$db_backup_filename"
rm 'dump_'"$POSTGRES_DB"'.sql'

echo 'Uploading PostgreSQL backup to S3 bucket'
aws s3 cp $BACKUP_FOLDER/$db_backup_filename s3://$S3_BACKUP_BUCKET$BACKUP_FOLDER

cd "$BACKUP_FOLDER"
md5sum * > MD5SUMS

echo 'Done. '


##### Restore backup locally
# edit /etc/postgresql/12/main/pg_hba.conf to change the peer to md5 
# sudo service postgresql restart
# psql -U climate_action -d postgres
# DROP DATABASE climate_action;
# CREATE DATABASE climate_action;
# psql climate_action < dump_climate_news.sql -U climate_action
