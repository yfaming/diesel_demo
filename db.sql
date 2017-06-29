CREATE TABLE `post`(
  id int(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  user_id int(11) UNSIGNED NOT NULL,
  url varchar(1024) NOT NULL,
  title varchar(256) NOT NULL,
  content text NOT NULL,
  created_at int(11) UNSIGNED NOT NULL,
  updated_at int(11) UNSIGNED NOT NULL,
  is_archived tinyint(1) UNSIGNED NOT NULL DEFAULT 0,
  archived_at int(11) UNSIGNED NOT NULL DEFAULT 0,
  PRIMARY KEY (id)
  index `unarchived`(user_id, is_archived, created_at),
  index `archived`(user_id, is_archived, archived_at),
);
