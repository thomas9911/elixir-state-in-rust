#! /bin/bash

MIX_ENV=bench mix run bench/new.exs
MIX_ENV=bench mix run bench/put.exs
MIX_ENV=bench mix run bench/get.exs

