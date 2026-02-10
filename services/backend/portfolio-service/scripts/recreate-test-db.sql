-- Dev only - Brute Force DROP DB for local dev and unit test
SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE
usename = 'portfolio_test_user' OR datname = 'portfolio_test_db';
DROP DATABASE IF EXISTS portfolio_test_db;
DROP USER IF EXISTS portfolio_test_user;

-- Dev only - Dev only password (for local dev and unit test)
CREATE USER portfolio_test_user PASSWORD 'dev_only_pwd' CREATEDB;
CREATE DATABASE portfolio_test_db OWNER portfolio_test_user ENCODING = 'UTF-8';