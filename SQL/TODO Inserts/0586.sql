-- Customer Placing the Largest Number of Orders
-- https://leetcode.com/problems/customer-placing-the-largest-number-of-orders/
SELECT
    customer_number
FROM
    (
        SELECT
            customer_number,
            COUNT(order_number) as num_orders
        FROM
            Orders
        GROUP BY
            customer_number
        ORDER BY
            num_orders DESC
    ) as Orders
limit
    1;