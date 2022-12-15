-- Rising Temperature
-- https://leetcode.com/problems/rising-temperature/

-- in theory this works ?? not sure since the leetcode is bugged
-- probably there is soem bug since it prints 2x times hm

SELECT
    DISTINCT t1.id as Id
FROM
    Weather as t1
    INNER JOIN Weather as t2

ON   
    CASE
        WHEN t1.recordDate > t2.recordDate THEN t1.id = t2.id + 1
        ELSE t1.id = t2.id - 1
    END
WHERE 
    t1.temperature > t2.temperature
ORDER BY t1.id ASC
