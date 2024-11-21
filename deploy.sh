docker build --no-cache -t etbcor/personal-site:latest . && \
docker push etbcor/personal-site:latest && \
fly deploy
