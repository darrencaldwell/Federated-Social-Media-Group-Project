-- `cs3099user-b5_project`.forums definition

CREATE TABLE `forums` (
  `forum_id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `forum_name` varchar(100) COLLATE utf8_bin NOT NULL,
  PRIMARY KEY (`forum_id`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8 COLLATE=utf8_bin;


-- `cs3099user-b5_project`.users definition

CREATE TABLE `users` (
  `username` varchar(30) COLLATE utf8_bin NOT NULL,
  `password_hash` varchar(60) COLLATE utf8_bin NOT NULL,
  `user_id` binary(16) NOT NULL,
  PRIMARY KEY (`user_id`),
  UNIQUE KEY `username` (`username`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_bin;


-- `cs3099user-b5_project`.subforums definition

CREATE TABLE `subforums` (
  `subforum_id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `subforum_name` varchar(100) COLLATE utf8_bin NOT NULL,
  `forum_id` bigint(20) unsigned NOT NULL,
  PRIMARY KEY (`subforum_id`),
  KEY `subforums_FK` (`forum_id`),
  CONSTRAINT `subforums_FK` FOREIGN KEY (`forum_id`) REFERENCES `forums` (`forum_id`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8 COLLATE=utf8_bin;


-- `cs3099user-b5_project`.posts definition

CREATE TABLE `posts` (
  `post_id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `post_title` varchar(100) COLLATE utf8_bin NOT NULL,
  `post_markup` text COLLATE utf8_bin NOT NULL,
  `subforum_id` bigint(20) unsigned NOT NULL,
  `user_id` binary(16) NOT NULL,
  PRIMARY KEY (`post_id`),
  KEY `subforum_id` (`subforum_id`),
  KEY `posts_FK` (`user_id`),
  CONSTRAINT `posts_FK` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`),
  CONSTRAINT `subforum_id` FOREIGN KEY (`subforum_id`) REFERENCES `subforums` (`subforum_id`)
) ENGINE=InnoDB AUTO_INCREMENT=36 DEFAULT CHARSET=utf8 COLLATE=utf8_bin;


-- `cs3099user-b5_project`.comments definition

CREATE TABLE `comments` (
  `comment_id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `comment` text COLLATE utf8_bin NOT NULL,
  `post_id` bigint(20) unsigned NOT NULL,
  `time_submitted` time DEFAULT NULL,
  `user_id` binary(16) NOT NULL,
  PRIMARY KEY (`comment_id`),
  KEY `post_id` (`post_id`),
  KEY `comments_FK` (`user_id`),
  CONSTRAINT `comments_FK` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`),
  CONSTRAINT `post_id` FOREIGN KEY (`post_id`) REFERENCES `posts` (`post_id`)
) ENGINE=InnoDB AUTO_INCREMENT=18 DEFAULT CHARSET=utf8 COLLATE=utf8_bin;

CREATE DEFINER=`root`@`%` FUNCTION `test`.`UuidFromBin`(_bin BINARY(16)) RETURNS binary(36)
    DETERMINISTIC
    SQL SECURITY INVOKER
RETURN
        LCASE(CONCAT_WS('-',
            HEX(SUBSTR(_bin,  5, 4)),
            HEX(SUBSTR(_bin,  3, 2)),
            HEX(SUBSTR(_bin,  1, 2)),
            HEX(SUBSTR(_bin,  9, 2)),
            HEX(SUBSTR(_bin, 11))
                 ));

CREATE DEFINER=`root`@`%` FUNCTION `test`.`UuidToBin`(_uuid BINARY(36)) RETURNS binary(16)
    DETERMINISTIC
    SQL SECURITY INVOKER
RETURN
        UNHEX(CONCAT(
            SUBSTR(_uuid, 15, 4),
            SUBSTR(_uuid, 10, 4),
            SUBSTR(_uuid,  1, 8),
            SUBSTR(_uuid, 20, 4),
            SUBSTR(_uuid, 25) ));
