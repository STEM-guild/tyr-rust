FROM postgres:bookworm
ADD ../../migrations/2024_03_04_06_00_00_initial_setup.sql /docker-entrypoint-initdb.d
RUN chmod a+r /docker-entrypoint-initdb.d/*
EXPOSE 6666