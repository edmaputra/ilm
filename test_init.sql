CREATE USER project_user WITH PASSWORD 'project_pwd';
CREATE USER task_user WITH PASSWORD 'task_pwd';

GRANT ALL PRIVILEGES ON DATABASE ilm TO project_user;
GRANT ALL PRIVILEGES ON DATABASE ilm TO task_user;
