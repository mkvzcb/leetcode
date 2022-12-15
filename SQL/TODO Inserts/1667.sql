-- Fix Names in a Table
-- https://leetcode.com/problems/fix-names-in-a-table/
SELECT
    user_id,
    CONCAT(
        UPPER(SUBSTRING(name, 1, 1)),
        LOWER(SUBSTRING(name, 2, LENGTH(name)))
    ) as name
FROM
    Users
ORDER BY
    user_id ASC;