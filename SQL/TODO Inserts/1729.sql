-- Find Followers Count
-- https://leetcode.com/problems/find-followers-count/

SELECT
    user_id,
    count(DISTINCT follower_id) as followers_count
FROM Followers
GROUP BY user_id