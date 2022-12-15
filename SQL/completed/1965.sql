-- Employees With Missing Information
-- https://leetcode.com/problems/employees-with-missing-information/

CREATE TABLE IF NOT EXISTS Employees (employee_id int, name varchar(30));
CREATE TABLE IF NOT EXISTS Salaries (employee_id int, salary int);
DELETE FROM Employees;
INSERT INTO Employees (employee_id, name) VALUES ('2', 'Crew');
INSERT INTO Employees (employee_id, name) VALUES ('4', 'Haven');
INSERT INTO Employees (employee_id, name) VALUES ('5', 'Kristian');
DELETE FROM Salaries;
INSERT INTO Salaries (employee_id, salary) VALUES ('5', '76071');
INSERT INTO Salaries (employee_id, salary) VALUES ('1', '22517');
INSERT INTO Salaries (employee_id, salary) VALUES ('4', '63539');

--

SELECT
    COALESCE(Employees.employee_id, Salaries.employee_id) as employee_id
FROM
    Employees
    LEFT JOIN Salaries ON Employees.employee_id = Salaries.employee_id
WHERE
    Salaries.employee_id IS NULL
    
UNION

SELECT
    COALESCE(Employees.employee_id, Salaries.employee_id) as employee_id
FROM
    Employees
    RIGHT JOIN Salaries ON Employees.employee_id = Salaries.employee_id
WHERE
    Employees.employee_id IS NULL
ORDER BY
    employee_id ASC;