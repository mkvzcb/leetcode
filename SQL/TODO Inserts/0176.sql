-- 176

WITH second_highest_salary AS (
    SELECT
        DISTINCT salary as SecondHighestSalary
    FROM
        Employee
    ORDER BY
        salary DESC
    LIMIT
        1 OFFSET 1
)
SELECT
    CASE
        WHEN EXISTS (SELECT * FROM second_highest_salary) THEN (SELECT SecondHighestSalary FROM second_highest_salary)
        ELSE NULL
    END as SecondHighestSalary; 