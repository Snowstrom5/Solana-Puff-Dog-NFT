#!/bin/bash
#configuration settings
POSTGRES_USER=dbusername
POSTGRES_DB=dbname
CURRENT_MONTH=$(date +%Y-%m)
CURRENT_DATE=$(date +%Y-%m-%d)
CURRENT_DATETIME=$(date +%d-%b-%Y_%H_%M_%Z)
BACKUPS_PATH=/backups
DOCKER_SWARM_SERVICE_NAME=swarm_postgres
####################################
#backup PostgreSQL database
BACKUP_FOLDER=$BACKUPS_PATH/$CURRENT_MONTH/$CURRENT_DATE
if [ ! -d "$BACKUP_FOLDER" ]; then
    mkdir -p "$BACKUP_FOLDER"
fi

echo 'Creating PostgreSQL backups...'
cd "$BACKUP_FOLDER"
if [ -f 'dump_'"$POSTGRES_DB"'.sql' ]; then
   rm 'dump_'"$POSTGRES_DB"'.sql'
fi
db_backup_filename=$POSTGRES_DB'_'$CURRENT_DATETIME'.tar.gz'
postgres_container_id=$(docker service ps -f "name=$DOCKER_SWARM_SERVICE_NAME" $DOCKER_SWARM_SERVICE_NAME -q --no-trunc | head -n1)
docker exec -t symfony-blog_postgres.1."$postgres_container_id" pg_dump -U $POSTGRES_USER $POSTGRES_DB > 'dump_'"$POSTGRES_DB"'.sql'
tar -cf - 'dump_'"$POSTGRES_DB"'.sql' | gzip -9 > "$db_backup_filename"
rm 'dump_'"$POSTGRES_DB"'.sql'

cd "$BACKUP_FOLDER"
md5sum * > MD5SUMS

echo 'Done.'