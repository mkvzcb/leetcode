-- Calculate Special Bonus
-- https://leetcode.com/problems/calculate-special-bonus/
SELECT
    employee_id,
    CASE
        WHEN employee_id % 2 <> 0
        AND name NOT LIKE 'M%' THEN salary
        ELSE 0
    END AS bonus
FROM
    Employees
ORDER BY
    employee_id ASC;