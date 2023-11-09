# EECS 240 Gem5 Project 1 Docker
A docker environment for Project 1.

## Instructions

1. Clone (you know how to do this)
2. Build Dockerfile:
  ```bash
  docker compose build
  ```
3. Modify volumes, if necessary:
  By default, a volume is not attached. THIS MEANS THAT YOU WILL LOSE ALL PROGRESS UNLESS YOU MOUNT A VOLUME! You can edit the `volumes` part of the `docker-compose.yml`.
  ```bash
  $EDITOR docker-compose.yml
  ```
4. Run (this uses a tiny helper script so you don't have to remember the specific command):
  ```bash
  ./start-interactive.sh
  ```
5. Run bench:
  ```bash
  pushd bench/5xx.xxx_r/data
  bash run_bench
  ```
