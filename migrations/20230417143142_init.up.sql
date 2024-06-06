-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS tikets (
        id BIGINT PRIMARY KEY NOT NULL AUTO_INCREMENT,
        -- title VARCHAR(255) NOT NULL UNIQUE,
        summary TEXT NOT NULL,
        priority VARCHAR(255) NOT NULL DEFAULT 'low',
        status VARCHAR(255) NOT NULL DEFAULT 'created',
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
    );


comments | CREATE TABLE `comments` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `content` varchar(255) NOT NULL,
  `create_date` timestamp NULL DEFAULT CURRENT_TIMEST
  `ticket_id` bigint NOT NULL,
  PRIMARY KEY (`id`),
  KEY `ticket_id` (`ticket_id`),
  CONSTRAINT `comments_ibfk_1` FOREIGN KEY (`ticket_id
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8`

-- this is a query to get comments from ticket ID
SELECT ticket.id, 
        comments.id, 
        comments.content, 
        comments.create_date 
FROM ticket 
JOIN comments ON ticket.id = comments.ticket_id 
WHERE ticket.id = 81 ORDER BY comments.create_date ASC;


-- show a table 
show create table <tableName>;