.PHONY:
dev: docker-compose-up run-migrations
	# done;

.PHONY:
docker-compose-up:
	# running docker compose up;
	docker compose up -d;
	# giving time for docker-compose containers to finish starting;
	sleep 15;

.PHONY:
run-migrations:
	# running migrations
	sqlx migrate run