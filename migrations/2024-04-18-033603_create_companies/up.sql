-- Your SQL goes here
CREATE TABLE IF NOT EXISTS companies (
    id SERIAL  PRIMARY KEY ,
    company_code VARCHAR(255),
    company_name VARCHAR(255),
    photo VARCHAR(255),
    address VARCHAR(255),
    latitude DOUBLE PRECISION,
    longitude DOUBLE PRECISION,
    status VARCHAR(255),
    created_at TIMESTAMP NULL,
    updated_at TIMESTAMP NULL
);

-- Seed data companies
INSERT INTO companies (company_code, company_name, photo, address, latitude, longitude, status, created_at, updated_at)
VALUES
    ('COMP001', 'Company 1', NULL, 'Address 1', 12.345, 67.890, 'Active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('COMP002', 'Company 2', NULL, 'Address 2', 23.456, 78.901, 'Active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('COMP003', 'Company 3', NULL, 'Address 3', 34.567, 89.012, 'Active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('COMP004', 'Company 4', NULL, 'Address 4', 45.678, 90.123, 'Active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('COMP005', 'Company 5', NULL, 'Address 5', 56.789, 01.234, 'Active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('COMP006', 'Company 6', NULL, 'Address 6', 67.890, 12.345, 'Active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('COMP007', 'Company 7', NULL, 'Address 7', 78.901, 23.456, 'Active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('COMP008', 'Company 8', NULL, 'Address 8', 89.012, 34.567, 'Active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('COMP009', 'Company 9', NULL, 'Address 9', 90.123, 45.678, 'Active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('COMP010', 'Company 10', NULL, 'Address 10', 01.234, 56.789, 'Active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);