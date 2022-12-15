-- Sales Person
-- https://leetcode.com/problems/sales-person/

SELECT
    t1.name
FROM
    SalesPerson as t1
WHERE
    t1.sales_id NOT IN (
        SELECT
            sales_id
        FROM
            Orders
        WHERE
            com_id IN (
                SELECT
                    com_id
                FROM
                    Company
                WHERE
                    name = 'Red'
            )
    )