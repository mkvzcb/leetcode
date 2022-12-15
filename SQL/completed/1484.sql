-- Group Sold Products By The Date
-- https://leetcode.com/problems/group-sold-products-by-the-date/

CREATE TABLE IF NOT EXISTS Activities (sell_date date, product varchar(20));
DELETE FROM Activities;
INSERT INTO Activities (sell_date, product) VALUES ('2020-05-30', 'Headphone');
INSERT INTO Activities (sell_date, product) VALUES ('2020-06-01', 'Pencil');
INSERT INTO Activities (sell_date, product) VALUES ('2020-06-02', 'Mask');
INSERT INTO Activities (sell_date, product) VALUES ('2020-05-30', 'Basketball');
INSERT INTO Activities (sell_date, product) VALUES ('2020-06-01', 'Bible');
INSERT INTO Activities (sell_date, product) VALUES ('2020-06-02', 'Mask');
INSERT INTO Activities (sell_date, product) VALUES ('2020-05-30', 'T-Shirt');

-------

SELECT
    DISTINCT(sell_date),
    COUNT(DISTINCT product) as num_sold,
    GROUP_CONCAT(DISTINCT product) as products
FROM
    Activities
GROUP BY sell_date
ORDER BY sell_date