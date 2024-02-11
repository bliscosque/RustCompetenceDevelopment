# run in linux (or WSL)
# 1st time
sudo docker run --name catdex-db -e POSTGRES_PASSWORD=mypassword -p 5432:5432 -d postgres:12.3-alpine

# testing
# docker ps
# sudo apt-get install postgresql-client
# psql -h localhost -p 5432 --username=postgres --password=mypassword
# sudo apt install libpq-dev

# 2nd time ahead
# docker start catdex-db

echo "Run this to set the databse URL:"
echo "export DATABASE_URL=postgres://postgres:mypassword@localhost"

