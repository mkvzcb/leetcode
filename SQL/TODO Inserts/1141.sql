-- User Activity for the Past 30 Days I
-- https://leetcode.com/problems/user-activity-for-the-past-30-days-i/
SELECT
    activity_date AS day,
    COUNT(DISTINCT user_id) as active_users
FROM
    Activity
WHERE
    Activity.activity_date <= '2019-07-27'
    AND Activity.activity_date >= '2019-06-28'
GROUP BY
    day