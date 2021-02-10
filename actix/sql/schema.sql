-- `cs3099user-b5_project`.casbin_rule definition

CREATE TABLE `casbin_rule` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `ptype` varchar(12) NOT NULL,
  `v0` varchar(128) NOT NULL,
  `v1` varchar(128) NOT NULL,
  `v2` varchar(128) NOT NULL,
  `v3` varchar(128) NOT NULL,
  `v4` varchar(128) NOT NULL,
  `v5` varchar(128) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `unique_key_sqlx_adapter` (`ptype`,`v0`,`v1`,`v2`,`v3`,`v4`,`v5`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;


-- `cs3099user-b5_project`.forums definition

CREATE TABLE `forums` (
  `forum_id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `forum_name` varchar(100) COLLATE utf8_bin NOT NULL,
  PRIMARY KEY (`forum_id`)
) ENGINE=InnoDB AUTO_INCREMENT=12 DEFAULT CHARSET=utf8 COLLATE=utf8_bin;


-- `cs3099user-b5_project`.users definition

CREATE TABLE `users` (
  `username` varchar(30) COLLATE utf8_bin NOT NULL,
  `password_hash` varchar(60) COLLATE utf8_bin DEFAULT NULL,
  `user_id` binary(16) NOT NULL,
  `server` varchar(100) COLLATE utf8_bin DEFAULT NULL,
  `description` text COLLATE utf8_bin NOT NULL DEFAULT '',
  `email` varchar(255) COLLATE utf8_bin DEFAULT NULL,
  `first_name` varchar(255) COLLATE utf8_bin DEFAULT NULL,
  `last_name` varchar(255) COLLATE utf8_bin DEFAULT NULL,
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
) ENGINE=InnoDB AUTO_INCREMENT=10 DEFAULT CHARSET=utf8 COLLATE=utf8_bin;


-- `cs3099user-b5_project`.posts definition

CREATE TABLE `posts` (
  `post_id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `post_title` varchar(100) CHARACTER SET utf8 COLLATE utf8_bin NOT NULL,
  `post_contents` text CHARACTER SET utf8 COLLATE utf8_bin NOT NULL,
  `subforum_id` bigint(20) unsigned NOT NULL,
  `user_id` binary(16) NOT NULL,
  PRIMARY KEY (`post_id`),
  KEY `subforum_id` (`subforum_id`),
  KEY `posts_FK` (`user_id`),
  CONSTRAINT `posts_FK` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`),
  CONSTRAINT `subforum_id` FOREIGN KEY (`subforum_id`) REFERENCES `subforums` (`subforum_id`)
) ENGINE=InnoDB AUTO_INCREMENT=49 DEFAULT CHARSET=utf8 COLLATE=utf8_croatian_ci;


-- `cs3099user-b5_project`.comments definition

CREATE TABLE `comments` (
  `comment_id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `comment` text COLLATE utf8_bin NOT NULL,
  `post_id` bigint(20) unsigned NOT NULL,
  `time_submitted` time DEFAULT NULL,
  `user_id` binary(16) NOT NULL,
<<<<<<< HEAD
  `parent_id` bigint(20) unsigned,
=======
  `parent_id` bigint(20) unsigned DEFAULT NULL,
>>>>>>> 9c55da2 (Fix post get)
  PRIMARY KEY (`comment_id`),
  KEY `post_id` (`post_id`),
  KEY `comments_FK` (`user_id`),
  KEY `parent_id` (`parent_id`),
  CONSTRAINT `comments_FK` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`),
<<<<<<< HEAD
  CONSTRAINT `post_id` FOREIGN KEY (`post_id`) REFERENCES `posts` (`post_id`),
  CONSTRAINT `parent_id` FOREIGN KEY (`parent_id`) REFERENCES `comments` (`comment_id`)
) ENGINE=InnoDB AUTO_INCREMENT=18 DEFAULT CHARSET=utf8 COLLATE=utf8_bin;
=======
  CONSTRAINT `parent_id` FOREIGN KEY (`parent_id`) REFERENCES `comments` (`comment_id`),
  CONSTRAINT `post_id` FOREIGN KEY (`post_id`) REFERENCES `posts` (`post_id`)
) ENGINE=InnoDB AUTO_INCREMENT=23 DEFAULT CHARSET=utf8 COLLATE=utf8_bin;
>>>>>>> 9c55da2 (Fix post get)

CREATE DEFINER=`root`@`%` FUNCTION `cs3099user-b5_project`.`UuidFromBin`(_bin BINARY(16)) RETURNS binary(36)
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

CREATE DEFINER=`root`@`%` FUNCTION `cs3099user-b5_project`.`UuidToBin`(_uuid BINARY(36)) RETURNS binary(16)
    DETERMINISTIC
    SQL SECURITY INVOKER
RETURN
        UNHEX(CONCAT(
            SUBSTR(_uuid, 15, 4),
            SUBSTR(_uuid, 10, 4),
            SUBSTR(_uuid,  1, 8),
            SUBSTR(_uuid, 20, 4),
            SUBSTR(_uuid, 25) ));
