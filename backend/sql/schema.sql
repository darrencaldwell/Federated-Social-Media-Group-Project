-- `cs3099user-b5_project`.casbin_rules definition

CREATE TABLE `casbin_rules` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `ptype` varchar(12) CHARACTER SET utf8 NOT NULL,
  `v0` varchar(128) CHARACTER SET utf8 NOT NULL,
  `v1` varchar(128) CHARACTER SET utf8 NOT NULL,
  `v2` varchar(128) CHARACTER SET utf8 NOT NULL,
  `v3` varchar(128) CHARACTER SET utf8 NOT NULL,
  `v4` varchar(128) CHARACTER SET utf8 NOT NULL,
  `v5` varchar(128) CHARACTER SET utf8 NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `unique_key_sqlx_adapter` (`ptype`,`v0`,`v1`,`v2`,`v3`,`v4`,`v5`)
) ENGINE=InnoDB AUTO_INCREMENT=28 DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;


-- `cs3099user-b5_project`.forums definition

CREATE TABLE `forums` (
  `forum_id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `forum_name` varchar(100) COLLATE utf8_unicode_ci NOT NULL,
  PRIMARY KEY (`forum_id`)
) ENGINE=InnoDB AUTO_INCREMENT=33 DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;


-- `cs3099user-b5_project`.implementations definition

CREATE TABLE `implementations` (
  `implementation_id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `implementation_url` varchar(500) COLLATE utf8mb4_unicode_ci NOT NULL,
  `implementation_name` varchar(100) COLLATE utf8mb4_unicode_ci NOT NULL,
  PRIMARY KEY (`implementation_id`)
) ENGINE=InnoDB AUTO_INCREMENT=18 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;


-- `cs3099user-b5_project`.subforums definition

CREATE TABLE `subforums` (
  `subforum_id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `subforum_name` varchar(100) COLLATE utf8_unicode_ci NOT NULL,
  `forum_id` bigint(20) unsigned NOT NULL,
  PRIMARY KEY (`subforum_id`),
  KEY `subforums_FK` (`forum_id`),
  CONSTRAINT `subforums_FK` FOREIGN KEY (`forum_id`) REFERENCES `forums` (`forum_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=33 DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;


-- `cs3099user-b5_project`.users definition

CREATE TABLE `users` (
  `username` varchar(30) COLLATE utf8_unicode_ci DEFAULT 'UNKOWN',
  `password_hash` varchar(60) COLLATE utf8_unicode_ci DEFAULT NULL,
  `user_id` varchar(36) COLLATE utf8_unicode_ci NOT NULL,
  `description` text COLLATE utf8_unicode_ci NOT NULL DEFAULT '',
  `email` varchar(255) COLLATE utf8_unicode_ci DEFAULT NULL,
  `first_name` varchar(255) COLLATE utf8_unicode_ci DEFAULT NULL,
  `last_name` varchar(255) COLLATE utf8_unicode_ci DEFAULT NULL,
  `date_joined` timestamp NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp(),
  `implementation_id` bigint(20) unsigned NOT NULL,
  `profile_picture` longblob DEFAULT NULL,
  PRIMARY KEY (`user_id`,`implementation_id`),
  KEY `users_FK` (`implementation_id`),
  CONSTRAINT `users_FK` FOREIGN KEY (`implementation_id`) REFERENCES `implementations` (`implementation_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;


-- `cs3099user-b5_project`.posts definition

CREATE TABLE `posts` (
  `post_id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `post_title` varchar(100) CHARACTER SET utf8 COLLATE utf8_unicode_ci NOT NULL,
  `post_contents` longtext NOT NULL,
  `subforum_id` bigint(20) unsigned NOT NULL,
  `created_time` datetime DEFAULT current_timestamp(),
  `modified_time` datetime DEFAULT current_timestamp(),
  `user_id` varchar(36) CHARACTER SET utf8 COLLATE utf8_unicode_ci NOT NULL,
  `implementation_id` bigint(20) unsigned NOT NULL,
  PRIMARY KEY (`post_id`),
  KEY `subforum_id` (`subforum_id`),
  KEY `posts_FK_1` (`user_id`,`implementation_id`),
  CONSTRAINT `posts_FK` FOREIGN KEY (`subforum_id`) REFERENCES `subforums` (`subforum_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `posts_FK_1` FOREIGN KEY (`user_id`, `implementation_id`) REFERENCES `users` (`user_id`, `implementation_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=110 DEFAULT CHARSET=utf8mb4;


-- `cs3099user-b5_project`.posts_votes definition

CREATE TABLE `posts_votes` (
  `post_id` bigint(20) unsigned NOT NULL,
  `user_id` varchar(36) CHARACTER SET utf8 COLLATE utf8_unicode_ci NOT NULL,
  `is_upvote` tinyint(1) NOT NULL,
  `implementation_id` bigint(20) unsigned NOT NULL,
  PRIMARY KEY (`implementation_id`,`user_id`,`post_id`),
  KEY `posts_votes_FK` (`post_id`),
  KEY `posts_votes_FK_1` (`user_id`,`implementation_id`),
  CONSTRAINT `posts_votes_FK` FOREIGN KEY (`post_id`) REFERENCES `posts` (`post_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `posts_votes_FK_1` FOREIGN KEY (`user_id`, `implementation_id`) REFERENCES `users` (`user_id`, `implementation_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;


-- `cs3099user-b5_project`.comments definition

CREATE TABLE `comments` (
  `comment_id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `comment` text CHARACTER SET utf8 COLLATE utf8_bin NOT NULL,
  `post_id` bigint(20) unsigned NOT NULL,
  `time_submitted` time DEFAULT NULL,
  `user_id` varchar(36) COLLATE utf8_unicode_ci NOT NULL,
  `parent_id` bigint(20) unsigned DEFAULT NULL,
  `created_time` datetime DEFAULT current_timestamp(),
  `modified_time` datetime DEFAULT current_timestamp(),
  `implementation_id` bigint(20) unsigned NOT NULL,
  PRIMARY KEY (`comment_id`),
  KEY `post_id` (`post_id`),
  KEY `comments_FK` (`user_id`),
  KEY `parent_id` (`parent_id`),
  KEY `comments_FK_2` (`user_id`,`implementation_id`),
  CONSTRAINT `comments_FK` FOREIGN KEY (`post_id`) REFERENCES `posts` (`post_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `comments_FK_1` FOREIGN KEY (`parent_id`) REFERENCES `comments` (`comment_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `comments_FK_2` FOREIGN KEY (`user_id`, `implementation_id`) REFERENCES `users` (`user_id`, `implementation_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=126 DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;


-- `cs3099user-b5_project`.comments_votes definition

CREATE TABLE `comments_votes` (
  `user_id` varchar(36) COLLATE utf8_unicode_ci NOT NULL,
  `implementation_id` bigint(20) unsigned NOT NULL,
  `comment_id` bigint(20) unsigned NOT NULL,
  `is_upvote` tinyint(1) NOT NULL,
  PRIMARY KEY (`user_id`,`implementation_id`,`comment_id`),
  KEY `comments_votes_FK` (`comment_id`),
  CONSTRAINT `comments_votes_FK` FOREIGN KEY (`comment_id`) REFERENCES `comments` (`comment_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `comments_votes_FK_1` FOREIGN KEY (`user_id`, `implementation_id`) REFERENCES `users` (`user_id`, `implementation_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
