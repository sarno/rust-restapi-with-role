-- Your SQL goes here
CREATE TABLE IF NOT EXISTS projects (
    id SERIAL  PRIMARY KEY,
    project_code VARCHAR(255),
    project_name VARCHAR(255),
    company_id INTEGER,
    created_at TIMESTAMP NULL,
    updated_at TIMESTAMP NULL
);

ALTER TABLE projects
ADD CONSTRAINT fk_projects_company_id
FOREIGN KEY (company_id)
REFERENCES companies(id)
ON DELETE CASCADE;



-- Seed data projects
INSERT INTO projects (project_code, project_name, company_id, created_at, updated_at)
SELECT 
    CONCAT('PROJ00', ROW_NUMBER() OVER(PARTITION BY c.id ORDER BY p.id)),
    CONCAT('Project ', ROW_NUMBER() OVER(PARTITION BY c.id ORDER BY p.id)),
    c.id,
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
FROM companies c
CROSS JOIN (
    SELECT GENERATE_SERIES(1, 2) AS id
) AS p;