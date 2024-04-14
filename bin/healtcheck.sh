#!/bin/bash

for attempt in {1..20}; do sleep 1; if curl 127.0.0.1:5005/healthcheck; then echo ready; break; fi; echo waiting...; done