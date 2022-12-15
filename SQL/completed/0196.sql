-- Delete Duplicate Emails
-- https://leetcode.com/problems/delete-duplicate-emails/

CREATE TABLE IF NOT EXISTS Person (Id int, Email varchar(255));
DELETE FROM Person;
INSERT INTO Person (id, email) VALUES ('1', 'john@example.com');
INSERT INTO Person (id, email) VALUES ('2', 'bob@example.com');
INSERT INTO Person (id, email) VALUES ('3', 'john@example.com');

--

DELETE person
FROM
    Person as person
    INNER JOIN Person as temp
WHERE
    person.id > temp.id AND person.email = temp.email;