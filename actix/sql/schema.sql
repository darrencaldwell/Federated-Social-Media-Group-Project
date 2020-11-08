-- `cs3099user-b5_project`.forums definition

CREATE TABLE `forums` (
      `forum_id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
      `forum_name` varchar(100) COLLATE utf8_bin NOT NULL,
      PRIMARY KEY (`forum_id`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8 COLLATE=utf8_bin;


-- `cs3099user-b5_project`.users definition

CREATE TABLE `users` (
      `user_id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
      `username` varchar(30) COLLATE utf8_bin NOT NULL,
      UNIQUE KEY `user_id` (`user_id`),
      UNIQUE KEY `username` (`username`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8 COLLATE=utf8_bin;


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
      `user_id` bigint(20) unsigned NOT NULL,
      `post_markup` text COLLATE utf8_bin NOT NULL,
      `subforum_id` bigint(20) unsigned NOT NULL,
      PRIMARY KEY (`post_id`),
      KEY `subforum_id` (`subforum_id`),
      KEY `users_id` (`user_id`),
      CONSTRAINT `subforum_id` FOREIGN KEY (`subforum_id`) REFERENCES `subforums` (`subforum_id`),
      CONSTRAINT `users_id` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`)
) ENGINE=InnoDB AUTO_INCREMENT=7 DEFAULT CHARSET=utf8 COLLATE=utf8_bin;


-- `cs3099user-b5_project`.comments definition

CREATE TABLE `comments` (
      `comment_id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
      `comment` text COLLATE utf8_bin NOT NULL,
      `user_id` bigint(20) unsigned NOT NULL,
      `post_id` bigint(20) unsigned NOT NULL,
      `time_submitted` time DEFAULT NULL,
      PRIMARY KEY (`comment_id`),
      KEY `post_id` (`post_id`),
      KEY `user_id` (`user_id`),
      CONSTRAINT `post_id` FOREIGN KEY (`post_id`) REFERENCES `posts` (`post_id`),
      CONSTRAINT `user_id` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_bin;
