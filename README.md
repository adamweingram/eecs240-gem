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

## To Run Spectre Experiment
1. Build the project, just as before.
2. Run the following command instead of the experiments
  ```bash
  /root/gem5/build/X86/gem5.opt -d /root/SpectrePoC/sim_output /root/gem5/configs/example/se.py --cmd /root/SpectrePoC/spectre.out --cpu-type=O3CPU --caches --l2cache --l1d_size=32kB --l1i_size=32kB --l2_size=512kB --l1d_assoc=4 --l1i_assoc=4 --l2_assoc=1 --cacheline_size=64
  ```
  (Note that you can use `--maxinsts 1000000000`, if you really want)
