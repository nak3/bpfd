#!/bin/bash

docker build \
 --build-arg PROGRAM_NAME=xdp_counter \
 --build-arg BPF_FUNCTION_NAME=xdp_stats \
 --build-arg PROGRAM_TYPE=xdp \
 --build-arg BYTECODE_FILENAME=bpf_bpfel.o \
 -f ../packaging/container-deployment/Containerfile.bytecode \
 ./go-xdp-counter -t ${IMAGE_XDP_BC}

docker build \
 --build-arg PROGRAM_NAME=tc_counter \
 --build-arg BPF_FUNCTION_NAME=stats \
 --build-arg PROGRAM_TYPE=tc \
 --build-arg BYTECODE_FILENAME=bpf_bpfel.o \
 -f ../packaging/container-deployment/Containerfile.bytecode \
 ./go-tc-counter -t $IMAGE_TC_BC

docker build \
 --build-arg PROGRAM_NAME=tracepoint_counter \
 --build-arg BPF_FUNCTION_NAME=tracepoint_kill_recorder \
 --build-arg PROGRAM_TYPE=tracepoint \
 --build-arg BYTECODE_FILENAME=bpf_bpfel.o \
 -f ../packaging/container-deployment/Containerfile.bytecode \
 ./go-tracepoint-counter -t $IMAGE_TP_BC
