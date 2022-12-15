-- Customer Who Visited but Did Not Make Any Transactions
-- https://leetcode.com/problems/customer-who-visited-but-did-not-make-any-transactions/
SELECT
    DISTINCT customer_id,
    COUNT(customer_id) as count_no_trans
FROM
    Visits
    LEFT JOIN Transactions ON Visits.visit_id = Transactions.visit_id
WHERE
    Transactions.visit_id IS NULL
GROUP BY
    customer_id
ORDER BY
    count_no_trans DESC