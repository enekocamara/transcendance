COMPOSE_FILE=src/docker-compose.yml

build:
	docker compose -f $(COMPOSE_FILE) build

run: build
	docker compose -f $(COMPOSE_FILE) up -d

status:
	docker ps -a

stop:
	docker compose -f $(COMPOSE_FILE) down

clean:
	docker compose -f $(COMPOSE_FILE) down
	docker system prune -f

cleanVolumens:
	docker volume ls -q
	docker volume rm --force $$(docker volume ls -q)
	rm -rf /home/ecamara/dataTranscendance/backend/*
	rm -rf /home/ecamara/dataTranscendance/postgre/*
re: clean cleanVolumens run  status

