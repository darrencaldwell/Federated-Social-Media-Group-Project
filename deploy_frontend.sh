#!/bin/bash
cd frontend
nolimit npm ci
nolimit npm run build
rm -rf /cs/home/cs3099user-b5/nginx_defualt/build/
cp -r build/* /cs/home/cs3099user-b5/nginx_default/
