-- Swap Salary
-- https://leetcode.com/problems/swap-salary/
UPDATE
    Salary
SET
    sex = CASE
        WHEN sex = 'm' THEN 'f'
        WHEN sex = 'f' THEN 'm'
        ELSE ''
    END;